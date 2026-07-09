import tkinter as tk
from tkinter import ttk


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
    "rank": "rank",
    "score": "score",
    "cartoner_pitch": "cartoner pitch",
    "pocket_types": "pocket types",
    "head_types": "head types",
    "max_layers": "max layers",
    "layer_penalty": "layer pen",
    "carryover_penalty": "carry pen",
    "grouping_penalty": "group pen",
    "stability_penalty": "stability pen",
    "carton_ab_penalty": "A/B pen",
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
        text="Top solutions - click a column header to filter",
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
    frame = ttk.LabelFrame(parent, text="Selected solution details", padding=8)

    summary_frame = ttk.LabelFrame(frame, text="Solution summary", padding=8)
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

    summary_vars = {}

    for index, field_name in enumerate(summary_fields):
        row = index // 3
        base_col = (index % 3) * 2

        ttk.Label(summary_frame, text=field_name).grid(
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

    overview_frame = ttk.LabelFrame(
        frame,
        text="Format overview - double click a row for full format detail",
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
        "format": "format",
        "stick_type": "stick type",
        "input_pitch": "input pitch",
        "grouping": "group",
        "dividers": "div",
        "pockets_per_pitch": "p/pitch",
        "pocket": "pocket W x L",
        "layers": "layers",
        "stack_height": "stack H",
        "carton_A": "A",
        "carton_B": "B",
        "carton_ab_ratio": "A/B",
        "carton_ab_penalty": "A/B pen",
        "carryover": "carryover",
        "head_type": "head type",
        "pocket_type": "pocket type",
    }

    overview_widths = {
        "format": 90,
        "stick_type": 90,
        "input_pitch": 80,
        "grouping": 60,
        "dividers": 50,
        "pockets_per_pitch": 70,
        "pocket": 90,
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
        text="Pocket types used",
        padding=8,
    )
    pocket_frame.pack(side="left", fill="both", expand=True, padx=(0, 4))

    head_frame = ttk.LabelFrame(
        commonality_frame,
        text="Robot head types used",
        padding=8,
    )
    head_frame.pack(side="left", fill="both", expand=True, padx=(4, 0))

    pocket_tree = _build_pocket_type_table(pocket_frame)
    head_tree = _build_robot_head_type_table(head_frame)

    widgets = {
        "summary_vars": summary_vars,
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
        "pocket_width": "pocket width",
        "pocket_length": "pocket length",
        "dividers": "dividers",
        "pockets_per_pitch": "p/pitch",
        "used_by": "used by formats",
    }

    widths = {
        "pocket_width": 90,
        "pocket_length": 90,
        "dividers": 70,
        "pockets_per_pitch": 70,
        "used_by": 220,
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
        "grouping": "grouping",
        "adjusted_input_pitch": "input pitch",
        "used_by": "used by formats",
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

    for index, candidate in enumerate(solution.candidates):
        pocket_size = f"{fmt(candidate.pocket_width)} x {fmt(candidate.pocket_length)}"
        carryover = "yes" if candidate.carryover_required else "no"

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
                pocket_size,
                candidate.layers,
                fmt(candidate.stack_height),
                fmt(getattr(candidate, "carton_A_mm", "")),
                fmt(getattr(candidate, "carton_B_mm", "")),
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

    for candidate in candidates:
        grouped.setdefault(candidate.pocket_type, []).append(candidate.format_name)

    for pocket_type, format_names in sorted(
        grouped.items(),
        key=lambda item: str(item[0]),
    ):
        pocket_width, pocket_length, dividers, pockets_per_pitch = pocket_type

        tree.insert(
            "",
            "end",
            values=(
                fmt(pocket_width),
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


def open_format_detail_popup(parent, candidate) -> None:
    """Open a popup with the complete detail for one format candidate."""
    window = tk.Toplevel(parent)
    window.title(f"Format detail - {candidate.format_name}")
    window.geometry("720x650")
    window.minsize(620, 480)
    window.transient(parent)

    frame = ttk.Frame(window, padding=10)
    frame.pack(fill="both", expand=True)

    header = ttk.Label(
        frame,
        text=f"{candidate.format_name} / {candidate.stick_type_name}",
        font=("TkDefaultFont", 11, "bold"),
    )
    header.pack(anchor="w", pady=(0, 8))

    tree = ttk.Treeview(
        frame,
        columns=("parameter", "value"),
        show="headings",
    )

    tree.heading("parameter", text="parameter")
    tree.heading("value", text="value")

    tree.column("parameter", width=260, anchor="w")
    tree.column("value", width=390, anchor="w")

    scrollbar = ttk.Scrollbar(frame, orient="vertical", command=tree.yview)
    tree.configure(yscrollcommand=scrollbar.set)

    tree.pack(side="left", fill="both", expand=True)
    scrollbar.pack(side="right", fill="y")

    populate_format_detail(tree, candidate)

    button_frame = ttk.Frame(window, padding=(10, 0, 10, 10))
    button_frame.pack(fill="x")

    ttk.Button(
        button_frame,
        text="Close",
        command=window.destroy,
    ).pack(side="right")


def populate_format_detail(tree, candidate) -> None:
    clear_tree(tree)

    sections = [
        (
            "Transfer",
            [
                ("format name", candidate.format_name),
                ("stick type", candidate.stick_type_name),
                ("adjusted input pitch", candidate.adjusted_input_pitch),
                ("grouping", candidate.grouping),
                ("deposits per set", candidate.deposits_per_set),
                ("pockets per pitch", candidate.pockets_per_pitch),
                ("pocket pitch", candidate.pocket_pitch),
                ("cartoner pitch", candidate.cartoner_pitch),
            ],
        ),
        (
            "Pocket",
            [
                ("pocket width", candidate.pocket_width),
                ("pocket length", candidate.pocket_length),
                ("dividers", candidate.dividers),
                ("occupied width", candidate.occupied_width),
                ("unused space", candidate.unused_space),
                ("pocket type", candidate.pocket_type),
            ],
        ),
        (
            "Stack and carryover",
            [
                ("layers", candidate.layers),
                ("stack height", candidate.stack_height),
                (
                    "carryover required",
                    "yes" if candidate.carryover_required else "no",
                ),
                ("carryover cycle length", candidate.carryover_cycle_length),
            ],
        ),
        (
            "Carton A/B",
            [
                ("carton A width", getattr(candidate, "carton_A_mm", "")),
                ("carton B height", getattr(candidate, "carton_B_mm", "")),
                ("carton A/B ratio", getattr(candidate, "carton_AB_ratio", "")),
                (
                    "carton A/B penalty",
                    getattr(candidate, "carton_AB_ratio_penalty", ""),
                ),
            ],
        ),
        (
            "Stability and penalties",
            [
                ("effective unsupported width", candidate.effective_unsupported_width),
                ("width ratio", candidate.width_ratio),
                ("layer penalty", candidate.layer_penalty),
                ("carryover penalty", candidate.carryover_penalty),
                ("grouping penalty", candidate.grouping_penalty),
                ("stability width penalty", candidate.stability_width_penalty),
            ],
        ),
        (
            "Types",
            [
                ("robot head type", candidate.robot_head_type),
                ("pocket type", candidate.pocket_type),
            ],
        ),
    ]

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
