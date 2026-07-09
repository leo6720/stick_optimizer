import dataclasses
import json
import tkinter as tk
from pathlib import Path
from tkinter import filedialog, messagebox, ttk

from defaults import (
    DEFAULT_FORMATS,
    DEFAULT_GLOBAL_SETTINGS,
    DEFAULT_STICK_TYPES,
    DEFAULT_WEIGHTS,
)
from export import export_solution_details_csv, export_solution_summary_csv
from models import GlobalSettings, Weights
from optimizer import optimize

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
from gui.tables import EditableTable


class OptimizerApp(tk.Tk):
    """Tkinter GUI orchestrator.

    This class only coordinates widgets, parsing, optimization calls and export.
    Engineering formulas stay in optimizer.py and scoring.py.
    """

    def __init__(self):
        super().__init__()

        self.title("Stickpack Transfer Optimizer")
        self.iconbitmap("stick_optimpizer_logo.ico")
        self.geometry("1450x900")
        self.minsize(1200, 720)

        self.project_root = Path(__file__).resolve().parent.parent
        self.user_defaults_path = self.project_root / "user_defaults.json"

        self.solutions = []
        self.candidates_by_format = {}
        self.selected_solution_index = None

        self.active_result_filters = {}
        self.filtered_solution_indices = []

        self.global_entries = {}
        self.cartoner_entries = {}
        self.current_weights = DEFAULT_WEIGHTS

        self.current_number_of_results_to_show = (
            DEFAULT_GLOBAL_SETTINGS.number_of_results_to_show
        )
        self.current_carton_AB_target = DEFAULT_GLOBAL_SETTINGS.carton_AB_target

        self._build_menu_bar()
        self._build_layout()
        self._load_defaults()

    # ------------------------------------------------------------------
    # Menu bar
    # ------------------------------------------------------------------
    def _build_menu_bar(self):
        menu_bar = tk.Menu(self)

        file_menu = tk.Menu(menu_bar, tearoff=False)
        export_menu = tk.Menu(file_menu, tearoff=False)

        export_menu.add_command(
            label="Summary CSV",
            command=self.export_summary,
        )
        export_menu.add_command(
            label="Selected detail CSV",
            command=self.export_details,
        )

        file_menu.add_cascade(label="Export", menu=export_menu)
        menu_bar.add_cascade(label="File", menu=file_menu)

        options_menu = tk.Menu(menu_bar, tearoff=False)

        defaults_menu = tk.Menu(options_menu, tearoff=False)
        defaults_menu.add_command(
            label="Save",
            command=self.save_defaults,
        )
        defaults_menu.add_command(
            label="Reload",
            command=self.reload_defaults,
        )

        options_menu.add_cascade(label="Defaults", menu=defaults_menu)

        options_menu.add_separator()

        options_menu.add_command(
            label="Clear result filters",
            command=lambda: self._clear_all_result_filters(None),
        )

        menu_bar.add_cascade(label="Options", menu=options_menu)
        self.config(menu=menu_bar)
        
        edit_menu = tk.Menu(menu_bar, tearoff=False)

        edit_menu.add_command(
            label="Scoring weights",
            command=self.open_weights_editor,
        )

        edit_menu.add_command(
            label="Number of results",
            command=self.open_number_of_results_editor,
        )

        edit_menu.add_command(
            label="Carton A/B target",
            command=self.open_carton_ab_target_editor,
        )

        edit_menu.add_command(
            label="Dati astucciatrice",
            command=self.open_cartoner_settings_editor,
        )

        menu_bar.add_cascade(
            label="Edit",
            menu=edit_menu,
        )

    # ------------------------------------------------------------------
    # Layout
    # ------------------------------------------------------------------
    def _build_layout(self):
        root = ttk.Frame(self, padding=8)
        root.pack(fill="both", expand=True)

        toolbar = ttk.Frame(root)
        toolbar.pack(fill="x", pady=(0, 8))

        self.run_button = ttk.Button(
            toolbar,
            text="Run optimization",
            command=self.run_optimization,
        )
        self.run_button.pack(side="left")

        main_pane = ttk.PanedWindow(root, orient="horizontal")
        main_pane.pack(fill="both", expand=True, pady=(0, 8))

        left_pane = ttk.Frame(main_pane)
        right_pane = ttk.Frame(main_pane)

        main_pane.add(left_pane, weight=1)
        main_pane.add(right_pane, weight=1)

        global_frame, self.global_entries = build_grouped_global_settings_form(
            left_pane,
            entry_width=14,
        )
        global_frame.pack(fill="x", pady=(0, 8))

        self._build_input_tables(left_pane)
        self._build_output_tables(right_pane)

        bottom = ttk.Frame(root)
        bottom.pack(fill="x", side="bottom")

        self.status_var = tk.StringVar(value="Ready")
        ttk.Label(
            bottom,
            textvariable=self.status_var,
        ).pack(side="right")

    def _build_input_tables(self, parent):
        self.stick_table = EditableTable(
            parent,
            title="Stick types",
            columns=[
                ("name", "stick_type_name", 130),
                ("length", "stick_length_mm", 110),
                ("width", "stick_width_mm", 100),
                ("thickness", "stick_thickness_mm", 120),
                ("fin", "fin_length_mm", 100),
            ],
            height=7,
        )
        self.stick_table.pack(fill="both", expand=True, pady=(0, 8))

        self.format_table = EditableTable(
            parent,
            title="Formats",
            columns=[
                ("format", "format_name", 150),
                ("stick_type", "stick_type_name", 150),
                ("sticks_per_pocket", "sticks_per_pocket", 130),
            ],
            height=7,
        )
        self.format_table.pack(fill="both", expand=True)

    def _build_output_tables(self, parent):
        results_frame, self.results_tree = build_results_section(
            parent,
            self._on_solution_selected,
            self._open_result_column_filter,
        )
        results_frame.pack(fill="both", expand=True, pady=(0, 8))

        detail_frame, self.detail_widgets = build_detail_section(
            parent,
            self._open_selected_format_popup,
        )
        detail_frame.pack(fill="both", expand=True)

    # ------------------------------------------------------------------
    # Defaults
    # ------------------------------------------------------------------
    def _load_defaults(self):
        if self.user_defaults_path.exists():
            try:
                self._load_defaults_from_file(self.user_defaults_path)
                self.status_var.set("User defaults loaded")
                return
            except Exception as exc:
                messagebox.showwarning(
                    "Defaults warning",
                    "Could not load user defaults. Built-in defaults will be used."
                    f"\n\n{exc}",
                )

        self._load_builtin_defaults()

    def _load_builtin_defaults(self):
        set_entries_from_dataclass(self.global_entries, DEFAULT_GLOBAL_SETTINGS)

        for field_name in CARTONER_FIELDS:
            entry = ttk.Entry(self)

            entry.insert(
                0,
                str(getattr(DEFAULT_GLOBAL_SETTINGS, field_name)),
            )

            self.cartoner_entries[field_name] = entry

        self.current_weights = DEFAULT_WEIGHTS
        self.current_number_of_results_to_show = (
            DEFAULT_GLOBAL_SETTINGS.number_of_results_to_show
        )
        self.current_carton_AB_target = DEFAULT_GLOBAL_SETTINGS.carton_AB_target

        self.stick_table.set_rows(
            [
                (
                    stick.stick_type_name,
                    stick.stick_length_mm,
                    stick.stick_width_mm,
                    stick.stick_thickness_mm,
                    stick.fin_length_mm,
                )
                for stick in DEFAULT_STICK_TYPES
            ]
        )

        self.format_table.set_rows(
            [
                (
                    fmt.format_name,
                    fmt.stick_type_name,
                    fmt.sticks_per_pocket,
                )
                for fmt in DEFAULT_FORMATS
            ]
        )

        self._clear_runtime_results()
        self.status_var.set("Built-in defaults loaded")

    def _load_defaults_from_file(self, path):
        with Path(path).open("r", encoding="utf-8") as file:
            data = json.load(file)

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

        self.stick_table.set_rows(
            [
                (
                    row["stick_type_name"],
                    row["stick_length_mm"],
                    row["stick_width_mm"],
                    row["stick_thickness_mm"],
                    row["fin_length_mm"],
                )
                for row in data.get("stick_types", [])
            ]
        )

        self.format_table.set_rows(
            [
                (
                    row["format_name"],
                    row["stick_type_name"],
                    row["sticks_per_pocket"],
                )
                for row in data.get("formats", [])
            ]
        )

        self._clear_runtime_results()

    def save_defaults(self):
        try:
            settings = parse_global_settings(
                self.global_entries,
                overrides={
                    "number_of_results_to_show": (
                        self.current_number_of_results_to_show
                    ),
                    "carton_AB_target": self.current_carton_AB_target,
                },
            )

            stick_types = parse_stick_types(self.stick_table.get_rows())
            formats = parse_formats(self.format_table.get_rows())

            data = {
                "global_settings": dataclasses.asdict(settings),
                "weights": dataclasses.asdict(self.current_weights),
                "stick_types": [
                    dataclasses.asdict(stick) for stick in stick_types
                ],
                "formats": [
                    dataclasses.asdict(fmt) for fmt in formats
                ],
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

    def reload_defaults(self):
        self._load_defaults()

    def _clear_runtime_results(self):
        self.solutions = []
        self.candidates_by_format = {}
        self.selected_solution_index = None
        self.active_result_filters = {}
        self.filtered_solution_indices = []

        clear_tree(self.results_tree)
        clear_solution_details(self.detail_widgets)
        update_result_headings_for_filters(
            self.results_tree,
            self.active_result_filters,
        )

    # ------------------------------------------------------------------
    # Scoring weights editor
    # ------------------------------------------------------------------
    def open_weights_editor(self):
        dialog = tk.Toplevel(self)
        dialog.title("Edit scoring weights")
        dialog.geometry("460x360")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        frame = ttk.Frame(dialog, padding=12)
        frame.pack(fill="both", expand=True)

        entries = {}

        for row, field in enumerate(dataclasses.fields(Weights)):
            ttk.Label(frame, text=field.name).grid(
                row=row,
                column=0,
                sticky="w",
                padx=(0, 8),
                pady=4,
            )

            entry = ttk.Entry(frame, width=18)
            entry.grid(
                row=row,
                column=1,
                sticky="w",
                pady=4,
            )

            current_value = getattr(self.current_weights, field.name)
            entry.insert(0, str(current_value))

            entries[field.name] = entry

        button_frame = ttk.Frame(frame)
        button_frame.grid(
            row=len(dataclasses.fields(Weights)),
            column=0,
            columnspan=2,
            sticky="e",
            pady=(16, 0),
        )

        ttk.Button(
            button_frame,
            text="Save",
            command=lambda: self._save_weights_from_dialog(entries, dialog),
        ).pack(side="right")

        ttk.Button(
            button_frame,
            text="Cancel",
            command=dialog.destroy,
        ).pack(side="right", padx=(0, 8))

    def _save_weights_from_dialog(self, entries, dialog):
        try:
            values = {}

            for field in dataclasses.fields(Weights):
                raw = entries[field.name].get().strip()

                if raw == "":
                    raise ValueError(f"{field.name} is required.")

                values[field.name] = float(raw)

            self.current_weights = Weights(**values)

            dialog.destroy()
            self.status_var.set("Scoring weights updated")

        except Exception as exc:
            messagebox.showerror("Invalid scoring weights", str(exc))

    # ------------------------------------------------------------------
    # Hidden/global option editors
    # ------------------------------------------------------------------
    def open_number_of_results_editor(self):
        dialog = tk.Toplevel(self)
        dialog.title("Edit number of results")
        dialog.geometry("360x150")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        frame = ttk.Frame(dialog, padding=12)
        frame.pack(fill="both", expand=True)

        ttk.Label(frame, text="number_of_results_to_show").grid(
            row=0,
            column=0,
            sticky="w",
            padx=(0, 8),
            pady=8,
        )

        entry = ttk.Entry(frame, width=16)
        entry.grid(row=0, column=1, sticky="w", pady=8)
        entry.insert(0, str(self.current_number_of_results_to_show))
        entry.focus_set()
        entry.select_range(0, tk.END)

        button_frame = ttk.Frame(frame)
        button_frame.grid(
            row=1,
            column=0,
            columnspan=2,
            sticky="e",
            pady=(16, 0),
        )

        ttk.Button(
            button_frame,
            text="Save",
            command=lambda: self._save_number_of_results_from_dialog(
                entry,
                dialog,
            ),
        ).pack(side="right")

        ttk.Button(
            button_frame,
            text="Cancel",
            command=dialog.destroy,
        ).pack(side="right", padx=(0, 8))

    def _save_number_of_results_from_dialog(self, entry, dialog):
        try:
            value = int(float(entry.get().strip()))

            if value <= 0:
                raise ValueError("number_of_results_to_show must be > 0.")

            self.current_number_of_results_to_show = value

            dialog.destroy()
            self.status_var.set(f"Number of results set to {value}")

        except Exception as exc:
            messagebox.showerror("Invalid number of results", str(exc))

    def open_carton_ab_target_editor(self):
        dialog = tk.Toplevel(self)
        dialog.title("Edit carton A/B target")
        dialog.geometry("360x150")
        dialog.resizable(False, False)
        dialog.transient(self)
        dialog.grab_set()

        frame = ttk.Frame(dialog, padding=12)
        frame.pack(fill="both", expand=True)

        ttk.Label(frame, text="carton_AB_target").grid(
            row=0,
            column=0,
            sticky="w",
            padx=(0, 8),
            pady=8,
        )

        entry = ttk.Entry(frame, width=16)
        entry.grid(row=0, column=1, sticky="w", pady=8)
        entry.insert(0, str(self.current_carton_AB_target))
        entry.focus_set()
        entry.select_range(0, tk.END)

        button_frame = ttk.Frame(frame)
        button_frame.grid(
            row=1,
            column=0,
            columnspan=2,
            sticky="e",
            pady=(16, 0),
        )

        ttk.Button(
            button_frame,
            text="Save",
            command=lambda: self._save_carton_ab_target_from_dialog(
                entry,
                dialog,
            ),
        ).pack(side="right")

        ttk.Button(
            button_frame,
            text="Cancel",
            command=dialog.destroy,
        ).pack(side="right", padx=(0, 8))

    def _save_carton_ab_target_from_dialog(self, entry, dialog):
        try:
            value = float(entry.get().strip())

            if value <= 0:
                raise ValueError("carton_AB_target must be > 0.")

            self.current_carton_AB_target = value

            dialog.destroy()
            self.status_var.set(f"Carton A/B target set to {value}")

        except Exception as exc:
            messagebox.showerror("Invalid carton A/B target", str(exc))
            
            
    def _cartoner_values_dict(self):
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

    def open_cartoner_settings_editor(self):
        dialog = tk.Toplevel(self)

        dialog.title("Dati astucciatrice")
        dialog.geometry("520x320")
        dialog.transient(self)
        dialog.grab_set()

        content = ttk.Frame(
            dialog,
            padding=12,
        )
        content.pack(
            fill="both",
            expand=True,
        )

        form_frame, popup_entries = build_cartoner_settings_form(
            content,
            entry_width=14,
        )

        form_frame.pack(
            fill="both",
            expand=True,
        )

        for field_name in CARTONER_FIELDS:
            popup_entries[field_name].insert(
                0,
                self.cartoner_entries[field_name].get(),
            )

        buttons = ttk.Frame(content)
        buttons.pack(
            fill="x",
            pady=(12, 0),
        )

        ttk.Button(
            buttons,
            text="Save",
            command=lambda: self._save_cartoner_popup(
                popup_entries,
                dialog,
            ),
        ).pack(side="right")

        ttk.Button(
            buttons,
            text="Cancel",
            command=dialog.destroy,
        ).pack(side="right", padx=(0, 8))
        
    def _save_cartoner_popup(
        self,
        popup_entries,
        dialog,
    ):
        try:
            for field_name in CARTONER_FIELDS:
                self.cartoner_entries[field_name].delete(0, "end")

                self.cartoner_entries[field_name].insert(
                    0,
                    popup_entries[field_name].get(),
                )

            dialog.destroy()

            self.status_var.set(
                "Cartoner settings updated"
            )

        except Exception as exc:
            messagebox.showerror(
                "Cartoner settings",
                str(exc),
            )
    # ------------------------------------------------------------------
    # Optimization
    # ------------------------------------------------------------------
    def run_optimization(self):
        try:
            overrides = {
                "number_of_results_to_show":
                    self.current_number_of_results_to_show,

                "carton_AB_target":
                    self.current_carton_AB_target,
            }

            overrides.update(
                self._cartoner_values_dict()
            )
            
            settings = parse_global_settings(
                self.global_entries,
                overrides={
                    "number_of_results_to_show": (
                        self.current_number_of_results_to_show
                    ),
                    "carton_AB_target": self.current_carton_AB_target,
                },
            )

            weights = self.current_weights
            stick_types = parse_stick_types(self.stick_table.get_rows())
            formats = parse_formats(self.format_table.get_rows())

            solutions, candidates_by_format = optimize(
                settings,
                stick_types,
                formats,
                weights,
            )

        except Exception as exc:
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
                "No feasible complete multi-format solution exists.\n\n"
                + counts,
            )

            self.status_var.set("No feasible solution")
            return

        self.status_var.set(
            f"Optimization complete: {len(solutions)} solution(s) shown"
        )

    def _on_solution_selected(self, _event=None):
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

    def _open_selected_format_popup(self, _event=None):
        if self.selected_solution_index is None:
            return

        overview_tree = self.detail_widgets["format_overview_tree"]
        selected = overview_tree.selection()

        if not selected:
            return

        candidate_index = int(selected[0])
        solution = self.solutions[self.selected_solution_index]
        candidate = solution.candidates[candidate_index]

        open_format_detail_popup(self, candidate)

    # ------------------------------------------------------------------
    # Result filtering
    # ------------------------------------------------------------------
    def _open_result_column_filter(self, column_name):
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

        values_frame = ttk.LabelFrame(frame, text="Exact values", padding=8)
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
            text="Select all",
            command=lambda: self._set_filter_value_checks(value_vars, True),
        ).pack(side="left")

        ttk.Button(
            select_buttons,
            text="Deselect all",
            command=lambda: self._set_filter_value_checks(value_vars, False),
        ).pack(side="left", padx=(8, 0))

        condition_frame = ttk.LabelFrame(
            frame,
            text="Optional condition",
            padding=8,
        )
        condition_frame.pack(fill="x", pady=(0, 8))

        ttk.Label(condition_frame, text="Operator").grid(
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

        ttk.Label(condition_frame, text="Value").grid(
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
            text="Apply",
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
            text="Clear column",
            command=lambda: self._clear_single_column_filter(column_name, dialog),
        ).pack(side="right", padx=(0, 8))

        ttk.Button(
            button_frame,
            text="Clear all",
            command=lambda: self._clear_all_result_filters(dialog),
        ).pack(side="right", padx=(0, 8))

    def _set_filter_value_checks(self, value_vars, state):
        for var in value_vars.values():
            var.set(state)

    def _unique_display_values_for_result_column(self, column_name):
        values = set()

        for solution in self.solutions:
            values.add(result_display_value(solution, column_name))

        return sorted(values, key=self._sort_filter_value)

    def _sort_filter_value(self, value):
        try:
            return (0, float(value))
        except Exception:
            return (1, str(value))

    def _apply_column_filter_from_popup(
        self,
        column_name,
        all_values,
        value_vars,
        operator_combo,
        value_entry,
        dialog,
    ):
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

    def _clear_single_column_filter(self, column_name, dialog=None):
        if column_name in self.active_result_filters:
            del self.active_result_filters[column_name]

        if dialog is not None:
            dialog.destroy()

        self._apply_result_filters()

    def _clear_all_result_filters(self, dialog=None):
        self.active_result_filters = {}

        if dialog is not None:
            dialog.destroy()

        self._apply_result_filters()

    def _apply_result_filters(self):
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

    def _solution_passes_all_filters(self, solution):
        for column_name, result_filter in self.active_result_filters.items():
            if not self._solution_passes_filter(solution, column_name, result_filter):
                return False

        return True

    def _solution_passes_filter(self, solution, column_name, result_filter):
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
        except Exception as exc:
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

    def _solution_value_for_filter_column(self, solution, column_name):
        attr_name = FILTERABLE_RESULT_COLUMNS[column_name]
        return getattr(solution, attr_name, 0.0)

    # ------------------------------------------------------------------
    # Export
    # ------------------------------------------------------------------
    def export_summary(self):
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

    def export_details(self):
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
