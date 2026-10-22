# LEO-18 — exact final-head search

- Issue: https://linear.app/leochist/issue/LEO-18
- Initial implementer: `/root/leo18_implementer`, actually spawned with `fork_turns:none`.
- Resume/checkpoint owner: `/root/leo18_resume_owner`, fresh isolated `fork_turns:none`.
- Ownership: only LEO-18, isolated `/workspace/stick-leo-18`, `codex/leo-18-async`.
- Baseline: `f2956653e5f6bb80d638fdbd28eab8cef50ef087` (merged prerequisite PR21).
- Initial reviewed head: `4b36c382a2929335b02ef1a53e313397c56f2d76`.
- Corrected product head: `7aca80b4627bcb99bd16a966b8c2cbeafba6ac92` (following checkpoint commit changes no product).
- Phase: cycle1 finding corrected and verified; awaiting cycle2 independent review on the final pushed head.
- Review cycles: **1 completed**, cycle2 pending coordinator handoff. One cycle1 standards P2 corrected; no unresolved finding known, correction not yet independently approved.
- Accepted scope: LEO16 definition 20 October, sizing 21 October, 140k child of 390k/3.
- No LEO19/7, merge, deployment, new automation or Linear state change by implementer.
- PR: draft https://github.com/leo6720/stick_optimizer/pull/23 (single PR, main base).

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

## Initial implementation verification (before cycle1 correction)

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

## Cycle1 review and bounded correction

The original cycle1 reviewers were interrupted by a usage limit without final
PASS reports. Cycle1 was resumed on the same exact head by fresh independent
`/root/leo18_spec_resume` and `/root/leo18_standards_resume`. Spec PASS; standards
reported one concrete P2, not PASS. Spec independently ran8/8 head-search tests.
No counter reset or approval was inferred from an interrupted review.

P2: raw f64 equality of drawer divider positions split physically identical
drawers. Independent public reproduction: widths20/20.01, g2, pickup60,
divider7.5, A/B target56/11. Shared-head drawers both width56 had divider edges
24.249999999999996/24.25; the search incorrectly counted2 drawer types and
kept a dominated separate-head arrangement. Correct identity yields metrics
[1,1,0,0,2,0,0], score580, and removes that separate-head arrangement by Pareto.

Resume owner verified clean head/ownership before editing. Public regression
was observed RED at2 types versus1; bounded fix is GREEN. PocketType equality
now compares canonical divider ticks on a fixed1e-9mm grid, retaining raw
physical coordinates and exact original-coordinate fallback above exact f64
integer tick precision or on scaling overflow. Equality is transitive; no
pairwise epsilon chaining, physical constraint, penalty formula or weight change.
Tests also exercise transitivity, distinct divider positions and huge finite
coordinates. GUI drawer-row membership uses this same core equality; study
results are rebuilt rather than cached. ADR0002 documents the identity precision.

## Correction verification (product head7aca80b)

Same pinned toolchain/cache/target paths and offline locked setup as above.
All following checks were actually rerun because core identity affects ranking,
GUI drawer membership and reconstructed study results:

| Check | Actual result |
| --- | --- |
| Public regression before fix | RED, drawer types2 instead of1 |
| `cargo test -p stick-optimizer-core --test head_search --locked --offline` | PASS10/10 |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --workspace --all-targets --all-features --features eframe/x11 --locked --offline` | PASS |
| `cargo clippy --workspace --all-targets --all-features --features eframe/x11 --locked --offline -- -D warnings` | PASS |
| `cargo test --workspace --all-features --features eframe/x11 --locked --offline` | PASS69 tests:26core+34desktop+9IO |
| `cargo build --workspace --release --features eframe/x11 --locked --offline` | PASS |
| `cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked --offline` | PASS, compilation only |
| `git diff --check` | PASS |

Logs: `/tmp/leo18-cycle2-{workspace,check,clippy,release,msvc}.log` (session-local).
No new GUI screenshot run for the equality-only correction; original four Linux
launch/capture/visual checks remain historical evidence on4b36c382. Full synthetic
desktop suite rerun passed after correction. Windows native execution, physical
keyboard/mouse, Windows file pickers and multiple-monitor DPI remain unexecuted.
MSVC cross-check does not certify native Windows behavior. No new functional
decision or necessary human test gate was identified by cycle1 reviewers.

## Next action and review boundary

Push the checkpoint head and hand exact baseline/final head to the coordinator
for cycle2 spec and standards review. Do not claim correction independently
approved until both reports complete. No product mutation during fixed-head
review. Coordinator owns PR/Linear status and handoff. No merge/deploy authorized.
