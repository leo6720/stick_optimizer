# Stick Optimizer

Static engineering optimizer for stickpacker-to-cartoner transfer design in pharmaceutical stickpack packaging lines.

The software evaluates large numbers of feasible transfer configurations and identifies the most efficient machine setup across multiple commercial product formats while minimizing:

- Number of cartoner pocket types
- Number of robot head types
- Transfer complexity
- Format changeover effort
- Stack stability risks

The application is intended as an engineering decision-support tool during machine concept design, quotation activities, format standardization projects, and retrofit studies.

---

# What Problem Does It Solve?

In a stickpack packaging line, sticks leave the stickpacker in parallel lanes and must be transferred into cartoner pockets by a robot.

Several design choices are possible:

- Stick pitch adjustment
- Robot grouping strategy
- Pocket divider configuration
- Pocket geometry
- Number of pockets per cartoner pitch
- Cartoner transport pitch
- Layer count inside the pocket

A solution that works for a single format is often far from optimal across an entire product portfolio because it may require:

- Dedicated robot heads
- Dedicated pocket sets
- Excessive stack heights
- Complex carryover logic
- Multiple machine change parts

The purpose of this software is not simply to find feasible solutions, but to identify solutions that maximize hardware commonality across all selected formats.

---

# Core Optimization Objectives

Solutions are ranked according to the following priorities:

1. Minimize the number of pocket types.
2. Minimize the number of robot head types.
3. Minimize layer-related penalties.
4. Minimize carryover complexity.
5. Prefer simpler robot groupings.
6. Improve stack stability.
7. Minimize carton aspect-ratio penalties (current versions include A/B ratio evaluation).

Pocket and robot-head commonality deliberately dominate the final ranking because they have the greatest impact on machine cost, parts inventory, and changeover complexity.

---

# Engineering Model

## Stick Types

Each stick type is defined by:

- Name
- Length
- Width
- Thickness
- Fin length

Simplifications:

- Sticks are treated as rigid rectangular solids.
- Sticks are always laid flat.
- No rotation is allowed.
- No alternating stacking patterns are considered.
- Dynamic behavior is ignored.

### Dimension Usage

| Dimension | Used For |
|------------|------------|
| Length | Pocket length and stability calculations |
| Width | Pocket width calculations |
| Thickness | Stack height calculations |
| Fin length | Stickpacker pitch calculations |

---

## Commercial Formats

A format is defined by:

- Format name
- Associated stick type
- Sticks per pocket

The optimizer treats formats independently and then searches for machine-wide common configurations.

---

## Robot Groupings

Supported robot groupings:

1
2
3
4

Feasibility rule:

sticks_per_beat % grouping == 0

Only groupings that exactly divide the stickpacker output are allowed.

Grouping complexity penalties:

| Grouping | Penalty |
|-----------|-----------|
| 1 | 0 |
| 2 | 1 |
| 3 | 2 |
| 4 | 2 |

---

## Robot Head Definition

A robot head type is uniquely defined by:

(grouping, adjusted_input_pitch)

Not included:

- Stick length
- Stick width
- Stick thickness
- Format name
- Product family

This allows meaningful tooling reuse across multiple products.

---

## Pocket Types

A pocket type is defined by:

(pocket_width, pocket_length, divider_count, pockets_per_pitch)

Changing any of these parameters creates a new pocket type.

Pocket commonality is one of the primary optimization targets.

---

## Divider Logic

Allowed divider configurations:

| Grouping | Allowed Dividers |
|-----------|-----------|
| 1 | 0 |
| 2 | 0, 1 |
| 3 | 0, 2 |
| 4 | 0, 1 |

Dividers are not directly penalized.

Their impact is indirect through:

- Pocket width
- Pocket type definition
- Effective unsupported width
- Stability calculations
- Layer comfort threshold

---

## Stickpacker Pitch Logic

Nominal pitch:

nominal_pitch =
2 × stick_width +
2 × fin_length

Valid adjusted pitches are generated inside:

nominal_pitch ± max_pitch_shift

using discrete increments of:

pitch_step

Pitch shift itself is not penalized as long as it remains within the allowable range.

---

## Cartoner Pitch Logic

Pocket pitch:

pocket_pitch =
grouping × adjusted_input_pitch

Cartoner pitch:

cartoner_pitch =
pockets_per_pitch × pocket_pitch

Supported values:

pockets_per_pitch = 1 or 2

All formats belonging to the same solution must share the same cartoner pitch.

This is one of the fundamental constraints of the optimization.

---

## Physical Fit Validation

The software verifies that the pockets physically fit inside the cartoner pitch.

Occupied width:

occupied_width =
pockets_per_pitch ×
(
pocket_width +
2 × pocket_wall_width
)

Constraint:

occupied_width <= cartoner_pitch

Unused space is reported but not penalized.

---

## Layer Calculation

Layer count:

layers =
ceil(sticks_per_pocket / grouping)

Stack height:

stack_height =
layers × stick_thickness

There is no hard maximum layer count.

Instead the optimizer applies progressive penalties above comfort thresholds:

- 7 layers without dividers
- 9 layers with dividers

---

## Carryover Logic

Carryover is required whenever:

sticks_per_pocket % grouping != 0

The optimizer calculates:

- Carryover required (Yes / No)
- Carryover cycle length
- Carryover penalty

