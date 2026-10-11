use super::*;
use std::path::{Path, PathBuf};
use stick_optimizer_io::{self as io, Document, Preferences};

#[derive(Clone, Copy)]
pub(super) enum FileAction {
    Open,
    Save,
    SaveAs,
    Summary,
    Details,
    Preferences,
}

fn preferences_path() -> Result<PathBuf, io::Error> {
    let base =
        std::env::var_os("LOCALAPPDATA").ok_or("Cartella delle preferenze non disponibile.")?;
    Ok(PathBuf::from(base)
        .join("StickOptimizer")
        .join("preferences-v1.json"))
}
impl PrototypeApp {
    pub(super) fn view_state(&self) -> ViewState {
        let selected = self
            .selected
            .filter(|id| self.visible_results().iter().any(|r| r.id == *id));
        ViewState {
            ranges: self.ranges,
            one_head_only: self.one_head_only,
            order_by_pitch: self.order_by_pitch,
            selected,
        }
    }
    pub(super) fn apply_view(&mut self, view: ViewState) {
        self.ranges = view.ranges;
        self.one_head_only = view.one_head_only;
        self.order_by_pitch = view.order_by_pitch;
        let rows = self.visible_results();
        self.selected = view
            .selected
            .filter(|id| rows.iter().any(|r| r.id == *id))
            .or_else(|| rows.first().map(|r| r.id));
    }
    pub(super) fn read_defaults(&mut self) {
        let result = (|| -> Result<(), io::Error> {
            let path = preferences_path()?;
            match io::load_preferences(&path) {
                Ok(p) => {
                    let (s, w) = p.parameters();
                    let parameters = crate::draft::Parameters::from_core(s, w);
                    self.study.parameters = parameters.clone();
                    self.defaults = Some(parameters);
                    Ok(())
                }
                Err(e)
                    if e.downcast_ref::<std::io::Error>()
                        .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
                {
                    Ok(())
                }
                Err(e) => Err(e),
            }
        })();
        if let Err(e) = result {
            self.diagnostic = Some(format!("Preferenze non caricate: {e}"));
        }
    }
    pub(super) fn document(&self) -> Result<Document, io::Error> {
        let study = self.study.to_study().map_err(|errors| errors.join("; "))?;
        Ok(Document::new(
            self.study.name.clone(),
            study,
            !self.results.is_empty(),
            self.view_state(),
        ))
    }
    pub(super) fn save_to(&mut self, path: &Path) -> Result<(), io::Error> {
        io::save(path, &self.document()?)?;
        self.current_path = Some(path.to_owned());
        self.dirty = false;
        self.status = format!("Studio salvato: {}", path.display());
        self.diagnostic = None;
        Ok(())
    }
    pub(super) fn open_from(&mut self, path: &Path) -> Result<(), io::Error> {
        if self.job.is_some() {
            return Err("Attendi la fine del calcolo prima di aprire uno studio.".into());
        }
        // Validate the whole file before replacing any current input or result.
        let document = io::load(path)?;
        self.invalidate_results();
        self.study = StudyDraft::from_study(document.name, document.study);
        self.collapsed.clear();
        self.selected_stick = None;
        self.selected_input = None;
        self.current_path = Some(path.to_owned());
        self.pending = None;
        self.parameters_open = false;
        self.parameters_draft = None;
        self.filter_editor.open = false;
        self.apply_view(document.view.clone());
        self.dirty = false;
        if document.recalculate {
            if self.start_search() {
                self.restore_view = Some((self.revision, document.view));
                self.dirty = false;
            }
        } else {
            self.status = format!("Studio aperto: {}", path.display());
        }
        if document.engine != "rust-geometry-v1" {
            self.diagnostic=Some("Studio creato con un altro motore: i risultati saranno ricalcolati con quello attuale.".into());
        }
        Ok(())
    }
    pub(super) fn save_current(&mut self, as_new: bool) -> bool {
        if self.job.is_some() {
            self.diagnostic = Some("Attendi la fine del calcolo prima di salvare.".into());
            return false;
        }
        // Avoid opening a file picker for a study that cannot be saved.
        if let Err(e) = self.document() {
            self.diagnostic = Some(format!("Studio non salvato: {e}"));
            return false;
        }
        let path = if !as_new {
            self.current_path.clone()
        } else {
            None
        };
        let path = path.or_else(|| {
            rfd::FileDialog::new()
                .set_title("Salva studio Stick Optimizer")
                .add_filter("Studio Rust", &["stic"])
                .set_file_name("studio.stic")
                .save_file()
        });
        let Some(path) = path else {
            return false;
        };
        match self.save_to(&path) {
            Ok(()) => true,
            Err(e) => {
                self.diagnostic = Some(format!("Studio non salvato: {e}"));
                false
            }
        }
    }
    pub(super) fn handle_file_action(&mut self) {
        let Some(action) = self.file_action.take() else {
            return;
        };
        let result = match action {
            FileAction::Open => {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Apri studio Stick Optimizer")
                    .add_filter("Studio Rust", &["stic"])
                    .pick_file()
                {
                    self.open_from(&path)
                } else {
                    Ok(())
                }
            }
            FileAction::Save | FileAction::SaveAs => {
                self.save_current(matches!(action, FileAction::SaveAs));
                Ok(())
            }
            FileAction::Preferences => (|| -> Result<(), io::Error> {
                // Validate parameters independently of incomplete input drafts.
                let mut sample = StudyDraft::example();
                sample.parameters = self.study.parameters.clone();
                let study = sample.to_study().map_err(|errors| errors.join("; "))?;
                let path = preferences_path()?;
                std::fs::create_dir_all(path.parent().expect("preferences directory"))?;
                io::save_preferences(&path, &Preferences::new(study.settings, study.weights))?;
                self.defaults = Some(self.study.parameters.clone());
                self.status =
                    "Parametri predefiniti salvati per i nuovi studi e il prossimo avvio.".into();
                Ok(())
            })(),
            FileAction::Summary | FileAction::Details => (|| -> Result<(), io::Error> {
                let rows = self.visible_results();
                let bytes = if matches!(action, FileAction::Summary) {
                    if rows.is_empty() {
                        return Err("Nessuna soluzione visibile da esportare.".into());
                    }
                    io::summary_csv(rows.iter().map(|r| (r.id, r.solution.as_ref())))
                } else {
                    let selected = rows
                        .iter()
                        .find(|r| Some(r.id) == self.selected)
                        .ok_or("Seleziona una soluzione visibile.")?;
                    io::details_csv(&selected.solution)
                };
                let filename = if matches!(action, FileAction::Summary) {
                    "soluzioni.csv"
                } else {
                    "dettaglio.csv"
                };
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Esporta CSV")
                    .add_filter("CSV", &["csv"])
                    .set_file_name(filename)
                    .save_file()
                {
                    io::write_atomic(&path, &bytes)?;
                    self.status = format!("CSV esportato: {}", path.display());
                }
                Ok(())
            })(),
        };
        if let Err(e) = result {
            self.diagnostic = Some(format!(
                "Operazione non completata: {e}. Lo studio corrente è stato conservato."
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "stick-desktop-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
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
    fn wait(app: &mut PrototypeApp) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        while app.job.is_some() {
            app.poll_search();
            assert!(std::time::Instant::now() < deadline);
            std::thread::yield_now();
        }
    }
    #[test]
    fn saved_study_restores_inputs_results_filters_selection_and_editable_ids() {
        let temp = Temp::new();
        let path = temp.0.join("studio.stic");
        let mut app = PrototypeApp::default();
        app.study.parameters.result_count = usize::MAX.to_string();
        assert!(app.start_search());
        wait(&mut app);
        assert_eq!(app.results.len(), 16);
        app.selected = Some(4);
        app.ranges[0] = Range {
            min: Some(app.results[3].pitch),
            max: None,
        };
        app.order_by_pitch = true;
        let expected = app
            .results
            .iter()
            .map(|r| r.solution.as_ref().clone())
            .collect::<Vec<_>>();
        let view = app.view_state();
        app.save_to(&path).unwrap();
        assert!(!app.dirty);
        app.study.name = "Changed".into();
        app.invalidate_results();
        app.open_from(&path).unwrap();
        assert!(app.job.is_some());
        wait(&mut app);
        assert_eq!(app.study.name, "Studio di esempio");
        assert_eq!(app.view_state(), view);
        assert!(!app.dirty);
        assert_eq!(
            app.results
                .iter()
                .map(|r| r.solution.as_ref().clone())
                .collect::<Vec<_>>(),
            expected
        );
        assert!(
            app.visible_results()
                .iter()
                .any(|r| Some(r.id) == app.selected)
        );
        let ids = app
            .study
            .sticks
            .iter()
            .map(|s| s.id)
            .chain(app.study.formats.iter().map(|f| f.id))
            .collect::<Vec<_>>();
        app.study.add_stick();
        app.study.add_format();
        assert!(!ids.contains(&app.study.formats.last().unwrap().id));
        assert!(app.study.input_errors().is_empty());
    }
    #[test]
    fn invalid_open_and_save_preserve_current_study_path_and_results() {
        let temp = Temp::new();
        let path = temp.0.join("valid.stic");
        let bad = temp.0.join("bad.stic");
        let mut app = PrototypeApp::default();
        assert!(app.start_search());
        wait(&mut app);
        app.save_to(&path).unwrap();
        std::fs::write(&bad, b"{broken").unwrap();
        assert!(app.open_from(&bad).is_err());
        assert_eq!(app.results.len(), 16);
        assert_eq!(app.current_path, Some(path.clone()));
        assert!(!app.dirty);
        app.study.formats[0].quantity = "abc".into();
        app.invalidate_results();
        let saved = std::fs::read(&path).unwrap();
        assert!(app.save_to(&path).is_err());
        assert!(app.dirty);
        assert_eq!(std::fs::read(&path).unwrap(), saved);
    }
    #[test]
    fn opening_inputs_without_results_does_not_start_a_search() {
        let temp = Temp::new();
        let path = temp.0.join("inputs.stic");
        let mut app = PrototypeApp::default();
        app.save_to(&path).unwrap();
        app.open_from(&path).unwrap();
        assert!(app.job.is_none());
        assert!(app.results.is_empty());
        assert!(!app.dirty);
        assert!(app.start_search());
        assert!(app.open_from(&path).is_err());
        wait(&mut app);
    }
    #[test]
    fn edits_during_reopening_discard_the_saved_view_and_old_results() {
        let temp = Temp::new();
        let path = temp.0.join("study.stic");
        let mut app = PrototypeApp::default();
        app.start_search();
        wait(&mut app);
        app.save_to(&path).unwrap();
        app.open_from(&path).unwrap();
        app.study.formats[0].quantity = "11".into();
        app.invalidate_results();
        wait(&mut app);
        assert!(app.results.is_empty());
        assert!(app.restore_view.is_none());
        assert!(app.dirty);
    }
    #[test]
    fn filters_changed_while_reopening_are_kept_instead_of_the_saved_view() {
        let temp = Temp::new();
        let path = temp.0.join("study.stic");
        let mut app = PrototypeApp::default();
        app.start_search();
        wait(&mut app);
        app.save_to(&path).unwrap();
        app.open_from(&path).unwrap();
        let range = Range {
            min: Some(180.0),
            max: Some(180.0),
        };
        app.ranges[0] = range;
        app.dirty = true;
        wait(&mut app);
        assert_eq!(app.ranges[0], range);
        assert!(app.dirty);
        assert!(!app.visible_results().is_empty());
        assert!(app.visible_results().iter().all(|r| r.pitch == 180.0));
    }
    #[test]
    fn new_study_applies_preferences_without_carrying_the_previous_file_path() {
        let mut app = PrototypeApp::default();
        let defaults = crate::draft::Parameters {
            channels: "18".into(),
            ..Default::default()
        };
        app.defaults = Some(defaults);
        app.current_path = Some(PathBuf::from("old.stic"));
        app.apply_pending(PendingAction::EmptyStudy);
        assert_eq!(app.study.parameters.channels, "18");
        assert!(app.current_path.is_none());
        assert!(app.dirty);
        app.pending = Some(PendingAction::CloseWindow);
        app.apply_pending(PendingAction::CloseWindow);
        assert!(app.allow_close);
    }
}
