//! Editable GUI drafts and conversion to a validated core snapshot.

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct StickDraft {
    pub id: u64,
    pub name: String,
    pub length: String,
    pub width: String,
    pub thickness: String,
    pub fin: String,
}

#[derive(Clone, Debug)]
pub struct FormatDraft {
    pub id: u64,
    pub name: String,
    pub stick_id: Option<u64>,
    pub quantity: String,
}

#[derive(Clone, Debug)]
pub struct Parameters {
    pub channels: String,
    pub offset: String,
    pub max_pitch: String,
    pub pitch_step: String,
    pub max_layers: String,
    pub ab_target: String,
    pub result_count: String,
    pub divider_width: String,
    pub wall_width: String,
    pub adjacent_clearance: String,
    pub wall_clearance: String,
    pub carton_b_extra: String,
    pub weights: [String; 8],
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            channels: "12".into(),
            offset: "20".into(),
            max_pitch: "300".into(),
            pitch_step: "5".into(),
            max_layers: "12".into(),
            ab_target: "1,5".into(),
            result_count: "20".into(),
            divider_width: "2".into(),
            wall_width: "3".into(),
            adjacent_clearance: "1".into(),
            wall_clearance: "2".into(),
            carton_b_extra: "5".into(),
            weights: stick_optimizer_core::Weights::default()
                .values()
                .map(|v| v.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StudyDraft {
    pub name: String,
    pub sticks: Vec<StickDraft>,
    pub formats: Vec<FormatDraft>,
    pub parameters: Parameters,
    next_id: u64,
}

impl Default for StudyDraft {
    fn default() -> Self {
        Self {
            name: "Nuovo studio".into(),
            sticks: Vec::new(),
            formats: Vec::new(),
            parameters: Parameters::default(),
            next_id: 1,
        }
    }
}

impl StudyDraft {
    pub fn example() -> Self {
        let mut study = Self {
            name: "Studio di esempio".into(),
            ..Self::default()
        };
        study.add_stick();
        study.add_stick();
        study.sticks[0].name = "Stick A".into();
        study.sticks[1].name = "Stick B".into();
        study.sticks[1].length = "100".into();
        study.sticks[1].width = "26".into();
        study.sticks[1].thickness = "8".into();
        for (name, stick_index, quantity) in [
            ("A — 10 stick", 0, "10"),
            ("A — 34 stick", 0, "34"),
            ("B — 7 stick", 1, "7"),
        ] {
            study.add_format();
            let row = study.formats.last_mut().expect("just inserted");
            row.name = name.into();
            row.stick_id = Some(study.sticks[stick_index].id);
            row.quantity = quantity.into();
        }
        study
    }

    fn allocate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_stick(&mut self) {
        let id = self.allocate_id();
        self.sticks.push(StickDraft {
            id,
            name: format!("Stick {id}"),
            length: "110".into(),
            width: "25".into(),
            thickness: "6".into(),
            fin: "5".into(),
        });
    }

    pub fn add_format(&mut self) {
        let id = self.allocate_id();
        self.formats.push(FormatDraft {
            id,
            name: format!("Formato {id}"),
            stick_id: self.sticks.first().map(|stick| stick.id),
            quantity: "10".into(),
        });
    }

    pub fn remove_stick(&mut self, id: u64) {
        self.sticks.retain(|stick| stick.id != id);
        self.formats.retain(|format| format.stick_id != Some(id));
    }

    pub fn input_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.name.trim().is_empty() {
            errors.push("Inserisci un nome per lo studio.".into());
        }
        if self.sticks.is_empty() {
            errors.push("Aggiungi almeno un tipo di stick.".into());
        }
        if self.formats.is_empty() {
            errors.push("Aggiungi almeno un formato.".into());
        }
        let mut names = HashSet::new();
        for (index, stick) in self.sticks.iter().enumerate() {
            let label = format!("Stick {}", index + 1);
            validate_name(&stick.name, &label, &mut names, &mut errors);
            for (field, value, zero_allowed) in [
                ("lunghezza", &stick.length, false),
                ("larghezza", &stick.width, false),
                ("spessore", &stick.thickness, false),
                ("pinna", &stick.fin, true),
            ] {
                validate_decimal(
                    value,
                    &format!("{label}, {field}"),
                    zero_allowed,
                    &mut errors,
                );
            }
        }
        names.clear();
        for (index, format) in self.formats.iter().enumerate() {
            let label = format!("Formato {}", index + 1);
            validate_name(&format.name, &label, &mut names, &mut errors);
            if !self
                .sticks
                .iter()
                .any(|stick| Some(stick.id) == format.stick_id)
            {
                errors.push(format!("{label}: seleziona uno stick esistente."));
            }
            validate_integer(&format.quantity, &format!("{label}, quantità"), &mut errors);
        }
        let p = &self.parameters;
        for (label, value) in [
            ("Canali MT", &p.channels),
            ("Numero risultati", &p.result_count),
        ] {
            validate_integer(value, label, &mut errors);
        }
        if !p.max_layers.trim().is_empty() {
            validate_integer(&p.max_layers, "Limite strati", &mut errors);
        }
        for (label, value, zero_allowed) in [
            ("Offset massimo", &p.offset, true),
            ("Passo massimo", &p.max_pitch, false),
            ("Incremento passo", &p.pitch_step, false),
            ("Obiettivo A/B", &p.ab_target, false),
            ("Spessore divisore", &p.divider_width, true),
            ("Spessore parete", &p.wall_width, true),
            ("Gioco tra stick", &p.adjacent_clearance, true),
            ("Gioco parete/divisore", &p.wall_clearance, true),
            ("Extra B", &p.carton_b_extra, true),
        ] {
            validate_decimal(value, label, zero_allowed, &mut errors);
        }
        for (index, weight) in p.weights.iter().enumerate() {
            validate_decimal(weight, &format!("Peso {}", index + 1), true, &mut errors);
        }
        if matches!(p.channels.trim().parse::<u32>(), Ok(1)) {
            errors.push("Canali MT: servono almeno 2 canali per definire l'offset.".into());
        }
        errors
    }

    pub fn to_study(&self) -> Result<stick_optimizer_core::Study, Vec<String>> {
        use stick_optimizer_core::{Format, Settings, Stick, Study, Weights};
        let errors = self.input_errors();
        if !errors.is_empty() {
            return Err(errors);
        }
        let p = &self.parameters;
        let decimal = |v: &str| parse_decimal(v).expect("syntax validated");
        let integer = |v: &str| v.trim().parse::<u32>().expect("syntax validated");
        let w = p.weights.each_ref().map(|v| decimal(v));
        let study = Study {
            sticks: self
                .sticks
                .iter()
                .map(|s| Stick {
                    id: s.id,
                    name: s.name.trim().into(),
                    length: decimal(&s.length),
                    width: decimal(&s.width),
                    thickness: decimal(&s.thickness),
                    fin: decimal(&s.fin),
                })
                .collect(),
            formats: self
                .formats
                .iter()
                .map(|f| Format {
                    id: f.id,
                    name: f.name.trim().into(),
                    stick_id: f.stick_id.expect("reference validated"),
                    quantity: integer(&f.quantity),
                })
                .collect(),
            settings: Settings {
                channels: integer(&p.channels),
                max_shift: decimal(&p.offset),
                max_pitch: decimal(&p.max_pitch),
                pitch_step: decimal(&p.pitch_step),
                max_layers: if p.max_layers.trim().is_empty() {
                    None
                } else {
                    Some(integer(&p.max_layers))
                },
                ab_target: decimal(&p.ab_target),
                result_count: integer(&p.result_count) as usize,
                divider_width: decimal(&p.divider_width),
                wall_width: decimal(&p.wall_width),
                adjacent_clearance: decimal(&p.adjacent_clearance),
                wall_clearance: decimal(&p.wall_clearance),
                carton_b_extra: decimal(&p.carton_b_extra),
            },
            weights: Weights {
                pockets: w[0],
                heads: w[1],
                layers: w[2],
                carryover: w[3],
                grouping: w[4],
                stability: w[5],
                pitch: w[6],
                ab: w[7],
            },
        };
        let errors = stick_optimizer_core::validate(&study);
        if errors.is_empty() {
            Ok(study)
        } else {
            Err(errors)
        }
    }
}

pub fn parse_decimal(value: &str) -> Result<f64, &'static str> {
    let number = value
        .trim()
        .replace(',', ".")
        .parse::<f64>()
        .map_err(|_| "inserisci un numero, usando virgola o punto per i decimali")?;
    if !number.is_finite() {
        return Err("il numero deve essere finito");
    }
    Ok(number)
}

