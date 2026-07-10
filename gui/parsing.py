import dataclasses
from typing import Any, Dict, Optional, Type, TypeVar

from models import Format, GlobalSettings, StickType, Weights


INTEGER_FIELDS = {
    "sticks_per_beat",
    "number_of_results_to_show",
    "max_allowed_layers",
}

OPTIONAL_FIELDS = {
    "max_allowed_layers",
}

T = TypeVar('T')


def parse_value(raw: str, field_name: str) -> Any:
    """Parse a single field value from string input."""
    if raw == "":
        if field_name in OPTIONAL_FIELDS:
            return None
        raise ValueError(f"{field_name} is required.")

    if field_name in INTEGER_FIELDS:
        return int(float(raw))

    return float(raw)


def parse_dataclass_from_entries(
    dataclass_type: Type[T],
    entries: Dict[str, Any],
    overrides: Optional[Dict[str, Any]] = None,
) -> T:
    """Generic parser for any dataclass from GUI entry widgets.
    
    Args:
        dataclass_type: The dataclass to instantiate
        entries: Dict mapping field names to entry widgets (with .get() method)
        overrides: Optional dict of values to override entry values
    
    Returns:
        Instance of dataclass_type
    """
    values = {}
    overrides = overrides or {}

    for field in dataclasses.fields(dataclass_type):
        if field.name in overrides:
            values[field.name] = overrides[field.name]
            continue

        if field.name not in entries:
            raise ValueError(f"{field.name} is not available in the GUI and has no override.")

        raw = entries[field.name].get().strip()
        values[field.name] = parse_value(raw, field.name)

    return dataclass_type(**values)


def parse_global_settings(
    entries: Dict[str, Any],
    overrides: Optional[Dict[str, Any]] = None,
) -> GlobalSettings:
    """Parse GlobalSettings from GUI entries."""
    return parse_dataclass_from_entries(GlobalSettings, entries, overrides)


def parse_weights(entries: Dict[str, Any]) -> Weights:
    """Parse Weights from GUI entries."""
    values = {}

    for field in dataclasses.fields(Weights):
        raw = entries[field.name].get().strip()

        if raw == "":
            raise ValueError(f"{field.name} is required.")

        values[field.name] = float(raw)

    return Weights(**values)


def parse_stick_types(rows: list[tuple]) -> list[StickType]:
    """Parse stick types from table rows."""
    stick_types = []

    for row_index, row in enumerate(rows, start=1):
        try:
            name, length, width, thickness, fin = row

            stick_types.append(
                StickType(
                    stick_type_name=str(name).strip(),
                    stick_length_mm=float(length),
                    stick_width_mm=float(width),
                    stick_thickness_mm=float(thickness),
                    fin_length_mm=float(fin),
                )
            )

        except Exception as exc:
            raise ValueError(f"Invalid stick type row {row_index}: {exc}") from exc

    return stick_types


def parse_formats(rows: list[tuple]) -> list[Format]:
    """Parse formats from table rows."""
    formats = []

    for row_index, row in enumerate(rows, start=1):
        try:
            name, stick_type_name, sticks_per_pocket = row

            formats.append(
                Format(
                    format_name=str(name).strip(),
                    stick_type_name=str(stick_type_name).strip(),
                    sticks_per_pocket=int(float(sticks_per_pocket)),
                )
            )

        except Exception as exc:
            raise ValueError(f"Invalid format row {row_index}: {exc}") from exc

    return formats
