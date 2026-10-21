use crate::Error;
use serde::{Deserialize, Serialize};
use stick_optimizer_core::{Format, Settings, Stick, Study, Weights};
macro_rules! mirror {
    ($dto:ident, $core:ident, {$($field:ident: $ty:ty),* $(,)?}) => {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        struct $dto { $($field: $ty),* }
        impl From<$core> for $dto {
            fn from(value: $core) -> Self {Self { $($field: value.$field),* }}
        }
        impl From<$dto> for $core {
            fn from(value: $dto) -> Self {Self { $($field: value.$field),* }}
        }
    }
}
mirror!(StickData, Stick, {id:u64, name:String, length:f64, width:f64, thickness:f64, fin:f64});
mirror!(FormatData, Format, {id:u64, name:String, stick_id:u64, quantity:u32});
mirror!(SettingsData, Settings, {
    channels:u32, max_shift:f64, divider_width:f64, max_extra_spacing:f64, wall_width:f64,
    adjacent_clearance:f64, wall_clearance:f64, carton_b_extra:f64,
    max_pitch:f64, pitch_step:f64, result_count:usize, max_layers:Option<u32>, ab_target:f64
});
mirror!(WeightsData, Weights, {pockets:f64, heads:f64, layers:f64, carryover:f64, grouping:f64, stability:f64, pitch:f64, ab:f64});
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Range {
    pub min: Option<f64>,
    pub max: Option<f64>,
}
impl Range {
    pub fn contains(self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }
    pub fn validate(self) -> Result<(), Error> {
        if [self.min, self.max]
            .into_iter()
            .flatten()
            .any(|v| !v.is_finite() || v < 0.0)
            || matches!((self.min, self.max), (Some(a), Some(b)) if a > b)
        {
            return Err("Filtri: intervallo non valido.".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ViewState {
    /// Pitch, pocket types, head types (excluding grouping 1), layers, score.
    pub ranges: [Range; 5],
    pub one_head_only: bool,
    pub order_by_pitch: bool,
    /// Original 1-based rank, preserved after filtering/sorting.
    pub selected: Option<usize>,
}
#[derive(Clone, Debug)]
pub struct Document {
    pub name: String,
    pub study: Study,
    pub recalculate: bool,
    pub view: ViewState,
    /// Informational; results are always recomputed using the current engine.
    pub engine: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    schema: String,
    version: u32,
    units: String,
    engine: String,
    name: String,
    sticks: Vec<StickData>,
    formats: Vec<FormatData>,
    settings: SettingsData,
    weights: WeightsData,
    recalculate: bool,
    view: ViewState,
}
impl Document {
    pub fn new(name: String, study: Study, recalculate: bool, view: ViewState) -> Self {
        Self {
            name,
            study,
            recalculate,
            view,
            engine: "rust-geometry-v1".into(),
        }
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.name.trim().is_empty() {
            return Err("Inserisci un nome per lo studio.".into());
        }
        let errors = stick_optimizer_core::validate(&self.study);
        if !errors.is_empty() {
            return Err(errors.join("; ").into());
        }
        let mut ids = std::collections::HashSet::new();
        for id in self
            .study
            .sticks
            .iter()
            .map(|s| s.id)
            .chain(self.study.formats.iter().map(|f| f.id))
        {
            if id >= u64::MAX - 1 || !ids.insert(id) {
                return Err("ID duplicati o fuori intervallo.".into());
            }
        }
        for range in self.view.ranges {
            range.validate()?;
        }
        if self
            .view
            .selected
            .is_some_and(|id| id == 0 || id > self.study.settings.result_count)
        {
            return Err("Selezione fuori intervallo.".into());
        }
        Ok(())
    }
}
pub fn encode(document: &Document) -> Result<Vec<u8>, Error> {
    document.validate()?;
    let s = document.study.clone();
    Ok(serde_json::to_vec_pretty(&Envelope {
        schema: "stick-optimizer-study".into(),
        version: 2,
        units: "mm".into(),
        engine: document.engine.clone(),
        name: document.name.clone(),
        sticks: s.sticks.into_iter().map(Into::into).collect(),
        formats: s.formats.into_iter().map(Into::into).collect(),
        settings: s.settings.into(),
        weights: s.weights.into(),
        recalculate: document.recalculate,
        view: document.view.clone(),
    })?)
}
pub fn decode(bytes: &[u8]) -> Result<Document, Error> {
    if bytes.len() > crate::MAX_FILE_BYTES {
        return Err("Il file supera il limite di 8 MiB.".into());
    }
    let mut value: serde_json::Value = serde_json::from_slice(bytes)?;
    upgrade_settings(&mut value)?;
    let e: Envelope = serde_json::from_value(value)?;
    if e.schema != "stick-optimizer-study" || e.version != 2 || e.units != "mm" {
        return Err("Formato, versione o unità dello studio non supportati.".into());
    }
    let document = Document {
        name: e.name,
        study: Study {
            sticks: e.sticks.into_iter().map(Into::into).collect(),
            formats: e.formats.into_iter().map(Into::into).collect(),
            settings: e.settings.into(),
            weights: e.weights.into(),
        },
        recalculate: e.recalculate,
        view: e.view,
        engine: e.engine,
    };
    document.validate()?;
    Ok(document)
}
#[derive(Clone, Serialize)]
pub struct Preferences {
    version: u32,
    settings: SettingsData,
    weights: WeightsData,
}
impl Preferences {
    pub fn new(settings: Settings, weights: Weights) -> Self {
        Self {
            version: 2,
            settings: settings.into(),
            weights: weights.into(),
        }
    }
    pub fn parameters(&self) -> (Settings, Weights) {
        (self.settings.clone().into(), self.weights.clone().into())
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.version != 2 {
            return Err("Versione delle preferenze non supportata.".into());
        }
        let (settings, weights) = self.parameters();
        let study = Study {
            sticks: vec![Stick {
                id: 1,
                name: "Validation".into(),
                length: 1.0,
                width: 1.0,
                thickness: 1.0,
                fin: 0.0,
            }],
            formats: vec![Format {
                id: 2,
                name: "Validation".into(),
                stick_id: 1,
                quantity: 1,
            }],
            settings,
            weights,
        };
        let errors = stick_optimizer_core::validate(&study);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; ").into())
        }
    }
}

// Upgrade only the documented missing v1 parameter. Never replace stored values
// or write a source file. Unknown fields remain rejected by the strict DTOs.
fn upgrade_settings(value: &mut serde_json::Value) -> Result<(), Error> {
    match value.get("version").and_then(serde_json::Value::as_u64) {
        Some(2) => Ok(()),
        Some(1) => {
            let settings = value
                .get_mut("settings")
                .and_then(serde_json::Value::as_object_mut)
                .ok_or("Impostazioni mancanti o non valide nello schema 1.")?;
            if settings.contains_key("max_extra_spacing") {
                return Err("Schema 1 non valido: interasse aggiuntivo inatteso.".into());
            }
            settings.insert("max_extra_spacing".into(), serde_json::json!(5.0));
            value["version"] = serde_json::json!(2);
            Ok(())
        }
        _ => Err("Versione del file non supportata; dati conservati.".into()),
    }
}

impl<'de> Deserialize<'de> for Preferences {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            version: u32,
            settings: SettingsData,
            weights: WeightsData,
        }
        let mut value = serde_json::Value::deserialize(deserializer)?;
        upgrade_settings(&mut value).map_err(serde::de::Error::custom)?;
        let wire: Wire = serde_json::from_value(value).map_err(serde::de::Error::custom)?;
        Ok(Self {
            version: wire.version,
            settings: wire.settings,
            weights: wire.weights,
        })
    }
}
