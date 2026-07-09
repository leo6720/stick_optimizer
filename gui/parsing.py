import dataclasses

from models import Format, GlobalSettings, StickType, Weights


INTEGER_FIELDS = {
    "sticks_per_beat",
    "number_of_results_to_show",
    "max_allowed_layers",
}


OPTIONAL_FIELDS = {
    "max_allowed_layers",
}


def parse_global_settings(entries: dict, overrides: dict | None = None) -> GlobalSettings:
    values = {}
    overrides = overrides or {}

    for field in dataclasses.fields(GlobalSettings):
        if field.name in overrides:
            values[field.name] = overrides[field.name]
            continue

        if field.name not in entries:
            raise ValueError(f"{field.name} is not available in the GUI and has no override.")

        raw = entries[field.name].get().strip()
        values[field.name] = parse_value(raw, field.name)

    return GlobalSettings(**values)


def parse_weights(entries: dict) -> Weights:
    values = {}

    for field in dataclasses.fields(Weights):
        raw = entries[field.name].get().strip()

        if raw == "":
            raise ValueError(f"{field.name} is required.")

        values[field.name] = float(raw)

    return Weights(**values)


def parse_stick_types(rows: list[tuple]) -> list[StickType]:
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


def parse_value(raw: str, field_name: str):
    if raw == "":
        if field_name in OPTIONAL_FIELDS:
            return None

        raise ValueError(f"{field_name} is required.")

    if field_name in INTEGER_FIELDS:
        return int(float(raw))

    return float(raw)
