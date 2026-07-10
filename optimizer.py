from collections import defaultdict
from functools import lru_cache
from itertools import product
from math import ceil
from typing import Callable, Optional

from defaults import (
    ALLOWED_GROUPINGS,
    ALLOWED_POCKETS_PER_PITCH,
    CARRYOVER_FIXED_PENALTY,
    COMPARTMENT_LAYOUT,
)

from models import Candidate, Format, GlobalSettings, Solution, StickType, Weights
from pareto import pareto_filter
from scoring import (
    build_solution,
    calculate_carryover,
    calculate_carton_ab_ratio_penalty,
    calculate_grouping_penalty,
    calculate_layer_penalty,
    calculate_stability_penalty,
)
from validation import validate_all


class CompartmentCache:
    """Caches compartment calculations to avoid redundant computation."""
    
    _cache = {}
    
    @classmethod
    def get_compartments(cls, grouping: int, dividers: int) -> list[int]:
        """Get cached compartments or compute and cache them."""
        key = (grouping, dividers)
        if key not in cls._cache:
            compartments = COMPARTMENT_LAYOUT.get(key)
            if compartments is None:
                raise ValueError(f"Invalid compartment layout: grouping={grouping}, dividers={dividers}")
            cls._cache[key] = compartments
        return cls._cache[key]
    
    @classmethod
    def clear(cls) -> None:
        """Clear cache (mainly for testing)."""
        cls._cache.clear()


