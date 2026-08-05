import sys
import os
import dataclasses
import json
import tkinter as tk
from pathlib import Path
from tkinter import filedialog, messagebox, ttk, simpledialog
from typing import Optional
from PIL import Image, ImageTk

from defaults import (
    DEFAULT_FORMATS,
    DEFAULT_GLOBAL_SETTINGS,
    DEFAULT_STICK_TYPES,
    DEFAULT_WEIGHTS,
)
from export import export_solution_details_csv, export_solution_summary_csv
from models import GlobalSettings, Weights, Solution, StickType, Format
from optimizer import optimize
from project_io import serialize_project, deserialize_project

from gui.forms import (
    CARTONER_FIELDS,
    build_cartoner_settings_form,
    build_grouped_global_settings_form,
    set_entries_from_dataclass,
)

from gui.parsing import (
    parse_formats,
    parse_global_settings,
    parse_stick_types,
)
from gui.results import (
    FILTER_OPERATORS,
    FILTERABLE_RESULT_COLUMNS,
    build_detail_section,
    build_results_section,
    clear_solution_details,
    clear_tree,
    open_format_detail_popup,
    populate_results,
    populate_solution_details,
    result_display_value,
    update_result_headings_for_filters,
)
from gui.tables import HierarchicalInputTable

SCORE_PENALTY_DETAIL_COLUMNS = {
    "score",
    "layer_penalty",
    "carryover_penalty",
    "grouping_penalty",
    "stability_penalty",
    "carton_ab_penalty",
}

SCORE_PENALTY_SUMMARY_FIELDS = {
    "score",
    "total_layer_penalty",
    "total_carryover_penalty",
    "total_grouping_penalty",
    "total_stability_width_penalty",
    "total_carton_ab_ratio_penalty",
}


def resource_path(relative_path):
    try:
        base_path = sys._MEIPASS
    except AttributeError:
        base_path = Path(__file__).resolve().parent.parent

    return str(Path(base_path) / relative_path)


