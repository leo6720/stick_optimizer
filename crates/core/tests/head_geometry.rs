use stick_optimizer_core::*;

fn assigned(id: u64, width: f64, grouping: u32, dividers: u32) -> HeadFormat {
    HeadFormat {
        format_id: id,
        stick_width: width,
        grouping,
        dividers,
        pickup_pitch: Pitch(60_000_000),
    }
}

#[test]
fn approved_common_spacing_and_fixed_channels_for_every_group() {
    for (grouping, dividers, expected, first_group) in [
        (2, 0, 27.0, vec![0.0, 27.0]),
        (2, 1, 37.5, vec![0.0, 37.5]),
        (3, 0, 27.0, vec![33.0, 60.0, 87.0]),
        (3, 2, 37.5, vec![22.5, 60.0, 97.5]),
        (4, 0, 27.0, vec![33.0, 60.0, 120.0, 147.0]),
        (4, 1, 27.0, vec![33.0, 60.0, 120.0, 147.0]),
    ] {
        let head = final_head_geometry(
            &Settings::default(),
            &[
                assigned(2, 26.0, grouping, dividers),
                assigned(1, 25.0, grouping, dividers),
            ],
        )
        .unwrap();
        assert_eq!(head.approach_spacing, expected);
        assert_eq!(head.format_ids, [1, 2]);
        assert_eq!(head.positions.len(), 12);
        for (index, point) in head.positions.iter().enumerate() {
            let group = index as u32 / grouping;
            assert_eq!(point.channel, index as u32);
            assert_eq!(point.group, group);
            assert_eq!(point.pickup_x, index as f64 * 60.0);
            assert_eq!(
                point.final_x,
                first_group[index % grouping as usize] + f64::from(group * grouping) * 60.0
            );
        }
    }
}

#[test]
fn all_assigned_formats_must_share_one_configuration() {
    for dividers in [0, 1] {
        let settings = Settings::default();
        let a = assigned(1, 25.0, 2, dividers);
        let b = assigned(2, 30.0, 2, dividers);
        let c = assigned(3, 35.0, 2, dividers);
        assert!(final_head_geometry(&settings, &[a.clone(), b.clone()]).is_ok());
        assert!(final_head_geometry(&settings, &[b.clone(), c.clone()]).is_ok());
        assert!(matches!(
            final_head_geometry(&settings, &[a.clone(), b, c]),
            Err(HeadGeometryError::Incompatible(_))
        ));
        let same =
            final_head_geometry(&settings, &[a.clone(), assigned(4, 25.0, 2, dividers)]).unwrap();
        assert_eq!(
            same.approach_spacing,
            if dividers == 0 { 26.0 } else { 36.5 }
        );
        assert!(
            final_head_geometry(&settings, &[a.clone(), assigned(5, 30.000001, 2, dividers)])
                .is_err()
        );
        assert!(final_head_geometry(&settings, &[a.clone(), assigned(5, 26.0, 3, 0)]).is_err());
        let mut different_pitch = a.clone();
        different_pitch.format_id = 6;
        different_pitch.pickup_pitch.0 += 1;
        assert!(final_head_geometry(&settings, &[a, different_pitch]).is_err());
    }
}

#[test]
fn order_invariance_mixed_dividers_and_exact_non_grid_lengths() {
    let settings = Settings {
        max_extra_spacing: 12.0,
        ..Settings::default()
    };
    let a = assigned(1, 25.123456789, 2, 0);
    let b = assigned(2, 25.123456789, 2, 1);
    let forward = final_head_geometry(&settings, &[a.clone(), b.clone()]).unwrap();
    assert_eq!(forward, final_head_geometry(&settings, &[b, a]).unwrap());
    assert_eq!(forward.approach_spacing, 36.623456789);
}

