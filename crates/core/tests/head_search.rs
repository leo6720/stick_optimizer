use stick_optimizer_core::*;

fn one_group(widths: &[f64], group: u32) -> Study {
    Study {
        sticks: widths
            .iter()
            .enumerate()
            .map(|(i, &width)| Stick {
                id: i as u64,
                name: format!("S{i}"),
                length: 110.0,
                width,
                thickness: 6.0,
                fin: 30.0 - width,
            })
            .collect(),
        formats: widths
            .iter()
            .enumerate()
            .map(|(i, _)| Format {
                id: i as u64,
                name: format!("F{i}"),
                stick_id: i as u64,
                quantity: group,
            })
            .collect(),
        settings: Settings {
            channels: group,
            max_shift: 0.0,
            max_layers: Some(1),
            max_pitch: group as f64 * 60.0,
            result_count: 10000,
            ..Settings::default()
        },
        weights: Weights::default(),
    }
}

#[test]
fn identical_divided_drawers_share_one_type_despite_coordinate_roundoff() {
    let mut study = one_group(&[20.0, 20.01], 2);
    study.settings.ab_target = 56.0 / 11.0;
    let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
    let shared = report
        .solutions
        .iter()
        .find(|s| s.head_types.len() == 1 && s.candidates.iter().all(|c| c.dividers == 1))
        .expect("the compatible shared divided head must survive Pareto");
    // Both physical drawers are 56mm wide with a centred 7.5mm divider:
    // its left edge is (56 - 7.5)/2 = 24.25mm for either stick width.
    assert!(shared.candidates.iter().all(|c| c.pocket_width == 56));
    assert!(shared.candidates.iter().all(|c| {
        c.geometry.divider_lefts.len() == 1 && (c.geometry.divider_lefts[0] - 24.25).abs() < 1e-9
    }));
    assert_eq!(shared.pocket_types.len(), 1);
    assert_eq!(shared.metrics(), [1.0, 1.0, 0.0, 0.0, 2.0, 0.0, 0.0]);
    assert_eq!(shared.score, 580.0); // 300 drawer +200 head +20 grouping +60 pitch.
    assert!(
        !report
            .solutions
            .iter()
            .any(|s| { s.head_types.len() == 2 && s.candidates.iter().all(|c| c.dividers == 1) }),
        "the shared head dominates identical drawers with separate heads"
    );
}

#[test]
fn drawer_identity_is_transitive_and_keeps_distinct_divider_positions() {
    let pocket = |position| PocketType {
        width: 56,
        length: 120.0,
        dividers: 1,
        pockets_per_pitch: 1,
        divider_lefts: vec![position],
    };
    let a = pocket(24.25 - 0.4e-9);
    let b = pocket(24.25);
    let c = pocket(24.25 + 0.4e-9);
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
    assert_ne!(a, pocket(24.25 + 2e-9));
    // Large finite positions must not collapse through integer saturation or
    // floating-point overflow during conversion to identity ticks.
    for position in [1e20_f64, f64::MAX / 2.0] {
        assert_eq!(pocket(position), pocket(position));
        assert_ne!(pocket(position), pocket(position * 1.5));
    }
}

#[test]
fn group_four_drawer_contains_two_pairs_at_the_fixed_pickup_pitch() {
    let study = one_group(&[25.0], 4);
    let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
    assert!(!report.solutions.is_empty());
    for solution in &report.solutions {
        let c = &solution.candidates[0];
        // Centres 34,60,120,146 => 112mm span +25mm stick +4mm wall games.
        assert_eq!(c.pocket_width, 141);
        let drawing = schematic(&study, c).unwrap();
        let mut xs: Vec<_> = drawing.sticks.iter().map(|r| r.x).collect();
        xs.sort_by(f64::total_cmp);
        assert_eq!(xs, vec![5.0, 31.0, 91.0, 117.0]);
    }
}

