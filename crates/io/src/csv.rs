use stick_optimizer_core::Solution;
fn row(out: &mut String, fields: impl IntoIterator<Item = String>) {
    for (i, field) in fields.into_iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        if field.contains([',', '"', '\r', '\n']) {
            out.push('"');
            out.push_str(&field.replace('"', "\"\""));
            out.push('"');
        } else {
            out.push_str(&field);
        }
    }
    out.push_str("\r\n");
}
/// UTF-8 BOM, comma-separated, full float precision, decimal point. Stable ranks.
pub fn summary_csv<'a>(solutions: impl IntoIterator<Item = (usize, &'a Solution)>) -> Vec<u8> {
    let mut out = "\u{feff}rank,score,cartoner_pitch_mm,pocket_types,head_types,max_layers,layer_penalty,carryover_penalty,grouping_penalty,stability_penalty,ab_penalty\r\n".to_owned();
    for (rank, s) in solutions {
        let mut fields = vec![
            rank.to_string(),
            s.score.to_string(),
            s.pitch.mm().to_string(),
            s.pocket_types.len().to_string(),
            s.head_types.len().to_string(),
            s.max_layers.to_string(),
        ];
        fields.extend(s.penalties.map(|v| v.to_string()));
        row(&mut out, fields);
    }
    out.into_bytes()
}
pub fn details_csv(solution: &Solution) -> Vec<u8> {
    let mut out = "\u{feff}format_name,stick_name,input_pitch_mm,grouping,dividers,pockets_per_pitch,pocket_width_mm,pocket_length_mm,pocket_pitch_mm,cartoner_pitch_mm,occupied_width_mm,unused_space_mm,layers,stack_height_mm,carton_a_mm,carton_b_mm,ab_ratio,carryover_required,carryover_cycle,unsupported_width_mm,width_ratio,deposits,head_width_mm,layer_penalty,carryover_penalty,grouping_penalty,stability_penalty,ab_penalty\r\n".to_owned();
    for c in &solution.candidates {
        let mut fields = vec![
            c.format_name.clone(),
            c.stick_name.clone(),
            c.input_pitch.mm().to_string(),
            c.grouping.to_string(),
            c.dividers.to_string(),
            c.pockets_per_pitch.to_string(),
            c.pocket_width.to_string(),
            c.pocket_length.to_string(),
            c.pocket_pitch.to_string(),
            c.cartoner_pitch.mm().to_string(),
            c.occupied_width.to_string(),
            c.unused_space.to_string(),
            c.layers.to_string(),
            c.stack_height.to_string(),
            c.carton_a.to_string(),
            c.carton_b.to_string(),
            c.ab_ratio.to_string(),
            c.carryover_required.to_string(),
            c.carryover_cycle.to_string(),
            c.unsupported_width.to_string(),
            c.width_ratio.to_string(),
            c.deposits.to_string(),
            c.head_width.to_string(),
        ];
        fields.extend(c.penalties.map(|v| v.to_string()));
        row(&mut out, fields);
    }
    out.into_bytes()
}
