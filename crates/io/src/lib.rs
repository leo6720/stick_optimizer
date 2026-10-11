//! Versioned studies, preferences and CSV adapters. The core has no serde dependency.
mod csv;
mod schema;
pub use csv::{details_csv, summary_csv};
pub use schema::{Document, Preferences, Range, ViewState, decode, encode};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub const MAX_FILE_BYTES: usize = 8 * 1024 * 1024;
static TEMP_ID: AtomicU64 = AtomicU64::new(0);
pub fn load(path: &Path) -> Result<Document, Error> {
    decode(&read_bounded(path)?)
}
pub fn save(path: &Path, document: &Document) -> Result<(), Error> {
    write_atomic(path, &encode(document)?)
}
pub fn load_preferences(path: &Path) -> Result<Preferences, Error> {
    let preferences: Preferences = serde_json::from_slice(&read_bounded(path)?)?;
    preferences.validate()?;
    Ok(preferences)
}
pub fn save_preferences(path: &Path, preferences: &Preferences) -> Result<(), Error> {
    preferences.validate()?;
    write_atomic(path, &serde_json::to_vec_pretty(preferences)?)
}
fn read_bounded(path: &Path) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take((MAX_FILE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err("Il file supera il limite di 8 MiB.".into());
    }
    Ok(bytes)
}
/// A failed sibling-file replacement leaves the previous file intact.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    if bytes.len() > MAX_FILE_BYTES {
        return Err("Il file supera il limite di 8 MiB.".into());
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let temp = parent.join(format!(
        ".stick-optimizer-{}-{}.tmp",
        std::process::id(),
        TEMP_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    let result = (|| -> Result<(), Error> {
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}