fn validate_decimal(value: &str, label: &str, zero_allowed: bool, errors: &mut Vec<String>) {
    match parse_decimal(value) {
        Ok(number) if number > 0.0 || (zero_allowed && number == 0.0) => {}
        _ => errors.push(format!(
            "{label}: serve un numero {}.",
            if zero_allowed {
                "maggiore o uguale a zero"
            } else {
                "maggiore di zero"
            }
        )),
    }
}

fn validate_integer(value: &str, label: &str, errors: &mut Vec<String>) {
    if !matches!(value.trim().parse::<u32>(), Ok(number) if number > 0) {
        errors.push(format!("{label}: serve un numero intero maggiore di zero."));
    }
}

fn validate_name<'a>(
    name: &'a str,
    label: &str,
    names: &mut HashSet<&'a str>,
    errors: &mut Vec<String>,
) {
    if name.trim().is_empty() {
        errors.push(format!("{label}: inserisci un nome."));
    } else if !names.insert(name.trim()) {
        errors.push(format!(
            "{label}: il nome «{}» è già utilizzato.",
            name.trim()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn italian_decimals_and_invalid_numbers() {
        assert_eq!(parse_decimal(" 25,75 "), Ok(25.75));
        assert_eq!(parse_decimal("25.75"), Ok(25.75));
        for value in ["", "NaN", "inf", "1e999", "1,2.3"] {
            assert!(parse_decimal(value).is_err(), "{value}");
        }
    }

    #[test]
    fn editing_and_removing_sticks_preserves_format_links() {
        let mut study = StudyDraft::example();
        assert!(study.input_errors().is_empty());
        let old_id = study.sticks[0].id;
        study.sticks[0].name = "Nome modificato".into();
        assert_eq!(study.formats[0].stick_id, Some(old_id));
        assert!(study.input_errors().is_empty());
        let retained_id = study.sticks[1].id;
        study.remove_stick(old_id);
        assert_eq!(study.formats.len(), 1);
        assert_eq!(study.formats[0].stick_id, Some(retained_id));
        study.add_stick();
        assert_ne!(study.sticks.last().unwrap().id, old_id);
    }

    #[test]
    fn malformed_drafts_cannot_generate_demo_results() {
        let mut study = StudyDraft::example();
        study.sticks[1].name = study.sticks[0].name.clone();
        study.formats[0].stick_id = Some(999);
        study.formats[1].quantity = "1,5".into();
        study.parameters.max_pitch = "-1".into();
        assert_eq!(study.input_errors().len(), 4);
        study.parameters.max_layers.clear();
        assert_eq!(study.input_errors().len(), 4);
    }

    #[test]
    fn snapshot_includes_all_geometry_and_weights_and_owns_its_inputs() {
        let mut draft = StudyDraft::example();
        draft.parameters.wall_width = "4,5".into();
        draft.parameters.weights[7] = "700".into();
        let snapshot = draft.to_study().unwrap();
        assert_eq!(snapshot.settings.wall_width, 4.5);
        assert_eq!(snapshot.weights.ab, 700.0);
        assert_eq!(snapshot.formats[0].stick_id, snapshot.sticks[0].id);
        draft.sticks[0].width = "99".into();
        assert_eq!(snapshot.sticks[0].width, 25.0);
        draft.parameters.pitch_step = "0,0000001".into();
        assert!(draft.to_study().is_err());
    }
}
