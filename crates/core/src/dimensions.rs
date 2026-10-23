//! Read-only physical quotations from a calculated candidate's effective geometry.
//! Kept in the core so GUI and exports consume identical distances in mm.
use crate::{Candidate, Study};

#[derive(Clone, Debug, PartialEq)]
pub struct PocketGap {
    /// Gap between adjacent slots in the first layer, zero based.
    pub centre_spacing: f64,
    /// Stick surface to stick surface; includes any intervening divider.
    pub stick_clearance: f64,
    /// Surface clearances on the left/right of the divider, when present.
    pub divider_clearances: Option<[f64; 2]>,
    /// The central fixed pickup-pitch gap of grouping 4.
    pub structural: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CandidateDimensions {
    pub stick_width: f64,
    pub approach_spacing: Option<f64>,
    pub minimum_approach_spacing: Option<f64>,
    pub extra_spacing: Option<f64>,
    pub left_wall_clearance: f64,
    pub right_wall_clearance: f64,
    pub gaps: Vec<PocketGap>,
}
/// Project the actual first layer. `None` means the candidate cannot be joined
/// to the supplied study, or its coordinate inventory is incomplete.
/// No search, new constraint, rounding or change to candidate metrics occurs.
pub fn candidate_dimensions(study: &Study, c: &Candidate) -> Option<CandidateDimensions> {
    let stick = study.sticks.iter().find(|s| s.id == c.stick_id)?;
    study
        .formats
        .iter()
        .find(|f| f.id == c.format_id && f.stick_id == stick.id)?;
    let centres = &c.geometry.stick_centres;
    if centres.len() != c.grouping as usize
        || centres.is_empty()
        || c.geometry.divider_lefts.len() != c.dividers as usize
    {
        return None;
    }
    let half = stick.width / 2.0;
    let gaps = centres
        .windows(2)
        .enumerate()
        .map(|(i, pair)| {
            let left = pair[0] + half;
            let right = pair[1] - half;
            let divider =
                c.geometry.divider_lefts.iter().find(|&&x| {
                    x >= left - 1e-9 && x + study.settings.divider_width <= right + 1e-9
                });
            PocketGap {
                centre_spacing: pair[1] - pair[0],
                stick_clearance: right - left,
                divider_clearances: divider
                    .map(|&x| [x - left, right - x - study.settings.divider_width]),
                structural: c.grouping == 4 && i == 1,
            }
        })
        .collect::<Vec<_>>();
    let approach_spacing = gaps.first().map(|g| g.centre_spacing);
    let minimum_approach_spacing = approach_spacing.map(|_| {
        crate::head::minimum_approach_spacing(&study.settings, stick.width, c.grouping, c.dividers)
    });
    Some(CandidateDimensions {
        stick_width: stick.width,
        approach_spacing,
        minimum_approach_spacing,
        extra_spacing: approach_spacing
            .zip(minimum_approach_spacing)
            .map(|(a, m)| a - m),
        left_wall_clearance: centres[0] - half,
        right_wall_clearance: c.pocket_width as f64 - centres.last()? - half,
        gaps,
    })
}
