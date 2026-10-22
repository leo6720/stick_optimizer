use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use stick_optimizer_core::{
    Format, SearchControl, SearchLimits, Settings, Stick, Study, Weights, optimize,
};
use stick_optimizer_io::*;
static ID: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "stick-io-test-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}
fn example() -> Document {
    Document::new(
        "Studio, prova \"β\"".into(),
        Study {
            sticks: vec![Stick {
                id: 1,
                name: "Stick, \"α\"\nLinea 2".into(),
                length: 110.0,
                width: 25.0,
                thickness: 6.0,
                fin: 5.0,
            }],
            formats: vec![Format {
                id: 2,
                name: "A — 10".into(),
                stick_id: 1,
                quantity: 10,
            }],
            settings: Settings::default(),
            weights: Weights::default(),
        },
        true,
        ViewState {
            selected: Some(1),
            ..ViewState::default()
        },
    )
}
#[test]
fn study_round_trip_reproduces_real_results_and_full_precision() {
    let temp = Temp::new();
    let path = temp.0.join("studio.stic");
    let mut original = example();
    original.study.settings.ab_target = 1.2345678901234567;
    original.view.ranges[0] = Range {
        min: Some(50.0),
        max: Some(300.0),
    };
    let report = optimize(
        &original.study,
        &SearchControl::default(),
        SearchLimits::default(),
    )
    .unwrap();
    assert!(!report.solutions.is_empty());
    save(&path, &original).unwrap();
    let reopened = load(&path).unwrap();
    assert_eq!(reopened.name, original.name);
    assert_eq!(reopened.view, original.view);
    assert!(reopened.recalculate);
    assert_eq!(
        reopened.study.settings.ab_target.to_bits(),
        original.study.settings.ab_target.to_bits()
    );
    assert_eq!(encode(&reopened).unwrap(), encode(&original).unwrap());
    let after = optimize(
        &reopened.study,
        &SearchControl::default(),
        SearchLimits::default(),
    )
    .unwrap();
    assert_eq!(after.solutions, report.solutions);
}
#[test]
fn malformed_versions_references_ranges_units_and_numbers_are_rejected() {
    let encoded = encode(&example()).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    for (pointer, replacement) in [
        ("/version", serde_json::json!(3)),
        ("/units", serde_json::json!("in")),
        ("/formats/0/stick_id", serde_json::json!(999)),
        ("/sticks/0/id", serde_json::json!(u64::MAX)),
        ("/formats/0/id", serde_json::json!(1)),
        ("/weights/layers", serde_json::json!(-1)),
        ("/settings/pitch_step", serde_json::json!(0)),
        ("/settings/max_pitch", serde_json::json!(null)),
        ("/view/selected", serde_json::json!(0)),
        ("/view/ranges/0", serde_json::json!({"min":300,"max":50})),
    ] {
        let mut v = value.clone();
        *v.pointer_mut(pointer).unwrap() = replacement;
        assert!(
            decode(&serde_json::to_vec(&v).unwrap()).is_err(),
            "{pointer}"
        );
    }
    for bytes in [
        &encoded[..encoded.len() / 2],
        b"{\"version\":NaN}".as_slice(),
        b"{}".as_slice(),
    ] {
        assert!(decode(bytes).is_err());
    }
    let mut v = value;
    v["unexpected"] = serde_json::json!(true);
    assert!(decode(&serde_json::to_vec(&v).unwrap()).is_err());
    assert!(decode(&vec![b' '; MAX_FILE_BYTES + 1]).is_err());
    let mut bad = example();
    bad.study.weights.ab = f64::NAN;
    assert!(encode(&bad).is_err());
}
#[test]
fn atomic_overwrite_and_failed_writes_preserve_existing_data() {
    let temp = Temp::new();
    let path = temp.0.join("studio.stic");
    let mut doc = example();
    save(&path, &doc).unwrap();
    doc.name = "Seconda versione".into();
    save(&path, &doc).unwrap();
    assert_eq!(load(&path).unwrap().name, doc.name);
    let bytes = std::fs::read(&path).unwrap();
    doc.study.formats[0].stick_id = 999;
    assert!(save(&path, &doc).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    assert!(write_atomic(&temp.0, b"cannot replace directory").is_err());
    assert!(save(&temp.0.join("missing/studio.stic"), &example()).is_err());
    assert_eq!(std::fs::read_dir(&temp.0).unwrap().count(), 1);
    assert!(write_atomic(&path, &vec![0; MAX_FILE_BYTES + 1]).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
}
#[test]
fn preferences_are_versioned_validated_and_independent_of_user_data() {
    let temp = Temp::new();
    let path = temp.0.join("preferences.json");
    let settings = Settings {
        max_layers: None,
        channels: 18,
        ..Settings::default()
    };
    let weights = Weights {
        ab: 777.123456789,
        ..Weights::default()
    };
    let prefs = Preferences::new(settings, weights);
    save_preferences(&path, &prefs).unwrap();
    let (s, w) = load_preferences(&path).unwrap().parameters();
    assert_eq!(s.channels, 18);
    assert_eq!(s.max_layers, None);
    assert_eq!(w.ab, 777.123456789);
    let original = std::fs::read(&path).unwrap();
    let invalid = Preferences::new(
        Settings {
            channels: 1,
            ..Settings::default()
        },
        Weights::default(),
    );
    assert!(save_preferences(&path, &invalid).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), original);
    let mut value: serde_json::Value = serde_json::from_slice(&original).unwrap();
    value["version"] = serde_json::json!(3);
    std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(load_preferences(&path).is_err());
}
/// Independent parser used only for checking RFC-style quoting and multiline fields.
fn records(bytes: &[u8]) -> Vec<Vec<String>> {
    assert_eq!(&bytes[..3], &[0xef, 0xbb, 0xbf]);
    let text = std::str::from_utf8(&bytes[3..]).unwrap();
    let mut records = vec![];
    let mut row = vec![];
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => row.push(std::mem::take(&mut field)),
            '\r' if !quoted => {
                assert_eq!(chars.next(), Some('\n'));
                row.push(std::mem::take(&mut field));
                records.push(std::mem::take(&mut row));
            }
            _ => field.push(ch),
        }
    }
    assert!(!quoted && field.is_empty() && row.is_empty());
    records
}
#[test]
fn csv_round_trip_checks_columns_numbers_quoting_and_stable_rank() {
    let doc = example();
    let report = optimize(
        &doc.study,
        &SearchControl::default(),
        SearchLimits::default(),
    )
    .unwrap();
    assert!(!report.solutions.is_empty());
    let s = &report.solutions[0];
    let summary = records(&summary_csv(&doc.study, [(7, s)]).unwrap());
    assert_eq!(summary.len(), 2);
    assert!(summary[0].len() > 11);
    assert_eq!(summary[1].len(), summary[0].len());
    assert_eq!(summary[1][0], "7");
    assert_eq!(summary[1][1].parse::<f64>().unwrap(), s.score);
    assert_eq!(summary[1][4].parse::<usize>().unwrap(), s.head_types.len());
    let detail = records(&details_csv(&doc.study, 7, s).unwrap());
    assert_eq!(detail.len(), 2);
    assert!(detail[0].len() > 28);
    assert_eq!(detail[1].len(), detail[0].len());
    assert_eq!(detail[1][1], doc.study.sticks[0].name);
    assert_eq!(detail[1][9].parse::<f64>().unwrap(), s.pitch.mm());
    for (column, value) in [
        (7, s.candidates[0].pocket_length),
        (14, s.candidates[0].carton_a),
        (15, s.candidates[0].carton_b),
        (27, s.candidates[0].penalties[4]),
    ] {
        assert_eq!(detail[1][column].parse::<f64>().unwrap(), value);
    }
}
#[test]
fn filter_intervals_are_inclusive_and_zero_heads_are_preserved() {
    let range = Range {
        min: Some(0.0),
        max: Some(1.0),
    };
    assert!(range.contains(0.0));
    assert!(range.contains(1.0));
    assert!(!range.contains(2.0));
    assert!(
        Range {
            min: Some(f64::INFINITY),
            max: None
        }
        .validate()
        .is_err()
    );
}

