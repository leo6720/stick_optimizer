from defaults import (
    DEFAULT_FORMATS,
    DEFAULT_GLOBAL_SETTINGS,
    DEFAULT_STICK_TYPES,
    DEFAULT_WEIGHTS,
)
from optimizer import generate_candidates_for_format, optimize


def fmt_float(value: float) -> str:
    return f"{value:.3f}".rstrip("0").rstrip(".")


def print_solution_summary(solutions):
    print("\nTOP SOLUTIONS")
    print("=" * 100)

    header = (
        "Rank | Score | Pitch | Pocket types | Head types | Max layers | "
        "Layer pen | Carry pen | Group pen | Stability pen"
    )

    print(header)
    print("-" * len(header))

    for rank, solution in enumerate(solutions, start=1):
        print(
            f"{rank:>4} | "
            f"{fmt_float(solution.score):>8} | "
            f"{fmt_float(solution.cartoner_pitch):>5} | "
            f"{solution.number_of_pocket_types:>12} | "
            f"{solution.number_of_robot_head_types:>10} | "
            f"{solution.max_layers:>10} | "
            f"{fmt_float(solution.total_layer_penalty):>9} | "
            f"{fmt_float(solution.total_carryover_penalty):>9} | "
            f"{fmt_float(solution.total_grouping_penalty):>9} | "
            f"{fmt_float(solution.total_stability_width_penalty):>13}"
        )


def print_best_solution_details(solution):
    print("\nBEST SOLUTION DETAILS")
    print("=" * 100)

    for candidate in solution.candidates:
        print(f"\nFormat: {candidate.format_name} / Stick type: {candidate.stick_type_name}")
        print(f"  adjusted_input_pitch     : {fmt_float(candidate.adjusted_input_pitch)} mm")
        print(f"  grouping                 : {candidate.grouping}")
        print(f"  dividers                 : {candidate.dividers}")
        print(f"  pockets_per_pitch        : {candidate.pockets_per_pitch}")
        print(f"  pocket_width             : {candidate.pocket_width} mm")
        print(f"  pocket_length            : {fmt_float(candidate.pocket_length)} mm")
        print(f"  pocket_pitch             : {fmt_float(candidate.pocket_pitch)} mm")
        print(f"  cartoner_pitch           : {fmt_float(candidate.cartoner_pitch)} mm")
        print(f"  occupied_width           : {fmt_float(candidate.occupied_width)} mm")
        print(f"  unused_space             : {fmt_float(candidate.unused_space)} mm")
        print(f"  layers                   : {candidate.layers}")
        print(f"  stack_height             : {fmt_float(candidate.stack_height)} mm")
        print(f"  carryover_required       : {'yes' if candidate.carryover_required else 'no'}")
        print(f"  carryover_cycle_length   : {candidate.carryover_cycle_length}")
        print(f"  effective_unsupported_w  : {fmt_float(candidate.effective_unsupported_width)} mm")
        print(f"  width_ratio              : {fmt_float(candidate.width_ratio)}")
        print(f"  layer_penalty            : {fmt_float(candidate.layer_penalty)}")
        print(f"  carryover_penalty        : {fmt_float(candidate.carryover_penalty)}")
        print(f"  grouping_penalty         : {fmt_float(candidate.grouping_penalty)}")
        print(f"  stability_width_penalty  : {fmt_float(candidate.stability_width_penalty)}")
        print(f"  pocket_type              : {candidate.pocket_type}")
        print(f"  robot_head_type          : {candidate.robot_head_type}")
        print(f"  carton_A_mm              : {fmt_float(candidate.carton_A_mm)} mm")
        print(f"  carton_B_mm              : {fmt_float(candidate.carton_B_mm)} mm")
        print(f"  carton_AB_ratio          : {fmt_float(candidate.carton_AB_ratio)}")
        print(f"  carton_AB_ratio_penalty  : {fmt_float(candidate.carton_AB_ratio_penalty)}")


def run_tests():
    settings = DEFAULT_GLOBAL_SETTINGS
    stick_types = DEFAULT_STICK_TYPES
    formats = DEFAULT_FORMATS
    weights = DEFAULT_WEIGHTS

    print("STICKPACK OPTIMIZER CORE TEST")
    print("=" * 100)
    print(f"Formats: {[f.format_name for f in formats]}")
    print(f"Stick types: {[s.stick_type_name for s in stick_types]}")
    print(f"Max allowed layers: {settings.max_allowed_layers}")

    stick_by_name = {s.stick_type_name: s for s in stick_types}

    for fmt in formats:
        candidates = generate_candidates_for_format(
            fmt,
            stick_by_name[fmt.stick_type_name],
            settings,
        )

        print(f"Candidate count for {fmt.format_name}: {len(candidates)}")

        if len(candidates) == 0:
            print(f"WARNING: no candidates generated for {fmt.format_name}")

    solutions, candidates_by_format = optimize(
        settings,
        stick_types,
        formats,
        weights,
    )

    if not solutions:
        print("\nNO FEASIBLE COMPLETE MULTI-FORMAT SOLUTION FOUND.")
        print("Candidate counts by format:")

        for name, candidates in candidates_by_format.items():
            print(f"  {name}: {len(candidates)}")

        return

    print_solution_summary(solutions)
    print_best_solution_details(solutions[0])

    # Basic non-framework assertions. If these fail, Python raises AssertionError.
    assert len(solutions) > 0, (
        "Optimizer should return at least one solution for default data."
    )

    assert all(len(s.candidates) == len(formats) for s in solutions), (
        "Each solution must cover every format."
    )

    assert all(len({c.cartoner_pitch for c in s.candidates}) == 1 for s in solutions), (
        "Each solution must have one common cartoner pitch."
    )

    assert all(
        c.occupied_width <= c.cartoner_pitch + 1e-9
        for s in solutions
        for c in s.candidates
    ), "Physical fit rule failed."

    print("\nTEST RESULT: PASS")


if __name__ == "__main__":
    run_tests()
