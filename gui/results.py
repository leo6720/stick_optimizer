import tkinter as tk
from tkinter import ttk
from typing import Optional


RESULT_COLUMNS = (
    "rank",
    "score",
    "cartoner_pitch",
    "pocket_types",
    "head_types",
    "max_layers",
    "layer_penalty",
    "carryover_penalty",
    "grouping_penalty",
    "stability_penalty",
    "carton_ab_penalty",
)


RESULT_HEADINGS = {
    "rank": "Posizione",
    "score": "Punteggio",
    "cartoner_pitch": "Passo astucciatrice",
    "pocket_types": "Tipi cassetto",
    "head_types": "Tipi testa",
    "max_layers": "Max strati",
    "layer_penalty": "Pen. strati",
    "carryover_penalty": "Pen. riporto",
    "grouping_penalty": "Pen. raggrup.",
    "stability_penalty": "Pen. stabilità",
    "carton_ab_penalty": "Pen. A/B",
}


RESULT_WIDTHS = {
    "rank": 45,
    "score": 85,
    "cartoner_pitch": 95,
    "pocket_types": 85,
    "head_types": 80,
    "max_layers": 75,
    "layer_penalty": 75,
    "carryover_penalty": 75,
    "grouping_penalty": 75,
    "stability_penalty": 85,
    "carton_ab_penalty": 75,
}


FILTERABLE_RESULT_COLUMNS = {
    "score": "score",
    "cartoner_pitch": "cartoner_pitch",
    "pocket_types": "number_of_pocket_types",
    "head_types": "number_of_robot_head_types",
    "max_layers": "max_layers",
    "layer_penalty": "total_layer_penalty",
    "carryover_penalty": "total_carryover_penalty",
    "grouping_penalty": "total_grouping_penalty",
    "stability_penalty": "total_stability_width_penalty",
    "carton_ab_penalty": "total_carton_ab_ratio_penalty",
}


FILTER_OPERATORS = (
    "",
    "<=",
    ">=",
    "<",
    ">",
    "=",
    "!=",
    "contains",
)


FORMAT_OVERVIEW_COLUMNS = (
    "format",
    "stick_type",
    "input_pitch",
    "grouping",
    "dividers",
    "pockets_per_pitch",
    "pocket",
    "pocket_length",
    "layers",
    "stack_height",
    "carton_A",
    "carton_B",
    "carton_ab_ratio",
    "carton_ab_penalty",
    "carryover",
    "head_type",
    "pocket_type",
)


POCKET_TYPE_COLUMNS = (
    "pocket_width",
    "pocket_height",
    "pocket_length",
    "dividers",
    "pockets_per_pitch",
    "used_by",
)


ROBOT_HEAD_TYPE_COLUMNS = (
    "grouping",
    "adjusted_input_pitch",
    "used_by",
)


def fmt(value):
    if isinstance(value, float):
        return f"{value:.4f}".rstrip("0").rstrip(".")
    return str(value)


def clear_tree(tree) -> None:
    for item in tree.get_children():
        tree.delete(item)


def result_display_value(solution, column_name):
    """Return the value displayed in the top-results table for a solution."""
    values = {
        "score": solution.score,
        "cartoner_pitch": solution.cartoner_pitch,
        "pocket_types": solution.number_of_pocket_types,
        "head_types": solution.number_of_robot_head_types,
        "max_layers": solution.max_layers,
        "layer_penalty": solution.total_layer_penalty,
        "carryover_penalty": solution.total_carryover_penalty,
        "grouping_penalty": solution.total_grouping_penalty,
        "stability_penalty": solution.total_stability_width_penalty,
        "carton_ab_penalty": getattr(
            solution,
            "total_carton_ab_ratio_penalty",
            0.0,
        ),
    }

    return fmt(values[column_name])


