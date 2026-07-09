from models import Format, GlobalSettings, StickType, Weights


def _require_positive(value: float, name: str, errors: list[str]) -> None:
    if value <= 0:
        errors.append(f"{name} must be > 0.")


def _require_non_negative(value: float, name: str, errors: list[str]) -> None:
    if value < 0:
        errors.append(f"{name} must be >= 0.")


def validate_global_settings(settings: GlobalSettings) -> list[str]:
    errors: list[str] = []

    _require_positive(settings.sticks_per_beat, "sticks_per_beat", errors)

    if settings.sticks_per_beat != int(settings.sticks_per_beat):
        errors.append("sticks_per_beat must be an integer.")

    _require_non_negative(settings.max_pitch_shift_mm, "max_pitch_shift_mm", errors)
    _require_non_negative(settings.divider_width_mm, "divider_width_mm", errors)
    _require_non_negative(settings.pocket_wall_width_mm, "pocket_wall_width_mm", errors)
    
    _require_non_negative(
        settings.clearance_between_adjacent_sticks_mm,
        "clearance_between_adjacent_sticks_mm",
        errors,
    )

    _require_non_negative(
        settings.clearance_stick_to_wall_or_divider_mm,
        "clearance_stick_to_wall_or_divider_mm",
        errors,
    )
    
    _require_non_negative(settings.carton_B_extra_mm, "carton_B_extra_mm", errors)
    _require_positive(settings.carton_AB_target, "carton_AB_target", errors)
    _require_positive(settings.max_cartoner_pitch_mm, "max_cartoner_pitch_mm", errors)
    _require_positive(settings.pitch_step_mm, "pitch_step_mm", errors)
    _require_positive(
        settings.number_of_results_to_show,
        "number_of_results_to_show",
        errors,
    )

    if settings.number_of_results_to_show != int(settings.number_of_results_to_show):
        errors.append("number_of_results_to_show must be an integer.")
    
    if settings.max_allowed_layers is not None:
        _require_positive(settings.max_allowed_layers, "max_allowed_layers", errors)

        if settings.max_allowed_layers != int(settings.max_allowed_layers):
            errors.append("max_allowed_layers must be an integer or None.")

    return errors


def validate_stick_types(stick_types: list[StickType]) -> list[str]:
    errors: list[str] = []
    names: set[str] = set()

    if not stick_types:
        errors.append("At least one stick type is required.")
        return errors

    for index, stick in enumerate(stick_types, start=1):
        prefix = f"Stick type row {index}"

        if not stick.stick_type_name.strip():
            errors.append(f"{prefix}: stick_type_name is required.")

        if stick.stick_type_name in names:
            errors.append(f"Duplicate stick_type_name: {stick.stick_type_name}.")

        names.add(stick.stick_type_name)

        _require_positive(stick.stick_length_mm, f"{prefix} stick_length_mm", errors)
        _require_positive(stick.stick_width_mm, f"{prefix} stick_width_mm", errors)
        _require_positive(
            stick.stick_thickness_mm,
            f"{prefix} stick_thickness_mm",
            errors,
        )
        _require_non_negative(stick.fin_length_mm, f"{prefix} fin_length_mm", errors)

    return errors


def validate_formats(
    formats: list[Format],
    stick_types: list[StickType],
) -> list[str]:
    errors: list[str] = []
    stick_names = {s.stick_type_name for s in stick_types}
    format_names: set[str] = set()

    if not formats:
        errors.append("At least one format is required.")
        return errors

    for index, fmt in enumerate(formats, start=1):
        prefix = f"Format row {index}"

        if not fmt.format_name.strip():
            errors.append(f"{prefix}: format_name is required.")

        if fmt.format_name in format_names:
            errors.append(f"Duplicate format_name: {fmt.format_name}.")

        format_names.add(fmt.format_name)

        if fmt.stick_type_name not in stick_names:
            errors.append(f"{prefix}: stick type '{fmt.stick_type_name}' does not exist.")

        _require_positive(fmt.sticks_per_pocket, f"{prefix} sticks_per_pocket", errors)

        if fmt.sticks_per_pocket != int(fmt.sticks_per_pocket):
            errors.append(f"{prefix}: sticks_per_pocket must be an integer.")

    return errors


def validate_weights(weights: Weights) -> list[str]:
    errors: list[str] = []

    for field_name, value in weights.__dict__.items():
        _require_non_negative(float(value), field_name, errors)

    return errors


def validate_all(
    settings: GlobalSettings,
    stick_types: list[StickType],
    formats: list[Format],
    weights: Weights,
) -> list[str]:
    errors: list[str] = []

    errors.extend(validate_global_settings(settings))
    errors.extend(validate_stick_types(stick_types))
    errors.extend(validate_formats(formats, stick_types))
    errors.extend(validate_weights(weights))

    return errors
