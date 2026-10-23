# 0002 — Exact separate/shared heads and effective drawers

Status: implemented for LEO-18, based on accepted LEO-16 definition20October2026
and its21October split. Prerequisite: ADR0001/LEO-17. Engine:
`rust-final-heads-v2`; study/preference schema remains2.

## Decision

For each common-cartoner-pitch candidate tuple, enumerate every set partition
within each `(grouping, pickup_pitch)` bucket. Group1 has no head assignment
and contributes zero heads. Restricted-growth strings enumerate each unordered
partition once, iteratively (no recursive stack). Buckets are ordered by key;
input candidate order is preserved; shared partitions precede separate ones.
There is no union-find, pairwise transitive closure, greedy sharing, or dominance
pruning before a complete physical solution. Equal Pareto metrics remain distinct.
Every complete block is passed to `final_head_geometry` with ALL assigned
formats, including mixed divider requirements. Rebuild its candidates using the
resulting common positions. Incompatible blocks/drawer fits reject only that
partition; numerical inability is an explicit search error.

`Solution.head_types` contains `HeadAssignment { id, geometry }`. IDs are zero
based within a solution, ordered by the minimum assigned format ID. The geometry
contains sorted format IDs and all channels' pickup/final positions. Each
candidate has its `head_id` (None for group1) and effective `PocketGeometry`.
The head table selects candidates by head ID, never the old pickup-key tuple.
Separate heads may have equal coordinates: their assignments/identities remain
separate alternatives. `head_width` retains pickup pitch×all MT channels;
it is not a graphic span or final drawer width.

## Effective dimensions and precision

Take one group's final centre coordinates from the public head geometry. The
required inner drawer span is last centre−first centre+stick width+two wall
clearances. Group4 therefore contains the structural fixed central pickup gap,
including without a divider. Preserve integer drawer dimensions but round the
required width **up**. Nearest-even could undersize a valid physical envelope
(e.g. g2 width25.1, gap26.1, wall clearances2 →55.2mm requires56mm, not55).
This intentional geometry precision change applies to group1 as well: a drawer
must not violate its wall clearance merely because an integer rounded down.
It does not add a penalty, change a weight, or redefine the pickup pitch.

Centre any sub-mm rounding surplus in the outer wall games. Divider positions
are midpoint between adjacent centres (g2 one gap, g3 both gaps, g4 the fixed
central gap), minus half the divider thickness. They satisfy both sides' wall
clearances; the public head API verifies these minima. Local stick centres and
divider left edges are stored in the candidate and consumed by the schematic.
Drawer occupied width and A/B use the resulting integer width. Unsupported width
uses the largest actual clear compartment span (outer wall to divider, between
dividers, or divider to outer wall); without dividers it is the inner width.
All existing stability/A-B/other penalty formulas and weights remain unchanged.
Pocket type identity includes divider locations as well as width, length,
divider count and pockets per pitch: equal counts are insufficient for physically
different divider arrangements. The pocket table uses this complete identity.
Divider identity uses rounded integer ticks on a fixed 1e-9 mm grid, matching
the geometric precision contract and avoiding width-dependent floating-point
roundoff splitting an identical physical drawer. Equality compares canonical
keys, not pairwise epsilon distances, so it stays transitive. Coordinates remain
unrounded for clearance checks and drawing. If scaled ticks are not finite or
exceed exact f64 integer precision, identity retains the original coordinate
instead of saturating or merging distinct large positions. This adds no physical
constraint and changes no penalty formula or weight.
The drawer translation is computed directly as `(rounded_width - head_span)/2`.
Divider left edges are derived from the centred drawer and its divider thickness:
the single g2/g4 divider is central; g3 dividers are symmetric, separated by half
the approved head span. These algebraically equivalent forms avoid introducing
stick-width-dependent cancellation into an identical drawer shape. Canonical
identity alone cannot repair raw coordinates straddling a grid half-tick; the
geometry construction must first be independent of the assigned stick width.

Pitch keys/tolerances and public head-API precision rejection remain ADR0001.
Drawers move as a local translation of the approved fixed head positions; no
kinematics is simulated. Structural g4 distance is never charged against extra
moving spacing. Final geometries/pocket constraints are checked before Pareto.

## Limits, progress and order

First bound the base candidate Cartesian product by `SearchLimits.combinations`.
Then compute exact structural work: for each candidate tuple multiply Bell(n)
for its non-group1 pickup-key bucket sizes, and sum across tuples. Bell counts
use checked Stirling arithmetic and stop above the bound. Count incompatible
partitions too: each requires a complete-assignment compatibility check. Reject
an overflow or total above the limit before publishing a partial ranking.
Estimation enumerates bounded base tuples, not partitions or results.

`estimated` is the complete structural-partition count; it remains0 while
preparing/estimating or after estimation itself exceeds a limit. `examined`
increments after each complete partition check, including physical rejection,
and its Pareto processing. Successful reports satisfy examined=estimated.
Cancellation is checked during candidate generation, estimation, each partition,
each head/rebuild and frontier comparisons, and before returning results.
Frontier/candidate/grid limits retain their explicit errors. No error/cancellation
returns a ranking. Output N truncates only after complete Pareto and stable sort.
Sort remains score, seven metrics, common pitch, then enumeration order.

## Persistence and alternatives

Files retain input recipes, not result caches. New documents mark
`rust-final-heads-v2`; opening an older engine reports the change and recomputes.
Reading preserves the original marker and source bytes; explicit save marks the
current engine. Schema2 and preference migration are unchanged.

Rejected alternatives: transitive compatibility loses the narrowest format's
limit; greedy maximal heads loses separate/shared Pareto alternatives; compact
g4 drawers displace fixed channels; rounded-down widths violate real clearances.
No optimization/performance claim or new search objective is introduced.

## Evidence and remaining scope

Public independent oracle enumerates all n-ary label maps, canonicalizes blocks
and materializes pairwise Pareto. It does not call production geometry,
aggregation or dominance for expected values. Covers g2/3/4, mixed dividers,
three-format nontransitivity, exact/over5mm, fractional widths and actual metrics.
Explicit regressions cover g4 coordinates/drawer141mm, head identity, different
pickup pitches, group1,40structural partitions, exact limits and running cancel.
Historical Python fixtures remain unchanged and check unchanged input/candidate
field semantics; obsolete compact geometry/rank expectations are not regenerated.
IO and GUI tests check reconstruction, changed-engine reporting, no source
rewrite, separate head rows, stable selection and render identity.

LEO-19 retains the final editor, rich details and CSV redesign; LEO-7 retains
the head popup. Neither is implemented here. Native Windows execution is a
separate verification from a Linux build or MSVC target check.