def build_results_section(parent, on_select_callback, on_header_filter_callback):
    frame = ttk.LabelFrame(
        parent,
        text="Migliori soluzioni - clicca sull'intestazione per filtrare",
        padding=8,
    )

    tree = ttk.Treeview(
        frame,
        columns=RESULT_COLUMNS,
        show="headings",
        height=8,
    )

    for col in RESULT_COLUMNS:
        tree.heading(
            col,
            text=RESULT_HEADINGS[col],
            command=lambda c=col: on_header_filter_callback(c),
        )
        tree.column(col, width=RESULT_WIDTHS[col], anchor="center")

    tree.pack(fill="both", expand=True)
    tree.bind("<<TreeviewSelect>>", on_select_callback)

    return frame, tree


def update_result_headings_for_filters(tree, active_filters):
    for col in RESULT_COLUMNS:
        label = RESULT_HEADINGS[col]

        if col in active_filters:
            label = f"{label} *"

        tree.heading(col, text=label)


def populate_results(tree, solutions, solution_indices=None) -> None:
    """Populate top results.

    Treeview item iid is the original index in self.solutions.
    This keeps selection working after filtering.
    """
    clear_tree(tree)

    if solution_indices is None:
        solution_indices = list(range(len(solutions)))

    for display_rank, solution_index in enumerate(solution_indices, start=1):
        solution = solutions[solution_index]

        tree.insert(
            "",
            "end",
            iid=str(solution_index),
            values=(
                display_rank,
                fmt(solution.score),
                fmt(solution.cartoner_pitch),
                solution.number_of_pocket_types,
                solution.number_of_robot_head_types,
                solution.max_layers,
                fmt(solution.total_layer_penalty),
                fmt(solution.total_carryover_penalty),
                fmt(solution.total_grouping_penalty),
                fmt(solution.total_stability_width_penalty),
                fmt(getattr(solution, "total_carton_ab_ratio_penalty", 0.0)),
            ),
        )