Solutions requiring carryover remain feasible but are ranked lower.

---

## Stability Evaluation

The software estimates stack stability using:

width_ratio =
effective_unsupported_width /
stick_length

Target:

width_ratio <= 0.8

Values above the threshold receive progressively larger penalties.

Candidates are not rejected solely because of stability concerns.

---

# Optimization Workflow

The optimization process follows these steps:

### 1. Candidate Generation

For every format:

- Generate valid pitches
- Generate valid groupings
- Generate valid divider configurations
- Generate valid pocket configurations
- Calculate all engineering metrics

### 2. Feasibility Filtering

Reject configurations that violate:

- Pitch limits
- Cartoner pitch limits
- Physical fit constraints
- Grouping divisibility rules

### 3. Multi-Format Solution Generation

Build complete machine solutions where:

- One candidate exists for every format
- All formats share the same cartoner pitch

### 4. Metric Aggregation

For every machine-wide solution calculate:

- Pocket type count
- Robot head count
- Maximum layers
- Layer penalties
- Carryover penalties
- Grouping penalties
- Stability penalties
- Carton A/B ratio penalties

### 5. Pareto Filtering

Remove solutions dominated by other solutions across primary metrics.

### 6. Weighted Ranking

Apply the final engineering score and return the best N results.

---

# Carton A/B Ratio Evaluation

Newer versions introduce carton aspect-ratio evaluation.

The optimizer stores:

- Carton A/B ratio
- Carton A/B ratio penalty

This metric can be used to discourage geometrically unfavorable carton proportions and is included in exports and overall scoring.

---

# Graphical User Interface

The application includes a full Tkinter desktop interface.

Main capabilities:

### Global Settings

- Sticks per beat
- Maximum pitch shift
- Divider width
- Pocket wall width
- Maximum cartoner pitch
- Pitch step
- Number of displayed results
- Scoring weights

### Stick Types Table

Editable list of stick geometries.

### Formats Table

Editable list of commercial formats.

### Optimization Results

Displays ranked solutions with:

- Score
- Cartoner pitch
- Number of pocket types
- Number of robot head types
- Maximum layers
- Engineering penalties

### Detailed Solution View

For each format:

- Stick type
- Adjusted pitch
- Grouping
- Divider configuration
- Pockets per pitch
- Pocket dimensions
- Occupied width
- Unused space
- Layers
- Stack height
- Carryover information
- Stability metrics
- Pocket type
- Robot head type

---

# CSV Export

The software can export:

## Solution Summary

Includes:

- Score
- Cartoner pitch
- Pocket type count
- Robot head count
- Layer penalties
- Grouping penalties
- Carton A/B penalties

## Solution Details

Includes:

- Format data
- Pocket geometry
- Pitch information
- Carryover information
- Stability metrics
- Pocket type definition
- Robot head definition
- Carton A/B ratio information

---

# User Defaults

User preferences can be persisted between sessions through:

user_defaults.json

Typical stored information includes:

- Global settings
- Commonly used values
- GUI defaults

---

# Project Structure

stick_optimizer/

├── gui/
│ ├── __init__.py
│ ├── app.py
│ ├── forms.py
│ ├── parsing.py
│ ├── results.py
│ └── tables.py
│
├── defaults.py
├── export.py
├── main.py
├── models.py
├── optimizer.py
├── pareto.py
├── scoring.py
├── validation.py
├── tests.py
├── user_defaults.json
└── stick_optimpizer_logo.ico

---

# Running the Application

## Requirements

- Python 3.10 or newer recommended
- No installation package required
- Designed to run directly from source

## Launch GUI

python main.py

## Run Test Suite

python tests.py

The test runner is intended to:

- Load example data
- Execute the optimizer
- Print top solutions
- Display detailed engineering information
- Verify that candidate and solution generation work correctly

---

# Design Principles

The project follows a few important rules:

- Optimization logic must stay outside the GUI.
- Engineering formulas should remain centralized.
- Results should be deterministic and reproducible.
- Business logic should be testable without the UI.
- Domain entities should be represented through dataclasses.

---

# Developer Notes

## File Responsibilities

### models.py

Domain models and dataclasses.

### optimizer.py

Candidate generation and multi-format search engine.

### scoring.py

All engineering penalties and final scoring logic.

### pareto.py

Pareto-efficient filtering.

### validation.py

Input validation and consistency checks.

### export.py

CSV export functionality.

### gui/*

User interface layer.

---

## Guidelines for Future Development

When adding new engineering criteria:

1. Add explicit fields to the domain models.
2. Calculate the metric in the optimizer.
3. Add ranking logic in scoring.py.
4. Expose the metric in exports.
5. Display it in the GUI where appropriate.

Avoid placing engineering formulas directly inside GUI callbacks.

---

# Limitations

This is a static geometric optimizer.

The software does not simulate:

- Robot kinematics
- Robot acceleration
- Robot speed limits
- Dynamic timing
- Buffer behavior
- Collision detection
- Mechanical deflection
- Product flow disturbances
- Real machine throughput limits
- Carton loading dynamics

A returned solution should therefore be interpreted as:

"Geometrically feasible and logically consistent"

and not as automatic proof of production viability.

Final machine validation remains an engineering responsibility.

---

# License

No license is currently included in the repository.

If the software will be used, redistributed, or integrated into commercial projects, define and add an explicit license before publication.
