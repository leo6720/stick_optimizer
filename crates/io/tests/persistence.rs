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
        ("/version", serde_json::json!(2)),
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
    value["version"] = serde_json::json!(2);
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
    let summary = records(&summary_csv([(7, s)]));
    assert_eq!(summary.len(), 2);
    assert_eq!(summary[0].len(), 11);
    assert_eq!(summary[1].len(), 11);
    assert_eq!(summary[1][0], "7");
    assert_eq!(summary[1][1].parse::<f64>().unwrap(), s.score);
    assert_eq!(summary[1][4].parse::<usize>().unwrap(), s.head_types.len());
    let detail = records(&details_csv(s));
    assert_eq!(detail.len(), 2);
    assert_eq!(detail[0].len(), 28);
    assert_eq!(detail[1].len(), 28);
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
