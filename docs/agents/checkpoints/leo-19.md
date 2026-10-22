# LEO-19 — editor, chosen head details and CSV

- Ticket: https://linear.app/leochist/issue/LEO-19
- Owner: `/root/leo19_implementer`, fresh actual `fork_turns:none`; coordinator owns Linear/PR/reviews.
- Baseline: `c4c458b60f98abea6bfb7684a63e9d16ec9ddaa0`, PR23 merged.
- Branch/worktree: `codex/leo-19-async`, `/workspace/stick-leo-19`.
- Accepted definition: parent LEO16 grill20Oct/sizing21Oct, 130k child of390k/3.
- Phase: implementation milestone; **review cycle0**, no independent review yet.
- Open findings: none known; implementation and final validation still in progress.
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
