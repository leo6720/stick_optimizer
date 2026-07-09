from math import gcd

from defaults import GROUPING_PENALTIES
from models import Candidate, Solution, Weights


def calculate_layer_penalty(layers: int, dividers: int) -> float:
    """Penalty for vertical stack height expressed as number of layers."""
    comfort_layers = 9 if dividers > 0 else 7
    return float(max(0, layers - comfort_layers) ** 2)


def calculate_carryover(
    sticks_per_pocket: int,
    grouping: int,
    fixed_penalty: float,
) -> tuple[bool, int, float]:
    """Return carryover required flag, cycle length, and penalty."""
    if sticks_per_pocket % grouping == 0:
        return False, 1, 0.0

    cycle_length = grouping // gcd(sticks_per_pocket, grouping)
    penalty = fixed_penalty + cycle_length
    return True, int(cycle_length), float(penalty)


def calculate_stability_penalty(width_ratio: float) -> float:
    """Progressive penalty above target unsupported width ratio of 0.8."""
    if width_ratio <= 0.8:
        return 0.0

    return float(((width_ratio - 0.8) / 0.1) ** 2)


def calculate_grouping_penalty(grouping: int) -> float:
    return float(GROUPING_PENALTIES[grouping])


def calculate_solution_score(solution: Solution, weights: Weights) -> float:
    return float(
        weights.pocket_type_weight * solution.number_of_pocket_types
        + weights.robot_head_type_weight * solution.number_of_robot_head_types
        + weights.layer_penalty_weight * solution.total_layer_penalty
        + weights.carryover_penalty_weight * solution.total_carryover_penalty
        + weights.grouping_penalty_weight * solution.total_grouping_penalty
        + weights.stability_penalty_weight * solution.total_stability_width_penalty
        + weights.cartoner_pitch_weight * solution.cartoner_pitch
        + weights.carton_ab_ratio_penalty_weight * solution.total_carton_ab_ratio_penalty
    )


def build_solution(candidates: list[Candidate], weights: Weights) -> Solution:
    """Aggregate per-format candidates into one scored multi-format solution."""
    if not candidates:
        raise ValueError("Cannot build a solution from an empty candidate list.")

    pitch = candidates[0].cartoner_pitch
    pocket_types = {c.pocket_type for c in candidates}
    robot_head_types = {c.robot_head_type for c in candidates}

    solution = Solution(
        candidates=list(candidates),
        cartoner_pitch=pitch,
        number_of_pocket_types=len(pocket_types),
        number_of_robot_head_types=len(robot_head_types),
        max_layers=max(c.layers for c in candidates),
        total_layer_penalty=sum(c.layer_penalty for c in candidates),
        total_carryover_penalty=sum(c.carryover_penalty for c in candidates),
        total_grouping_penalty=sum(c.grouping_penalty for c in candidates),
        total_stability_width_penalty=sum(c.stability_width_penalty for c in candidates),
        total_carton_ab_ratio_penalty=sum(c.carton_AB_ratio_penalty for c in candidates),
        pocket_types=pocket_types,
        robot_head_types=robot_head_types,
    )

    solution.score = calculate_solution_score(solution, weights)
    return solution
    
    
def calculate_carton_ab_ratio_penalty(
    carton_ab_ratio: float,
    carton_ab_target: float,
) -> float:
    """Penalty for deviation from target carton A/B ratio.

    A = carton width = pocket width
    B = stack height + B extra

    Penalty is normalized by the target ratio.
    """
    if carton_ab_target <= 0:
        raise ValueError("carton_AB_target must be > 0.")

    return float(((carton_ab_ratio - carton_ab_target) / carton_ab_target) ** 2)