class OptimizerApp(tk.Tk):
    """Tkinter GUI orchestrator.

    This class only coordinates widgets, parsing, optimization calls and export.
    Engineering formulas stay in optimizer.py and scoring.py.
    """

    def __init__(self):
        super().__init__()

        self.title("Calcolatore abbinamenti stick")
        self.home_frame: Optional[ttk.Frame] = None
        self.main_container: Optional[ttk.Frame] = None
        self.iconbitmap(resource_path("stick_optimizer_logo.ico"))
        self.geometry("1450x900")
        self.minsize(1200, 720)

        self._configure_styles()

        # Ensure container structure exists before loading UI
        self.main_container = None
        self.home_frame = None

        self.project_root = Path(__file__).resolve().parent.parent
        self.user_defaults_path = self.project_root / "user_defaults.json"
        self.current_project_path: Optional[Path] = None
        self.current_project_name: str = ""

        self.solutions: list[Solution] = []
        self.candidates_by_format = {}
        self.selected_solution_index: Optional[int] = None

        self.active_result_filters = {}
        self.filtered_solution_indices = []

        self.global_entries = {}
        self.cartoner_entries = {}
        for field_name in CARTONER_FIELDS:
            entry = ttk.Entry(self)
            entry.insert(0, str(getattr(DEFAULT_GLOBAL_SETTINGS, field_name)))
            self.cartoner_entries[field_name] = entry
        self.current_weights = DEFAULT_WEIGHTS

        self.current_number_of_results_to_show = (
            DEFAULT_GLOBAL_SETTINGS.number_of_results_to_show
        )
        self.current_carton_AB_target = DEFAULT_GLOBAL_SETTINGS.carton_AB_target
        self.current_max_pitch_shift_mm = DEFAULT_GLOBAL_SETTINGS.max_pitch_shift_mm

        self.status_var = tk.StringVar(value="Ready")
        self.project_name_var = tk.StringVar(value="")

        self.mt_image = self._load_ui_image("dati_mt")
        self.cartoner_image = self._load_ui_image("dati_astucciatrice")
        self.stick_types_image = self._load_ui_image("stick_dim")

        self.show_score_penalty_details = tk.BooleanVar(value=False)

        self._build_menu_bar()
        self._build_layout()
        
        self._load_defaults()

        self._show_home_screen()

    def _load_ui_image(self, base_name):
        img_dir = Path(resource_path("img"))

        for extension in ("png", "jpg", "jpeg", "gif"):
            image_path = img_dir / f"{base_name}.{extension}"

            if image_path.exists():
                try:
                    img = Image.open(image_path)

                    if base_name == "dati_mt":
                        new_width = 330
                    elif base_name == "dati_astucciatrice":
                        new_width = 450
                    elif base_name == "stick_dim":
                        new_width = 150
                    else:
                        new_width = 300

                    ratio = new_width / img.width
                    new_height = int(img.height * ratio)

                    img = img.resize(
                        (new_width, new_height),
                        Image.Resampling.LANCZOS
                    )

                    return ImageTk.PhotoImage(img)

                except Exception as e:
                    print("ERROR:", e)

        return None

    def _configure_styles(self) -> None:
        """Centralized modern industrial/engineering style configuration."""
        style = ttk.Style()
        style.theme_use("clam")

        # Fonts
        default_font = ("Segoe UI", 10)
        bold_font = ("Segoe UI", 11, "bold")
        small_font = ("Segoe UI", 9)

        # Colors
        bg_main = "#ffffff"
        bg_sidebar = "#f3f4f6"
        bg_card = "#ffffff"
        fg_text = "#1f2937"
        primary_red = "#dc2626"
        primary_red_active = "#b91c1c"

        self.configure(bg=bg_main)

        style.configure(".", font=default_font, background=bg_main, foreground=fg_text)
        style.configure("TFrame", background=bg_main)
        style.configure("Sidebar.TFrame", background=bg_sidebar)
        style.configure("Card.TFrame", background=bg_card, relief="flat")
        style.configure("TLabel", background=bg_main, foreground=fg_text)
        style.configure("Sidebar.TLabel", background=bg_sidebar, foreground=fg_text)
        style.configure("Card.TLabel", background=bg_card, foreground=fg_text)
        style.configure("Header.TLabel", font=bold_font, foreground="#111827", background=bg_main)
        style.configure("SidebarHeader.TLabel", font=bold_font, foreground="#111827", background=bg_sidebar)

        # Primary Action Button ("Calcola")
        style.configure(
            "Primary.TButton",
            font=("Segoe UI", 11, "bold"),
            background=primary_red,
            foreground="#ffffff",
            padding=(20, 8),
            borderwidth=0,
            focusthickness=0,
        )
        style.map(
            "Primary.TButton",
            background=[("active", primary_red_active), ("disabled", "#9ca3af")],
            foreground=[("disabled", "#f3f4f6")],
        )

        # Standard Buttons
        style.configure("TButton", font=default_font, padding=(10, 5), borderwidth=1)

        # Entry fields
        style.configure("TEntry", fieldbackground="#ffffff", padding=4)
        style.configure("Yellow.TEntry", fieldbackground="#fef08a")

        # Treeview
        style.configure(
            "Treeview",
            font=default_font,
            rowheight=26,
            fieldbackground="#ffffff",
            background="#ffffff",
            borderColor="#e5e7eb",
        )
        style.configure(
            "Treeview.Heading",
            font=("Segoe UI", 9, "bold"),
            background="#f3f4f6",
            foreground="#374151",
            padding=(4, 6),
            borderwidth=0,
            relief="flat",
        )
        style.map(
            "Treeview.Heading",
            background=[("active", "#f3f4f6"), ("pressed", "#e5e7eb")],
            relief=[("active", "flat"), ("pressed", "flat")],
        )
        style.map("Treeview", background=[("selected", "#fee2e2")], foreground=[("selected", "#991b1b")])

        # Notebook / Tabs
        style.configure("TNotebook", background=bg_main, tabmargins=[2, 5, 2, 0])
        style.configure(
            "TNotebook.Tab",
            font=("Segoe UI", 10, "bold"),
            padding=(14, 6),
            background="#f3f4f6",
            foreground="#4b5563",
        )
        style.map(
            "TNotebook.Tab",
            background=[("selected", "#ffffff")],
            foreground=[("selected", primary_red)],
        )

    # ------------------------------------------------------------------
    # Menu bar
    # ------------------------------------------------------------------
    def _build_menu_bar(self) -> None:
        """Build application menu bar."""
        self._update_menu_bar(is_homepage=True)

    def _update_menu_bar(self, is_homepage: bool = False) -> None:
        """Update menu bar items based on whether homepage is active."""
        menu_bar = tk.Menu(self)

        # File menu
        file_menu = tk.Menu(menu_bar, tearoff=False)
        file_menu.add_command(label="Nuovo progetto", command=self.new_project)
        file_menu.add_command(label="Apri progetto...", command=self.open_project)

        if not is_homepage:
            file_menu.add_separator()
            file_menu.add_command(label="Salva progetto", command=self.save_project)
            file_menu.add_command(label="Salva progetto con nome...", command=self.save_project_as)
            file_menu.add_separator()

            export_menu = tk.Menu(file_menu, tearoff=False)
            export_menu.add_command(
                label="CSV riepilogo",
                command=self.export_summary,
            )
            export_menu.add_command(
                label="CSV dettagli selezionati",
                command=self.export_details,
            )

            file_menu.add_cascade(label="Esporta", menu=export_menu)

        menu_bar.add_cascade(label="File", menu=file_menu)

        # Options menu (only visible when not on homepage)
        if not is_homepage:
            options_menu = tk.Menu(menu_bar, tearoff=False)

            options_menu.add_command(
                label="Rimuovi filtri risultati",
                command=lambda: self._clear_all_result_filters(None),
            )

            options_menu.add_separator()

            options_menu.add_checkbutton(
                label="Mostra dettagli punteggio e penalità",
                variable=self.show_score_penalty_details,
                command=self._update_score_penalty_columns_visibility,
            )

            menu_bar.add_cascade(label="Opzioni", menu=options_menu)

        # Edit menu
        edit_menu = tk.Menu(menu_bar, tearoff=False)

        edit_menu.add_command(
            label="Pesi di calcolo",
            command=self.open_weights_editor,
        )

        edit_menu.add_command(
            label="Numero di risultati",
            command=self.open_number_of_results_editor,
        )

        edit_menu.add_command(
            label="Obiettivo A/B astuccio",
            command=self.open_carton_ab_target_editor,
        )

        edit_menu.add_command(
            label="Dati astucciatrice",
            command=self.open_cartoner_settings_editor,
        )

        edit_menu.add_command(
            label="Dati aggiuntivi MT",
            command=self.open_mt_extra_settings_editor,
        )

        menu_bar.add_cascade(label="Modifica", menu=edit_menu)
        self.config(menu=menu_bar)

    # ------------------------------------------------------------------
    # Layout
    # ------------------------------------------------------------------
    def _show_home_screen(self):
        """Show the initial home screen."""
        if self.main_container:
            self.main_container.pack_forget()
        
        if self.home_frame:
            self.home_frame.destroy()

        self._update_menu_bar(is_homepage=True)

        self.home_frame = ttk.Frame(self)
        self.home_frame.pack(expand=True, fill="both")
        
        inner = ttk.Frame(self.home_frame)
        inner.place(relx=0.5, rely=0.5, anchor="center")
        
        ttk.Label(inner, text="Calcolatore abbinamenti stick", font=("TkDefaultFont", 24, "bold")).pack(pady=(0, 40))
        
        btn_frame = ttk.Frame(inner)
        btn_frame.pack()
        
        ttk.Button(btn_frame, text="Nuovo Progetto", width=25, command=self.new_project).pack(pady=10)
        ttk.Button(btn_frame, text="Apri Progetto", width=25, command=self.open_project).pack(pady=10)

    def _build_layout(self) -> None:
        """Build main window layout (hidden initially)."""
        self.main_container = ttk.Frame(self, padding=0)
        root = self.main_container

        # Main splitter layout
        main_pane = tk.PanedWindow(root, orient="horizontal")
        main_pane.pack(fill="both", expand=True)

        left_pane = ttk.Frame(main_pane, style="Sidebar.TFrame", padding=12)
        right_pane = ttk.Frame(main_pane, padding=(12, 12, 12, 0))

        main_pane.add(left_pane, minsize=150, stretch="never")
        main_pane.add(right_pane, stretch="always")

        right_toolbar = ttk.Frame(right_pane)
        right_toolbar.pack(fill="x", pady=(0, 12))

        self.run_button = ttk.Button(
            right_toolbar,
            text="Calcola",
            style="Primary.TButton",
            command=self.run_optimization,
        )
        self.run_button.pack(side="right")

        # Left Sidebar (Inputs)
        global_frame, self.global_entries = build_grouped_global_settings_form(
            left_pane,
            entry_width=14,
            mt_image=self.mt_image,
        )
        global_frame.pack(fill="x", pady=(0, 12))

        self.input_table = HierarchicalInputTable(
            left_pane,
            title="Formati",
            header_image=self.stick_types_image,
        )
        self.input_table.pack(fill="both", expand=True)

        # Right Main Area (Results & Solution Details)
        self._build_output_tables(right_pane)


    def _build_output_tables(self, parent: ttk.Frame) -> None:
        """Build results and detail output sections."""
        container = ttk.Frame(parent)
        container.pack(fill="both", expand=True)

        container.rowconfigure(0, weight=5)
        container.rowconfigure(1, weight=15)
        container.columnconfigure(0, weight=1)

        results_frame, self.results_tree = build_results_section(
            container,
            self._on_solution_selected,
            self._open_result_column_filter,
        )
        results_frame.grid(row=0, column=0, sticky="nsew", pady=(0, 8))

        detail_frame, self.detail_widgets = build_detail_section(
            container,
            self._open_selected_format_popup,
        )
        detail_frame.grid(row=1, column=0, sticky="nsew")
        
        self._update_score_penalty_columns_visibility()
        
    def _update_score_penalty_columns_visibility(self) -> None:
        """Show or hide score and penalty details in results and summary."""
        if not hasattr(self, "results_tree"):
            return

        show_details = self.show_score_penalty_details.get()

        # -------------------------------------------------------------
        # Top-solutions Treeview
        # -------------------------------------------------------------
        all_columns = list(self.results_tree["columns"])

        if show_details:
            visible_columns = all_columns
        else:
            visible_columns = [
                column
                for column in all_columns
                if column not in SCORE_PENALTY_DETAIL_COLUMNS
            ]

        self.results_tree.configure(
            displaycolumns=visible_columns
        )

        # -------------------------------------------------------------
        # Format overview Treeview
        # -------------------------------------------------------------
        if hasattr(self, "detail_widgets") and "format_overview_tree" in self.detail_widgets:
            overview_tree = self.detail_widgets["format_overview_tree"]
            overview_all_cols = list(overview_tree["columns"])
            if show_details:
                overview_visible = overview_all_cols
            else:
                overview_visible = [c for c in overview_all_cols if c != "carton_ab_penalty"]
            overview_tree.configure(displaycolumns=overview_visible)

        # -------------------------------------------------------------
        # Selected-solution summary
        # -------------------------------------------------------------
        if not hasattr(self, "detail_widgets"):
            return

        summary_vars = self.detail_widgets["summary_vars"]
        summary_name_labels = self.detail_widgets["summary_name_labels"]
        summary_fields_order = self.detail_widgets["summary_fields_order"]

        if show_details:
            visible_summary_fields = list(summary_fields_order)
        else:
            visible_summary_fields = [
                field_name
                for field_name in summary_fields_order
                if field_name not in SCORE_PENALTY_SUMMARY_FIELDS
            ]

        # Nasconde prima tutti i campi.
        for field_name in summary_fields_order:
            summary_name_labels[field_name].grid_remove()
            summary_vars[field_name].grid_remove()

        # Ricolloca i campi visibili senza lasciare spazi vuoti.
        for index, field_name in enumerate(visible_summary_fields):
            row = index // 4
            base_col = (index % 4) * 2

            summary_name_labels[field_name].grid(
                row=row,
                column=base_col,
                sticky="w",
                padx=(0, 4),
                pady=2,
            )

            summary_vars[field_name].grid(
                row=row,
                column=base_col + 1,
                sticky="w",
                padx=(0, 12),
                pady=2,
            )
    
    # ------------------------------------------------------------------
    # Defaults management
    # ------------------------------------------------------------------
    def _load_defaults(self) -> None:
        """Load user defaults or fall back to built-in defaults."""
        if self.user_defaults_path.exists():
            try:
                self._load_defaults_from_file(self.user_defaults_path)
                if hasattr(self, "status_var"):
                    self.status_var.set("User defaults loaded")
                return
            except Exception as exc:
                messagebox.showwarning(
                    "Defaults warning",
                    "Could not load user defaults. Built-in defaults will be used."
                    f"\n\n{exc}",
                )

        self._load_builtin_defaults()

    def _load_builtin_defaults(self) -> None:
        """Load built-in default values into the GUI."""
        set_entries_from_dataclass(self.global_entries, DEFAULT_GLOBAL_SETTINGS)

        for field_name in CARTONER_FIELDS:
            if field_name not in self.cartoner_entries:
                entry = ttk.Entry(self)
                self.cartoner_entries[field_name] = entry
            self.cartoner_entries[field_name].delete(0, "end")
            self.cartoner_entries[field_name].insert(
                0,
                str(getattr(DEFAULT_GLOBAL_SETTINGS, field_name)),
            )

        self.current_weights = DEFAULT_WEIGHTS
        self.current_number_of_results_to_show = (
            DEFAULT_GLOBAL_SETTINGS.number_of_results_to_show
        )
        self.current_carton_AB_target = DEFAULT_GLOBAL_SETTINGS.carton_AB_target
        self.current_max_pitch_shift_mm = DEFAULT_GLOBAL_SETTINGS.max_pitch_shift_mm

        if hasattr(self, "input_table"):
            self.input_table.clear()
            stick_map = {}
            for s in DEFAULT_STICK_TYPES:
                sid = self.input_table.add_stick(s.stick_type_name)
                self.input_table.tree.item(sid, values=(s.stick_length_mm, s.stick_width_mm, s.stick_thickness_mm, s.fin_length_mm))
                stick_map[s.stick_type_name] = sid
            for f in DEFAULT_FORMATS:
                if f.stick_type_name in stick_map:
                    self.input_table.add_format(stick_map[f.stick_type_name], str(f.sticks_per_pocket))

        if hasattr(self, "status_var"):
            self.status_var.set("Built-in defaults loaded")

    def _load_defaults_from_file(self, path: Path) -> None:
        """Load defaults from a JSON file."""
        with path.open("r", encoding="utf-8") as file:
            data = json.load(file)

        # Load global settings
        global_data = dataclasses.asdict(DEFAULT_GLOBAL_SETTINGS)
        global_data.update(data.get("global_settings", {}))

        valid_global_fields = {
            field.name for field in dataclasses.fields(GlobalSettings)
        }
        global_data = {
            key: value
            for key, value in global_data.items()
            if key in valid_global_fields
        }

        settings = GlobalSettings(**global_data)

        self.current_number_of_results_to_show = settings.number_of_results_to_show
        self.current_carton_AB_target = settings.carton_AB_target
        self.current_max_pitch_shift_mm = settings.max_pitch_shift_mm

        # Load weights
        weight_data = dataclasses.asdict(DEFAULT_WEIGHTS)
        weight_data.update(data.get("weights", {}))

        valid_weight_fields = {
            field.name for field in dataclasses.fields(Weights)
        }
        weight_data = {
            key: value
            for key, value in weight_data.items()
            if key in valid_weight_fields
        }

        self.current_weights = Weights(**weight_data)

        set_entries_from_dataclass(self.global_entries, settings)

        # Populate cartoner entries from the loaded settings
        for field_name in CARTONER_FIELDS:
            value = getattr(settings, field_name)
            if field_name not in self.cartoner_entries:
                entry = ttk.Entry(self)
                self.cartoner_entries[field_name] = entry
            else:
                self.cartoner_entries[field_name].delete(0, "end")
            self.cartoner_entries[field_name].insert(
                0,
                "" if value is None else str(value),
            )

        if hasattr(self, "input_table"):
            self.input_table.clear()
            stick_map = {}
            for s in data.get("stick_types", []):
                sid = self.input_table.add_stick(s["stick_type_name"])
                self.input_table.tree.item(
                    sid,
                    values=(
                        s["stick_length_mm"],
                        s["stick_width_mm"],
                        s["stick_thickness_mm"],
                        s["fin_length_mm"],
                    ),
                )
                stick_map[s["stick_type_name"]] = sid
            for f in data.get("formats", []):
                if f["stick_type_name"] in stick_map:
                    self.input_table.add_format(
                        stick_map[f["stick_type_name"]], str(f["sticks_per_pocket"])
                    )

    def save_defaults(self) -> None:
        """Save current configuration to user defaults file (Weights, Cartoner, etc)."""
        try:
            # We only save the "hidden" logic defaults: Weights, Cartoner data, and UI prefs.
            # Main window data (Sticks, Formats, MT) are project-specific.
            
            cartoner_data = self._cartoner_values_dict()
            
            data = {
                "global_settings": {
                    "number_of_results_to_show": self.current_number_of_results_to_show,
                    "carton_AB_target": self.current_carton_AB_target,
                    "max_pitch_shift_mm": self.current_max_pitch_shift_mm,
                    **cartoner_data
                },
                "weights": dataclasses.asdict(self.current_weights),
            }

            with self.user_defaults_path.open("w", encoding="utf-8") as file:
                json.dump(data, file, indent=4)

            self.status_var.set(f"Defaults saved: {self.user_defaults_path}")

            messagebox.showinfo(
                "Defaults saved",
                f"Defaults saved to:\n{self.user_defaults_path}",
            )

        except Exception as exc:
            messagebox.showerror("Save defaults error", str(exc))
            self.status_var.set("Error saving defaults")

    def reload_defaults(self) -> None:
        """Reload defaults from file."""
        self._load_defaults()

    def _update_window_title(self) -> None:
        """Update window title with current project name."""
        base_title = "Calcolatore abbinamenti stick"
        
        display_name = ""
        if self.current_project_path:
            display_name = self.current_project_path.name
        elif self.current_project_name:
            display_name = self.current_project_name

        if display_name:
            self.title(f"{base_title} - {display_name}")
        else:
            self.title(base_title)

        self.project_name_var.set(display_name)

    def new_project(self) -> None:
        """Reset application to a new project state."""
        dialog = tk.Toplevel(self)
        dialog.title("Nuovo Progetto")
        dialog.geometry("350x200")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        result = {"name": None, "use_defaults": False}

        ttk.Label(dialog, text="Nome Progetto:").pack(pady=(15, 5))
        name_entry = ttk.Entry(dialog, width=35)
        name_entry.pack(pady=5)
        name_entry.focus_set()

        use_defaults_var = tk.BooleanVar(value=False)
        ttk.Checkbutton(dialog, text="usa dati di esempio per i formati stick", variable=use_defaults_var).pack(pady=10)

        def on_ok():
            name = name_entry.get().strip()
            if not name:
                messagebox.showwarning("Attenzione", "Inserire un nome per il progetto.")
                return
            result["name"] = name
            result["use_defaults"] = use_defaults_var.get()
            dialog.destroy()

        ttk.Button(dialog, text="Crea", command=on_ok).pack(pady=10)

        self.wait_window(dialog)

        if result["name"] is None:
            return

        self.current_project_path = None
        self.current_project_name = result["name"]
        
        self._load_defaults()
        
        # Always clear main window data for a new project unless example data is requested
        self.input_table.clear()
        if "sticks_per_beat" in self.global_entries:
            self.global_entries["sticks_per_beat"].delete(0, tk.END)

        if result["use_defaults"]:
            # Load example data (formerly built-in defaults) into main window
            set_entries_from_dataclass(self.global_entries, DEFAULT_GLOBAL_SETTINGS)
            stick_map = {}
            for s in DEFAULT_STICK_TYPES:
                sid = self.input_table.add_stick(s.stick_type_name)
                self.input_table.tree.item(sid, values=(s.stick_length_mm, s.stick_width_mm, s.stick_thickness_mm, s.fin_length_mm))
                stick_map[s.stick_type_name] = sid
            for f in DEFAULT_FORMATS:
                if f.stick_type_name in stick_map:
                    self.input_table.add_format(stick_map[f.stick_type_name], str(f.sticks_per_pocket))

        if self.home_frame:
            self.home_frame.pack_forget()
        self.main_container.pack(fill="both", expand=True)
        self._update_menu_bar(is_homepage=False)
        
        self._update_window_title()

    def open_project(self) -> None:
        """Open a project from a .sop file."""
        path = filedialog.askopenfilename(
            title="Open Project",
            filetypes=[("Stick Optimizer Project", "*.sop")],
        )
        if not path:
            return

        try:
            with open(path, "r", encoding="utf-8") as f:
                data = deserialize_project(f.read())

            self.current_project_path = Path(path)
            self.current_project_name = ""
            
            # Restore state
            self.current_weights = data["weights"]
            self.current_number_of_results_to_show = data["settings"].number_of_results_to_show
            self.current_carton_AB_target = data["settings"].carton_AB_target
            
            set_entries_from_dataclass(self.global_entries, data["settings"])
            self.current_max_pitch_shift_mm = data["settings"].max_pitch_shift_mm
            for field_name in CARTONER_FIELDS:
                val = getattr(data["settings"], field_name)
                if field_name in self.cartoner_entries:
                    self.cartoner_entries[field_name].delete(0, "end")
                    self.cartoner_entries[field_name].insert(0, "" if val is None else str(val))
            for field_name, entry in self.global_entries.items():
                val = getattr(data["settings"], field_name, None)
                if val is None:
                    entry.delete(0, "end")

            self.input_table.clear()
            stick_map = {}
            for s in data["stick_types"]:
                sid = self.input_table.add_stick(s.stick_type_name)
                self.input_table.tree.item(sid, values=(s.stick_length_mm, s.stick_width_mm, s.stick_thickness_mm, s.fin_length_mm))
                stick_map[s.stick_type_name] = sid
            for f in data["formats"]:
                if f.stick_type_name in stick_map:
                    self.input_table.add_format(stick_map[f.stick_type_name], str(f.sticks_per_pocket))

            self.solutions = data["results"]
            self.active_result_filters = data["active_filters"]
            
            if self.home_frame:
                self.home_frame.pack_forget()
            self.main_container.pack(fill="both", expand=True)
            self._update_menu_bar(is_homepage=False)

            self._apply_result_filters()
            
            if data["selected_index"] is not None and data["selected_index"] < len(self.solutions):
                # Find the item in treeview by index
                for item in self.results_tree.get_children():
                    if int(item) == data["selected_index"]:
                        self.results_tree.selection_set(item)
                        self.results_tree.see(item)
                        break

            self._update_window_title()
            self.status_var.set(f"Project loaded: {self.current_project_path.name}")

        except Exception as exc:
            messagebox.showerror("Open Project Error", str(exc))

    def save_project(self) -> None:
        """Save current project to current path or prompt if new."""
        if not self.current_project_path:
            self.save_project_as()
            return
        
        self._do_save_project(self.current_project_path)

    def save_project_as(self) -> None:
        """Prompt for a path and save current project."""
        initial_file = f"{self.current_project_name}.sop" if self.current_project_name else "project.sop"
        path = filedialog.asksaveasfilename(
            title="Save Project As",
            defaultextension=".sop",
            initialfile=initial_file,
            filetypes=[("Stick Optimizer Project", "*.sop")],
        )
        if not path:
            return
        
        self.current_project_path = Path(path)
        self._update_window_title()
        self._do_save_project(self.current_project_path)

    def _do_save_project(self, path: Path) -> None:
        try:
            overrides = {
                "number_of_results_to_show": self.current_number_of_results_to_show,
                "carton_AB_target": self.current_carton_AB_target,
                "max_pitch_shift_mm": self.current_max_pitch_shift_mm,
            }
            overrides.update(self._cartoner_values_dict())
            
            # Build settings directly from current global entries and overrides without falling back to defaults for empty fields
            settings_dict = {}
            if self.global_entries:
                for field in dataclasses.fields(GlobalSettings):
                    field_name = field.name
                    if field_name in overrides and overrides[field_name] is not None:
                        settings_dict[field_name] = overrides[field_name]
                    elif field_name in self.global_entries:
                        raw = self.global_entries[field_name].get().strip()
                        if raw == "":
                            settings_dict[field_name] = None
                        else:
                            try:
                                settings_dict[field_name] = field.type(raw)
                            except Exception:
                                settings_dict[field_name] = None
                    else:
                        settings_dict[field_name] = None

            settings_dict.update(overrides)
            settings = GlobalSettings(**settings_dict)
            
            stick_types = []
            formats = []
            for sid in self.input_table.tree.get_children():
                name = self.input_table.tree.item(sid, "text")
                v = self.input_table.tree.item(sid, "values")
                try:
                    stick_types.append(StickType(name, float(v[0]), float(v[1]), float(v[2]), float(v[3])))
                except: continue
                for fid in self.input_table.tree.get_children(sid):
                    count = self.input_table.tree.item(fid, "text")
                    formats.append(Format(f"{name}_{count}", name, int(count)))

            json_str = serialize_project(
                settings,
                self.current_weights,
                stick_types,
                formats,
                self.solutions,
                self.active_result_filters,
                self.selected_solution_index
            )

            with open(path, "w", encoding="utf-8") as f:
                f.write(json_str)

            self.status_var.set(f"Project saved: {path.name}")
        except Exception as exc:
            messagebox.showerror("Save Project Error", str(exc))


    def _clear_runtime_results(self) -> None:
        """Clear optimization results and filters."""
        self.solutions = []
        self.candidates_by_format = {}
        self.selected_solution_index = None
        self.active_result_filters = {}
        self.filtered_solution_indices = []

        if hasattr(self, "results_tree"):
            clear_tree(self.results_tree)
        if hasattr(self, "detail_widgets"):
            clear_solution_details(self.detail_widgets)
        if hasattr(self, "results_tree"):
            update_result_headings_for_filters(
                self.results_tree,
                self.active_result_filters,
            )

    # ------------------------------------------------------------------
    # Scoring weights editor
    # ------------------------------------------------------------------
    def open_weights_editor(self) -> None:
        """Open dialog to edit scoring weights."""
        dialog = tk.Toplevel(self)
        dialog.title("Modifica pesi di calcolo")
        dialog.geometry("500x340")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        main_frame = ttk.Frame(dialog)
        main_frame.pack(fill="both", expand=True, padx=12, pady=12)

        base_weights = DEFAULT_WEIGHTS
        if self.user_defaults_path.exists():
            try:
                with open(self.user_defaults_path, "r", encoding="utf-8") as file:
                    udata = json.load(file).get("weights", {})
                    if udata:
                        base_weights = Weights(**{f.name: udata.get(f.name, getattr(DEFAULT_WEIGHTS, f.name)) for f in dataclasses.fields(Weights)})
            except Exception:
                pass

        weight_translations = {
            "layer_penalty_weight": "Peso penalità strati",
            "carryover_penalty_weight": "Peso penalità riporto",
            "grouping_penalty_weight": "Peso penalità raggruppamento",
            "stability_width_penalty_weight": "Peso penalità stabilità",
            "carton_ab_ratio_penalty_weight": "Peso penalità rapporto A/B",
        }

        current_defaults = {f.name: getattr(base_weights, f.name) for f in dataclasses.fields(Weights)}

        button_frame = ttk.Frame(main_frame)
        button_frame.pack(side="bottom", fill="x", pady=(12, 0))

        left_btn_frame = ttk.Frame(button_frame)
        left_btn_frame.pack(side="left")

        right_btn_frame = ttk.Frame(button_frame)
        right_btn_frame.pack(side="right")

        ttk.Button(
            right_btn_frame,
            text="Applica",
            command=lambda: self._save_weights_from_dialog(entries, dialog),
        ).pack(side="right", padx=(4, 0))

        ttk.Button(
            right_btn_frame,
            text="Annulla",
            command=dialog.destroy,
        ).pack(side="right", padx=(4, 0))

        def reset_weights():
            for field in dataclasses.fields(Weights):
                dv = getattr(DEFAULT_WEIGHTS, field.name)
                entries[field.name].delete(0, "end")
                entries[field.name].insert(0, str(dv))
                entries[field.name].event_generate("<KeyRelease>")

        def overwrite_defaults():
            self._save_weights_from_dialog(entries, None)
            self.save_defaults()
            for field in dataclasses.fields(Weights):
                val = entries[field.name].get()
                current_defaults[field.name] = val
                friendly_name = weight_translations.get(field.name, field.name.replace("_", " "))
                lbl_text = f"{friendly_name}  ({val})"
                popup_labels[field.name].config(text=lbl_text)
                entries[field.name].event_generate("<KeyRelease>")

        ttk.Button(left_btn_frame, text="Ripristina predefiniti", command=reset_weights).pack(side="left", padx=(0, 4))
        ttk.Button(left_btn_frame, text="Sovrascrivi predefiniti", command=overwrite_defaults).pack(side="left")

        from gui.forms import _add_fields_to_frame_with_defaults
        form_frame = ttk.Frame(main_frame)
        form_frame.pack(fill="both", expand=True)

        field_names = [f.name for f in dataclasses.fields(Weights)]
        entries = {}
        popup_labels = {}
        defaults_map = {f.name: getattr(base_weights, f.name) for f in dataclasses.fields(Weights)}

        from gui.forms import DISPLAY_LABELS
        old_labels = dict(DISPLAY_LABELS)
        DISPLAY_LABELS.update(weight_translations)
        try:
            _add_fields_to_frame_with_defaults(
                form_frame,
                field_names,
                entries,
                popup_labels,
                defaults_map,
                entry_width=14,
                label_width=28,
            )
        finally:
            DISPLAY_LABELS.clear()
            DISPLAY_LABELS.update(old_labels)

        for field in dataclasses.fields(Weights):
            entries[field.name].delete(0, "end")
            current_value = getattr(self.current_weights, field.name)
            entries[field.name].insert(0, str(current_value))
            entries[field.name].event_generate("<KeyRelease>")

    def _save_weights_from_dialog(self, entries: dict, dialog: Optional[tk.Toplevel]) -> None:
        """Save weights from editor dialog."""
        try:
            values = {}

            for field in dataclasses.fields(Weights):
                raw = entries[field.name].get().strip()

                if raw == "":
                    raise ValueError(f"{field.name} is required.")

                values[field.name] = float(raw)

            self.current_weights = Weights(**values)

            if dialog:
                dialog.destroy()
            self.status_var.set("Scoring weights updated")

        except Exception as exc:
            messagebox.showerror("Invalid scoring weights", str(exc))

    # ------------------------------------------------------------------
    # Global option editors
    # ------------------------------------------------------------------
    def _open_simple_numeric_editor(
        self,
        title: str,
        field_name: str,
        current_value: float,
        min_value: Optional[float] = None,
        value_type: type = float,
        image=None,
    ) -> None:
        """Generic editor for single numeric value."""
        dialog = tk.Toplevel(self)
        dialog.title(title)
        if image:
            dialog.geometry("480x420")
        else:
            dialog.geometry("480x180")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        default_val = getattr(DEFAULT_GLOBAL_SETTINGS, field_name)
        if self.user_defaults_path.exists():
            try:
                with open(self.user_defaults_path, "r", encoding="utf-8") as file:
                    udata = json.load(file).get("global_settings", {})
                    if field_name in udata and udata[field_name] is not None:
                        default_val = udata[field_name]
            except Exception:
                pass

        frame = ttk.Frame(dialog, padding=12)
        frame.pack(fill="both", expand=True)

        button_frame = ttk.Frame(frame)
        button_frame.pack(side="bottom", fill="x", pady=(16, 0))

        if image:
            img_lbl = ttk.Label(frame, image=image)
            img_lbl.image = image
            img_lbl.pack(side="top", pady=(0, 10))

        field_labels = {
            "number_of_results_to_show": "Numero di risultati da mostrare",
            "carton_AB_target": "Obiettivo rapporto A/B astuccio",
            "max_pitch_shift_mm": "D - offset max stick [mm]"
        }
        display_field_name = field_labels.get(field_name, field_name)
        start_row = 1 if image else 0
        input_frame = ttk.Frame(frame)
        input_frame.pack(fill="x")
        
        lbl = ttk.Label(input_frame, text=f"{display_field_name} ({default_val})")
        lbl.grid(row=0, column=0, sticky="w", padx=(0, 8), pady=8)

        entry = ttk.Entry(input_frame, width=16)
        entry.grid(row=0, column=1, sticky="w", pady=8)
        entry.insert(0, str(current_value))

        def _update_style(*args):
            try:
                if entry.get().strip() != str(default_val):
                    entry.configure(style="Yellow.TEntry")
                else:
                    entry.configure(style="TEntry")
            except:
                pass

        _update_style()
        entry.bind("<KeyRelease>", _update_style)

        def save_value() -> None:
            try:
                value = value_type(entry.get().strip())
                if min_value is not None and value <= min_value:
                    raise ValueError(f"{field_name} must be > {min_value}.")

                if field_name == "number_of_results_to_show":
                    self.current_number_of_results_to_show = value
                elif field_name == "carton_AB_target":
                    self.current_carton_AB_target = value
                elif field_name == "max_pitch_shift_mm":
                    self.current_max_pitch_shift_mm = value

                dialog.destroy()
                self.status_var.set(f"{field_name} set to {value}")
            except Exception as exc:
                messagebox.showerror(f"Invalid {field_name}", str(exc))

        def reset():
            entry.delete(0, tk.END)
            entry.insert(0, str(default_val))
            _update_style()

        def overwrite():
            try:
                value = value_type(entry.get().strip())
                if field_name == "number_of_results_to_show":
                    self.current_number_of_results_to_show = value
                elif field_name == "carton_AB_target":
                    self.current_carton_AB_target = value
                self.save_defaults()
                nonlocal default_val
                default_val = value
                lbl.config(text=f"{field_name} ({default_val})")
                _update_style()
            except Exception as exc:
                messagebox.showerror("Error", str(exc))

        ttk.Button(button_frame, text="Annulla", command=dialog.destroy).pack(side="right", padx=(4, 0))
        ttk.Button(button_frame, text="Applica", command=save_value).pack(side="right", padx=(4, 0))
        ttk.Button(button_frame, text="Ripristina predefiniti", command=reset).pack(side="left", padx=(0, 4))
        ttk.Button(button_frame, text="Sovrascrivi predefiniti", command=overwrite).pack(side="left")

    def open_number_of_results_editor(self) -> None:
        """Open editor for number of results to show."""
        self._open_simple_numeric_editor(
            "Modifica numero di risultati",
            "number_of_results_to_show",
            self.current_number_of_results_to_show,
            min_value=0,
            value_type=int,
        )

    def open_carton_ab_target_editor(self) -> None:
        """Open editor for carton A/B target ratio."""
        self._open_simple_numeric_editor(
            "Modifica obiettivo A/B astuccio",
            "carton_AB_target",
            self.current_carton_AB_target,
            min_value=0,
            value_type=float,
        )

    def open_mt_extra_settings_editor(self) -> None:
        """Open editor for additional MT data (Offset max stick D)."""
        self._open_simple_numeric_editor(
            "Dati aggiuntivi MT",
            "max_pitch_shift_mm",
            self.current_max_pitch_shift_mm,
            min_value=0,
            value_type=float,
            image=self._load_ui_image("dati_mt_menu")
        )

    def _cartoner_values_dict(self) -> dict:
        """Extract cartoner entry values into a dict."""
        data = {}

        for field_name, entry in self.cartoner_entries.items():
            raw = entry.get().strip()

            if raw == "":
                data[field_name] = None
                continue

            if field_name == "max_allowed_layers":
                data[field_name] = int(float(raw))
            else:
                data[field_name] = float(raw)

        return data

    def open_cartoner_settings_editor(self) -> None:
        """Open dialog to edit cartoner/machine settings."""
        dialog = tk.Toplevel(self)
        dialog.title("Dati astucciatrice")
        dialog.geometry("520x720")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        # Main container with proper layout
        main_frame = ttk.Frame(dialog)
        main_frame.pack(fill="both", expand=True, padx=12, pady=12)

        # Form content
        current_defaults = {}
        for f in CARTONER_FIELDS:
            val = getattr(DEFAULT_GLOBAL_SETTINGS, f)
            if self.user_defaults_path.exists():
                try:
                    with open(self.user_defaults_path, "r", encoding="utf-8") as file:
                        udata = json.load(file).get("global_settings", {})
                        if f in udata and udata[f] is not None:
                            val = udata[f]
                except Exception:
                    pass
            current_defaults[f] = val

        # Button frame at bottom (pack first so it claims bottom space)
        button_frame = ttk.Frame(main_frame)
        button_frame.pack(side="bottom", fill="x", pady=(12, 0))

        left_btn_frame = ttk.Frame(button_frame)
        left_btn_frame.pack(side="left")

        right_btn_frame = ttk.Frame(button_frame)
        right_btn_frame.pack(side="right")

        ttk.Button(
            right_btn_frame,
            text="Applica",
            command=lambda: self._save_cartoner_settings(popup_entries, dialog),
        ).pack(side="right", padx=(4, 0))

        ttk.Button(
            right_btn_frame,
            text="Annulla",
            command=dialog.destroy,
        ).pack(side="right", padx=(4, 0))

        def reset():
            for field_name in CARTONER_FIELDS:
                dv = getattr(DEFAULT_GLOBAL_SETTINGS, field_name)
                popup_entries[field_name].delete(0, "end")
                popup_entries[field_name].insert(0, str(dv))
                popup_entries[field_name].event_generate("<KeyRelease>")

        def overwrite():
            from gui.forms import DISPLAY_LABELS
            self._save_cartoner_settings(popup_entries, None)
            self.save_defaults()
            # Update labels in the current dialog using newly saved defaults
            for field_name in CARTONER_FIELDS:
                val = popup_entries[field_name].get()
                current_defaults[field_name] = val
                lbl_text = f"{DISPLAY_LABELS.get(field_name, field_name)}  ({val})"
                popup_labels[field_name].config(text=lbl_text)
                popup_entries[field_name].event_generate("<KeyRelease>")

        ttk.Button(left_btn_frame, text="Ripristina predefiniti", command=reset).pack(side="left", padx=(0, 4))
        ttk.Button(left_btn_frame, text="Sovrascrivi predefiniti", command=overwrite).pack(side="left")

        form_frame, popup_entries, popup_labels = build_cartoner_settings_form(
            main_frame,
            entry_width=14,
            defaults=current_defaults,
            cartoner_image=self.cartoner_image
        )
        form_frame.pack(fill="both", expand=True)

        # Populate entries with current values from self.cartoner_entries or defaults
        for field_name in CARTONER_FIELDS:
            popup_entries[field_name].delete(0, "end")
            val = ""
            if field_name in self.cartoner_entries:
                val = self.cartoner_entries[field_name].get()
            if not val:
                val = str(current_defaults.get(field_name, ""))
            popup_entries[field_name].insert(0, val)
            # Trigger style update
            popup_entries[field_name].event_generate("<KeyRelease>")

    def _save_cartoner_settings(self, popup_entries: dict, dialog: Optional[tk.Toplevel]) -> None:
        """Save cartoner settings from popup."""
        try:
            for field_name in CARTONER_FIELDS:
                self.cartoner_entries[field_name].delete(0, "end")
                self.cartoner_entries[field_name].insert(
                    0,
                    popup_entries[field_name].get(),
                )

            if dialog:
                dialog.destroy()
            self.status_var.set("Cartoner settings updated")

        except Exception as exc:
            messagebox.showerror("Cartoner settings error", str(exc))

    # ------------------------------------------------------------------
    # Optimization
    # ------------------------------------------------------------------
    def run_optimization(self) -> None:
        """Run the optimization and display results."""
        try:
            overrides = {
                "number_of_results_to_show": self.current_number_of_results_to_show,
                "carton_AB_target": self.current_carton_AB_target,
                "max_pitch_shift_mm": self.current_max_pitch_shift_mm,
            }
            overrides.update(self._cartoner_values_dict())

            settings = parse_global_settings(self.global_entries, overrides=overrides)
            weights = self.current_weights
            
            stick_types = []
            formats = []
            for sid in self.input_table.tree.get_children():
                name = self.input_table.tree.item(sid, "text")
                v = self.input_table.tree.item(sid, "values")
                try:
                    stick_types.append(StickType(name, float(v[0]), float(v[1]), float(v[2]), float(v[3])))
                except: continue
                for fid in self.input_table.tree.get_children(sid):
                    count = self.input_table.tree.item(fid, "text")
                    formats.append(Format(f"{name}_{count}", name, int(count)))

            self.status_var.set("Optimization running...")
            self.run_button.config(state="disabled")
            self.update()

            solutions, candidates_by_format = optimize(
                settings,
                stick_types,
                formats,
                weights,
            )

            self.run_button.config(state="normal")

        except Exception as exc:
            self.run_button.config(state="normal")
            messagebox.showerror("Optimization error", str(exc))
            self.status_var.set("Error")
            return

        self.solutions = solutions
        self.candidates_by_format = candidates_by_format
        self.selected_solution_index = None

        self.active_result_filters = {}
        self.filtered_solution_indices = list(range(len(self.solutions)))

        populate_results(
            self.results_tree,
            self.solutions,
            self.filtered_solution_indices,
        )

        update_result_headings_for_filters(
            self.results_tree,
            self.active_result_filters,
        )

        clear_solution_details(self.detail_widgets)

        if not solutions:
            counts = "\n".join(
                f"{name}: {len(candidates)} candidates"
                for name, candidates in candidates_by_format.items()
            )

            messagebox.showwarning(
                "No feasible solution",
                "No feasible complete multi-format solution exists.\n\n" + counts,
            )

            self.status_var.set("No feasible solution")
            return

        self.status_var.set(
            f"Optimization complete: {len(solutions)} solution(s) shown"
        )

    def _on_solution_selected(self, _event: Optional[tk.Event] = None) -> None:
        """Handle solution selection in results tree."""
        try:
            selected = self.results_tree.selection()

            if not selected:
                return

            index = int(selected[0])
            self.selected_solution_index = index

            populate_solution_details(
                self.detail_widgets,
                self.solutions[index],
            )

            self.status_var.set(f"Selected solution {index + 1}")

        except Exception as exc:
            messagebox.showerror("Detail view error", str(exc))
            self.status_var.set("Error displaying selected solution")

    def _open_selected_format_popup(self, _event: Optional[tk.Event] = None) -> None:
        """Open popup with full format details."""
        if self.selected_solution_index is None:
            return

        overview_tree = self.detail_widgets["format_overview_tree"]
        selected = overview_tree.selection()

        if not selected:
            return

        item_id = selected[0]
        if overview_tree.parent(item_id) == "":
            return
        
        candidate_index = int(item_id.split('_')[-1])
        solution = self.solutions[self.selected_solution_index]
        candidate = solution.candidates[candidate_index]

        max_b_by_pocket = {}
        for c in solution.candidates:
            b_val = getattr(c, "carton_B_mm", 0.0) or 0.0
            max_b_by_pocket[c.pocket_type] = max(max_b_by_pocket.get(c.pocket_type, 0.0), b_val)

        pocket_height = max_b_by_pocket.get(candidate.pocket_type, getattr(candidate, "carton_B_mm", 0.0) or 0.0)

        open_format_detail_popup(self, candidate, pocket_height=pocket_height, show_details=self.show_score_penalty_details.get())

    # ------------------------------------------------------------------
    # Result filtering
    # ------------------------------------------------------------------
    def _open_result_column_filter(self, column_name: str) -> None:
        """Open filter dialog for a result column."""
        if column_name == "rank":
            messagebox.showinfo(
                "Filter not available",
                "Rank is only the displayed row number. Filter another column.",
            )
            return

        if column_name not in FILTERABLE_RESULT_COLUMNS:
            messagebox.showinfo(
                "Filter not available",
                f"Column '{column_name}' cannot be filtered.",
            )
            return

        all_values = self._unique_display_values_for_result_column(column_name)
        existing_filter = self.active_result_filters.get(column_name, {})

        existing_selected_values = existing_filter.get("selected_values", None)
        existing_operator = existing_filter.get("operator", "")
        existing_value = existing_filter.get("value", "")

        dialog = tk.Toplevel(self)
        dialog.title(f"Filter: {column_name}")
        dialog.geometry("410x520")
        dialog.minsize(360, 420)
        dialog.transient(self)
        dialog.grab_set()

        frame = ttk.Frame(dialog, padding=12)
        frame.pack(fill="both", expand=True)

        ttk.Label(
            frame,
            text=f"Column: {column_name}",
            font=("TkDefaultFont", 10, "bold"),
        ).pack(anchor="w", pady=(0, 8))

        values_frame = ttk.LabelFrame(frame, text="Valori esatti", padding=8)
        values_frame.pack(fill="both", expand=True, pady=(0, 8))

        canvas = tk.Canvas(values_frame, highlightthickness=0)
        scrollbar = ttk.Scrollbar(
            values_frame,
            orient="vertical",
            command=canvas.yview,
        )
        scroll_frame = ttk.Frame(canvas)

        scroll_frame.bind(
            "<Configure>",
            lambda _event: canvas.configure(scrollregion=canvas.bbox("all")),
        )

        canvas_window = canvas.create_window(
            (0, 0),
            window=scroll_frame,
            anchor="nw",
        )

        canvas.bind(
            "<Configure>",
            lambda event: canvas.itemconfigure(canvas_window, width=event.width),
        )

        canvas.configure(yscrollcommand=scrollbar.set)

        canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

        value_vars = {}

        for value in all_values:
            var = tk.BooleanVar()

            if existing_selected_values is None:
                var.set(True)
            else:
                var.set(value in existing_selected_values)

            check = ttk.Checkbutton(
                scroll_frame,
                text=value,
                variable=var,
            )
            check.pack(anchor="w")

            value_vars[value] = var

        select_buttons = ttk.Frame(frame)
        select_buttons.pack(fill="x", pady=(0, 8))

        ttk.Button(
            select_buttons,
            text="Seleziona tutto",
            command=lambda: self._set_filter_value_checks(value_vars, True),
        ).pack(side="left")

        ttk.Button(
            select_buttons,
            text="Deseleziona tutto",
            command=lambda: self._set_filter_value_checks(value_vars, False),
        ).pack(side="left", padx=(8, 0))

        condition_frame = ttk.LabelFrame(
            frame,
            text="Condizione opzionale",
            padding=8,
        )
        condition_frame.pack(fill="x", pady=(0, 8))

        ttk.Label(condition_frame, text="Operatore").grid(
            row=0,
            column=0,
            sticky="w",
            padx=(0, 8),
            pady=4,
        )

        operator_combo = ttk.Combobox(
            condition_frame,
            values=FILTER_OPERATORS,
            state="readonly",
            width=12,
        )
        operator_combo.grid(row=0, column=1, sticky="w", pady=4)
        operator_combo.set(existing_operator)

        ttk.Label(condition_frame, text="Valore").grid(
            row=1,
            column=0,
            sticky="w",
            padx=(0, 8),
            pady=4,
        )

        value_entry = ttk.Entry(condition_frame, width=18)
        value_entry.grid(row=1, column=1, sticky="w", pady=4)
        value_entry.insert(0, existing_value)

        button_frame = ttk.Frame(frame)
        button_frame.pack(fill="x")

        ttk.Button(
            button_frame,
            text="Applica",
            command=lambda: self._apply_column_filter_from_popup(
                column_name,
                all_values,
                value_vars,
                operator_combo,
                value_entry,
                dialog,
            ),
        ).pack(side="right")

        ttk.Button(
            button_frame,
            text="Azzera colonna",
            command=lambda: self._clear_single_column_filter(column_name, dialog),
        ).pack(side="right", padx=(0, 8))

        ttk.Button(
            button_frame,
            text="Azzera tutto",
            command=lambda: self._clear_all_result_filters(dialog),
        ).pack(side="right", padx=(0, 8))

    def _set_filter_value_checks(self, value_vars: dict, state: bool) -> None:
        """Set all filter checkboxes to a state."""
        for var in value_vars.values():
            var.set(state)

    def _unique_display_values_for_result_column(self, column_name: str) -> list[str]:
        """Get unique display values for a result column."""
        values = set()

        for solution in self.solutions:
            values.add(result_display_value(solution, column_name))

        return sorted(values, key=self._sort_filter_value)

    @staticmethod
    def _sort_filter_value(value: str) -> tuple:
        """Sort key for filter values (numeric first, then strings)."""
        try:
            return (0, float(value))
        except ValueError:
            return (1, str(value))

    def _apply_column_filter_from_popup(
        self,
        column_name: str,
        all_values: list[str],
        value_vars: dict,
        operator_combo: ttk.Combobox,
        value_entry: ttk.Entry,
        dialog: tk.Toplevel,
    ) -> None:
        """Apply filter from dialog."""
        try:
            selected_values = {
                value
                for value, var in value_vars.items()
                if var.get()
            }

            operator = operator_combo.get().strip()
            condition_value = value_entry.get().strip()

            if operator and condition_value == "":
                raise ValueError("Condition value is required when operator is set.")

            if condition_value and not operator:
                raise ValueError("Operator is required when condition value is set.")

            all_values_set = set(all_values)

            selected_values_filter = None
            if selected_values != all_values_set:
                selected_values_filter = selected_values

            has_values_filter = selected_values_filter is not None
            has_condition_filter = bool(operator and condition_value)

            if not has_values_filter and not has_condition_filter:
                if column_name in self.active_result_filters:
                    del self.active_result_filters[column_name]
            else:
                self.active_result_filters[column_name] = {
                    "selected_values": selected_values_filter,
                    "operator": operator,
                    "value": condition_value,
                }

            dialog.destroy()
            self._apply_result_filters()

        except Exception as exc:
            messagebox.showerror("Filter error", str(exc))

    def _clear_single_column_filter(self, column_name: str, dialog: Optional[tk.Toplevel] = None) -> None:
        """Clear filter for a single column."""
        if column_name in self.active_result_filters:
            del self.active_result_filters[column_name]

        if dialog is not None:
            dialog.destroy()

        self._apply_result_filters()

    def _clear_all_result_filters(self, dialog: Optional[tk.Toplevel] = None) -> None:
        """Clear all active result filters."""
        self.active_result_filters = {}

        if dialog is not None:
            dialog.destroy()

        self._apply_result_filters()

    def _apply_result_filters(self) -> None:
        """Apply all active result filters and update display."""
        if not self.solutions:
            update_result_headings_for_filters(
                self.results_tree,
                self.active_result_filters,
            )
            return

        filtered_indices = []

        for index, solution in enumerate(self.solutions):
            if self._solution_passes_all_filters(solution):
                filtered_indices.append(index)

        self.filtered_solution_indices = filtered_indices

        populate_results(
            self.results_tree,
            self.solutions,
            self.filtered_solution_indices,
        )

        update_result_headings_for_filters(
            self.results_tree,
            self.active_result_filters,
        )

        clear_solution_details(self.detail_widgets)
        self.selected_solution_index = None

        self.status_var.set(
            f"Filters applied: {len(filtered_indices)} / {len(self.solutions)} solutions shown"
        )

    def _solution_passes_all_filters(self, solution) -> bool:
        """Check if a solution passes all active filters."""
        for column_name, result_filter in self.active_result_filters.items():
            if not self._solution_passes_filter(solution, column_name, result_filter):
                return False

        return True

    def _solution_passes_filter(self, solution, column_name: str, result_filter: dict) -> bool:
        """Check if a solution passes a single filter."""
        selected_values = result_filter.get("selected_values", None)

        display_value = result_display_value(solution, column_name)

        if selected_values is not None:
            if display_value not in selected_values:
                return False

        operator = result_filter.get("operator", "")
        raw_filter_value = result_filter.get("value", "")

        if not operator:
            return True

        solution_value = self._solution_value_for_filter_column(solution, column_name)

        if operator == "contains":
            return str(raw_filter_value).lower() in str(solution_value).lower()

        try:
            numeric_solution_value = float(solution_value)
            numeric_filter_value = float(raw_filter_value)
        except ValueError as exc:
            raise ValueError(
                f"Filter on '{column_name}' requires numeric values for operator '{operator}'."
            ) from exc

        if operator == "<=":
            return numeric_solution_value <= numeric_filter_value

        if operator == ">=":
            return numeric_solution_value >= numeric_filter_value

        if operator == "<":
            return numeric_solution_value < numeric_filter_value

        if operator == ">":
            return numeric_solution_value > numeric_filter_value

        if operator == "=":
            return numeric_solution_value == numeric_filter_value

        if operator == "!=":
            return numeric_solution_value != numeric_filter_value

        raise ValueError(f"Unsupported filter operator: {operator}")

    @staticmethod
    def _solution_value_for_filter_column(solution, column_name: str) -> float:
        """Get the raw solution value for a filter column."""
        attr_name = FILTERABLE_RESULT_COLUMNS[column_name]
        return getattr(solution, attr_name, 0.0)

    # ------------------------------------------------------------------
    # Export
    # ------------------------------------------------------------------
    def export_summary(self) -> None:
        """Export solution summary to CSV."""
        if not self.solutions:
            messagebox.showinfo("No data", "Run an optimization before exporting.")
            return

        path = filedialog.asksaveasfilename(
            title="Export solution summary",
            defaultextension=".csv",
            filetypes=[
                ("CSV files", "*.csv"),
                ("All files", "*.*"),
            ],
        )

        if not path:
            return

        try:
            export_solution_summary_csv(self.solutions, path)
            self.status_var.set(f"Summary exported: {path}")

        except Exception as exc:
            messagebox.showerror("Export error", str(exc))

    def export_details(self) -> None:
        """Export selected solution details to CSV."""
        if self.selected_solution_index is None:
            messagebox.showinfo(
                "No selection",
                "Select a solution before exporting details.",
            )
            return

        path = filedialog.asksaveasfilename(
            title="Export selected solution details",
            defaultextension=".csv",
            filetypes=[
                ("CSV files", "*.csv"),
                ("All files", "*.*"),
            ],
        )

        if not path:
            return

        try:
            export_solution_details_csv(
                self.solutions[self.selected_solution_index],
                path,
            )
            self.status_var.set(f"Details exported: {path}")

        except Exception as exc:
            messagebox.showerror("Export error", str(exc))