def build_detail_section(parent, on_format_open_callback):
    """Build selected solution area.

    Layout:
    - Solution summary
    - Format overview
    - Pocket types used
    - Robot head types used

    Full format details are opened in a popup on double click.
    """
    frame = ttk.LabelFrame(parent, text="Dettagli soluzione selezionata", padding=8)

    summary_frame = ttk.LabelFrame(frame, text="Riepilogo soluzione", padding=8)
    summary_frame.pack(fill="x", pady=(0, 8))

    summary_fields = [
        "score",
        "cartoner_pitch",
        "number_of_pocket_types",
        "number_of_robot_head_types",
        "max_layers",
        "total_layer_penalty",
        "total_carryover_penalty",
        "total_grouping_penalty",
        "total_stability_width_penalty",
        "total_carton_ab_ratio_penalty",
    ]

    summary_translations = {
        "score": "punteggio",
        "cartoner_pitch": "passo astucciatrice",
        "number_of_pocket_types": "numero tipi cassetto",
        "number_of_robot_head_types": "numero tipi testa robot",
        "max_layers": "max strati",
        "total_layer_penalty": "penalità totale strati",
        "total_carryover_penalty": "penalità totale riporto",
        "total_grouping_penalty": "penalità totale raggruppamento",
        "total_stability_width_penalty": "penalità totale stabilità",
        "total_carton_ab_ratio_penalty": "penalità totale rapporto A/B",
    }

    summary_vars = {}
    summary_name_labels = {}
    summary_fields_order = list(summary_fields)

    for index, field_name in enumerate(summary_fields):
        row = index // 3
        base_col = (index % 3) * 2

        name_label = ttk.Label(
            summary_frame,
            text=summary_translations.get(field_name, field_name.replace("_", " ")),
        )

        name_label.grid(
            row=row,
            column=base_col,
            sticky="w",
            padx=(0, 4),
            pady=2,
        )

        value_label = ttk.Label(summary_frame, text="-", width=18)
        value_label.grid(
            row=row,
            column=base_col + 1,
            sticky="w",
            padx=(0, 16),
            pady=2,
        )

        summary_vars[field_name] = value_label
        summary_name_labels[field_name] = name_label

    overview_frame = ttk.LabelFrame(
        frame,
        text="Panoramica formati - doppio clic su una riga per i dettagli completi",
        padding=8,
    )
    overview_frame.pack(fill="both", expand=True, pady=(0, 8))

    overview_tree = ttk.Treeview(
        overview_frame,
        columns=FORMAT_OVERVIEW_COLUMNS,
        show="headings",
        height=7,
    )

    overview_headings = {
        "format": "Formato",
        "stick_type": "Tipo stick",
        "input_pitch": "Passo ingr.",
        "grouping": "Raggrup.",
        "dividers": "Div.",
        "pockets_per_pitch": "Cass/Passo",
        "pocket": "Cassetto L x A",
        "pocket_length": "Lungh. cassetto",
        "layers": "Strati",
        "stack_height": "Alt. impil.",
        "carton_A": "A",
        "carton_B": "B",
        "carton_ab_ratio": "A/B",
        "carton_ab_penalty": "Pen. A/B",
        "carryover": "Riporto",
        "head_type": "Tipo testa",
        "pocket_type": "Tipo cassetto",
    }

    overview_widths = {
        "format": 90,
        "stick_type": 90,
        "input_pitch": 80,
        "grouping": 60,
        "dividers": 50,
        "pockets_per_pitch": 70,
        "pocket": 90,
        "pocket_length": 90,
        "layers": 60,
        "stack_height": 70,
        "carton_A": 60,
        "carton_B": 60,
        "carton_ab_ratio": 60,
        "carton_ab_penalty": 75,
        "carryover": 80,
        "head_type": 110,
        "pocket_type": 160,
    }

    for col in FORMAT_OVERVIEW_COLUMNS:
        overview_tree.heading(col, text=overview_headings[col])
        overview_tree.column(col, width=overview_widths[col], anchor="center")

    overview_scroll_x = ttk.Scrollbar(
        overview_frame,
        orient="horizontal",
        command=overview_tree.xview,
    )
    overview_tree.configure(xscrollcommand=overview_scroll_x.set)

    overview_tree.pack(fill="both", expand=True)
    overview_scroll_x.pack(fill="x")

    overview_tree.bind("<Double-1>", on_format_open_callback)

    commonality_frame = ttk.Frame(frame)
    commonality_frame.pack(fill="both", expand=True)

    pocket_frame = ttk.LabelFrame(
        commonality_frame,
        text="Tipi di cassetti utilizzati",
        padding=8,
    )
    pocket_frame.pack(side="left", fill="both", expand=True, padx=(0, 4))

    head_frame = ttk.LabelFrame(
        commonality_frame,
        text="Tipi di teste robot utilizzate",
        padding=8,
    )
    head_frame.pack(side="left", fill="both", expand=True, padx=(4, 0))

    pocket_tree = _build_pocket_type_table(pocket_frame)
    head_tree = _build_robot_head_type_table(head_frame)

    widgets = {
        "summary_frame": summary_frame,
        "summary_vars": summary_vars,
        "summary_name_labels": summary_name_labels,
        "summary_fields_order": summary_fields_order,
        "format_overview_tree": overview_tree,
        "pocket_type_tree": pocket_tree,
        "robot_head_type_tree": head_tree,
    }

    return frame, widgets


def _build_pocket_type_table(parent):
    tree = ttk.Treeview(
        parent,
        columns=POCKET_TYPE_COLUMNS,
        show="headings",
        height=6,
    )

    headings = {
        "pocket_width": "larghezza cassetto",
        "pocket_height": "altezza cassetto",
        "pocket_length": "lunghezza cassetto",
        "dividers": "divisori",
        "pockets_per_pitch": "cass/passo",
        "used_by": "usato da formati",
    }

    widths = {
        "pocket_width": 90,
        "pocket_height": 90,
        "pocket_length": 90,
        "dividers": 70,
        "pockets_per_pitch": 70,
        "used_by": 160,
    }

    for col in POCKET_TYPE_COLUMNS:
        tree.heading(col, text=headings[col])
        tree.column(col, width=widths[col], anchor="center")

    tree.column("used_by", anchor="w")
    tree.pack(fill="both", expand=True)

    return tree


