//! Geometry for a complete assignment to one fixed-final-position head.
//! Exact search verifies each separate/shared assignment through this public API.
use crate::{Pitch, Settings};
use std::{collections::BTreeSet, fmt};

const EPS: f64 = 1e-9;
const MAX_PITCH_TICKS: i64 = 1_i64 << 53;

/// One format assigned to a head. IDs must be unique within the assignment.
#[derive(Clone, Debug)]
pub struct HeadFormat {
    pub format_id: u64,
    pub stick_width: f64,
    pub pickup_pitch: Pitch,
    pub grouping: u32,
    pub dividers: u32,
}

/// Zero-based channel and group identities, with stick-centre coordinates in mm.
#[derive(Clone, Debug, PartialEq)]
pub struct HeadPosition {
    pub channel: u32,
    pub group: u32,
    pub pickup_x: f64,
    pub final_x: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeadGeometry {
    pub pickup_pitch: Pitch,
    pub grouping: u32,
    /// Minimum common centre spacing at each moving approach. For grouping 4
    /// this is the distance within each pair, never the fixed central distance.
    pub approach_spacing: f64,
    /// Sorted assignment IDs, independent of caller order.
    pub format_ids: Vec<u64>,
    pub positions: Vec<HeadPosition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HeadGeometryError {
    InvalidInput(&'static str),
    Incompatible(&'static str),
}
impl fmt::Display for HeadGeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(reason) => write!(f, "Input testa non valido: {reason}"),
            Self::Incompatible(reason) => write!(f, "Formati incompatibili: {reason}"),
        }
    }
}
impl std::error::Error for HeadGeometryError {}

/// Resolve ALL formats jointly; pairwise compatibility must not be chained.
/// Pickup pitch uses the core's exact millionth-mm key. Other lengths remain
/// unrounded f64 mm, with the core geometric tolerance of 1e-9 mm.
/// Grouping 1 is deliberately excluded from shared-head geometry/counting.
/// Dividers: g2 has 0/1, g3 0/2, g4 0/1 (the g4 divider separates the two pairs).
/// This validates final clearances and inward travel, not pocket fit or kinetics.
pub fn final_head_geometry(
    settings: &Settings,
    formats: &[HeadFormat],
) -> Result<HeadGeometry, HeadGeometryError> {
    use HeadGeometryError::{Incompatible, InvalidInput};
    let first = formats
        .first()
        .ok_or(InvalidInput("nessun formato assegnato"))?;
    if !matches!(first.grouping, 2..=4)
        || settings.channels < first.grouping
        || !settings.channels.is_multiple_of(first.grouping)
    {
        return Err(InvalidInput("raggruppamento o canali non validi"));
    }
    if first.pickup_pitch.0 <= 0 || first.pickup_pitch.0 >= MAX_PITCH_TICKS {
        return Err(InvalidInput("passo di prelievo fuori intervallo"));
    }
    for value in [
        settings.adjacent_clearance,
        settings.wall_clearance,
        settings.divider_width,
        settings.max_extra_spacing,
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(InvalidInput(
                "giochi, divisore e limite devono essere finiti e non negativi",
            ));
        }
    }
    let mut ids = BTreeSet::new();
    let mut minima = Vec::with_capacity(formats.len());
    let mut common: f64 = 0.0;
    let mut central_minimum: f64 = 0.0;
    let pitch = first.pickup_pitch.mm();
    for format in formats {
        if !ids.insert(format.format_id)
            || !format.stick_width.is_finite()
            || format.stick_width <= 0.0
        {
            return Err(InvalidInput("ID duplicato o larghezza stick non valida"));
        }
        if format.grouping != first.grouping || format.pickup_pitch != first.pickup_pitch {
            return Err(Incompatible("passo di prelievo o raggruppamento diverso"));
        }
        if !matches!(
            (format.grouping, format.dividers),
            (2, 0 | 1) | (3, 0 | 2) | (4, 0 | 1)
        ) {
            return Err(InvalidInput("divisori non ammessi per il raggruppamento"));
        }
        let adjacent = format.stick_width + settings.adjacent_clearance;
        let divided = format.stick_width + settings.divider_width + 2.0 * settings.wall_clearance;
        if !adjacent.is_finite() || !divided.is_finite() {
            return Err(InvalidInput("interasse minimo fuori intervallo"));
        }
        let minimum = if format.dividers > 0 && format.grouping != 4 {
            divided
        } else {
            adjacent
        };
        // The two g4 central channels stay one pickup pitch apart. Their
        // clearance must fit, but this structural spacing has no extra limit.
        if format.grouping == 4
            && pitch + EPS
                < if format.dividers > 0 {
                    divided
                } else {
                    adjacent
                }
        {
            return Err(Incompatible("gioco fra i centrali fissi insufficiente"));
        }
        if format.grouping == 4 {
            central_minimum = central_minimum.max(if format.dividers > 0 {
                divided
            } else {
                adjacent
            });
        }
        common = common.max(minimum);
        minima.push(minimum);
    }
    if common > pitch + EPS {
        return Err(Incompatible(
            "l'avvicinamento richiederebbe movimento verso l'esterno",
        ));
    }
    if minima
        .iter()
        .any(|minimum| common - minimum > settings.max_extra_spacing + EPS)
    {
        return Err(Incompatible("interasse aggiuntivo massimo superato"));
    }
    let mut positions = Vec::new();
    positions
        .try_reserve_exact(settings.channels as usize)
        .map_err(|_| InvalidInput("numero di canali non rappresentabile"))?;
    for channel in 0..settings.channels {
        let group = channel / first.grouping;
        let local = channel % first.grouping;
        let origin = f64::from(group * first.grouping) * pitch;
        let final_local = match (first.grouping, local) {
            (2, 0) => 0.0,
            (2, 1) => common,
            (3 | 4, 0) => pitch - common,
            (3 | 4, 1) => pitch,
            (3, 2) => pitch + common,
            (4, 2) => 2.0 * pitch,
            (4, 3) => 2.0 * pitch + common,
            _ => unreachable!("validated grouping and channel"),
        };
        positions.push(HeadPosition {
            channel,
            group,
            pickup_x: f64::from(channel) * pitch,
            final_x: origin + final_local,
        });
    }
    // Absolute coordinates can lose small gaps at large origins. Validate the
    // emitted f64 geometry, not only the spacing used to construct it.
    let precision_error = InvalidInput("precisione f64 insufficiente per le coordinate finali");
    for pair in positions.windows(2) {
        let left = &pair[0];
        let right = &pair[1];
        let gap = right.final_x - left.final_x;
        if left.group != right.group {
            if minima.iter().any(|minimum| gap + EPS < *minimum) {
                return Err(precision_error);
            }
        } else if first.grouping == 4 && left.channel % 4 == 1 {
            if (gap - pitch).abs() > EPS || gap + EPS < central_minimum {
                return Err(precision_error);
            }
        } else if (gap - common).abs() > EPS
            || minima.iter().any(|minimum| {
                gap + EPS < *minimum || gap - minimum > settings.max_extra_spacing + EPS
            })
        {
            return Err(precision_error);
        }
    }
    for position in &positions {
        let local = position.channel % first.grouping;
        let fixed = match first.grouping {
            2 => local == 0,
            3 => local == 1,
            4 => local == 1 || local == 2,
            _ => unreachable!("validated grouping"),
        };
        if fixed && (position.final_x - position.pickup_x).abs() > EPS {
            return Err(precision_error);
        }
    }
    Ok(HeadGeometry {
        pickup_pitch: first.pickup_pitch,
        grouping: first.grouping,
        approach_spacing: common,
        format_ids: ids.into_iter().collect(),
        positions,
    })
}
