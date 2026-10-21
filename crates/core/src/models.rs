#[derive(Clone, Debug)]
pub struct Stick {
    pub id: u64,
    pub name: String,
    pub length: f64,
    pub width: f64,
    pub thickness: f64,
    pub fin: f64,
}

#[derive(Clone, Debug)]
pub struct Format {
    pub id: u64,
    pub name: String,
    pub stick_id: u64,
    pub quantity: u32,
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub channels: u32,
    pub max_shift: f64,
    pub divider_width: f64,
    /// Maximum excess over each assigned format's minimum moving-gap spacing, mm.
    /// Used by final_head_geometry; search integration is deferred to LEO-18.
    pub max_extra_spacing: f64,
    pub wall_width: f64,
    pub adjacent_clearance: f64,
    pub wall_clearance: f64,
    pub carton_b_extra: f64,
    pub max_pitch: f64,
    pub pitch_step: f64,
    pub result_count: usize,
    pub max_layers: Option<u32>,
    pub ab_target: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            channels: 12,
            max_shift: 20.0,
            divider_width: 7.5,
            max_extra_spacing: 5.0,
            wall_width: 3.0,
            adjacent_clearance: 1.0,
            wall_clearance: 2.0,
            carton_b_extra: 5.0,
            max_pitch: 300.0,
            pitch_step: 5.0,
            result_count: 20,
            max_layers: Some(12),
            ab_target: 1.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Weights {
    pub pockets: f64,
    pub heads: f64,
    pub layers: f64,
    pub carryover: f64,
    pub grouping: f64,
    pub stability: f64,
    pub pitch: f64,
    pub ab: f64,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            pockets: 300.0,
            heads: 200.0,
            layers: 300.0,
            carryover: 30.0,
            grouping: 10.0,
            stability: 30.0,
            pitch: 0.5,
            ab: 600.0,
        }
    }
}

impl Weights {
    pub fn values(&self) -> [f64; 8] {
        [
            self.pockets,
            self.heads,
            self.layers,
            self.carryover,
            self.grouping,
            self.stability,
            self.pitch,
            self.ab,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Study {
    pub sticks: Vec<Stick>,
    pub formats: Vec<Format>,
    pub settings: Settings,
    pub weights: Weights,
}

/// Keys use millionths of a millimetre. Input pitches are rounded to six
/// decimals, matching Python's grid; common-pitch grouping uses these keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(pub i64);

impl Pitch {
    pub fn mm(self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    pub format_id: u64,
    pub format_name: String,
    pub stick_id: u64,
    pub stick_name: String,
    pub input_pitch: Pitch,
    pub grouping: u32,
    pub dividers: u32,
    pub pockets_per_pitch: u32,
    pub pocket_width: u64,
    pub pocket_length: f64,
    pub pocket_pitch: f64,
    pub cartoner_pitch: Pitch,
    pub occupied_width: f64,
    pub unused_space: f64,
    pub layers: u32,
    pub stack_height: f64,
    pub carton_a: f64,
    pub carton_b: f64,
    pub ab_ratio: f64,
    pub carryover_required: bool,
    pub carryover_cycle: u32,
    pub unsupported_width: f64,
    pub width_ratio: f64,
    pub deposits: u32,
    pub head_width: f64,
    /// Layers, carryover, grouping, stability, A/B; all minimized.
    pub penalties: [f64; 5],
}

#[derive(Clone, Debug, PartialEq)]
pub struct Solution {
    pub candidates: Vec<Candidate>,
    pub pitch: Pitch,
    pub pocket_types: Vec<(u64, f64, u32, u32)>,
    pub head_types: Vec<(u32, Pitch)>,
    pub max_layers: u32,
    pub penalties: [f64; 5],
    pub score: f64,
}

impl Solution {
    pub fn metrics(&self) -> [f64; 7] {
        [
            self.pocket_types.len() as f64,
            self.head_types.len() as f64,
            self.penalties[0],
            self.penalties[1],
            self.penalties[2],
            self.penalties[3],
            self.penalties[4],
        ]
    }

    pub fn dominates(&self, other: &Self) -> bool {
        let a = self.metrics();
        let b = other.metrics();
        a.iter().zip(b).all(|(&x, y)| x <= y) && a.iter().zip(b).any(|(&x, y)| x < y)
    }

    pub(crate) fn aggregate(candidates: Vec<Candidate>, weights: &Weights) -> Self {
        let mut pocket_types = Vec::new();
        let mut head_types = Vec::new();
        let mut penalties = [0.0; 5];
        let mut max_layers = 0;
        for c in &candidates {
            let pocket = (
                c.pocket_width,
                c.pocket_length,
                c.dividers,
                c.pockets_per_pitch,
            );
            if !pocket_types.contains(&pocket) {
                pocket_types.push(pocket);
            }
            let head = (c.grouping, c.input_pitch);
            if c.grouping != 1 && !head_types.contains(&head) {
                head_types.push(head);
            }
            max_layers = max_layers.max(c.layers);
            for (total, value) in penalties.iter_mut().zip(c.penalties) {
                *total += value;
            }
        }
        let pitch = candidates[0].cartoner_pitch;
        let score = weights.pockets * pocket_types.len() as f64
            + weights.heads * head_types.len() as f64
            + weights.layers * penalties[0]
            + weights.carryover * penalties[1]
            + weights.grouping * penalties[2]
            + weights.stability * penalties[3]
            + weights.pitch * pitch.mm()
            + weights.ab * penalties[4];
        Self {
            candidates,
            pitch,
            pocket_types,
            head_types,
            max_layers,
            penalties,
            score,
        }
    }
}