#[derive(Clone, Debug)]
struct Expected {
    widths: Vec<u64>,
    dividers: Vec<u32>,
    heads: Vec<Vec<u64>>,
    metrics: [f64; 7],
    score: f64,
}

// Independent reference: enumerate EVERY n-ary label map, canonicalize unordered
// blocks with a set, then use a materialized pairwise Pareto filter. Production
// uses restricted-growth iteration and an incremental frontier. No production
// geometry, assignment, aggregation or dominance helper builds these expected values.
fn oracle(study: &Study, group: u32) -> Vec<Expected> {
    use std::collections::BTreeSet;
    let n = study.formats.len();
    let mut partitions = BTreeSet::new();
    for map in 0..n.pow(n as u32) {
        let mut code = map;
        let mut blocks = vec![Vec::new(); n];
        for i in 0..n {
            blocks[code % n].push(i);
            code /= n;
        }
        blocks.retain(|b| !b.is_empty());
        blocks.sort();
        partitions.insert(blocks);
    }
    let mut all = Vec::new();
    for divmask in 0..(1usize << n) {
        let dividers: Vec<_> = (0..n)
            .map(|i| {
                if divmask & (1 << i) == 0 {
                    0
                } else if group == 3 {
                    2
                } else {
                    1
                }
            })
            .collect();
        for blocks in &partitions {
            let mut spacing = vec![0.0f64; n];
            let mut valid = true;
            for block in blocks {
                let minima: Vec<_> = block
                    .iter()
                    .map(|&i| {
                        study.sticks[i].width
                            + if dividers[i] > 0 && group != 4 {
                                11.5
                            } else {
                                1.0
                            }
                    })
                    .collect();
                let max = minima.iter().copied().fold(0.0f64, f64::max);
                if max > 60.0 + 1e-9
                    || minima
                        .iter()
                        .any(|m| max - m > study.settings.max_extra_spacing + 1e-9)
                {
                    valid = false;
                }
                for &i in block {
                    spacing[i] = max;
                    if group == 4
                        && study.sticks[i].width + if dividers[i] > 0 { 11.5 } else { 1.0 }
                            > 60.0 + 1e-9
                    {
                        valid = false;
                    }
                }
            }
            if !valid {
                continue;
            }
            let mut widths = Vec::new();
            let mut pocket_types = BTreeSet::new();
            let mut stability = 0.0;
            let mut ab = 0.0;
            for i in 0..n {
                let w = study.sticks[i].width;
                // Physical positions from the approved fixed-channel examples,
                // with arbitrary translation removed.
                let centres = match group {
                    2 => vec![0.0, spacing[i]],
                    3 => vec![-spacing[i], 0.0, spacing[i]],
                    4 => vec![-spacing[i], 0.0, 60.0, 60.0 + spacing[i]],
                    _ => unreachable!(),
                };
                let span = centres.last().unwrap() - centres[0];
                let width = (span + w + 4.0).ceil() as u64;
                if width as f64 + 6.0 > group as f64 * 60.0 + 1e-9 {
                    valid = false;
                    break;
                }
                widths.push(width);
                pocket_types.insert((
                    width,
                    dividers[i],
                    if group == 3 && dividers[i] > 0 {
                        spacing[i].to_bits()
                    } else {
                        0
                    },
                ));
                let unsupported = if dividers[i] == 0 {
                    width as f64
                } else {
                    let mut boundaries = vec![0.0];
                    let shift = (width as f64 - span) / 2.0 - centres[0];
                    let gaps = if group == 2 {
                        vec![0]
                    } else if group == 3 {
                        vec![0, 1]
                    } else {
                        vec![1]
                    };
                    let mut widths = Vec::new();
                    for j in gaps {
                        let divider = (centres[j] + centres[j + 1]) / 2.0 + shift;
                        widths.push(divider - 3.75 - boundaries.last().unwrap());
                        boundaries.push(divider + 3.75);
                    }
                    widths.push(width as f64 - boundaries.last().unwrap());
                    widths.into_iter().fold(0.0f64, f64::max)
                };
                let ratio = unsupported / 110.0;
                if ratio > 0.8 {
                    stability += ((ratio - 0.8) / 0.1).powi(2);
                }
                ab += ((width as f64 / 11.0 - 1.5) / 1.5).powi(2);
            }
            if !valid {
                continue;
            }
            let metrics = [
                pocket_types.len() as f64,
                blocks.len() as f64,
                0.0,
                0.0,
                n as f64 * if group == 2 { 1.0 } else { 2.0 },
                stability,
                ab,
            ];
            let score = metrics[0] * 300.0
                + metrics[1] * 200.0
                + metrics[4] * 10.0
                + stability * 30.0
                + group as f64 * 30.0
                + ab * 600.0;
            all.push(Expected {
                widths,
                dividers: dividers.clone(),
                heads: blocks
                    .iter()
                    .map(|b| b.iter().map(|&i| i as u64).collect())
                    .collect(),
                metrics,
                score,
            });
        }
    }
    let pareto: Vec<_> = all
        .iter()
        .filter(|a| {
            !all.iter().any(|b| {
                (0..7).all(|i| b.metrics[i] <= a.metrics[i])
                    && (0..7).any(|i| b.metrics[i] < a.metrics[i])
            })
        })
        .cloned()
        .collect();
    pareto
}