def _build_robot_head_type_table(parent):
    tree = ttk.Treeview(
        parent,
        columns=ROBOT_HEAD_TYPE_COLUMNS,
        show="headings",
        height=6,
    )

    headings = {
        "grouping": "raggruppamento",
        "adjusted_input_pitch": "passo ingresso",
        "used_by": "usato da formati",
    }

    widths = {
        "grouping": 80,
        "adjusted_input_pitch": 100,
        "used_by": 260,
    }

    for col in ROBOT_HEAD_TYPE_COLUMNS:
        tree.heading(col, text=headings[col])
        tree.column(col, width=widths[col], anchor="center")

    tree.column("used_by", anchor="w")
    tree.pack(fill="both", expand=True)

    return tree


def clear_solution_details(widgets) -> None:
    for label in widgets["summary_vars"].values():
        label.configure(text="-")

    clear_tree(widgets["format_overview_tree"])
    clear_tree(widgets["pocket_type_tree"])
    clear_tree(widgets["robot_head_type_tree"])


def populate_solution_details(widgets, solution) -> None:
    """Populate summary, format overview and commonality tables."""
    summary_vars = widgets["summary_vars"]
    overview_tree = widgets["format_overview_tree"]
    pocket_tree = widgets["pocket_type_tree"]
    head_tree = widgets["robot_head_type_tree"]

    summary_values = {
        "score": solution.score,
        "cartoner_pitch": solution.cartoner_pitch,
        "number_of_pocket_types": solution.number_of_pocket_types,
        "number_of_robot_head_types": solution.number_of_robot_head_types,
        "max_layers": solution.max_layers,
        "total_layer_penalty": solution.total_layer_penalty,
        "total_carryover_penalty": solution.total_carryover_penalty,
        "total_grouping_penalty": solution.total_grouping_penalty,
        "total_stability_width_penalty": solution.total_stability_width_penalty,
        "total_carton_ab_ratio_penalty": getattr(
            solution,
            "total_carton_ab_ratio_penalty",
            0.0,
        ),
    }

    for key, value in summary_values.items():
        if key in summary_vars:
            summary_vars[key].configure(text=fmt(value))

    clear_tree(overview_tree)
    clear_tree(pocket_tree)
    clear_tree(head_tree)

    max_b_by_pocket = {}
    for candidate in solution.candidates:
        b_val = getattr(candidate, "carton_B_mm", 0.0) or 0.0
        max_b_by_pocket[candidate.pocket_type] = max(max_b_by_pocket.get(candidate.pocket_type, 0.0), b_val)

    for index, candidate in enumerate(solution.candidates):
        carton_b = getattr(candidate, "carton_B_mm", 0.0) or 0.0
        pocket_height = max_b_by_pocket.get(candidate.pocket_type, carton_b)
        pocket_wh = f"{fmt(candidate.pocket_width)} x {fmt(pocket_height)}"
        carryover = "sì" if candidate.carryover_required else "no"

        overview_tree.insert(
            "",
            "end",
            iid=str(index),
            values=(
                candidate.format_name,
                candidate.stick_type_name,
                fmt(candidate.adjusted_input_pitch),
                candidate.grouping,
                candidate.dividers,
                candidate.pockets_per_pitch,
                pocket_wh,
                fmt(candidate.pocket_length),
                candidate.layers,
                fmt(candidate.stack_height),
                fmt(getattr(candidate, "carton_A_mm", "")),
                fmt(carton_b),
                fmt(getattr(candidate, "carton_AB_ratio", "")),
                fmt(getattr(candidate, "carton_AB_ratio_penalty", "")),
                carryover,
                candidate.robot_head_type,
                candidate.pocket_type,
            ),
        )

    _populate_pocket_type_commonality(
        pocket_tree,
        solution.candidates,
    )

    _populate_robot_head_type_commonality(
        head_tree,
        solution.candidates,
    )


