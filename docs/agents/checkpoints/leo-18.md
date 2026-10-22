# LEO-18 — exact final-head search

- Issue: https://linear.app/leochist/issue/LEO-18
- Implementer: `/root/leo18_implementer`, actually spawned with `fork_turns:none`.
- Ownership: only LEO-18, isolated `/workspace/stick-leo-18`, `codex/leo-18-async`.
- Baseline: `f2956653e5f6bb80d638fdbd28eab8cef50ef087` (merged prerequisite PR21).
- Implementation head: `b727d58` (this checkpoint-only commit does not change code).
- Phase: implementation verified, awaiting two new independent reviewers on fixed head.
- Review cycles: **0**. Open findings: none independently reviewed yet.
- Accepted scope: LEO16 definition 20 October, sizing 21 October, 140k child of 390k/3.
- No LEO19/7, merge, deployment, new automation or Linear state change by implementer.
- PR: coordinator will create one after fixed-head review coordination.

## Scope and technical decisions

Every structural head partition is enumerated, jointly checked and rebuilt with
effective drawer geometry before Pareto. Explicit solution-local head IDs and
full assignment geometries replace the old key collapse. Drawers include g4's
fixed central gap, local stick/divider coordinates and upward integer rounding
so required clearances are never rounded away. Existing penalty/score formulas
and weights are preserved with effective dimensions; group1 excluded.
Physical pocket identity includes divider locations. Engine marker changes to
`rust-final-heads-v2`; schema2/migration unchanged. Details: ADR0002.
No final parameter editor, rich CSV redesign, or head popup implemented.

## Actual verification

Read-only preflight on exact baseline: 26 core/IO tests and full workspace Linux
check passed offline. Product/Cargo/toolchain diff vs reviewed498bdbe was empty.
Pinned Rust1.99.0, cached Cargo.lock dependencies, Linux and MSVC target installed.
First public g4 regression RED: old107mm instead of approved141mm. GREEN:
141mm drawer, exact schematic stick x5/31/91/117mm.

All Cargo commands used `CARGO_HOME=/workspace/scratch/rust-cargo`,
`RUSTUP_HOME=/workspace/scratch/rust-toolchains`, shared
`CARGO_TARGET_DIR=/workspace/stick-rust-validation/target`, pinned cargo executable,
`--locked --offline`. Desktop Linux checks additionally enable `eframe/x11`
without changing Cargo manifests.

| Check | Actual result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS after final test addition |
| `cargo check --workspace --all-targets --all-features --features eframe/x11 --locked --offline` | PASS |
| `cargo clippy --workspace --all-targets --all-features --features eframe/x11 --locked --offline -- -D warnings` | PASS, repeated after final test addition |
| `cargo test --workspace --all-features --features eframe/x11 --locked --offline` | PASS **66 tests:23 core+34 desktop+9 IO** |
| Final `cargo test -p stick-optimizer-core --test head_search --locked --offline` | PASS **8 tests**, including one additional shared-drawer constraint regression after the 66-test full run |
| `cargo build --workspace --release --features eframe/x11 --locked --offline` | PASS |
| `cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked --offline` | PASS (cross compilation check only) |
| Debug desktop build with `ui-preview,eframe/x11` | PASS |
| Actual Linux Xvfb launch/capture/visual inspection | PASS heads, pockets, detail, detail-carry |
| `git diff --check` | PASS |

Final inventory is67 tests; full workspace was run at66, then the new public
constraint test and all7 preceding head-search tests passed together. Production
code did not change after the passing full workspace run. No unnecessary second
full GUI test run. Final Clippy includes the new test. The first workspace run
failed only old hardcoded16-result expectations, which were replaced by
meaningful nonempty/solution/selection/identity checks; later full run passed.
Historical Python fixture remains unchanged; unchanged candidate-field semantics
are still checked. The independently defined physical oracle supersedes obsolete
compact geometry/rank expectations, rather than regenerating them from the engine.

The public oracle enumerates every n-ary label map and materializes pairwise
Pareto; production uses iterative restricted-growth partitions and an incremental
frontier. Eighteen small cases cover g2/3/4, mixed dividers,2/3 formats, exact/over
5mm, nontransitivity, fractional wall clearance, dimensions, metrics and score.
Separate assertions cover canonical identity, different pickup pitches, group1,
40 partitions with exact limits, completed progress, running cancellation,
output truncation and compatible heads whose widened drawer exceeds pitch.
GUI/IO cover distinct head rows across rerenders and complete reconstruction on
reopening, old-engine warning, and source bytes preserved without saving.

Linux screenshots show input/results together, correct assignments/counts,
g4 drawer141mm with two pairs and central divider, updated group2 drawer56mm,
and the partial final layer for34 sticks. Artifacts/logs currently in
`/tmp/leo18-gui/`; text logs `/tmp/leo18-workspace-tests.log`,
`/tmp/leo18-check.log`, `/tmp/leo18-clippy-final.log`, `/tmp/leo18-release.log`,
`/tmp/leo18-msvc.log`. They are session-local; durable evidence is this checkpoint,
versioned tests and ADR. Windows native execution, physical keyboard/mouse,
native Windows file pickers and multiple-monitor DPI were not exercised.
MSVC check is not described as native execution.

## Next action and review boundary

Push this checkpoint-only final head, give coordinator exact baseline/head and
wait for two new independent spec/standards reviewers (`fork_turns:none`). Keep
review cycle count0 until coordinator starts cycle1; do not reset across resumes.
No product mutation while fixed-head review runs. Coordinator owns Linear states,
PR creation and review handoff. No merge/deploy authorized.
