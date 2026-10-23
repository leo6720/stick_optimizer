# LEO-7 — static head front views

- Ticket: https://linear.app/leochist/issue/LEO-7
- Implementer: `/root/leo7_implementer`, actual fresh `fork_turns:none`.
- Coordinator owns Linear, PR and independent reviewers.
- Accepted grill20October/sizing21October120k, N1; scope unchanged.
- Baseline: `86c8e9ddb6c9d25a855a5b7583126c0acdf95cbb`.
- Branch/worktree: `codex/leo-7-async`, `/workspace/stick-leo-7`.
- Cycle0; no independent review yet, no open finding.
- Phase: implementation and final validation complete; frozen for two independent reviews.
- Product milestone pushed: `d253a6d`; final refinement/checkpoint commit follows.

## Dependency acceptance verified before claim

Root closed nonexecutive LEO16 only after this isolated implementer's exact-main
audit. Fresh Linear children17/18/19Done, no comments/revocations; parent acceptance
checkboxes and evidence then persisted by root. Main tree identical to reviewed
PR24 head ef7a1fdc392ceeb0ada1074085a1ca6fcf827b2d. Merged main-base PR21
f2956653e5f6bb80d638fdbd28eab8cef50ef087, PR23
c4c458b60f98abea6bfb7684a63e9d16ec9ddaa0 and PR24 baseline86c8e9d all
verified reachable. Exact-main archive76tests PASS(28core/37desktop/11IO), preview
build and native Linux Xvfb launch/capture/visual inspection PASS.

| Parent criterion | Current-main evidence, all PASS |
| --- | --- |
| Complete shared final geometry | public complete-set/nontransitive API tests and exact search oracle |
| Different pickup pitches separate | public different-pitch search/API regressions |
| Extra limit per format with/without divider | approved25/26→27/37.5, exact/over5, joint3-format g2/3/4 |
| Editable5mm/default divider7.5 | core/draft defaults, real egui editor typing/validation/Apply/Cancel |
| Actual drawer/divider fit | fixed-central g4 two-pair drawer, wall rounding/bounds and public dimensions |
| Coherent counts/Pareto/score, no new penalties | independent18-case oracle, seven metrics/score, aggregate inspection |
| Details/CSV/study/preferences chosen identity | full six-case separate/shared g2/3/4 egui+IO solution/CSV roundtrip |
| No automatic user overwrite | migration source bytes/custom values preserved, Apply no write, explicit preference save |

Prior independent reviewers PASS for17cycle2,18cycle3,19cycle1, no open findings
or additional mandatory human gate. Native Windows/physical input/file pickers/
monitor DPI unexecuted; MSVC compilation alone never claimed native execution.

## Milestone

Public quotation API observed RED absent, then GREEN for fixed expected g2/3/4
coordinates, final/pickup spans, group pitch and27/60/27 structural g4 gaps.
Read-only projection only, no new physics/search/metric/penalty or head_width change.
Actual egui double-click test initially RED missing popup, then GREEN for two heads,
solution identity, Esc and invalidation; additional selector/button checks ongoing.
Direct renderer test PASS all12 grippers/sticks, actual25/26 widths, fixed approved
coordinates and no core mutation. Named actual-format selector, black traverse/
grippers and red sticks, before above after, zoom/scroll and explicit centre-span
labels. First six native Linux previews exit0; refining fit at zoom1 and obtaining
scrolled captures before final visual verification.

Use pinned1.99 with CARGO_HOME=/workspace/scratch/rust-cargo,
RUSTUP_HOME=/workspace/scratch/rust-toolchains,
CARGO_TARGET_DIR=/workspace/stick-rust-validation/target; --locked --offline.
Final fmt/check/Clippy/workspace tests/release/MSVC and Linux captures remain.
No user data touched, no merge/deploy/new automation, no other ticket.


## Final validation and frozen review handoff

Scope remains accepted120k/N1; no further split or product decision indicated.
No product changes after the final Rust checks below. The final commit adds the
already-checked precision/fit/identity refinements and this documentary evidence.
Cycle0 until coordinator starts two fresh `fork_turns:none` reviewers.

| Actual command | Result |
| --- | --- |
| cargo fmt --all -- --check | PASS |
| cargo check --workspace --all-targets --all-features --features eframe/x11 --locked --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --features eframe/x11 --locked --offline -- -D warnings | PASS |
| cargo test --workspace --all-features --features eframe/x11 --locked --offline | PASS79:29core+39desktop+11IO |
| cargo build --workspace --release --features eframe/x11 --locked --offline | PASS |
| cargo check --workspace --all-targets --all-features --target x86_64-pc-windows-msvc --locked --offline | PASS compilation only |
| cargo build -p stick-optimizer-desktop --features ui-preview,eframe/x11 --locked --offline | PASS |
| git diff --check | PASS |

Test-first public API and missing popup compilation failures were observed before
implementation. The final renderer checks independently approved expected centre
arrays, all12 black grippers/red sticks and actual25/26mm widths for g2/3/4, before
and after; no solution mutation. Actual egui double clicks open two different
heads, preserve rank/ID, change the shared format via ComboBox, and close via
Chiudi/Esc; invalidation closes stale views. The existing same-pickup-key test now
also double clicks the second separate head and verifies its own rank/ID/format.

Initial selector test clicked an identically named sidebar field; limiting the
coordinate lookup to the popup fixed the harness, without production change.
Immediate close/reopen clicks in one context were intercepted by egui's closing
window fade, so independent-head interactions use fresh contexts. Clippy first
requested one collapsed nested-if; corrected and all final checks above passed.
No failure or test omission is hidden. Measurements use full f64 round-trip
precision from the read-only core projection, with Italian decimal separators.

Nine fresh actual native Linux Xvfb runs exited0, captured and individually
visually inspected: head-popup, head-popup-compact, head-popup-compact-bottom,
head-popup-scaled, head-popup-scaled-bottom, head-popup-zoom, head-popup-g3,
head-popup-g4, head-popup-alternate. All12 channels fit at drawing zoom1; compact
and150% UI scroll reach the complete lower view/centre quotation while Chiudi
remains visible. Drawing zoom1.5 exposes horizontal scroll. Shared alternate
shows actual B26mm rather than A25mm with the same fixed geometry. G3 shows
36.5/36.5mm and613mm final span; G4 shows26/60/26mm,592mm final span and fixed
central pair; pickup span660mm distinct from retained720mm head_width.
Screenshots/logs in /tmp/leo7-gui; Rust logs /tmp/leo7-{fmt,check,clippy,workspace,
release,msvc,preview-build}.log. Temporary display server terminated.

Self-review: core diff adds a read-only measurement projection only; final_head_geometry,
search, dimensions/constraints, metrics, weights, ranking, CSV and file schema are
unchanged. GUI uses explicit solution-local IDs and selected actual format widths;
only screen projection arithmetic. New view does not mutate solutions. This
self-review does not replace two fresh independent reviewer contexts.

Native Windows execution, physical input, Windows file pickers and multi-monitor
DPI remain unexecuted; MSVC is compilation only. No new mandatory human gate is
identified by this implementation, subject to independent review. No user data
written, no merge/deploy/automation/new ticket. Coordinator owns PR/Linear.
Next: review entire baseline→final remote head, fix concrete findings within
accepted scope and persist review cycles (maximum3 unresolved), no product edits
until review feedback.
