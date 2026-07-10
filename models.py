from dataclasses import dataclass, field
from typing import List, Optional, Tuple


@dataclass(frozen=True)
class StickType:
    stick_type_name: str
    stick_length_mm: float
    stick_width_mm: float
    stick_thickness_mm: float
    fin_length_mm: float


@dataclass(frozen=True)
class Format:
    format_name: str
    stick_type_name: str
    sticks_per_pocket: int


@dataclass(frozen=True)
class Weights:
    pocket_type_weight: float = 300.0
    robot_head_type_weight: float = 200.0
    layer_penalty_weight: float = 300.0
    carryover_penalty_weight: float = 30.0
    grouping_penalty_weight: float = 10.0
    stability_penalty_weight: float = 30.0
    cartoner_pitch_weight: float = 0.5
    carton_ab_ratio_penalty_weight: float = 600.0


@dataclass(frozen=True)
class GlobalSettings:
    sticks_per_beat: int
    max_pitch_shift_mm: float
    divider_width_mm: float
    pocket_wall_width_mm: float
    clearance_between_adjacent_sticks_mm: float = 1.0
    clearance_stick_to_wall_or_divider_mm: float = 2.0
    carton_B_extra_mm: float = 5.0
    max_cartoner_pitch_mm: float = 300.0
    pitch_step_mm: float = 5.0
    number_of_results_to_show: int = 20
    max_allowed_layers: Optional[int] = None
    carton_AB_target: float = 1.5


@dataclass(frozen=True)
class Candidate:
    format_name: str
    stick_type_name: str
    adjusted_input_pitch: float
    grouping: int
    dividers: int
    pockets_per_pitch: int
    pocket_width: int
    pocket_length: float
    pocket_pitch: float
    cartoner_pitch: float
    occupied_width: float
    unused_space: float
    layers: int
    stack_height: float
    carton_A_mm: float
    carton_B_mm: float
    carton_AB_ratio: float
    carton_AB_ratio_penalty: float
    carryover_required: bool
    carryover_cycle_length: int
    carryover_penalty: float
    effective_unsupported_width: float
    width_ratio: float
    stability_width_penalty: float
    layer_penalty: float
    grouping_penalty: float
    deposits_per_set: int
    pocket_type: Tuple[int, float, int, int]
    robot_head_type: Tuple[int, float]


@dataclass
class Solution:
    candidates: List[Candidate]
    cartoner_pitch: float
    number_of_pocket_types: int
    number_of_robot_head_types: int
    max_layers: int
    total_layer_penalty: float
    total_carryover_penalty: float
    total_grouping_penalty: float
    total_stability_width_penalty: float
    total_carton_ab_ratio_penalty: float
    score: float = 0.0
    pocket_types: set = field(default_factory=set)
    robot_head_types: set = field(default_factory=set)

    def metric_tuple(self) -> Tuple[float, float, float, float, float, float, float]:
        """Metrics used for Pareto comparison. Lower is better for all values."""
        return (
            self.number_of_pocket_types,
            self.number_of_robot_head_types,
            self.total_layer_penalty,
            self.total_carryover_penalty,
            self.total_grouping_penalty,
            self.total_stability_width_penalty,
            self.total_carton_ab_ratio_penalty,
        )
