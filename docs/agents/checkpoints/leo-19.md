# LEO-19 — editor, chosen head details and CSV

- Ticket: https://linear.app/leochist/issue/LEO-19
- Owner: `/root/leo19_implementer`, fresh actual `fork_turns:none`; coordinator owns Linear/PR/reviews.
- Baseline: `c4c458b60f98abea6bfb7684a63e9d16ec9ddaa0`, PR23 merged.
- Branch/worktree: `codex/leo-19-async`, `/workspace/stick-leo-19`.
- Accepted definition: parent LEO16 grill20Oct/sizing21Oct, 130k child of390k/3.
- Product/documentation head: `cfb8a4cd06246bdffdcd498018229195f8b39c4e`.
- Phase: implementation and final verification complete; **review cycle0**, awaiting independent review.
- Open findings: none known; no independent review claimed.
- No merge/deploy/new automation/LEO7 work or external status change by implementer.

## Read-only prerequisite proof

Exact main fetched; reviewed prerequisite head `77c389aad3c13df3182d2c4becc456a7f1623bf1`
is reachable and product/Cargo/toolchain diff against main is empty. On isolated
exact-main archive:36 core+IO tests PASS (27+9), workspace Linux all-targets /
all-features check PASS, preview build PASS; actual Linux Xvfb launch exited0,
heads screenshot captured and visually inspected (128 combinations,10 results).
Linear19/16/18 descriptions/comments/relations freshly read using Leochist link
metadata; scope/sizing unchanged,18Done,19Todo before coordinator claim.
`get_agent_context` returned ToolNotFound; repository/tickets authoritative.
Coordinator reconciled ready and claimed InProgress, verified before branch creation.

## Implemented milestone

- Cartoner popup exposes max extra spacing, validated with Apply/Cancel; defaults5
  and7.5 divider remain, only explicit preference save writes files.
- Read-only core quotation projection gives actual wall/stick/divider gaps and
  moving/minimum/extra spacing; GUI contains no physical formulas. Existing minimum
  expression extracted and reused, no search/penalty/constraint change.
- Tables show actual head IDs, separate/shared assignments, spacing and divider
  locations; existing format popup shows core quotations, no new head popup.
- CSV version2 appends stable IDs/configuration/settings/full coordinates and gaps;
  original columns and `head_width = pickup pitch × channels` preserved.
- Study/preferences schema2 and engine `rust-final-heads-v2` unchanged: existing
  complete recipe/rank deterministically reconstructs the chosen configuration.

## Actual tests at this milestone

All Cargo commands use pinned Rust1.99.0, CARGO_HOME=/workspace/scratch/rust-cargo,
RUSTUP_HOME=/workspace/scratch/rust-toolchains,
CARGO_TARGET_DIR=/workspace/stick-rust-validation/target, `--locked --offline`.
Linux desktop commands enable `eframe/x11`; no manifest/toolchain change.

- New egui editor test observed RED: missing label, then GREEN for invalid values,
  negative/nonfinite inputs, Cancel, Apply and no implicit preference save.
- New public dimensions API initially absent (compile RED); independent physical
  g2/g3/g4 shared/divided gap and fixed60mm g4 quotation test GREEN.
- Workspace run PASS72 tests (28core/35desktop/9IO), before final extra tests below.
- New IO chosen roundtrip test PASS: six separate/shared mixed25/26mm g2/g3/g4
  configurations, source bytes preserved, full chosen solution and CSV identical
  after reopening; expected143mm g4 drawer and27/60/27 gaps,13.75mm divider sides.
- New egui end-to-end test PASS: actual typing/Apply/Calcola, head/detail presentation,
  CSV and save/reopen for six separate/shared g2/g3/g4 choices with sorted view;
  full selected solution and CSV bytes match, source file unchanged.
- Explicit preferences test PASS: stored custom2.125mm divider/3.125mm extra spacing
  survive loading; Apply does not write; explicit save writes1.25mm; invalid read
  preserves current study/results and malformed source bytes.

Native Windows execution, physical keyboard/mouse/file pickers and multi-monitor
DPI are unavailable. MSVC is an installed cross-check target, not native execution.
Final fmt/check/Clippy/test/release/MSVC and updated native Linux compact/zoom
screenshots remain to run; no final verification or independent review claimed.

## Next action

