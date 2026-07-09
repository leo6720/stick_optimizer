import dataclasses
from tkinter import ttk


DISPLAY_LABELS = {
    "sticks_per_beat": "sticks/beat",
    "max_pitch_shift_mm": "max pitch shift",

    "divider_width_mm": "divider width",
    "pocket_wall_width_mm": "wall width",
    "clearance_between_adjacent_sticks_mm": "stick-stick clearance",
    "clearance_stick_to_wall_or_divider_mm": "stick-wall/div clearance",
    "carton_B_extra_mm": "B extra",
    "max_cartoner_pitch_mm": "max cartoner pitch",
    "pitch_step_mm": "pitch step",
    "max_allowed_layers": "max layers",
}


CARTONER_FIELDS = [
    "divider_width_mm",
    "pocket_wall_width_mm",
    "clearance_between_adjacent_sticks_mm",
    "clearance_stick_to_wall_or_divider_mm",
    "carton_B_extra_mm",
    "max_cartoner_pitch_mm",
    "pitch_step_mm",
    "max_allowed_layers",
]


def build_grouped_global_settings_form(parent, entry_width: int = 10):
    """
    Main page only shows MT fields.
    Cartoner settings are edited from menu popup.
    """
    outer_frame = ttk.Frame(parent)

    entries = {}

    mt_frame = ttk.LabelFrame(
        outer_frame,
        text="Dati MT",
        padding=8,
    )
    mt_frame.pack(fill="x")

    mt_fields = [
        "sticks_per_beat",
        "max_pitch_shift_mm",
    ]

    _add_fields_to_frame(
        mt_frame,
        mt_fields,
        entries,
        entry_width=entry_width,
        label_width=20,
    )

    return outer_frame, entries


def build_cartoner_settings_form(parent, entry_width: int = 12):
    frame = ttk.Frame(parent)

    entries = {}

    _add_fields_to_frame(
        frame,
        CARTONER_FIELDS,
        entries,
        entry_width=entry_width,
        label_width=28,
    )

    return frame, entries


def _add_fields_to_frame(
    frame,
    field_names,
    entries,
    entry_width,
    label_width,
):
    frame.grid_columnconfigure(0, weight=1)
    frame.grid_columnconfigure(1, weight=0)

    for row, field_name in enumerate(field_names):
        label = ttk.Label(
            frame,
            text=DISPLAY_LABELS.get(field_name, field_name),
            width=label_width,
            anchor="w",
        )

        label.grid(
            row=row,
            column=0,
            sticky="ew",
            padx=(0, 6),
            pady=2,
        )

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


def set_entries_from_dataclass(entries: dict, instance):
    data = dataclasses.asdict(instance)

    for name, entry in entries.items():
        entry.delete(0, "end")
        value = data.get(name)
        entry.insert(
            0,
            "" if value is None else str(value),
        )
