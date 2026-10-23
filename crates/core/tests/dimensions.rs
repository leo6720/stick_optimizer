use stick_optimizer_core::*;

fn example(group: u32) -> Study {
    Study {
        sticks: [25.0, 26.0]
            .into_iter()
            .enumerate()
            .map(|(i, width)| Stick {
                id: i as u64 + 1,
                name: format!("S{i}"),
                length: 110.0,
                width,
                thickness: 6.0,
                fin: 30.0 - width,
            })
            .collect(),
        formats: (0..2)
            .map(|i| Format {
                id: i + 3,
                name: format!("F{i}"),
                stick_id: i + 1,
                quantity: group,
            })
            .collect(),
        settings: Settings {
            channels: group,
            max_shift: 0.0,
            max_layers: Some(1),
            max_pitch: group as f64 * 60.0,
            result_count: 10000,
            ab_target: match group {
                2 => 68.0 / 11.0,
                3 => 105.0 / 11.0,
                _ => 144.0 / 11.0,
            },
            ..Settings::default()
        },
        weights: Weights::default(),
    }
}
#[test]
fn effective_dimensions_report_real_shared_divided_gaps_and_fixed_group_four() {
    for group in [2, 3, 4] {
        let study = example(group);
        let report = optimize(&study, &SearchControl::default(), SearchLimits::default()).unwrap();
        let solution = report
            .solutions
            .iter()
            .find(|s| s.head_types.len() == 1 && s.candidates.iter().all(|c| c.dividers > 0))
            .unwrap();
        let c = &solution.candidates[0]; // width25, shared with26.
        let d = candidate_dimensions(&study, c).unwrap();
        // g2/3: 26+7.5+2*2 =37.5; g4 pair:26+1=27, fixed middle=60.
        let (spacing, minimum) = if group == 4 {
            (27.0, 26.0)
        } else {
            (37.5, 36.5)
        };
        assert_eq!(d.approach_spacing, Some(spacing));
        assert_eq!(d.minimum_approach_spacing, Some(minimum));
        assert_eq!(d.extra_spacing, Some(1.0));
        assert_eq!(d.gaps.len(), group as usize - 1);
        assert!(d.left_wall_clearance >= 2.0 && d.right_wall_clearance >= 2.0);
        if group == 4 {
            assert_eq!(d.gaps[1].centre_spacing, 60.0);
            assert!(d.gaps[1].structural);
            assert_eq!(d.gaps[1].stick_clearance, 35.0);
            assert_eq!(d.gaps[1].divider_clearances, Some([13.75, 13.75]));
            assert_eq!(d.gaps[0].stick_clearance, 2.0);
        } else {
            assert_eq!(d.gaps[0].centre_spacing, 37.5);
            assert_eq!(d.gaps[0].stick_clearance, 12.5);
            assert_eq!(d.gaps[0].divider_clearances, Some([2.5, 2.5]));
        }
        assert_eq!(c.head_width, 60.0 * f64::from(group));
    }
}