fn signature(widths: &[u64], dividers: &[u32], heads: &[Vec<u64>]) -> String {
    format!("{widths:?}/{dividers:?}/{heads:?}")
}

#[test]
fn exact_small_search_matches_independent_assignments_geometry_metrics_and_pareto() {
    use std::collections::BTreeMap;
    for group in [2, 3, 4] {
        for widths in [
            vec![25.0, 26.0],
            vec![20.0, 25.0, 26.0],
            vec![20.0, 25.0],
            vec![20.0, 25.000001],
            vec![25.1],
            vec![25.1, 25.2],
        ] {
            let study = one_group(&widths, group);
            let expected = oracle(&study, group);
            let expected: BTreeMap<_, _> = expected
                .into_iter()
                .map(|e| (signature(&e.widths, &e.dividers, &e.heads), e))
                .collect();
            assert!(!expected.is_empty());
            let report =
                optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
            assert_eq!(
                report.solutions.len(),
                expected.len(),
                "g{group} {widths:?}"
            );
            let mut actual = BTreeMap::new();
            for solution in &report.solutions {
                let widths: Vec<_> = solution.candidates.iter().map(|c| c.pocket_width).collect();
                let dividers: Vec<_> = solution.candidates.iter().map(|c| c.dividers).collect();
                let heads: Vec<_> = solution
                    .head_types
                    .iter()
                    .map(|h| h.geometry.format_ids.clone())
                    .collect();
                let key = signature(&widths, &dividers, &heads);
                assert!(
                    actual.insert(key.clone(), ()).is_none(),
                    "duplicate alternative {key}"
                );
                let e = expected
                    .get(&key)
                    .unwrap_or_else(|| panic!("unexpected g{group} {key}"));
                for (got, want) in solution.metrics().into_iter().zip(e.metrics) {
                    assert!((got - want).abs() < 1e-10, "{got} != {want}");
                }
                assert!((solution.score - e.score).abs() < 1e-8);
                for c in &solution.candidates {
                    let h = &solution.head_types[c.head_id.unwrap()];
                    assert!(h.geometry.format_ids.contains(&c.format_id));
                    assert_eq!(c.head_width, group as f64 * 60.0);
                    let relative: Vec<_> = h
                        .geometry
                        .positions
                        .iter()
                        .take(group as usize)
                        .map(|p| p.final_x - h.geometry.positions[0].final_x)
                        .collect();
                    for (i, x) in relative.iter().enumerate() {
                        assert!(
                            (c.geometry.stick_centres[i] - c.geometry.stick_centres[0] - x).abs()
                                < 1e-9
                        );
                    }
                }
            }
            assert_eq!(report.estimated, report.examined);
            let again =
                optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
            assert_eq!(report.solutions, again.solutions);
        }
    }
}

