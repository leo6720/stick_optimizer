use crate::Error;
use stick_optimizer_core::{Solution, Study, candidate_dimensions};
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
pub fn summary_csv<'a>(
    study: &Study,
    solutions: impl IntoIterator<Item = (usize, &'a Solution)>,
) -> Result<Vec<u8>, Error> {
    let mut out = "\u{feff}rank,score,cartoner_pitch_mm,pocket_types,head_types,max_layers,layer_penalty,carryover_penalty,grouping_penalty,stability_penalty,ab_penalty,csv_version,engine,channels,max_extra_spacing_mm,divider_width_mm,wall_width_mm,wall_clearance_min_mm,stick_clearance_min_mm,configuration\r\n".to_owned();
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
        fields.extend(parameters(study));
        fields.push(configuration(s)?);
        row(&mut out, fields);
    }
    Ok(out.into_bytes())
}
pub fn details_csv(study: &Study, rank: usize, solution: &Solution) -> Result<Vec<u8>, Error> {
    let mut out = "\u{feff}format_name,stick_name,input_pitch_mm,grouping,dividers,pockets_per_pitch,pocket_width_mm,pocket_length_mm,pocket_pitch_mm,cartoner_pitch_mm,occupied_width_mm,unused_space_mm,layers,stack_height_mm,carton_a_mm,carton_b_mm,ab_ratio,carryover_required,carryover_cycle,unsupported_width_mm,width_ratio,deposits,head_width_mm,layer_penalty,carryover_penalty,grouping_penalty,stability_penalty,ab_penalty,csv_version,engine,channels,max_extra_spacing_mm,divider_width_mm,wall_width_mm,wall_clearance_min_mm,stick_clearance_min_mm,rank,format_id,stick_id,stick_width_mm,head_id,head_format_ids,head_shared,approach_spacing_mm,minimum_approach_spacing_mm,extra_spacing_mm,head_pickup_centres_mm,head_final_centres_mm,pocket_stick_centres_mm,divider_lefts_mm,left_wall_clearance_mm,right_wall_clearance_mm,gap_centre_spacings_mm,gap_stick_clearances_mm,gap_divider_clearances_mm,gap_structural\r\n".to_owned();
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
        fields.extend(parameters(study));
        let d = candidate_dimensions(study, c)
            .ok_or("Geometria del dettaglio non coerente con lo studio.")?;
        let head = c
            .head_id
            .map(|id| {
                solution
                    .head_types
                    .iter()
                    .find(|h| h.id == id)
                    .ok_or("Assegnazione testa mancante.")
            })
            .transpose()?;
        fields.extend([
            rank.to_string(),
            c.format_id.to_string(),
            c.stick_id.to_string(),
            d.stick_width.to_string(),
            c.head_id.map(|id| id.to_string()).unwrap_or_default(),
            serde_json::to_string(
                &head
                    .map(|h| h.geometry.format_ids.as_slice())
                    .unwrap_or_default(),
            )?,
            head.map(|h| (h.geometry.format_ids.len() > 1).to_string())
                .unwrap_or_default(),
            optional(d.approach_spacing),
            optional(d.minimum_approach_spacing),
            optional(d.extra_spacing),
            serde_json::to_string(
                &head
                    .map(|h| {
                        h.geometry
                            .positions
                            .iter()
                            .map(|p| p.pickup_x)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
            )?,
            serde_json::to_string(
                &head
                    .map(|h| {
                        h.geometry
                            .positions
                            .iter()
                            .map(|p| p.final_x)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
            )?,
            serde_json::to_string(&c.geometry.stick_centres)?,
            serde_json::to_string(&c.geometry.divider_lefts)?,
            d.left_wall_clearance.to_string(),
            d.right_wall_clearance.to_string(),
            serde_json::to_string(&d.gaps.iter().map(|g| g.centre_spacing).collect::<Vec<_>>())?,
            serde_json::to_string(&d.gaps.iter().map(|g| g.stick_clearance).collect::<Vec<_>>())?,
            serde_json::to_string(
                &d.gaps
                    .iter()
                    .map(|g| g.divider_clearances)
                    .collect::<Vec<_>>(),
            )?,
            serde_json::to_string(&d.gaps.iter().map(|g| g.structural).collect::<Vec<_>>())?,
        ]);
        row(&mut out, fields);
    }
    Ok(out.into_bytes())
}

fn optional(value: Option<f64>) -> String {
    value.map(|v| v.to_string()).unwrap_or_default()
}
fn parameters(study: &Study) -> [String; 8] {
    let p = &study.settings;
    [
        "2".into(),
        crate::ENGINE_ID.into(),
        p.channels.to_string(),
        p.max_extra_spacing.to_string(),
        p.divider_width.to_string(),
        p.wall_width.to_string(),
        p.wall_clearance.to_string(),
        p.adjacent_clearance.to_string(),
    ]
}
// Stable IDs and full coordinates distinguish separate heads even when their
// pickup keys and geometries coincide. Names are labels, never identity keys.
fn configuration(solution: &Solution) -> Result<String, Error> {
    let value = serde_json::json!({
        "heads":solution.head_types.iter().map(|h|serde_json::json!({"id":h.id,"format_ids":h.geometry.format_ids,"pickup_centres_mm":h.geometry.positions.iter().map(|p|p.pickup_x).collect::<Vec<_>>(),"final_centres_mm":h.geometry.positions.iter().map(|p|p.final_x).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "formats":solution.candidates.iter().map(|c|serde_json::json!({"format_id":c.format_id,"stick_id":c.stick_id,"head_id":c.head_id,"grouping":c.grouping,"dividers":c.dividers,"pockets_per_pitch":c.pockets_per_pitch,"input_pitch_mm":c.input_pitch.mm(),"pocket_width_mm":c.pocket_width,"pocket_stick_centres_mm":c.geometry.stick_centres,"divider_lefts_mm":c.geometry.divider_lefts})).collect::<Vec<_>>()
    });
    Ok(serde_json::to_string(&value)?)
}
