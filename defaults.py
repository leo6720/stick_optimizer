from models import Format, GlobalSettings, StickType, Weights


ALLOWED_GROUPINGS = [1, 2, 3, 4]
ALLOWED_POCKETS_PER_PITCH = [1, 2]

CARRYOVER_FIXED_PENALTY = 2.0

GROUPING_PENALTIES = {
    1: 0.0,
    2: 1.0,
    3: 2.0,
    4: 2.0,
}

# Compartment layout: (grouping, dividers) -> [stick counts per compartment]
COMPARTMENT_LAYOUT = {
    (1, 0): [1],
    (2, 0): [2],
    (2, 1): [1, 1],
    (3, 0): [3],
    (3, 2): [1, 1, 1],
    (4, 0): [4],
    (4, 1): [2, 2],
}

DEFAULT_GLOBAL_SETTINGS = GlobalSettings(
    sticks_per_beat=12,
    max_pitch_shift_mm=10.0,
    divider_width_mm=2.0,
    pocket_wall_width_mm=3.0,
    clearance_between_adjacent_sticks_mm=1.0,
    clearance_stick_to_wall_or_divider_mm=2.0,
    carton_B_extra_mm=5.0,
    max_cartoner_pitch_mm=300.0,
    pitch_step_mm=5.0,
    number_of_results_to_show=5,
    max_allowed_layers=12,
    carton_AB_target=1.5,
)

DEFAULT_WEIGHTS = Weights()

# Example data is intentionally small but multi-format.
# Both stick types have overlapping valid input pitches, so common cartoner pitch
# solutions exist and tests can verify the complete optimizer path.
DEFAULT_STICK_TYPES = [
    StickType(
        stick_type_name="STICK_A",
        stick_length_mm=110.0,
        stick_width_mm=25.0,
        stick_thickness_mm=6.0,
        fin_length_mm=5.0,
    ),
    StickType(
        stick_type_name="STICK_B",
        stick_length_mm=100.0,
        stick_width_mm=26.0,
        stick_thickness_mm=8,
        fin_length_mm=5.0,
    ),
]

DEFAULT_FORMATS = [
    Format(format_name="A_10", stick_type_name="STICK_A", sticks_per_pocket=10),
    Format(format_name="A_34", stick_type_name="STICK_A", sticks_per_pocket=34),
    Format(format_name="B_4", stick_type_name="STICK_B", sticks_per_pocket=4),
    Format(format_name="B_7", stick_type_name="STICK_B", sticks_per_pocket=7),
    Format(format_name="B_10", stick_type_name="STICK_B", sticks_per_pocket=10),
    Format(format_name="B_16", stick_type_name="STICK_B", sticks_per_pocket=16),
    Format(format_name="B_25", stick_type_name="STICK_B", sticks_per_pocket=25),
    Format(format_name="B_30", stick_type_name="STICK_B", sticks_per_pocket=30),
    Format(format_name="B_50", stick_type_name="STICK_B", sticks_per_pocket=50)
]
