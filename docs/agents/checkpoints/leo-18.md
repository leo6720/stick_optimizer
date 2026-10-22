# LEO-18 — exact final-head search

- Issue: https://linear.app/leochist/issue/LEO-18
- Implementer: `/root/leo18_implementer`, actually spawned with `fork_turns:none`.
- Ownership: only LEO-18, isolated `/workspace/stick-leo-18`, `codex/leo-18-async`.
- Baseline: `f2956653e5f6bb80d638fdbd28eab8cef50ef087` (merged prerequisite PR21).
- Phase: implementation and public oracle verified; consumer regression checks in progress.
- Review cycles: 0. Open findings: none independently reviewed yet.
- Scope: accepted LEO16 definition20October, sizing21October,140k child of390k/3.
- No LEO19/7, merge, deployment or automation work.

## Evidence so far

Read-only preflight: exact baseline26core/IO tests and full workspace Linux
check passed offline, pinned Rust1.99.0, unchanged product diff vs reviewed498bdbe.
First public regression RED: g4 drawer107mm instead of approved141mm.
GREEN after implementation:141mm, schematic stick x5/31/91/117mm.
Independent public oracle enumerates every n-ary label map and materializes
pairwise Pareto; production uses iterative restricted-growth partitions and an
incremental frontier. Tests cover g2/3/4, mixed dividers,2/3formats,5mm/over,
nontransitivity, actual dimensions/penalties/score, fractionalwallclearance,
head identity, different pickup pitches, group1exclusion, partitioncounts,
operationallimits, outputtruncation and cancellation during live assignmentwork.
Historical Python fixture unchanged; only unchanged field definitions compared.

Full workspace first run uncovered assertions hardcoding old16results (new
geometry changes Pareto); adapting consumer checks to require nonempty results
and preserve actual solution identity. Full final checks NOT yet passed.

Next: engine-marker reopening and headtable regression, formal ADR/docs,
fmt/check/Clippy/workspace tests/release/MSVC, actual LinuxXvfb previews, then
commit/push stable head for two new independent reviewers. Windows native
execution unavailable; cross-check will not be reported as native execution.
