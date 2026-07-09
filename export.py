import csv
from pathlib import Path

from models import Solution


SUMMARY_FIELDS = [
    "rank",
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

DETAIL_FIELDS = [
    "format_name",
    "stick_type_name",
    "adjusted_input_pitch",
    "grouping",
    "dividers",
    "pockets_per_pitch",
    "pocket_width",
    "pocket_length",
    "pocket_pitch",
    "cartoner_pitch",
    "occupied_width",
    "unused_space",
    "layers",
    "stack_height",
    "carryover_required",
    "carryover_cycle_length",
    "effective_unsupported_width",
    "width_ratio",
    "layer_penalty",
    "carryover_penalty",
    "grouping_penalty",
    "stability_width_penalty",
    "pocket_type",
    "robot_head_type",
    "carton_A_mm",
    "carton_B_mm",
    "carton_AB_ratio",
    "carton_AB_ratio_penalty",
]


def export_solution_summary_csv(
    solutions: list[Solution],
    file_path: str | Path,
) -> None:
    path = Path(file_path)

    with path.open("w", newline="", encoding="utf-8-sig") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=SUMMARY_FIELDS)
        writer.writeheader()

        for rank, solution in enumerate(solutions, start=1):
            writer.writerow(
                {
                    "rank": rank,
                    "score": solution.score,
                    "cartoner_pitch": solution.cartoner_pitch,
                    "number_of_pocket_types": solution.number_of_pocket_types,
                    "number_of_robot_head_types": solution.number_of_robot_head_types,
                    "max_layers": solution.max_layers,
                    "total_layer_penalty": solution.total_layer_penalty,
                    "total_carryover_penalty": solution.total_carryover_penalty,
                    "total_grouping_penalty": solution.total_grouping_penalty,
                    "total_stability_width_penalty": solution.total_stability_width_penalty,
                    "total_carton_ab_ratio_penalty": solution.total_carton_ab_ratio_penalty,
                }
            )


def export_solution_details_csv(
    solution: Solution,
    file_path: str | Path,
) -> None:
    path = Path(file_path)

    with path.open("w", newline="", encoding="utf-8-sig") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=DETAIL_FIELDS)
        writer.writeheader()

        for candidate in solution.candidates:
            writer.writerow(
                {
                    "format_name": candidate.format_name,
                    "stick_type_name": candidate.stick_type_name,
                    "adjusted_input_pitch": candidate.adjusted_input_pitch,
                    "grouping": candidate.grouping,
                    "dividers": candidate.dividers,
                    "pockets_per_pitch": candidate.pockets_per_pitch,
                    "pocket_width": candidate.pocket_width,
                    "pocket_length": candidate.pocket_length,
                    "pocket_pitch": candidate.pocket_pitch,
                    "cartoner_pitch": candidate.cartoner_pitch,
                    "occupied_width": candidate.occupied_width,
                    "unused_space": candidate.unused_space,
                    "layers": candidate.layers,
                    "stack_height": candidate.stack_height,
                    "carryover_required": "yes" if candidate.carryover_required else "no",
                    "carryover_cycle_length": candidate.carryover_cycle_length,
                    "effective_unsupported_width": candidate.effective_unsupported_width,
                    "width_ratio": candidate.width_ratio,
                    "layer_penalty": candidate.layer_penalty,
                    "carryover_penalty": candidate.carryover_penalty,
                    "grouping_penalty": candidate.grouping_penalty,
                    "stability_width_penalty": candidate.stability_width_penalty,
                    "pocket_type": candidate.pocket_type,
                    "robot_head_type": candidate.robot_head_type,
                    "carton_A_mm": candidate.carton_A_mm,
                    "carton_B_mm": candidate.carton_B_mm,
                    "carton_AB_ratio": candidate.carton_AB_ratio,
                    "carton_AB_ratio_penalty": candidate.carton_AB_ratio_penalty,
                }
            )
