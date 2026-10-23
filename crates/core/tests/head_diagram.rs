use stick_optimizer_core::*;

#[test]
fn diagram_quotations_use_actual_centres_and_preserve_head_width_semantics() {
    for (grouping, expected, span) in [
        (2, vec![0.0, 27.0], 627.0),
        (3, vec![33.0, 60.0, 87.0], 594.0),
        (4, vec![33.0, 60.0, 120.0, 147.0], 594.0),
    ] {
        let head = final_head_geometry(
            &Settings::default(),
            &[HeadFormat {
                format_id: 1,
                stick_width: 26.0,
                pickup_pitch: Pitch(60_000_000),
                grouping,
                dividers: 0,
            }],
        )
        .unwrap();
        let quotes = head_diagram_dimensions(&head);
        assert_eq!(quotes.pickup_span, 660.0);
        assert_eq!(quotes.final_span, span);
        assert_eq!(quotes.group_pitch, Some(f64::from(grouping) * 60.0));
        assert_eq!(
            quotes.final_spacings,
            if grouping == 4 {
                vec![27.0, 60.0, 27.0]
            } else {
                vec![27.0; grouping as usize - 1]
            }
        );
        assert_eq!(
            head.positions
                .iter()
                .take(grouping as usize)
                .map(|p| p.final_x)
                .collect::<Vec<_>>(),
            expected
        );
        assert_ne!(quotes.pickup_span, head.pickup_pitch.mm() * 12.0);
    }
}
