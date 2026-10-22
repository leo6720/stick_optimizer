# ADR0003 — Chosen head quotations and exports

Status: implemented for LEO-19 (22 October2026); accepted scope is LEO16's
20October definition,21October split,130k child. No new physical rule.

## Decision

The Cartoner parameter popup edits `max_extra_spacing` with the existing draft,
validation, Apply/Cancel and explicit-defaults lifecycle. The new field belongs
to this popup's copied group; changing MT channels or weights elsewhere cannot
be overwritten by applying the popup. Stored divider/extra values take precedence
over incorporated7.5/5mm defaults; opening/applying never writes preferences.

`core::candidate_dimensions` is a read-only projection of a calculated candidate's
first-layer coordinates. It reports wall clearances, adjacent centre distances,
stick surface gaps, divider-side clearances, and approach minimum/excess. It uses
the same minimum-spacing helper as the head validator; extracting this expression
does not alter it. Group4's central fixed gap is marked structural. GUI and CSV
consume this projection, never duplicate its physical arithmetic. Existing metrics,
search enumeration, geometry construction and Pareto remain unchanged.

Tables join assignments using solution-local `head_id`; separate heads with equal
pickup keys remain separate. Human labels Testa1/2 are ID+1; raw IDs0/1 are shown
and exported. Assigned formats are never inferred from names or old pickup keys.
The existing format popup adds quotations; no head popup or simulation is added.
`head_width` remains pickup pitch times all MT channels, as in ADR0002.

## Files and chosen identity

Study/preferences schema2 already persists every relevant parameter, and the
current `rust-final-heads-v2` engine reconstructs assignments in a deterministic
order. A saved1-based original rank selects the same complete configuration,
independently of filtered/pitch-sorted display positions. Tests compare the entire
chosen solution and byte-identical detail CSV after public save/load/recalculation,
not just rank or result counts. No new result cache, preference path, engine or
schema bump is needed; existing strict version migration/error preservation remains.

CSV changes independently to version2. Keep original11 summary/28 detail columns
in place, append engine/version/parameters, and explicit identities/coordinates.
Summary includes a JSON configuration, so equal metrics cannot hide different
assignments. Detail has one candidate per row and full lists with JSON syntax,
including explicit empty/null representations for group1 and absent dividers.
Full precision, CSV quoting and mm units are preserved. See rust-files.md for
all added columns and origins. Export construction returns errors before any
file write if a candidate cannot be joined to its study/assignment.

## Alternatives and consequences

Keeping only counts/old head keys loses separate/shared choices. Exporting only
rounded labels or first-to-last-centre spans loses precision or redefines
head_width. Serializing result caches duplicates a deterministic engine and
creates stale geometry risks. Repeating formulas in GUI/CSV violates architecture.
The selected approach adds a small public projection and richer exports without
changing persisted input recipes or physical search behavior.

Public core quotations and IO tests cover mixed25/26mm, groups2/3/4, divided heads,
separate/shared choices and group4's27/60/27mm gaps. Egui tests exercise typing,
Apply/Cancel/validation, calculation, actual details, CSV and selected reconstruction
with pitch ordering, plus explicit preference writes. Native Linux screenshots
and Windows cross-check are distinct from physical native Windows tests.