Complete CSV column documentation/ADR, additional quotation presentation checks,
full checks and Linux screenshots; commit/push stable head for coordinator to
assign two fresh independent reviewers. Preserve cycle counter. Parent16 only
complete after integration and overall acceptance verification.


## Final implementation verification

Product/doc head `cfb8a4cd06246bdffdcd498018229195f8b39c4e`; this final checkpoint
commit changes documentation only. Baseline remains c4c458. No product changes
after the following final checks, except rustfmt-only whitespace in the new
preview pattern and test; final fmt/Clippy/workspace tests include that formatting.

| Actual command/check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --workspace --all-targets --all-features --features eframe/x11 --locked --offline` | PASS |
| `cargo clippy --workspace --all-targets --all-features --features eframe/x11 --locked --offline -- -D warnings` | PASS final source |
| `cargo test --workspace --all-features --features eframe/x11 --locked --offline` | PASS76 tests:28core+37desktop+11IO |
| `cargo build --workspace --release --features eframe/x11 --locked --offline` | PASS |
| `cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked --offline` | PASS compilation only |
| `cargo build -p stick-optimizer-desktop --features ui-preview,eframe/x11 --locked --offline` | PASS |
| Actual native Linux Xvfb launches/captures/visual inspection | PASS9 views |
| `git diff --check` | PASS |

The final workspace run initially failed in an added direct presenter assertion:
existing `text_position` excludes text atx=0; the quote-only root UI started there.
Its panic also hit egui's intentionally unhandled headless texture delta guard.
The harness now uses an isolated context/inset and clears texture updates, as
other headless tests do. Targeted six-case test then passed, followed by the
complete76-test run above. No production defect was hidden or test omitted.
The new group1 CSV test fixes pickup pitch at60mm and verifies explicit absent-head
representations,720mm original head_width, and error on a mismatched study.

Final app end-to-end tests also render the shared quotation presenter and assert
independent37.5mm (g2/3),27mm (g4) spacing and1mm excess, in addition to actual
popup/head table interaction. IO parses CSV independently, tests full precision,
quoted names, all56 detail/20 summary columns, IDs and JSON coordinate/gap lists.
Full chosen-solution equality after reopening covers both separate/shared IDs and
coordinates, with pitch-sorted view preserving original rank. Existing malformed
file/save/legacy migration/source-preservation tests remain passing.

Nine native Linux modes actually launched, exited0, captured and were individually
visually inspected: `heads`, `pockets`, `parameters`, `parameters-compact`,
`parameters-scaled`, `detail-bottom`, `detail-carry-bottom`, `detail-compact`,
`detail-scaled`. New field and Apply/Cancel are visible in compact/150% parameter
views, scroll works in existing popup, explicit shared/separate IDs/spacing appear
in head table. Group4 bottom shows fixed60mm central gap and13.75mm divider-side
clearances, separately from moving26mm spacing and0mm excess for the example.
Screenshots/logs: `/tmp/leo19-gui/`, `/tmp/leo19-{workspace-final,clippy,release,msvc,preview-build,e2e}.log`.
These local artifacts are temporary; durable evidence is versioned tests/ADR/checkpoint.
Temporary Xvfb terminated; no user study or preferences touched.

Native Windows execution, native Windows file pickers, physical keyboard/mouse,
and multi-monitor/DPI remain unexecuted. MSVC is only cross compilation. Linux
captures and synthetic egui events do not certify those native Windows paths.

Self-review performed against baseline: core change is an additive read-only
quotation projection plus extraction of an unchanged minimum expression; no
search/physical constraint/metric/penalty change. Tables/CSV use explicit head IDs,
legacy head_width preserved; source migrations and explicit-defaults lifecycle
unchanged. Documentation records CSVv2 units/origins/empty values; existing study
schema2/current engine are intentionally retained. This self-review does not
replace the coordinator's two fresh independent reviewers.

## Frozen handoff and next action

Implementation complete within accepted130k scope, no new functional decision or
further split indicated. Push this checkpoint, report exact final branch SHA,
then STOP product mutations. Coordinator assigns two fresh `fork_turns:none`
reviewers to the entire baseline→final-head diff, specification and standards;
cycle0 until review begins. Coordinator owns the single PR/Linear updates.
Any concrete findings are fixed and reverified with cycle count preserved,
maximum3 unresolved cycles. No merge/deploy/LEO7 or parent completion claim.
Parent16 requires integration and overall acceptance verification after all PRs.
