//! Static section drawing coordinates in millimetres, derived from validated candidates.
use crate::{Candidate, Study};
#[derive(Clone, Copy, Debug)]
pub struct GeometryRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[derive(Debug)]
pub struct Geometry {
    pub width: f64,
    pub height: f64,
    pub pockets: Vec<GeometryRect>,
    pub dividers: Vec<GeometryRect>,
    pub sticks: Vec<GeometryRect>,
    pub truncated: bool,
}
/// The outline height is carton B, not an assumed physical pocket height.
/// At most 32 layers are drawn; quotas remain those of the full candidate.
pub fn schematic(study: &Study, c: &Candidate) -> Option<Geometry> {
    let s = study.sticks.iter().find(|s| s.id == c.stick_id)?;
    let f = study
        .formats
        .iter()
        .find(|f| f.id == c.format_id && f.stick_id == s.id)?;
    if !matches!(
        (c.grouping, c.dividers),
        (1, 0) | (2, 0) | (2, 1) | (3, 0) | (3, 2) | (4, 0) | (4, 1)
    ) || !(1..=2).contains(&c.pockets_per_pitch)
        || c.layers == 0
        || !c.carton_b.is_finite()
        || c.carton_b <= 0.0
        || c.cartoner_pitch.0 <= 0
    {
        return None;
    }
    let p = &study.settings;
    let mut g = Geometry {
        width: c.cartoner_pitch.mm(),
        height: c.carton_b,
        pockets: vec![],
        dividers: vec![],
        sticks: vec![],
        truncated: c.layers > 32,
    };
    for pocket in 0..c.pockets_per_pitch {
        let left = f64::from(pocket) * c.pocket_pitch + p.wall_width;
        g.pockets.push(GeometryRect {
            x: left,
            y: 0.0,
            width: c.pocket_width as f64,
            height: c.carton_b,
        });
        let mut x = left;
        let mut slot = 0;
        let layout = crate::search::layout(c.grouping, c.dividers);
        for (i, &count) in layout.iter().enumerate() {
            x += p.wall_clearance;
            for _ in 0..count {
                for layer in 0..c.layers.min(32) {
                    if u64::from(layer) * u64::from(c.grouping) + slot < u64::from(f.quantity) {
                        g.sticks.push(GeometryRect {
                            x,
                            y: f64::from(layer) * s.thickness,
                            width: s.width,
                            height: s.thickness,
                        });
                    }
                }
                slot += 1;
                x += s.width + p.adjacent_clearance;
            }
            x += p.wall_clearance - p.adjacent_clearance;
            if i + 1 < layout.len() {
                g.dividers.push(GeometryRect {
                    x,
                    y: 0.0,
                    width: p.divider_width,
                    height: c.carton_b,
                });
                x += p.divider_width;
            }
        }
    }
    Some(g)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Format, SearchControl, SearchLimits, Settings, Stick, Weights, optimize};
    #[test]
    fn drawing_matches_real_candidates_partial_layers_and_all_compartments() {
        let mut study = Study {
            sticks: vec![Stick {
                id: 1,
                name: "A".into(),
                length: 110.0,
                width: 25.0,
                thickness: 6.0,
                fin: 5.0,
            }],
            formats: vec![Format {
                id: 2,
                name: "A7".into(),
                stick_id: 1,
                quantity: 7,
            }],
            settings: Settings {
                result_count: 100,
                max_layers: None,
                ..Settings::default()
            },
            weights: Weights::default(),
        };
        let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
        assert!(!report.solutions.is_empty());
        for solution in report.solutions {
            let c = &solution.candidates[0];
            let g = schematic(&study, c).unwrap();
            assert_eq!(g.pockets.len(), c.pockets_per_pitch as usize);
            assert_eq!(
                g.dividers.len(),
                (c.dividers * c.pockets_per_pitch) as usize
            );
            assert_eq!(g.sticks.len(), 7 * c.pockets_per_pitch as usize);
            assert_eq!(g.width, c.cartoner_pitch.mm());
            assert_eq!(g.height, c.carton_b);
            assert!(!g.truncated);
            for r in g.sticks {
                assert_eq!(r.width, 25.0);
                assert_eq!(r.height, 6.0);
                assert!(r.y + r.height <= c.stack_height);
                assert!(r.x + r.width <= g.width + 0.5);
            }
            // Protect render cost even with very high layer limits.
            let mut huge = c.clone();
            huge.layers = u32::MAX;
            study.formats[0].quantity = u32::MAX;
            let limited = schematic(&study, &huge).unwrap();
            assert!(limited.truncated);
            assert!(limited.sticks.len() <= 256);
            study.formats[0].quantity = 7;
            let mut invalid = c.clone();
            invalid.grouping = 9;
            assert!(schematic(&study, &invalid).is_none());
            invalid = c.clone();
            invalid.stick_id = 999;
            assert!(schematic(&study, &invalid).is_none());
            invalid = c.clone();
            invalid.layers = 0;
            assert!(schematic(&study, &invalid).is_none());
        }
    }
}