def generate_valid_pitches(
    nominal_pitch: float,
    max_shift: float,
    pitch_step: float,
) -> list[float]:
    """Generate discrete pitch-step multiples inside the allowed pitch window."""
    lower = nominal_pitch - max_shift
    upper = nominal_pitch + max_shift

    first_step = ceil((lower - 1e-9) / pitch_step)
    last_step = int((upper + 1e-9) // pitch_step)

    pitches: list[float] = []

    for step_index in range(first_step, last_step + 1):
        pitch = round(step_index * pitch_step, 6)

        if lower - 1e-9 <= pitch <= upper + 1e-9:
            pitches.append(pitch)

    return pitches


def calculate_pocket_width_with_clearances(
    grouping: int,
    dividers: int,
    stick_width_mm: float,
    divider_width_mm: float,
    clearance_between_adjacent_sticks_mm: float,
    clearance_stick_to_wall_or_divider_mm: float,
) -> int:
    """Calculate pocket width including stick clearances.
    
    Components:
    - stick physical widths
    - divider physical widths
    - clearance between adjacent sticks in the same compartment
    - clearance between sticks and pocket walls/dividers
    
    The number of wall/divider clearances is two per compartment.
    """
    compartments = CompartmentCache.get_compartments(grouping, dividers)

    adjacent_stick_gaps = sum(max(0, count - 1) for count in compartments)
    wall_or_divider_clearances = 2 * len(compartments)

    pocket_width = (
        grouping * stick_width_mm
        + dividers * divider_width_mm
        + adjacent_stick_gaps * clearance_between_adjacent_sticks_mm
        + wall_or_divider_clearances * clearance_stick_to_wall_or_divider_mm
    )

    return int(round(pocket_width))


def calculate_effective_unsupported_width_with_clearances(
    grouping: int,
    dividers: int,
    stick_width_mm: float,
    clearance_between_adjacent_sticks_mm: float,
    clearance_stick_to_wall_or_divider_mm: float,
) -> float:
    """Calculate maximum unsupported compartment width including clearances."""
    compartments = CompartmentCache.get_compartments(grouping, dividers)

    compartment_widths = []

    for stick_count in compartments:
        width = (
            stick_count * stick_width_mm
            + max(0, stick_count - 1) * clearance_between_adjacent_sticks_mm
            + 2 * clearance_stick_to_wall_or_divider_mm
        )
        compartment_widths.append(width)

    return float(max(compartment_widths))


def compute_candidate(
    fmt: Format,
    stick: StickType,
    settings: GlobalSettings,
    adjusted_input_pitch: float,
    grouping: int,
    dividers: int,
    pockets_per_pitch: int,
) -> Optional[Candidate]:
    """Compute one candidate and return None if any hard feasibility rule fails."""
    if settings.sticks_per_beat % grouping != 0:
        return None

    pocket_width = calculate_pocket_width_with_clearances(
        grouping=grouping,
        dividers=dividers,
        stick_width_mm=stick.stick_width_mm,
        divider_width_mm=settings.divider_width_mm,
        clearance_between_adjacent_sticks_mm=settings.clearance_between_adjacent_sticks_mm,
        clearance_stick_to_wall_or_divider_mm=(
            settings.clearance_stick_to_wall_or_divider_mm
        ),
    )

    pocket_length = stick.stick_length_mm

    pocket_pitch = grouping * adjusted_input_pitch
    cartoner_pitch = pockets_per_pitch * pocket_pitch

    if cartoner_pitch > settings.max_cartoner_pitch_mm + 1e-9:
        return None

    step_ratio = cartoner_pitch / settings.pitch_step_mm

    if abs(step_ratio - round(step_ratio)) > 1e-6:
        return None

    occupied_width = pockets_per_pitch * (
        pocket_width + 2 * settings.pocket_wall_width_mm
    )

    if occupied_width > cartoner_pitch + 1e-9:
        return None

    unused_space = cartoner_pitch - occupied_width

    layers = int(ceil(fmt.sticks_per_pocket / grouping))
    stack_height = layers * stick.stick_thickness_mm
    
    carton_A_mm = float(pocket_width)
    carton_B_mm = stack_height + settings.carton_B_extra_mm

    if carton_B_mm <= 0:
        return None

    carton_AB_ratio = carton_A_mm / carton_B_mm
    carton_AB_ratio_penalty = calculate_carton_ab_ratio_penalty(
        carton_ab_ratio=carton_AB_ratio,
        carton_ab_target=settings.carton_AB_target,
    )
    
    if settings.max_allowed_layers is not None:
        if layers > settings.max_allowed_layers:
            return None

    carryover_required, carryover_cycle_length, carryover_penalty = calculate_carryover(
        fmt.sticks_per_pocket,
        grouping,
        CARRYOVER_FIXED_PENALTY,
    )

    effective_width = calculate_effective_unsupported_width_with_clearances(
        grouping=grouping,
        dividers=dividers,
        stick_width_mm=stick.stick_width_mm,
        clearance_between_adjacent_sticks_mm=(
            settings.clearance_between_adjacent_sticks_mm
        ),
        clearance_stick_to_wall_or_divider_mm=(
            settings.clearance_stick_to_wall_or_divider_mm
        ),
    )

    width_ratio = effective_width / stick.stick_length_mm
    stability_penalty = calculate_stability_penalty(width_ratio)
    layer_penalty = calculate_layer_penalty(layers, dividers)
    grouping_penalty = calculate_grouping_penalty(grouping)

    pocket_type = (pocket_width, pocket_length, dividers, pockets_per_pitch)
    robot_head_type = (grouping, adjusted_input_pitch)

    return Candidate(
        format_name=fmt.format_name,
        stick_type_name=stick.stick_type_name,
        adjusted_input_pitch=adjusted_input_pitch,
        grouping=grouping,
        dividers=dividers,
        pockets_per_pitch=pockets_per_pitch,
        pocket_width=pocket_width,
        pocket_length=pocket_length,
        pocket_pitch=pocket_pitch,
        cartoner_pitch=cartoner_pitch,
        occupied_width=occupied_width,
        unused_space=unused_space,
        layers=layers,
        stack_height=stack_height,
        carton_A_mm=carton_A_mm,
        carton_B_mm=carton_B_mm,
        carton_AB_ratio=carton_AB_ratio,
        carton_AB_ratio_penalty=carton_AB_ratio_penalty,
        carryover_required=carryover_required,
        carryover_cycle_length=carryover_cycle_length,
        carryover_penalty=carryover_penalty,
        effective_unsupported_width=effective_width,
        width_ratio=width_ratio,
        stability_width_penalty=stability_penalty,
        layer_penalty=layer_penalty,
        grouping_penalty=grouping_penalty,
        deposits_per_set=int(settings.sticks_per_beat / grouping),
        pocket_type=pocket_type,
        robot_head_type=robot_head_type,
    )


def generate_candidates_for_format(
    fmt: Format,
    stick: StickType,
    settings: GlobalSettings,
) -> list[Candidate]:
    """Generate all feasible static transfer configurations for one format."""
    nominal_pitch = 2 * stick.stick_width_mm + 2 * stick.fin_length_mm

    effective_max_pitch_shift_mm = (
        ceil(settings.max_pitch_shift_mm * 2.0 / (settings.sticks_per_beat - 1))
    )

    valid_pitches = generate_valid_pitches(
        nominal_pitch=nominal_pitch,
        max_shift=effective_max_pitch_shift_mm,
        pitch_step=settings.pitch_step_mm,
    )

    candidates: list[Candidate] = []

    for adjusted_pitch in valid_pitches:
        for grouping in ALLOWED_GROUPINGS:
            if settings.sticks_per_beat % grouping != 0:
                continue

            # Get allowed dividers for this grouping from defaults
            allowed_dividers = {
                1: [0],
                2: [0, 1],
                3: [0, 2],
                4: [0, 1],
            }[grouping]

            for dividers in allowed_dividers:
                for pockets_per_pitch in ALLOWED_POCKETS_PER_PITCH:
                    candidate = compute_candidate(
                        fmt=fmt,
                        stick=stick,
                        settings=settings,
                        adjusted_input_pitch=adjusted_pitch,
                        grouping=grouping,
                        dividers=dividers,
                        pockets_per_pitch=pockets_per_pitch,
                    )

                    if candidate is not None:
                        candidates.append(candidate)

    return candidates


def build_multi_format_solutions(
    candidates_by_format: dict[str, list[Candidate]],
    weights: Weights,
) -> list[Solution]:
    """Build complete solutions containing exactly one candidate per format.

    A complete solution is feasible only if all selected candidates share one
    common cartoner_pitch.
    """
    if not candidates_by_format:
        return []

    by_format_and_pitch: dict[str, dict[float, list[Candidate]]] = {}

    for format_name, candidates in candidates_by_format.items():
        pitch_map: dict[float, list[Candidate]] = defaultdict(list)

        for candidate in candidates:
            pitch_map[candidate.cartoner_pitch].append(candidate)

        by_format_and_pitch[format_name] = dict(pitch_map)

    format_names = list(candidates_by_format.keys())

    common_pitches = set(by_format_and_pitch[format_names[0]].keys())

    for name in format_names[1:]:
        common_pitches &= set(by_format_and_pitch[name].keys())

    solutions: list[Solution] = []

    for pitch in sorted(common_pitches):
        candidate_lists = [by_format_and_pitch[name][pitch] for name in format_names]

        for combination in product(*candidate_lists):
            solutions.append(build_solution(list(combination), weights))

    return solutions


def optimize(
    settings: GlobalSettings,
    stick_types: list[StickType],
    formats: list[Format],
    weights: Weights,
    progress_callback: Optional[Callable[[str], None]] = None,
) -> tuple[list[Solution], dict[str, list[Candidate]]]:
    """Run complete optimization and return ranked Pareto solutions plus candidates.
    
    Args:
        settings: Global optimization settings
        stick_types: List of stick type definitions
        formats: List of commercial formats
        weights: Scoring weights
        progress_callback: Optional callback for progress updates
    
    Returns:
        Tuple of (ranked solutions, candidates by format)
    """
    errors = validate_all(settings, stick_types, formats, weights)

    if errors:
        raise ValueError("Invalid input:\n" + "\n".join(errors))

    stick_by_name = {stick.stick_type_name: stick for stick in stick_types}

    candidates_by_format: dict[str, list[Candidate]] = {}

    for fmt in formats:
        if progress_callback:
            progress_callback(f"Generating candidates for {fmt.format_name}...")
        
        stick = stick_by_name[fmt.stick_type_name]
        candidates_by_format[fmt.format_name] = generate_candidates_for_format(
            fmt,
            stick,
            settings,
        )

    if any(len(candidates) == 0 for candidates in candidates_by_format.values()):
        if progress_callback:
            progress_callback("No candidates found for one or more formats.")
        return [], candidates_by_format

    if progress_callback:
        progress_callback("Building multi-format solutions...")
    
    all_solutions = build_multi_format_solutions(candidates_by_format, weights)

    if not all_solutions:
        if progress_callback:
            progress_callback("No feasible complete solutions found.")
        return [], candidates_by_format

    if progress_callback:
        progress_callback(f"Pareto filtering {len(all_solutions)} solutions...")
    
    efficient = pareto_filter(all_solutions)

    efficient.sort(
        key=lambda s: (
            s.score,
            s.number_of_pocket_types,
            s.number_of_robot_head_types,
            s.total_layer_penalty,
            s.total_carryover_penalty,
            s.total_grouping_penalty,
            s.total_stability_width_penalty,
            s.total_carton_ab_ratio_penalty,
            s.cartoner_pitch,
        )
    )

    result_count = int(settings.number_of_results_to_show)
    if progress_callback:
        progress_callback(f"Optimization complete. Top {result_count} solutions ready.")
    
    return efficient[:result_count], candidates_by_format