def _populate_pocket_type_commonality(tree, candidates):
    grouped = {}
    max_b_by_pocket = {}

    for candidate in candidates:
        grouped.setdefault(candidate.pocket_type, []).append(candidate.format_name)
        b_val = getattr(candidate, "carton_B_mm", 0.0) or 0.0
        max_b_by_pocket[candidate.pocket_type] = max(max_b_by_pocket.get(candidate.pocket_type, 0.0), b_val)

    for pocket_type, format_names in sorted(
        grouped.items(),
        key=lambda item: str(item[0]),
    ):
        pocket_width, pocket_length, dividers, pockets_per_pitch = pocket_type
        pocket_height = max_b_by_pocket.get(pocket_type, 0.0)

        tree.insert(
            "",
            "end",
            values=(
                fmt(pocket_width),
                fmt(pocket_height),
                fmt(pocket_length),
                dividers,
                pockets_per_pitch,
                ", ".join(format_names),
            ),
        )


def _populate_robot_head_type_commonality(tree, candidates):
    grouped = {}

    for candidate in candidates:
        grouped.setdefault(candidate.robot_head_type, []).append(candidate.format_name)

    for robot_head_type, format_names in sorted(
        grouped.items(),
        key=lambda item: str(item[0]),
    ):
        grouping, adjusted_input_pitch = robot_head_type

        tree.insert(
            "",
            "end",
            values=(
                grouping,
                fmt(adjusted_input_pitch),
                ", ".join(format_names),
            ),
        )


