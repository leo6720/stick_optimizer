use crate::Study;
use std::collections::HashSet;

pub fn validate(study: &Study) -> Vec<String> {
    let mut errors = Vec::new();
    let p = &study.settings;
    if p.channels < 2 {
        errors.push("Canali MT: servono almeno 2 canali per definire l'offset.".into());
    }
    if p.result_count == 0 {
        errors.push("Numero risultati: serve un intero positivo.".into());
    }
    if p.max_layers == Some(0) {
        errors.push("Limite strati: serve un intero positivo o nessun limite.".into());
    }
    for (label, value, positive) in [
        ("Offset massimo", p.max_shift, false),
        ("Spessore divisore", p.divider_width, false),
        ("Spessore parete", p.wall_width, false),
        ("Gioco tra stick", p.adjacent_clearance, false),
        ("Gioco parete/divisore", p.wall_clearance, false),
        ("Extra B", p.carton_b_extra, false),
        ("Passo massimo", p.max_pitch, true),
        ("Incremento passo", p.pitch_step, true),
        ("Obiettivo A/B", p.ab_target, true),
    ] {
        if !value.is_finite() || value < 0.0 || (positive && value == 0.0) {
            errors.push(format!(
                "{label}: numero finito {} richiesto.",
                if positive { "positivo" } else { "non negativo" }
            ));
        }
    }
    if p.pitch_step.is_finite() && p.pitch_step > 0.0 && p.pitch_step < 0.000001 {
        errors.push("Incremento passo: la precisione minima è 0,000001 mm.".into());
    }
    if study
        .weights
        .values()
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0)
    {
        errors.push("Pesi: tutti i valori devono essere finiti e non negativi.".into());
    }
    if study.sticks.is_empty() {
        errors.push("Aggiungi almeno uno stick.".into());
    }
    if study.formats.is_empty() {
        errors.push("Aggiungi almeno un formato.".into());
    }
    let mut ids = HashSet::new();
    let mut names = HashSet::new();
    for s in &study.sticks {
        if !ids.insert(s.id) || s.name.trim().is_empty() || !names.insert(s.name.trim()) {
            errors.push("Stick: nomi e ID devono essere presenti e univoci.".into());
        }
        for (label, v, positive) in [
            ("lunghezza", s.length, true),
            ("larghezza", s.width, true),
            ("spessore", s.thickness, true),
            ("pinna", s.fin, false),
        ] {
            if !v.is_finite() || v < 0.0 || (positive && v == 0.0) {
                errors.push(format!("{}: {label} non valida.", s.name));
            }
        }
    }
    let mut format_ids = HashSet::new();
    names.clear();
    for f in &study.formats {
        if !format_ids.insert(f.id) || f.name.trim().is_empty() || !names.insert(f.name.trim()) {
            errors.push("Formati: nomi e ID devono essere presenti e univoci.".into());
        }
        if !ids.contains(&f.stick_id) {
            errors.push(format!("{}: stick inesistente.", f.name));
        }
        if f.quantity == 0 {
            errors.push(format!("{}: quantità positiva richiesta.", f.name));
        }
    }
    errors
}
