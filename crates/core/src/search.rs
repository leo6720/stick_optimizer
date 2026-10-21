use crate::{Candidate, Format, Pitch, Settings, Solution, Stick, Study, validate};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Default, Debug)]
pub struct SearchControl {
    pub cancelled: AtomicBool,
    pub examined: AtomicU64,
    pub estimated: AtomicU64,
}

#[derive(Clone, Copy, Debug)]
pub struct SearchLimits {
    pub combinations: u64,
    pub grid_steps: u64,
    pub candidates: usize,
    pub frontier: usize,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self {
            combinations: 1_000_000,
            grid_steps: 100_000,
            candidates: 100_000,
            frontier: 10_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchError {
    InvalidInput(Vec<String>),
    NoCandidates(Vec<String>),
    NoCommonPitch,
    Cancelled,
    LimitExceeded(String),
    NumericalRange,
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(errors) => write!(f, "Dati da correggere: {}", errors.join("; ")),
            Self::NoCandidates(names) => write!(
                f,
                "Nessun candidato per: {}. Controlla geometria e limite strati.",
                names.join(", ")
            ),
            Self::NoCommonPitch => write!(
                f,
                "I formati hanno candidati, ma nessun passo astucciatrice comune."
            ),
            Self::Cancelled => write!(
                f,
                "Calcolo annullato. Nessun risultato parziale viene mostrato."
            ),
            Self::LimitExceeded(reason) => write!(
                f,
                "Ricerca non completata: {reason}. Riduci il caso di studio."
            ),
            Self::NumericalRange => write!(
                f,
                "Valori fuori dalla precisione o dall'intervallo numerico supportato."
            ),
        }
    }
}
impl std::error::Error for SearchError {}

#[derive(Debug)]
pub struct SearchReport {
    pub solutions: Vec<Solution>,
    pub candidates_by_format: Vec<(u64, usize)>,
    pub estimated: u64,
    pub examined: u64,
    pub pareto_count: usize,
}

fn checkpoint(control: &SearchControl) -> Result<(), SearchError> {
    if control.cancelled.load(Ordering::Relaxed) {
        Err(SearchError::Cancelled)
    } else {
        Ok(())
    }
}

fn pitch(value: f64) -> Result<Pitch, SearchError> {
    let ticks = (value * 1_000_000.0).round_ties_even();
    // f64 represents integers exactly up to 2^53. Preserve that guarantee.
    if !ticks.is_finite() || ticks <= 0.0 || ticks >= (1u64 << 53) as f64 {
        return Err(SearchError::NumericalRange);
    }
    Ok(Pitch(ticks as i64))
}

