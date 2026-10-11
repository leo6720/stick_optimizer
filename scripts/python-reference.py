"""Generate small, bounded Python fixtures for the Rust migration.

Run from the repository root. Uses incorporated settings, never user_defaults.
Each case estimates its Cartesian product before calling optimize().
"""
import sys
from pathlib import Path
from collections import Counter
from dataclasses import replace
from math import prod

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from defaults import DEFAULT_GLOBAL_SETTINGS, DEFAULT_WEIGHTS
from models import StickType, Format
from optimizer import generate_candidates_for_format, optimize

root = Path(__file__).resolve().parents[1]
cases = [
    ("multi", DEFAULT_GLOBAL_SETTINGS,
     [StickType("A", 110, 25, 6, 5), StickType("B", 100, 26, 8, 5)],
     [Format("A10", "A", 10), Format("A34", "A", 34), Format("B7", "B", 7)]),
    ("group1", replace(DEFAULT_GLOBAL_SETTINGS, sticks_per_beat=11),
     [StickType("A", 110, 25, 6, 5)], [Format("A4", "A", 4)]),
    ("half", DEFAULT_GLOBAL_SETTINGS,
     [StickType("A", 110, 25.5, 6, 4.5)], [Format("A7", "A", 7)]),
    ("decimal", replace(DEFAULT_GLOBAL_SETTINGS, pitch_step_mm=0.1, max_pitch_shift_mm=0),
     [StickType("A", 110, 25, 6, 5)], [Format("A7", "A", 7)]),
]
lines = ["# kind case format/input/group/dividers/pockets + geometry and penalties; generated from python-beta-reference"]
for name, settings, sticks, formats in cases:
    settings = replace(settings, number_of_results_to_show=10000)
    by_name = {s.stick_type_name: s for s in sticks}
    candidates = [generate_candidates_for_format(f, by_name[f.stick_type_name], settings) for f in formats]
    maps = [Counter(c.cartoner_pitch for c in group) for group in candidates]
    common = set(maps[0]).intersection(*(set(m) for m in maps[1:]))
    estimated = sum(prod(m[p] for m in maps) for p in common)
    assert 0 < estimated < 10000, (name, estimated)
    solutions, _ = optimize(settings, sticks, formats, DEFAULT_WEIGHTS)
    assert solutions, name
    for group in candidates:
        for c in group:
            values = [c.format_name, c.adjusted_input_pitch, c.grouping, c.dividers,
                      c.pockets_per_pitch, c.pocket_width, c.pocket_length, c.pocket_pitch,
                      c.cartoner_pitch, c.occupied_width, c.unused_space, c.layers,
                      c.stack_height, c.carton_A_mm, c.carton_B_mm, c.carton_AB_ratio,
                      int(c.carryover_required), c.carryover_cycle_length,
                      c.effective_unsupported_width, c.width_ratio, c.deposits_per_set,
                      c.robot_head_width, c.layer_penalty, c.carryover_penalty,
                      c.grouping_penalty, c.stability_width_penalty, c.carton_AB_ratio_penalty]
            lines.append("\t".join(map(str, ["C", name, *values])))
    for s in solutions:
        signature = ";".join(f"{c.format_name}:{c.adjusted_input_pitch}:{c.grouping}:{c.dividers}:{c.pockets_per_pitch}" for c in s.candidates)
        lines.append("\t".join(map(str, ["S", name, signature, s.cartoner_pitch, *s.metric_tuple(), s.max_layers, s.score])))
    lines.append("\t".join(map(str, ["R", name, estimated, len(solutions)])))
    print(f"{name}: {sum(map(len, candidates))} candidates, {estimated} combinations, {len(solutions)} Pareto solutions")
path = root / "crates/core/tests/fixtures/python-reference.tsv"
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text("\n".join(lines) + "\n", encoding="utf-8")