#[test]
fn legacy_study_and_preferences_migrate_in_memory_without_rewriting_files() {
    let temp = Temp::new();
    let mut legacy: serde_json::Value =
        serde_json::from_slice(&encode(&example()).unwrap()).unwrap();
    legacy["version"] = serde_json::json!(1);
    legacy["settings"]
        .as_object_mut()
        .unwrap()
        .remove("max_extra_spacing");
    legacy["settings"]["divider_width"] = serde_json::json!(2.125);
    let path = temp.0.join("legacy.stic");
    let bytes = serde_json::to_vec(&legacy).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let loaded = load(&path).unwrap();
    assert_eq!(loaded.study.settings.divider_width, 2.125);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let current: serde_json::Value = serde_json::from_slice(&encode(&loaded).unwrap()).unwrap();
    assert_eq!(current["version"], 2);
    assert_eq!(current["settings"]["max_extra_spacing"], 5.0);
    let mut expected = legacy.clone();
    expected["version"] = serde_json::json!(2);
    expected["settings"]["max_extra_spacing"] = serde_json::json!(5.0);
    assert_eq!(current, expected);
    let prefs_path = temp.0.join("preferences-v1.json");
    let prefs =
        serde_json::json!({"version":1,"settings":legacy["settings"],"weights":legacy["weights"]});
    let prefs_bytes = serde_json::to_vec(&prefs).unwrap();
    std::fs::write(&prefs_path, &prefs_bytes).unwrap();
    let preferences = load_preferences(&prefs_path).unwrap();
    assert_eq!(preferences.parameters().0.divider_width, 2.125);
    assert_eq!(std::fs::read(&prefs_path).unwrap(), prefs_bytes);
    let saved = serde_json::to_value(preferences).unwrap();
    assert_eq!(saved["version"], 2);
    assert_eq!(saved["settings"]["max_extra_spacing"], 5.0);
}