#[test]
fn integer_drawer_never_rounds_below_final_wall_clearance() {
    let study = one_group(&[25.1], 2);
    let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
    let c = report
        .solutions
        .iter()
        .flat_map(|s| &s.candidates)
        .find(|c| c.dividers == 0)
        .unwrap();
    assert_eq!(c.pocket_width, 56); // physical minimum55.2mm, legacy nearest gave55.
    assert!(c.geometry.stick_centres[0] - 25.1 / 2.0 >= 2.0 - 1e-9);
    assert!(56.0 - c.geometry.stick_centres[1] - 25.1 / 2.0 >= 2.0 - 1e-9);
}

#[test]
fn partitions_count_completed_work_and_limits_never_return_partial_rankings() {
    use std::sync::atomic::Ordering;
    let study = one_group(&[20.0, 25.0, 26.0], 2);
    let control = SearchControl::default();
    let report = optimize(&study, &control, SearchLimits::default()).unwrap();
    // 2 divider choices per format, Bell(3)=5 unordered head partitions.
    // Includes rejected joint assignments (20,25,26 cannot all share).
    assert_eq!((report.estimated, report.examined), (40, 40));
    assert_eq!(control.estimated.load(Ordering::Relaxed), 40);
    assert_eq!(control.examined.load(Ordering::Relaxed), 40);
    assert!(report.solutions.iter().all(|s| s.head_types.len() >= 2));
    let bound = SearchLimits {
        combinations: 39,
        ..SearchLimits::default()
    };
    assert!(matches!(
        optimize(&study, &SearchControl::default(), bound),
        Err(SearchError::LimitExceeded(_))
    ));
    let exact = SearchLimits {
        combinations: 40,
        ..SearchLimits::default()
    };
    assert_eq!(
        optimize(&study, &SearchControl::default(), exact)
            .unwrap()
            .solutions,
        report.solutions
    );
    assert!(matches!(
        optimize(
            &study,
            &SearchControl::default(),
            SearchLimits {
                frontier: 0,
                ..exact
            }
        ),
        Err(SearchError::LimitExceeded(_))
    ));
    let mut short = study.clone();
    short.settings.result_count = 1;
    let limited = optimize(&short, &SearchControl::default(), exact).unwrap();
    assert_eq!(limited.solutions, report.solutions[..1]);
    assert_eq!(limited.pareto_count, report.pareto_count);
    assert_eq!(limited.examined, 40);

    let nearby = optimize(
        &one_group(&[25.0, 26.0], 2),
        &SearchControl::default(),
        SearchLimits::default(),
    )
    .unwrap();
    assert!(nearby.solutions.iter().any(|s| s.head_types.len() == 1));
    assert!(nearby.solutions.iter().any(|s| s.head_types.len() == 2));
}

