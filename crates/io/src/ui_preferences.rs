//! Local presentation preferences, deliberately independent of study parameters.
use crate::{Error, read_bounded, write_atomic};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiPreferences {
    pub theme: Theme,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    schema: String,
    version: u32,
    theme: Theme,
}
const SCHEMA: &str = "stick-optimizer-ui-preferences";

/// Missing file uses light without writing. Invalid data remains an explicit error.
pub fn load_ui_preferences(path: &Path) -> Result<UiPreferences, Error> {
    let bytes = match read_bounded(path) {
        Ok(bytes) => bytes,
        Err(error)
            if error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
        {
            return Ok(UiPreferences::default());
        }
        Err(error) => return Err(error),
    };
    let file: File = serde_json::from_slice(&bytes)?;
    if file.schema != SCHEMA || file.version != 1 {
        return Err("Preferenze UI: schema o versione non supportati.".into());
    }
    Ok(UiPreferences { theme: file.theme })
}

pub fn save_ui_preferences(path: &Path, preferences: &UiPreferences) -> Result<(), Error> {
    let bytes = serde_json::to_vec_pretty(&File {
        schema: SCHEMA.into(),
        version: 1,
        theme: preferences.theme,
    })?;
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    write_atomic(path, &bytes)
}