#[test]
fn current_schema_requires_parameter_and_preserves_custom_values() {
    let temp = Temp::new();
    let mut doc = example();
    doc.study.settings.max_extra_spacing = 3.123456789012345;
    doc.study.settings.divider_width = 9.125;
    let bytes = encode(&doc).unwrap();
    let loaded = decode(&bytes).unwrap();
    assert_eq!(
        loaded.study.settings.max_extra_spacing.to_bits(),
        doc.study.settings.max_extra_spacing.to_bits()
    );
    assert_eq!(loaded.study.settings.divider_width, 9.125);
    let prefs = Preferences::new(loaded.study.settings, loaded.study.weights);
    let path = temp.0.join("preferences.json");
    save_preferences(&path, &prefs).unwrap();
    assert_eq!(
        load_preferences(&path)
            .unwrap()
            .parameters()
            .0
            .max_extra_spacing,
        doc.study.settings.max_extra_spacing
    );
    let current: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for version in [1, 2, 3] {
        let mut value = current.clone();
        value["version"] = serde_json::json!(version);
        if version == 2 {
            value["settings"]
                .as_object_mut()
                .unwrap()
                .remove("max_extra_spacing");
        }
        // v1 cannot contain the new field; v2 cannot omit it; v3 is unknown.
        assert!(decode(&serde_json::to_vec(&value).unwrap()).is_err());
        let prefs = serde_json::json!({"version":version,"settings":value["settings"],"weights":value["weights"]});
        std::fs::write(&path, serde_json::to_vec(&prefs).unwrap()).unwrap();
        let before = std::fs::read(&path).unwrap();
        assert!(load_preferences(&path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
}

#[test]
fn shared_and_separate_head_assignments_reconstruct_after_reopening() {
    let mut doc = example();
    doc.study.settings = Settings {
        channels: 2,
        max_shift: 0.0,
        max_layers: Some(1),
        max_pitch: 120.0,
        result_count: 10000,
        ..Settings::default()
    };
    doc.study.formats[0].quantity = 2;
    let mut stick = doc.study.sticks[0].clone();
    stick.id = 3;
    stick.name = "Altro".into();
    stick.width = 26.0;
    stick.fin = 4.0;
    doc.study.sticks.push(stick);
    doc.study.formats.push(Format {
        id: 4,
        name: "Altro formato".into(),
        stick_id: 3,
        quantity: 2,
    });
    assert_eq!(doc.engine, ENGINE_ID);
    let expected = optimize(
        &doc.study,
        &SearchControl::default(),
        SearchLimits::default(),
    )
    .unwrap();
    assert!(expected.solutions.iter().any(|s| s.head_types.len() == 1));
    assert!(expected.solutions.iter().any(|s| s.head_types.len() == 2));
    doc.engine = "rust-geometry-v1".into();
    let bytes = encode(&doc).unwrap();
    let reopened = decode(&bytes).unwrap();
    assert_eq!(reopened.engine, "rust-geometry-v1"); // original contract never mislabelled.
    assert!(reopened.recalculate);
    assert_eq!(
        optimize(
            &reopened.study,
            &SearchControl::default(),
            SearchLimits::default()
        )
        .unwrap()
        .solutions,
        expected.solutions
    );
    assert_eq!(
        Document::new(reopened.name, reopened.study, true, reopened.view).engine,
        ENGINE_ID
    );
}

#[test]
fn chosen_configuration_csv_and_public_file_roundtrip_agree_for_mixed_width_heads() {
    let temp = Temp::new();
    for group in [2, 3, 4] {
        let mut doc = example();
        doc.study.settings = Settings {
            channels: group,
            max_shift: 0.0,
            max_layers: Some(1),
            max_pitch: 60.0 * f64::from(group),
            result_count: 10000,
            ab_target: match group {
                2 => 68.0 / 11.0,
                3 => 105.0 / 11.0,
                _ => 144.0 / 11.0,
            },
            ..Settings::default()
        };
        doc.study.formats[0].quantity = group;
        doc.study.sticks.push(Stick {
            id: 3,
            name: "Altro β".into(),
            width: 26.0,
            fin: 4.0,
            ..doc.study.sticks[0].clone()
        });
        doc.study.formats.push(Format {
            id: 4,
            name: "F β".into(),
            stick_id: 3,
            quantity: group,
        });
        let report = optimize(
            &doc.study,
            &SearchControl::default(),
            SearchLimits::default(),
        )
        .unwrap();
        for shared in [true, false] {
            let mut roundtrip_study = doc.study.clone();
            if !shared {
                roundtrip_study.settings.ab_target = match group {
                    2 => 55.0 / 11.0,
                    3 => 81.0 / 11.0,
                    _ => 141.0 / 11.0,
                };
            }
            let alternatives = optimize(
                &roundtrip_study,
                &SearchControl::default(),
                SearchLimits::default(),
            )
            .unwrap();
            let (index, chosen) = alternatives
                .solutions
                .iter()
                .enumerate()
                .find(|(_, s)| s.head_types.len() == if shared { 1 } else { 2 })
                .expect("both alternatives remain distinguishable");
            let mut doc = Document::new(
                doc.name.clone(),
                roundtrip_study,
                true,
                ViewState::default(),
            );
            doc.view.selected = Some(index + 1);
            doc.view.order_by_pitch = true;
            let before = details_csv(&doc.study, index + 1, chosen).unwrap();
            let rows = records(&before);
            let cell = |row: usize, name: &str| {
                rows[row][rows[0].iter().position(|h| h == name).unwrap()].as_str()
            };
            assert_eq!(cell(1, "rank"), (index + 1).to_string());
            assert_eq!(cell(1, "format_id"), "2");
            assert_eq!(cell(2, "format_id"), "4"); // ID remains explicit alongside quoted names.
            assert_eq!(cell(1, "head_shared"), shared.to_string());
            assert_eq!(cell(1, "head_id") == cell(2, "head_id"), shared);
            assert_eq!(cell(1, "max_extra_spacing_mm"), "5");
            assert_eq!(cell(1, "divider_width_mm"), "7.5");
            assert_eq!(cell(1, "head_width_mm"), (60 * group).to_string());
            let summary = records(&summary_csv(&doc.study, [(index + 1, chosen)]).unwrap());
            let config: serde_json::Value =
                serde_json::from_str(summary[1].last().unwrap()).unwrap();
            assert_eq!(
                config["heads"].as_array().unwrap().len(),
                if shared { 1 } else { 2 }
            );
            let path = temp.0.join(format!("g{group}-{shared}.stic"));
            save(&path, &doc).unwrap();
            let source = std::fs::read(&path).unwrap();
            let reopened = load(&path).unwrap();
            assert_eq!(std::fs::read(&path).unwrap(), source);
            assert_eq!(reopened.view, doc.view);
            let after = optimize(
                &reopened.study,
                &SearchControl::default(),
                SearchLimits::default(),
            )
            .unwrap();
            let restored = &after.solutions[reopened.view.selected.unwrap() - 1];
            assert_eq!(restored, chosen);
            assert_eq!(
                details_csv(&reopened.study, index + 1, restored).unwrap(),
                before
            );
        }
        // Independent approved physical quotations for shared divided heads.
        let (index, chosen) = report
            .solutions
            .iter()
            .enumerate()
            .find(|(_, s)| s.head_types.len() == 1 && s.candidates.iter().all(|c| c.dividers > 0))
            .unwrap();
        let rows = records(&details_csv(&doc.study, index + 1, chosen).unwrap());
        let cell = |name: &str| rows[1][rows[0].iter().position(|h| h == name).unwrap()].as_str();
        assert_eq!(cell("head_format_ids"), "[2,4]");
        assert_eq!(cell("extra_spacing_mm"), "1");
        assert_eq!(
            cell("approach_spacing_mm"),
            if group == 4 { "27" } else { "37.5" }
        );
        if group == 4 {
            assert_eq!(cell("pocket_width_mm"), "143");
            assert_eq!(cell("pocket_stick_centres_mm"), "[14.5,41.5,101.5,128.5]");
            assert_eq!(cell("gap_centre_spacings_mm"), "[27.0,60.0,27.0]");
            assert_eq!(cell("gap_structural"), "[false,true,false]");
            assert_eq!(
                cell("gap_divider_clearances_mm"),
                "[null,[13.75,13.75],null]"
            );
        } else if group == 2 {
            assert_eq!(cell("pocket_width_mm"), "67");
            assert_eq!(cell("pocket_stick_centres_mm"), "[14.75,52.25]");
            assert_eq!(cell("divider_lefts_mm"), "[29.75]");
            assert_eq!(cell("gap_divider_clearances_mm"), "[[2.5,2.5]]");
        }
    }
}