#[test]
fn central_fixed_gap_is_exempt_but_must_fit_divider_and_clearances() {
    let settings = Settings {
        max_extra_spacing: 0.0,
        ..Settings::default()
    };
    let mut format = assigned(1, 25.0, 4, 1);
    let head = final_head_geometry(&settings, &[format.clone()]).unwrap();
    assert_eq!(head.approach_spacing, 26.0);
    assert_eq!(head.positions[2].final_x - head.positions[1].final_x, 60.0);
    format.pickup_pitch = Pitch(36_500_000);
    assert!(final_head_geometry(&settings, &[format.clone()]).is_ok());
    format.pickup_pitch = Pitch(36_499_999);
    assert!(final_head_geometry(&settings, &[format]).is_err());
}

#[test]
fn invalid_and_outward_geometries_are_rejected() {
    let settings = Settings::default();
    assert!(final_head_geometry(&settings, &[]).is_err());
    let good = assigned(1, 25.0, 2, 0);
    assert!(final_head_geometry(&settings, &[good.clone(), good.clone()]).is_err());
    for width in [0.0, -1.0, f64::NAN, f64::INFINITY, 61.0] {
        let mut format = good.clone();
        format.stick_width = width;
        assert!(final_head_geometry(&settings, &[format]).is_err());
    }
    for (grouping, dividers) in [(1, 0), (5, 0), (2, 2), (3, 1), (4, 2)] {
        assert!(final_head_geometry(&settings, &[assigned(1, 25.0, grouping, dividers)]).is_err());
    }
    for pitch in [0, -1, 1_i64 << 53, (1_i64 << 53) + 1] {
        let mut format = good.clone();
        format.pickup_pitch = Pitch(pitch);
        assert!(final_head_geometry(&settings, &[format]).is_err());
    }
    assert!(
        final_head_geometry(
            &Settings {
                channels: 11,
                ..settings.clone()
            },
            std::slice::from_ref(&good)
        )
        .is_err()
    );
    for limit in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(
            final_head_geometry(
                &Settings {
                    max_extra_spacing: limit,
                    ..settings.clone()
                },
                std::slice::from_ref(&good)
            )
            .is_err()
        );
    }
}

#[test]
fn joint_three_format_limits_apply_to_every_moving_gap() {
    for (grouping, dividers, expected) in [
        (2, 0, 31.0),
        (2, 1, 41.5),
        (3, 0, 31.0),
        (3, 2, 41.5),
        (4, 0, 31.0),
        (4, 1, 31.0),
    ] {
        let formats = [
            assigned(3, 30.0, grouping, dividers),
            assigned(1, 25.0, grouping, dividers),
            assigned(2, 26.0, grouping, dividers),
        ];
        assert_eq!(
            final_head_geometry(&Settings::default(), &formats)
                .unwrap()
                .approach_spacing,
            expected
        );
        let mut over = formats.clone();
        over[0].stick_width += 0.000001;
        assert!(final_head_geometry(&Settings::default(), &over).is_err());
    }
}

#[test]
fn settings_validate_default_and_nonnegative_extra_spacing() {
    assert_eq!(Settings::default().max_extra_spacing, 5.0);
    assert_eq!(Settings::default().divider_width, 7.5);
    let mut study = Study {
        sticks: vec![Stick {
            id: 1,
            name: "A".into(),
            length: 100.0,
            width: 25.0,
            thickness: 6.0,
            fin: 5.0,
        }],
        formats: vec![Format {
            id: 2,
            name: "A10".into(),
            stick_id: 1,
            quantity: 10,
        }],
        settings: Settings::default(),
        weights: Weights::default(),
    };
    for limit in [0.0, 5.0, 8.125] {
        study.settings.max_extra_spacing = limit;
        assert!(validate(&study).is_empty());
    }
    for limit in [-0.1, f64::NAN, f64::INFINITY] {
        study.settings.max_extra_spacing = limit;
        assert!(
            validate(&study)
                .iter()
                .any(|error| error.contains("Interasse aggiuntivo massimo"))
        );
    }
}