pub(crate) fn layout(group: u32, dividers: u32) -> &'static [u32] {
    match (group, dividers) {
        (1, 0) => &[1],
        (2, 0) => &[2],
        (2, 1) => &[1, 1],
        (3, 0) => &[3],
        (3, 2) => &[1, 1, 1],
        (4, 0) => &[4],
        (4, 1) => &[2, 2],
        _ => unreachable!("internal allowed layout"),
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn compute(
    f: &Format,
    s: &Stick,
    p: &Settings,
    input: Pitch,
    grouping: u32,
    dividers: u32,
    pockets_per_pitch: u32,
) -> Result<Option<Candidate>, SearchError> {
    let compartments = layout(grouping, dividers);
    let adjacent: u32 = compartments.iter().map(|&n| n - 1).sum();
    let raw_width = grouping as f64 * s.width
        + dividers as f64 * p.divider_width
        + adjacent as f64 * p.adjacent_clearance
        + 2.0 * compartments.len() as f64 * p.wall_clearance;
    let rounded = raw_width.round_ties_even();
    if !rounded.is_finite() || rounded >= (1u64 << 53) as f64 {
        return Err(SearchError::NumericalRange);
    }
    let pocket_width = rounded as u64;
    let pocket_pitch = grouping as f64 * input.mm();
    let cartoner = pockets_per_pitch as f64 * pocket_pitch;
    let ratio = cartoner / p.pitch_step;
    let occupied = pockets_per_pitch as f64 * (rounded + 2.0 * p.wall_width);
    if cartoner > p.max_pitch + 1e-9
        || (ratio - ratio.round_ties_even()).abs() > 1e-6
        || occupied > cartoner + 1e-9
    {
        return Ok(None);
    }
    let layers = f.quantity.div_ceil(grouping);
    if p.max_layers.is_some_and(|max| layers > max) {
        return Ok(None);
    }
    let stack_height = layers as f64 * s.thickness;
    let carton_b = stack_height + p.carton_b_extra;
    let ab_ratio = rounded / carton_b;
    let unsupported = compartments
        .iter()
        .map(|&n| {
            n as f64 * s.width + (n - 1) as f64 * p.adjacent_clearance + 2.0 * p.wall_clearance
        })
        .fold(0.0, f64::max);
    let width_ratio = unsupported / s.length;
    let carryover_required = !f.quantity.is_multiple_of(grouping);
    let carryover_cycle = if carryover_required {
        grouping / gcd(f.quantity, grouping)
    } else {
        1
    };
    let comfort = if dividers > 0 { 9 } else { 7 };
    let penalties = [
        (layers.saturating_sub(comfort) as f64).powi(2),
        if carryover_required {
            2.0 + carryover_cycle as f64
        } else {
            0.0
        },
        match grouping {
            1 => 0.0,
            2 => 1.0,
            _ => 2.0,
        },
        if width_ratio <= 0.8 {
            0.0
        } else {
            ((width_ratio - 0.8) / 0.1).powi(2)
        },
        ((ab_ratio - p.ab_target) / p.ab_target).powi(2),
    ];
    let head_width = input.mm() * p.channels as f64;
    if ![
        occupied,
        stack_height,
        carton_b,
        ab_ratio,
        unsupported,
        width_ratio,
        head_width,
    ]
    .iter()
    .chain(penalties.iter())
    .all(|n| n.is_finite())
    {
        return Err(SearchError::NumericalRange);
    }
    Ok(Some(Candidate {
        format_id: f.id,
        format_name: f.name.clone(),
        stick_id: s.id,
        stick_name: s.name.clone(),
        input_pitch: input,
        grouping,
        dividers,
        pockets_per_pitch,
        pocket_width,
        pocket_length: s.length,
        pocket_pitch,
        cartoner_pitch: pitch(cartoner)?,
        occupied_width: occupied,
        unused_space: cartoner - occupied,
        layers,
        stack_height,
        carton_a: rounded,
        carton_b,
        ab_ratio,
        carryover_required,
        carryover_cycle,
        unsupported_width: unsupported,
        width_ratio,
        deposits: p.channels / grouping,
        head_width,
        penalties,
    }))
}

fn generate(
    f: &Format,
    s: &Stick,
    p: &Settings,
    control: &SearchControl,
    limits: SearchLimits,
) -> Result<Vec<Candidate>, SearchError> {
    let nominal = 2.0 * s.width + 2.0 * s.fin;
    let shift = (p.max_shift * 2.0 / (p.channels - 1) as f64).ceil();
    let lower = nominal - shift;
    let upper = nominal + shift;
    let first = ((lower - 1e-9) / p.pitch_step).ceil();
    let last = ((upper + 1e-9) / p.pitch_step).floor();
    if !first.is_finite() || !last.is_finite() || first.abs().max(last.abs()) >= (1u64 << 53) as f64
    {
        return Err(SearchError::NumericalRange);
    }
    if last - first + 1.0 > limits.grid_steps as f64 {
        return Err(SearchError::LimitExceeded(format!(
            "griglia oltre {} passi per formato",
            limits.grid_steps
        )));
    }
    let mut candidates = Vec::new();
    for index in first as i64..=last as i64 {
        checkpoint(control)?;
        let raw = index as f64 * p.pitch_step;
        if raw <= 0.0 {
            continue;
        }
        let adjusted = pitch(raw)?;
        if adjusted.mm() < lower - 1e-9 || adjusted.mm() > upper + 1e-9 {
            continue;
        }
        for grouping in 1..=4 {
            if !p.channels.is_multiple_of(grouping) {
                continue;
            }
            let dividers: &[u32] = match grouping {
                1 => &[0],
                3 => &[0, 2],
                _ => &[0, 1],
            };
            for &divider in dividers {
                for pockets in 1..=2 {
                    if let Some(candidate) = compute(f, s, p, adjusted, grouping, divider, pockets)?
                    {
                        candidates.push(candidate);
                        if candidates.len() > limits.candidates {
                            return Err(SearchError::LimitExceeded("troppi candidati".into()));
                        }
                    }
                }
            }
        }
    }
    Ok(candidates)
}

pub fn optimize(
    study: &Study,
    control: &SearchControl,
    limits: SearchLimits,
) -> Result<SearchReport, SearchError> {
    checkpoint(control)?;
    let errors = validate(study);
    if !errors.is_empty() {
        return Err(SearchError::InvalidInput(errors));
    }
    control.examined.store(0, Ordering::Relaxed);
    control.estimated.store(0, Ordering::Relaxed);
    let mut by_format = Vec::new();
    let mut counts = Vec::new();
    let mut missing = Vec::new();
    let mut total_candidates = 0usize;
    for format in &study.formats {
        checkpoint(control)?;
        let stick = study
            .sticks
            .iter()
            .find(|s| s.id == format.stick_id)
            .expect("validated reference");
        let candidates = generate(format, stick, &study.settings, control, limits)?;
        counts.push((format.id, candidates.len()));
        if candidates.is_empty() {
            missing.push(format.name.clone());
        }
        total_candidates = total_candidates.saturating_add(candidates.len());
        if total_candidates > limits.candidates {
            return Err(SearchError::LimitExceeded(
                "troppi candidati complessivi".into(),
            ));
        }
        let mut by_pitch = BTreeMap::<Pitch, Vec<Candidate>>::new();
        for candidate in candidates {
            by_pitch
                .entry(candidate.cartoner_pitch)
                .or_default()
                .push(candidate);
        }
        by_format.push(by_pitch);
    }
    if !missing.is_empty() {
        return Err(SearchError::NoCandidates(missing));
    }
    let common: Vec<_> = by_format[0]
        .keys()
        .copied()
        .filter(|pitch| by_format.iter().all(|map| map.contains_key(pitch)))
        .collect();
    if common.is_empty() {
        return Err(SearchError::NoCommonPitch);
    }
    let mut estimated = 0u64;
    for pitch in &common {
        let count = by_format
            .iter()
            .try_fold(1u64, |acc, map| acc.checked_mul(map[pitch].len() as u64));
        let count = count
            .ok_or_else(|| SearchError::LimitExceeded("stima combinazioni oltre u64".into()))?;
        estimated = estimated
            .checked_add(count)
            .ok_or_else(|| SearchError::LimitExceeded("stima combinazioni oltre u64".into()))?;
    }
    control.estimated.store(estimated, Ordering::Relaxed);
    if estimated > limits.combinations {
        return Err(SearchError::LimitExceeded(format!(
            "{estimated} combinazioni stimate, limite {}",
            limits.combinations
        )));
    }
    let mut frontier: Vec<Solution> = Vec::new();
    let mut examined = 0;
    for pitch in &common {
        let lists: Vec<_> = by_format.iter().map(|map| &map[pitch]).collect();
        let mut indices = vec![0usize; lists.len()];
        loop {
            checkpoint(control)?;
            let candidates = lists
                .iter()
                .zip(&indices)
                .map(|(list, &i)| list[i].clone())
                .collect();
            let solution = Solution::aggregate(candidates, &study.weights);
            if !solution.score.is_finite() || solution.penalties.iter().any(|v| !v.is_finite()) {
                return Err(SearchError::NumericalRange);
            }
            let mut dominated = false;
            for existing in &frontier {
                checkpoint(control)?;
                if existing.dominates(&solution) {
                    dominated = true;
                    break;
                }
            }
            if !dominated {
                frontier.retain(|existing| !solution.dominates(existing));
                frontier.push(solution);
                if frontier.len() > limits.frontier {
                    return Err(SearchError::LimitExceeded(format!(
                        "frontiera Pareto oltre {} soluzioni",
                        limits.frontier
                    )));
                }
            }
            examined += 1;
            control.examined.store(examined, Ordering::Relaxed);
            let mut position = indices.len();
            while position > 0 {
                position -= 1;
                indices[position] += 1;
                if indices[position] < lists[position].len() {
                    break;
                }
                indices[position] = 0;
            }
            if position == 0 && indices[0] == 0 {
                break;
            }
        }
    }
    // Stable sort: Python's metric/pitch tie order, then enumeration order.
    frontier.sort_by(|a, b| {
        a.score
            .total_cmp(&b.score)
            .then_with(|| {
                a.metrics()
                    .partial_cmp(&b.metrics())
                    .expect("finite metrics")
            })
            .then(a.pitch.cmp(&b.pitch))
    });
    let pareto_count = frontier.len();
    frontier.truncate(study.settings.result_count);
    checkpoint(control)?;
    Ok(SearchReport {
        solutions: frontier,
        candidates_by_format: counts,
        estimated,
        examined,
        pareto_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Weights;

    fn study() -> Study {
        Study {
            sticks: vec![Stick {
                id: 1,
                name: "A".into(),
                length: 110.0,
                width: 25.5,
                thickness: 6.0,
                fin: 4.5,
            }],
            formats: vec![Format {
                id: 1,
                name: "A7".into(),
                stick_id: 1,
                quantity: 7,
            }],
            settings: Settings {
                result_count: 10000,
                // Original Python-reference fixture input (before LEO-17 defaults).
                divider_width: 2.0,
                ..Settings::default()
            },
            weights: Weights::default(),
        }
    }

    #[test]
    fn widths_round_half_to_even_and_penalties_follow_reference_thresholds() {
        let study = study();
        let s = &study.sticks[0];
        let f = &study.formats[0];
        let p = &study.settings;
        let one = compute(f, s, p, Pitch(60_000_000), 1, 0, 1)
            .unwrap()
            .unwrap();
        let three = compute(f, s, p, Pitch(60_000_000), 3, 0, 1)
            .unwrap()
            .unwrap();
        assert_eq!(one.pocket_width, 30); // 29.5 -> 30
        assert_eq!(three.pocket_width, 82); // 82.5 -> 82
        assert_eq!(three.carryover_cycle, 3);
        assert_eq!(three.penalties[1], 5.0);
        let solution = Solution::aggregate(vec![one], &study.weights);
        assert!(solution.head_types.is_empty());
        let mut eight = f.clone();
        eight.quantity = 8;
        let a = compute(&eight, s, p, Pitch(60_000_000), 1, 0, 1)
            .unwrap()
            .unwrap();
        assert_eq!(a.penalties[0], 1.0);
        let four = compute(f, s, p, Pitch(60_000_000), 4, 1, 1)
            .unwrap()
            .unwrap();
        assert_eq!(four.carryover_cycle, 4);
        assert_eq!(four.penalties[1], 6.0);
        assert_eq!(four.penalties[3], 0.0);
    }

    #[test]
    fn generated_candidates_match_every_python_field_including_dominated_candidates() {
        let study = study();
        let generated = generate(
            &study.formats[0],
            &study.sticks[0],
            &study.settings,
            &SearchControl::default(),
            SearchLimits::default(),
        )
        .unwrap();
        let rows: Vec<Vec<_>> = include_str!("../tests/fixtures/python-reference.tsv")
            .lines()
            .filter(|l| l.starts_with("C\thalf\t"))
            .map(|l| l.split('\t').collect())
            .collect();
        assert_eq!(generated.len(), rows.len());
        for (c, row) in generated.iter().zip(rows) {
            let values = [
                c.input_pitch.mm(),
                c.grouping as f64,
                c.dividers as f64,
                c.pockets_per_pitch as f64,
                c.pocket_width as f64,
                c.pocket_length,
                c.pocket_pitch,
                c.cartoner_pitch.mm(),
                c.occupied_width,
                c.unused_space,
                c.layers as f64,
                c.stack_height,
                c.carton_a,
                c.carton_b,
                c.ab_ratio,
                u8::from(c.carryover_required) as f64,
                c.carryover_cycle as f64,
                c.unsupported_width,
                c.width_ratio,
                c.deposits as f64,
                c.head_width,
                c.penalties[0],
                c.penalties[1],
                c.penalties[2],
                c.penalties[3],
                c.penalties[4],
            ];
            for (value, expected) in values.into_iter().zip(&row[3..]) {
                let expected: f64 = expected.parse().unwrap();
                assert!(
                    (value - expected).abs() < 1e-10 * expected.abs().max(1.0),
                    "{value} != {expected}"
                );
            }
        }
    }

    #[test]
    fn incremental_pareto_matches_materialized_brute_force_with_equal_metrics() {
        let mut study = study();
        study.formats.push(Format {
            id: 2,
            name: "A10".into(),
            stick_id: 1,
            quantity: 10,
        });
        let control = SearchControl::default();
        let limits = SearchLimits::default();
        let lists: Vec<_> = study
            .formats
            .iter()
            .map(|f| generate(f, &study.sticks[0], &study.settings, &control, limits).unwrap())
            .collect();
        let mut all = Vec::new();
        for a in &lists[0] {
            for b in &lists[1] {
                if a.cartoner_pitch == b.cartoner_pitch {
                    all.push(Solution::aggregate(
                        vec![a.clone(), b.clone()],
                        &study.weights,
                    ));
                }
            }
        }
        let mut expected: Vec<_> = all
            .iter()
            .filter(|s| !all.iter().any(|other| other.dominates(s)))
            .cloned()
            .collect();
        expected.sort_by(|a, b| {
            a.score
                .total_cmp(&b.score)
                .then(a.metrics().partial_cmp(&b.metrics()).unwrap())
                .then(a.pitch.cmp(&b.pitch))
        });
        let report = optimize(&study, &control, limits).unwrap();
        assert_eq!(report.examined, all.len() as u64);
        assert_eq!(report.solutions, expected);
        let mut equal = expected[0].clone();
        equal.pitch = Pitch(equal.pitch.0 + 5_000_000);
        assert!(!equal.dominates(&expected[0]));
        assert!(!expected[0].dominates(&equal));
    }
}
