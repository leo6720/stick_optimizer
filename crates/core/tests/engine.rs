use std::sync::atomic::Ordering;
use stick_optimizer_core::*;

fn example() -> Study {
    Study {
        sticks: vec![
            Stick {
                id: 1,
                name: "A".into(),
                length: 110.0,
                width: 25.0,
                thickness: 6.0,
                fin: 5.0,
            },
            Stick {
                id: 2,
                name: "B".into(),
                length: 100.0,
                width: 26.0,
                thickness: 8.0,
                fin: 5.0,
            },
        ],
        formats: vec![
            Format {
                id: 1,
                name: "A10".into(),
                stick_id: 1,
                quantity: 10,
            },
            Format {
                id: 2,
                name: "A34".into(),
                stick_id: 1,
                quantity: 34,
            },
            Format {
                id: 3,
                name: "B7".into(),
                stick_id: 2,
                quantity: 7,
            },
        ],
        settings: Settings {
            result_count: 10000,
            ..Settings::default()
        },
        weights: Weights::default(),
    }
}

fn run(study: &Study) -> SearchReport {
    optimize(study, &SearchControl::default(), SearchLimits::default()).unwrap()
}

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-10 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

fn signature(solution: &Solution) -> String {
    solution
        .candidates
        .iter()
        .map(|c| {
            format!(
                "{}:{:.1}:{}:{}:{}",
                c.format_name,
                c.input_pitch.mm(),
                c.grouping,
                c.dividers,
                c.pockets_per_pitch
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

#[test]
fn small_cases_match_all_python_candidate_fields_and_ranked_pareto_solutions() {
    let fixture = include_str!("fixtures/python-reference.tsv");
    for name in ["multi", "group1", "half", "decimal"] {
        let mut study = example();
        if name != "multi" {
            study.sticks.truncate(1);
            study.formats = vec![Format {
                id: 1,
                name: if name == "group1" { "A4" } else { "A7" }.into(),
                stick_id: 1,
                quantity: if name == "group1" { 4 } else { 7 },
            }];
        }
        match name {
            "group1" => study.settings.channels = 11,
            "half" => {
                study.sticks[0].width = 25.5;
                study.sticks[0].fin = 4.5;
            }
            "decimal" => {
                study.settings.pitch_step = 0.1;
                study.settings.max_shift = 0.0;
            }
            _ => {}
        }
        let report = run(&study);
        let rows: Vec<Vec<&str>> = fixture
            .lines()
            .filter(|l| !l.starts_with('#'))
            .map(|l| l.split('\t').collect::<Vec<_>>())
            .filter(|r| r[1] == name)
            .collect();
        let summary = rows.iter().find(|r| r[0] == "R").unwrap();
        assert_eq!(report.estimated, summary[2].parse::<u64>().unwrap());
        assert_eq!(report.examined, report.estimated);
        let expected: Vec<_> = rows.iter().filter(|r| r[0] == "S").collect();
        assert_eq!(report.solutions.len(), expected.len(), "{name}");
        for (s, row) in report.solutions.iter().zip(expected) {
            assert_eq!(signature(s), row[2], "{name}");
            let actual: Vec<f64> = [s.pitch.mm()]
                .into_iter()
                .chain(s.metrics())
                .chain([s.max_layers as f64, s.score])
                .collect();
            for (a, e) in actual.into_iter().zip(&row[3..]) {
                near(a, e.parse().unwrap());
            }
            for c in &s.candidates {
                let row = rows
                    .iter()
                    .find(|r| {
                        r[0] == "C"
                            && r[2] == c.format_name
                            && r[3].parse::<f64>().unwrap() == c.input_pitch.mm()
                            && r[4].parse::<u32>().unwrap() == c.grouping
                            && r[5].parse::<u32>().unwrap() == c.dividers
                            && r[6].parse::<u32>().unwrap() == c.pockets_per_pitch
                    })
                    .unwrap();
                let actual = [
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
                for (a, e) in actual.into_iter().zip(&row[3..]) {
                    near(a, e.parse().unwrap());
                }
            }
        }
        for (id, count) in &report.candidates_by_format {
            let name = &study.formats.iter().find(|f| f.id == *id).unwrap().name;
            assert_eq!(
                *count,
                rows.iter().filter(|r| r[0] == "C" && r[2] == name).count()
            );
        }
    }
}

#[test]
fn every_solution_obeys_common_pitch_geometry_layers_and_format_coverage() {
    let study = example();
    let report = run(&study);
    assert!(!report.solutions.is_empty());
    for s in &report.solutions {
        assert_eq!(s.candidates.len(), study.formats.len());
        for (c, f) in s.candidates.iter().zip(&study.formats) {
            assert_eq!(c.format_id, f.id);
            assert_eq!(c.cartoner_pitch, s.pitch);
            assert!(c.occupied_width <= s.pitch.mm() + 1e-9);
            assert!(s.pitch.mm() <= study.settings.max_pitch + 1e-9);
            assert_eq!(c.layers, f.quantity.div_ceil(c.grouping));
            assert!(c.layers <= study.settings.max_layers.unwrap());
            assert!(study.settings.channels.is_multiple_of(c.grouping));
        }
        assert!(!report.solutions.iter().any(|other| other.dominates(s)));
    }
    assert_eq!(report.solutions, run(&study).solutions);
}

#[test]
fn no_candidates_and_no_common_pitch_have_distinct_errors() {
    let mut study = example();
    study.formats[2].quantity = 50;
    assert!(
        matches!(optimize(&study, &SearchControl::default(), SearchLimits::default()), Err(SearchError::NoCandidates(names)) if names == vec!["B7"])
    );
    study.formats[2].quantity = 7;
    study.settings.max_shift = 0.0;
    study.sticks[1].width = 52.5;
    study.sticks[1].fin = 5.0;
    assert_eq!(
        optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap_err(),
        SearchError::NoCommonPitch
    );
}

#[test]
fn validation_rejects_nonfinite_zero_invalid_references_and_one_channel() {
    let mut study = example();
    study.settings.channels = 1;
    study.sticks[0].width = f64::NAN;
    study.formats[0].stick_id = 999;
    study.weights.ab = f64::INFINITY;
    assert!(validate(&study).len() >= 4);
    assert!(matches!(
        optimize(&study, &SearchControl::default(), SearchLimits::default()),
        Err(SearchError::InvalidInput(_))
    ));
}

#[test]
fn resource_limits_and_cancellation_never_return_partial_rankings() {
    let study = example();
    let control = SearchControl::default();
    let limits = SearchLimits {
        combinations: 1,
        ..SearchLimits::default()
    };
    assert!(matches!(
        optimize(&study, &control, limits),
        Err(SearchError::LimitExceeded(_))
    ));
    assert_eq!(control.examined.load(Ordering::Relaxed), 0);
    assert!(control.estimated.load(Ordering::Relaxed) > 1);
    control.cancelled.store(true, Ordering::Relaxed);
    assert_eq!(
        optimize(&study, &control, SearchLimits::default()).unwrap_err(),
        SearchError::Cancelled
    );
    let limits = SearchLimits {
        frontier: 0,
        ..SearchLimits::default()
    };
    assert!(matches!(
        optimize(&study, &SearchControl::default(), limits),
        Err(SearchError::LimitExceeded(_))
    ));
}

#[test]
fn output_limit_is_applied_only_after_full_search_and_pareto() {
    let mut study = example();
    let full = run(&study);
    study.settings.result_count = 1;
    let limited = run(&study);
    assert_eq!(limited.solutions, full.solutions[..1]);
    assert_eq!(limited.examined, full.examined);
    assert_eq!(limited.pareto_count, full.pareto_count);
}

#[test]
fn progress_reports_completed_combinations_before_final_preparation() {
    let control = SearchControl::default();
    assert_eq!(control.estimated.load(Ordering::Relaxed), 0);
    assert!(!control.finalizing.load(Ordering::Acquire));
    let report = optimize(&example(), &control, SearchLimits::default()).unwrap();
    // The reference study has exactly 40 combinations, not an estimate of time.
    assert_eq!(control.examined.load(Ordering::Relaxed), 40);
    assert_eq!(control.estimated.load(Ordering::Relaxed), 40);
    assert_eq!(report.examined, 40);
    assert_eq!(report.pareto_count, 16);
    assert!(control.finalizing.load(Ordering::Acquire));

    let control = SearchControl::default();
    let limits = SearchLimits {
        combinations: 1,
        ..SearchLimits::default()
    };
    assert!(matches!(
        optimize(&example(), &control, limits),
        Err(SearchError::LimitExceeded(_))
    ));
    assert_eq!(control.examined.load(Ordering::Relaxed), 0);
    assert!(!control.finalizing.load(Ordering::Acquire));
}