#[test]
fn running_assignment_search_can_be_cancelled_without_a_ranking() {
    use std::sync::{Arc, atomic::Ordering};
    let study = one_group(&[25.0; 8], 2);
    let control = Arc::new(SearchControl::default());
    let worker_control = Arc::clone(&control);
    let handle = std::thread::spawn(move || {
        optimize(
            &study,
            &worker_control,
            SearchLimits {
                combinations: 2_000_000,
                ..SearchLimits::default()
            },
        )
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while control.examined.load(Ordering::Relaxed) == 0 && std::time::Instant::now() < deadline {
        std::thread::yield_now();
    }
    assert!(
        control.examined.load(Ordering::Relaxed) > 0,
        "search must reach assignment evaluation"
    );
    control.cancelled.store(true, Ordering::Relaxed);
    assert_eq!(handle.join().unwrap().unwrap_err(), SearchError::Cancelled);
    assert!(control.examined.load(Ordering::Relaxed) < control.estimated.load(Ordering::Relaxed));
}

#[test]
fn head_identity_is_canonical_and_group_one_does_not_count() {
    let study = one_group(&[20.0, 25.0, 26.0], 2);
    let normal = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
    let mut reordered = study.clone();
    reordered.formats.reverse();
    reordered.sticks.reverse();
    let reversed = optimize(
        &reordered,
        &SearchControl::default(),
        SearchLimits::default(),
    )
    .unwrap();
    for a in &normal.solutions {
        let b = reversed
            .solutions
            .iter()
            .find(|b| {
                b.head_types == a.head_types
                    && a.candidates
                        .iter()
                        .all(|c| b.candidates.iter().any(|d| d == c))
            })
            .unwrap();
        assert!((a.score - b.score).abs() < 1e-9);
        for (id, h) in a.head_types.iter().enumerate() {
            assert_eq!(h.id, id);
        }
    }
    let mut group1 = one_group(&[25.0], 2);
    group1.settings.channels = 5;
    group1.settings.max_layers = None;
    let report = optimize(&group1, &SearchControl::default(), SearchLimits::default()).unwrap();
    assert!(
        report
            .solutions
            .iter()
            .all(|s| s.head_types.is_empty() && s.candidates.iter().all(|c| c.head_id.is_none()))
    );
}

#[test]
fn different_pickup_pitches_never_share_even_at_a_common_cartoner_pitch() {
    let mut study = one_group(&[20.0, 20.0], 4);
    study.settings.channels = 12;
    study.settings.max_layers = Some(2);
    study.settings.max_pitch = 240.0;
    study.sticks[1].fin = 10.0; // first pickup60, second pickup60 initially; force120 below.
    study.sticks[0].fin = 40.0; // first pickup120 -> group2, one pocket =>240.
    study.sticks[1].fin = 10.0; // second pickup60 -> group4, one pocket =>240.
    let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
    assert!(!report.solutions.is_empty());
    for s in report.solutions {
        assert_eq!(s.pitch, Pitch(240_000_000));
        assert_eq!(s.head_types.len(), 2);
        assert_ne!(s.candidates[0].input_pitch, s.candidates[1].input_pitch);
        assert!(
            s.head_types
                .iter()
                .all(|h| h.geometry.format_ids.len() == 1)
        );
    }
}

#[test]
fn compatible_head_is_rejected_when_effective_shared_drawer_exceeds_pitch() {
    let mut study = one_group(&[25.0, 30.0], 2);
    study.sticks.iter_mut().for_each(|s| s.fin = 0.0);
    study.settings.max_shift = 11.5;
    study.settings.pitch_step = 37.5;
    study.settings.max_pitch = 75.0;
    study.settings.max_extra_spacing = 6.0;
    // Both standalone drawers fit75mm (66+6 and65+6). Jointspacing36.5
    // is compatible, but the wider stick then needs71+6>75mm.
    let head = final_head_geometry(
        &study.settings,
        &[
            HeadFormat {
                format_id: 0,
                stick_width: 25.0,
                pickup_pitch: Pitch(37_500_000),
                grouping: 2,
                dividers: 1,
            },
            HeadFormat {
                format_id: 1,
                stick_width: 30.0,
                pickup_pitch: Pitch(37_500_000),
                grouping: 2,
                dividers: 0,
            },
        ],
    )
    .unwrap();
    assert_eq!(head.approach_spacing, 36.5);
    let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
    assert!(!report.solutions.is_empty());
    for s in &report.solutions {
        assert!(s.candidates.iter().all(|c| c.occupied_width <= 75.0 + 1e-9));
        assert!(
            !(s.head_types.len() == 1
                && s.candidates[0].dividers == 1
                && s.candidates[1].dividers == 0)
        );
    }
}
