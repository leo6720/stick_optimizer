use std::collections::BTreeSet;
use stick_optimizer_core::*;

#[test]
fn single_layer_view_preserves_real_geometry_and_solution_data() {
    let mut covered = BTreeSet::new();
    let mut partial = false;
    let mut many_layers = false;
    for quantity in [1, 7, 12, 200] {
        let study = Study {
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
                name: "A".into(),
                stick_id: 1,
                quantity,
            }],
            settings: Settings {
                result_count: 10_000,
                max_layers: None,
                ..Settings::default()
            },
            weights: Weights::default(),
        };
        let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
        assert!(!report.solutions.is_empty());
        for solution in &report.solutions {
            let c = &solution.candidates[0];
            let before = c.clone();
            let g = schematic(&study, c).unwrap();
            assert_eq!(c, &before);
            assert_eq!(g.pockets.len(), c.pockets_per_pitch as usize);
            assert_eq!(g.sticks.len(), (c.grouping * c.pockets_per_pitch) as usize);
            assert_eq!(
                g.dividers.len(),
                (c.dividers * c.pockets_per_pitch) as usize
            );
            assert_eq!(g.width, c.cartoner_pitch.mm());
            assert_eq!(g.height, c.carton_b);
            for pocket in &g.pockets {
                assert_eq!(pocket.height, c.carton_b);
                assert_eq!(pocket.width, c.pocket_width as f64);
                let sticks: Vec<_> = g
                    .sticks
                    .iter()
                    .filter(|s| s.x >= pocket.x && s.x + s.width <= pocket.x + pocket.width + 0.5)
                    .collect();
                assert_eq!(sticks.len(), c.grouping as usize);
                // Preserve the chosen assignment's actual horizontal coordinates,
                // including the fixed central spacing in group four.
                for (stick, &centre) in sticks.iter().zip(&c.geometry.stick_centres) {
                    assert!((stick.x + stick.width / 2.0 - pocket.x - centre).abs() < 1e-9);
                }
                let dividers: Vec<_> = g
                    .dividers
                    .iter()
                    .filter(|d| d.x >= pocket.x && d.x < pocket.x + pocket.width)
                    .collect();
                assert_eq!(dividers.len(), c.geometry.divider_lefts.len());
                for (divider, &left) in dividers.iter().zip(&c.geometry.divider_lefts) {
                    assert!((divider.x - pocket.x - left).abs() < 1e-9);
                }
                assert!(
                    sticks
                        .iter()
                        .all(|s| s.y == 0.0 && s.height == 6.0 && s.width == 25.0)
                );
                if c.layers > 1 {
                    assert!(pocket.height > 6.0);
                }
                for stick in sticks {
                    // No horizontal collision with any divider; the real gaps remain.
                    assert!(
                        g.dividers
                            .iter()
                            .all(|d| stick.x + stick.width <= d.x + 1e-9
                                || stick.x >= d.x + d.width - 1e-9)
                    );
                }
            }
            covered.insert((c.grouping, c.dividers, c.pockets_per_pitch));
            partial |= c.carryover_required;
            many_layers |= c.layers > 32;
        }
    }
    assert!(covered.iter().any(|&(group, _, _)| group == 1));
    assert!(covered.iter().any(|&(group, _, _)| group > 1));
    assert!(covered.iter().any(|&(_, dividers, _)| dividers > 0));
    assert!(covered.iter().any(|&(_, dividers, _)| dividers == 0));
    assert!(covered.iter().any(|&(_, _, pockets)| pockets == 1));
    assert!(covered.iter().any(|&(_, _, pockets)| pockets == 2));
    assert!(partial, "include a real partial last layer");
    assert!(
        many_layers,
        "include a real candidate above the former 32-layer cap"
    );
}