def open_format_detail_popup(parent, candidate, pocket_height: Optional[float] = None, show_details: bool = True) -> None:
    """Open a popup with the complete detail for one format candidate."""
    window = tk.Toplevel(parent)
    window.title(f"Dettaglio formato - {candidate.format_name}")
    window.geometry("760x680")
    window.minsize(640, 500)
    window.transient(parent)

    main_frame = ttk.Frame(window, padding=10)
    main_frame.pack(fill="both", expand=True)

    header = ttk.Label(
        main_frame,
        text=f"{candidate.format_name} / {candidate.stick_type_name}",
        font=("TkDefaultFont", 11, "bold"),
    )
    header.pack(anchor="w", pady=(0, 8))

    content_pane = ttk.PanedWindow(main_frame, orient="vertical")
    content_pane.pack(fill="both", expand=True, pady=(0, 8))

    param_frame = ttk.Frame(content_pane)
    vis_frame = ttk.Frame(content_pane)

    content_pane.add(vis_frame, weight=1)
    content_pane.add(param_frame, weight=1)

    tree = ttk.Treeview(
        param_frame,
        columns=("parameter", "value"),
        show="headings",
    )

    tree.heading("parameter", text="parametro")
    tree.heading("value", text="valore")

    tree.column("parameter", width=200, anchor="w")
    tree.column("value", width=180, anchor="w")

    scrollbar = ttk.Scrollbar(param_frame, orient="vertical", command=tree.yview)
    tree.configure(yscrollcommand=scrollbar.set)

    tree.pack(side="left", fill="both", expand=True)
    scrollbar.pack(side="right", fill="y")

    populate_format_detail(tree, candidate, show_details=show_details)

    canvas = tk.Canvas(vis_frame, bg="white", highlightthickness=0)
    canvas.pack(fill="both", expand=True)

    def draw_pocket_canvas(event=None):
        canvas.delete("all")
        width = canvas.winfo_width()
        height = canvas.winfo_height()
        if width < 50 or height < 50:
            width = 380
            height = 500

        pw = getattr(candidate, "pocket_width", 100.0) or 100.0
        carton_b = getattr(candidate, "carton_B_mm", 0.0) or 0.0
        dividers = getattr(candidate, "dividers", 0) or 0
        grouping = getattr(candidate, "grouping", 1) or 1
        layers = getattr(candidate, "layers", 1) or 1
        stack_height = getattr(candidate, "stack_height", 0.0) or 0.0
        pitch = getattr(candidate, "cartoner_pitch", 0.0) or (pw * 1.5)
        pockets_count = getattr(candidate, "pockets_per_pitch", 1) or 1

        pocket_h = pocket_height if pocket_height and pocket_height > 0 else (carton_b if carton_b > 0 else max(stack_height * 1.2, 40.0))

        margin_x = 75
        margin_y = 65

        draw_w = width - 2 * margin_x
        draw_h = height - 2 * margin_y

        scale_px_per_mm = min(draw_w / max(pitch, pw, 1.0), draw_h / max(pocket_h, 1.0))
        pitch_px = pitch * scale_px_per_mm
        rect_w = pw * scale_px_per_mm
        rect_h = pocket_h * scale_px_per_mm

        origin_x = (width - pitch_px) / 2
        origin_y = (height - rect_h) / 2

        wall_width = max(3, int(2.0 * scale_px_per_mm * 0.1))

        # Pitch space calculation for pockets distribution
        pocket_spacing = pitch_px / pockets_count

        # Render each pocket inside this pitch
        for p_idx in range(pockets_count):
            p_origin_x = origin_x + p_idx * pocket_spacing

            # Fill pocket interior
            canvas.create_rectangle(
                p_origin_x, origin_y,
                p_origin_x + rect_w, origin_y + rect_h,
                outline="", fill="#ffffff"
            )

            # Draw squared U pocket walls (left wall -> bottom -> right wall, open top)
            canvas.create_line(
                p_origin_x, origin_y,
                p_origin_x, origin_y + rect_h,
                p_origin_x + rect_w, origin_y + rect_h,
                p_origin_x + rect_w, origin_y,
                fill="#111111", width=wall_width, capstyle=tk.PROJECTING, joinstyle=tk.MITER
            )

            compartments = dividers + 1
            comp_w = rect_w / compartments
            if dividers > 0:
                for i in range(1, compartments):
                    dx = p_origin_x + i * comp_w
                    canvas.create_line(
                        dx, origin_y, dx, origin_y + rect_h,
                        fill="#111111", width=wall_width
                    )

            stick_h_mm = (stack_height / max(layers, 1)) if stack_height > 0 else (pocket_h * 0.6 / max(layers, 1))
            scaled_layer_h = stick_h_mm * scale_px_per_mm
            base_y = origin_y + rect_h
            sticks_per_comp = max(1, grouping // compartments)

            for layer in range(layers):
                ly_bottom = base_y - layer * scaled_layer_h
                ly_top = ly_bottom - scaled_layer_h
                for comp in range(compartments):
                    comp_start_x = p_origin_x + comp * comp_w
                    stick_slot_w = comp_w / max(sticks_per_comp, 1)
                    for s in range(sticks_per_comp):
                        sx = comp_start_x + s * stick_slot_w + 2
                        sy = ly_top + 2
                        sw = stick_slot_w - 4
                        sh = scaled_layer_h - 4
                        if sw > 2 and sh > 2:
                            canvas.create_oval(
                                sx, sy, sx + sw, sy + sh,
                                fill="#73a6ff", outline="#1d4ed8", width=1.5
                            )

            # Pocket width annotation on first pocket
            if p_idx == 0:
                canvas.create_line(p_origin_x, origin_y - 20, p_origin_x + rect_w, origin_y - 20, arrow=tk.BOTH, fill="#4b5563", width=1.2)
                canvas.create_text(p_origin_x + rect_w / 2, origin_y - 32, text=f"L: {fmt(pw)} mm", fill="#1f2937", font=("TkDefaultFont", 8, "bold"))

        # Wall marking start of the next pitch (same height & thickness as pocket wall)
        next_pitch_x = origin_x + pitch_px
        canvas.create_line(
            next_pitch_x, origin_y,
            next_pitch_x, origin_y + rect_h,
            fill="#111111", width=wall_width
        )

        # Full pitch dimension annotation line & label
        canvas.create_line(origin_x, origin_y + rect_h + 25, next_pitch_x, origin_y + rect_h + 25, arrow=tk.BOTH, fill="#111827", width=1.5)
        canvas.create_text(origin_x + pitch_px / 2, origin_y + rect_h + 40, text=f"Passo: {fmt(pitch)} mm", fill="#111827", font=("TkDefaultFont", 9, "bold"))

        # Summary info text at the top
        info_str = f"Cassetti/Passo: {pockets_count} | Raggruppamento: {grouping} | Divisori: {dividers} | Strati: {layers}"
        canvas.create_text(width / 2, 15, text=info_str, fill="#1f2937", font=("TkDefaultFont", 9, "bold"))

        # Height dimension annotation on the left
        canvas.create_line(origin_x - 20, origin_y, origin_x - 20, origin_y + rect_h, arrow=tk.BOTH, fill="#4b5563", width=1.2)
        canvas.create_text(origin_x - 48, origin_y + rect_h / 2, text=f"Altezza: {fmt(pocket_h)} mm", fill="#1f2937", font=("TkDefaultFont", 9, "bold"), angle=90)

    canvas.bind("<Configure>", draw_pocket_canvas)

    button_frame = ttk.Frame(window, padding=(10, 0, 10, 10))
    button_frame.pack(fill="x")

    ttk.Button(
        button_frame,
        text="Chiudi",
        command=window.destroy,
    ).pack(side="right")


def populate_format_detail(tree, candidate, show_details: bool = True) -> None:
    clear_tree(tree)

    sections = [
        (
            "Trasferimento",
            [
                ("nome formato", candidate.format_name),
                ("tipo stick", candidate.stick_type_name),
                ("passo ingresso regolato", candidate.adjusted_input_pitch),
                ("raggruppamento", candidate.grouping),
                ("depositi per set", candidate.deposits_per_set),
                ("cassetti per passo", candidate.pockets_per_pitch),
                ("passo cassetto", candidate.pocket_pitch),
                ("passo astucciatrice", candidate.cartoner_pitch),
            ],
        ),
        (
            "Cassetto",
            [
                ("larghezza cassetto", candidate.pocket_width),
                ("lunghezza cassetto", candidate.pocket_length),
                ("divisori", candidate.dividers),
                ("larghezza occupata", candidate.occupied_width),
                ("spazio non utilizzato", candidate.unused_space),
                ("tipo cassetto", candidate.pocket_type),
            ],
        ),
        (
            "Pila e riporto",
            [
                ("strati", candidate.layers),
                ("altezza pila", candidate.stack_height),
                (
                    "riporto richiesto",
                    "sì" if candidate.carryover_required else "no",
                ),
                ("lunghezza ciclo riporto", candidate.carryover_cycle_length),
            ],
        ),
    ]

    if show_details:
        sections.extend([
            (
                "Astuccio A/B",
                [
                    ("larghezza A astuccio", getattr(candidate, "carton_A_mm", "")),
                    ("altezza B astuccio", getattr(candidate, "carton_B_mm", "")),
                    ("rapporto A/B astuccio", getattr(candidate, "carton_AB_ratio", "")),
                    (
                        "penalità rapporto A/B",
                        getattr(candidate, "carton_AB_ratio_penalty", ""),
                    ),
                ],
            ),
            (
                "Stabilità e penalità",
                [
                    ("larghezza non supportata effettiva", candidate.effective_unsupported_width),
                    ("rapporto larghezza", candidate.width_ratio),
                    ("penalità strati", candidate.layer_penalty),
                    ("penalità riporto", candidate.carryover_penalty),
                    ("penalità raggruppamento", candidate.grouping_penalty),
                    ("penalità larghezza stabilità", candidate.stability_width_penalty),
                ],
            ),
        ])

    sections.append(
        (
            "Tipi",
            [
                ("tipo testa robot", candidate.robot_head_type),
                ("tipo cassetto", candidate.pocket_type),
            ],
        )
    )

    for section_name, rows in sections:
        parent = tree.insert(
            "",
            "end",
            values=(section_name, ""),
            open=True,
        )

        for name, value in rows:
            tree.insert(
                parent,
                "end",
                values=(name, fmt(value)),
            )
