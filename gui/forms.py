import dataclasses
from tkinter import ttk


DISPLAY_LABELS = {
    "sticks_per_beat": "Np - Canali MT",
    "max_pitch_shift_mm": "D - Offset max stick [mm]",

    "divider_width_mm": "C_1 - Larghezza divisore",
    "pocket_wall_width_mm": "C_2 - Larghezza parete",
    "clearance_stick_to_wall_or_divider_mm": "C_3 - Gioco stick-parete/div",
    "clearance_between_adjacent_sticks_mm": "C_4 - Gioco stick-stick",
    "carton_B_extra_mm": "B_extra - Quota B aggiuntiva",
    "max_cartoner_pitch_mm": "P_tp - Passo max trasporto prodotto",
    "pitch_step_mm": "Incremento passo trasporto prodotto",
    "max_allowed_layers": "Max strati",
}


CARTONER_FIELDS = [
    "divider_width_mm",
    "pocket_wall_width_mm",
    "clearance_stick_to_wall_or_divider_mm",
    "clearance_between_adjacent_sticks_mm",
    "carton_B_extra_mm",
    "max_cartoner_pitch_mm",
    "pitch_step_mm",
    "max_allowed_layers",
]


def build_grouped_global_settings_form(parent, entry_width: int = 10, mt_image=None):
    """
    Main page only shows MT fields.
    Cartoner settings are edited from menu popup.
    """
    outer_frame = ttk.Frame(parent, style="Sidebar.TFrame", padding=12)

    entries = {}

    title_label = ttk.Label(
        outer_frame,
        text="Dati MT",
        style="SidebarHeader.TLabel",
    )
    title_label.pack(anchor="w", pady=(0, 8))

    mt_content = ttk.Frame(outer_frame, style="Sidebar.TFrame")
    mt_content.pack(fill="x")

    if mt_image is not None:
        image_label = ttk.Label(mt_content, image=mt_image, style="Sidebar.TLabel")
        image_label.image = mt_image
        image_label.grid(row=0, column=0, columnspan=2, sticky="w", pady=(0, 8))

    mt_fields = [
        "sticks_per_beat",
    ]

    start_row = 1 if mt_image is not None else 0
    _add_fields_to_frame(
        mt_content,
        mt_fields,
        entries,
        entry_width=entry_width,
        label_width=20,
        start_row=start_row,
        style_prefix="Sidebar.",
    )

    return outer_frame, entries


def build_cartoner_settings_form(parent, entry_width: int = 12, defaults: dict = None, cartoner_image=None):
    frame = ttk.Frame(parent)

    entries = {}
    labels = {}
    defaults = defaults or {}

    start_row = 0
    if cartoner_image is not None:
        image_label = ttk.Label(frame, image=cartoner_image)
        image_label.image = cartoner_image
        image_label.grid(row=0, column=0, columnspan=2, pady=(0, 20))
        start_row = 1

    _add_fields_to_frame_with_defaults(
        frame,
        CARTONER_FIELDS,
        entries,
        labels,
        defaults,
        entry_width=entry_width,
        label_width=32,
        start_row=start_row,
    )

    return frame, entries, labels


def _add_fields_to_frame(
    frame,
    field_names,
    entries,
    entry_width,
    label_width,
    start_row: int = 0,
    labels=None,
    style_prefix: str = "",
):
    frame.grid_columnconfigure(0, weight=1)
    frame.grid_columnconfigure(1, weight=0)

    label_style = f"{style_prefix}TLabel" if style_prefix else "TLabel"

    for row, field_name in enumerate(field_names, start=start_row):
        label = ttk.Label(
            frame,
            text=DISPLAY_LABELS.get(field_name, field_name),
            width=label_width,
            anchor="w",
            style=label_style,
        )

        label.grid(
            row=row,
            column=0,
            sticky="ew",
            padx=(0, 6),
            pady=2,
        )
        if isinstance(labels, dict):
            labels[field_name] = label

        entry = ttk.Entry(
            frame,
            width=entry_width,
        )

        entry.grid(
            row=row,
            column=1,
            sticky="e",
            pady=2,
        )

        entries[field_name] = entry


def _add_fields_to_frame_with_defaults(
    frame,
    field_names,
    entries,
    labels,
    defaults,
    entry_width,
    label_width,
    start_row: int = 0,
):
    frame.grid_columnconfigure(0, weight=1)
    frame.grid_columnconfigure(1, weight=0)

    style = ttk.Style()
    style.configure("Yellow.TEntry", fieldbackground="lightyellow")
    style.map("Yellow.TEntry", fieldbackground=[("active", "lightyellow"), ("!disabled", "lightyellow")])

    for row, field_name in enumerate(field_names, start=start_row):
        default_val = defaults.get(field_name, "")
        display_text = f"{DISPLAY_LABELS.get(field_name, field_name)}  ({default_val})"

        label = ttk.Label(
            frame,
            text=display_text,
            width=label_width + 12,
            anchor="w",
        )

        label.grid(
            row=row,
            column=0,
            sticky="ew",
            padx=(0, 6),
            pady=2,
        )
        if isinstance(labels, dict):
            labels[field_name] = label

        entry = ttk.Entry(
            frame,
            width=entry_width,
        )

        entry.grid(
            row=row,
            column=1,
            sticky="e",
            pady=2,
        )

        def make_validation(e, d):
            def check(*_args):
                try:
                    current_val = e.get().strip()
                    expected = str(d).strip()
                    if current_val != expected:
                        e.configure(style="Yellow.TEntry")
                        try:
                            e.config(background="lightyellow")
                        except Exception:
                            pass
                    else:
                        e.configure(style="TEntry")
                        try:
                            e.config(background="white")
                        except Exception:
                            pass
                except Exception:
                    pass
            return check

        validation_cmd = make_validation(entry, default_val)
        entry.bind("<KeyRelease>", validation_cmd)
        # Check initial state on creation
        validation_cmd()

        entries[field_name] = entry


def set_entries_from_dataclass(entries: dict, instance):
    data = dataclasses.asdict(instance)

    for name, entry in entries.items():
        entry.delete(0, "end")
        value = data.get(name)
        entry.insert(
            0,
            "" if value is None else str(value),
        )
        try:
            entry.event_generate("<KeyRelease>")
        except Exception:
            pass
