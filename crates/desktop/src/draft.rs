//! Editable GUI drafts, deliberately separate from the future calculation core.
//! These checks validate input syntax only, not mechanical feasibility.

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
        ] {
            validate_decimal(value, label, zero_allowed, &mut errors);
        }
        errors
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
}
