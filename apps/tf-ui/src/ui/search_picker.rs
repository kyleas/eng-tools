use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct PickerOption {
    pub id: String,
    pub label: String,
    pub group: Option<String>,
}

pub fn searchable_picker(
    ui: &mut egui::Ui,
    id: egui::Id,
    selected_id: &mut String,
    options: &[PickerOption],
    picker_queries: &mut HashMap<String, String>,
) -> Option<String> {
    let query_key = format!("{:?}", id);
    let mut picked: Option<String> = None;
    let selected_label = options
        .iter()
        .find(|o| o.id == *selected_id)
        .map(|o| o.label.clone())
        .unwrap_or_else(|| {
            if selected_id.is_empty() {
                "<select>".to_string()
            } else {
                selected_id.clone()
            }
        });
    ui.push_id(id, |ui| {
        ui.menu_button(selected_label, |ui| {
            let query = picker_queries.entry(query_key.clone()).or_default();
            ui.label("Search");
            ui.text_edit_singleline(query);
            ui.separator();
            let filtered = filter_options(options, query);
            let mut grouped: BTreeMap<String, Vec<&PickerOption>> = BTreeMap::new();
            for opt in filtered {
                let group = opt.group.clone().unwrap_or_else(|| "Other".to_string());
                grouped.entry(group).or_default().push(opt);
            }
            for (group, group_items) in grouped {
                ui.small(egui::RichText::new(group).italics());
                for opt in group_items {
                    if ui
                        .selectable_label(
                            *selected_id == opt.id,
                            format!("{} ({})", opt.label, opt.id),
                        )
                        .clicked()
                    {
                        picked = Some(opt.id.clone());
                        ui.close_menu();
                    }
                }
                ui.add_space(4.0);
            }
        });
    });
    picked
}

pub fn filter_options<'a>(items: &'a [PickerOption], query: &str) -> Vec<&'a PickerOption> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return items.iter().collect::<Vec<_>>();
    }
    items
        .iter()
        .filter(|item| {
            item.id.to_lowercase().contains(&q)
                || item.label.to_lowercase().contains(&q)
                || item
                    .group
                    .as_ref()
                    .map(|g| g.to_lowercase().contains(&q))
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_matches_case_insensitive_substrings() {
        let opts = vec![
            PickerOption {
                id: "compressible.isentropic_pressure_ratio".to_string(),
                label: "Isentropic Pressure Ratio".to_string(),
                group: Some("Compressible".to_string()),
            },
            PickerOption {
                id: "structures.hoop_stress".to_string(),
                label: "Thin-Wall Hoop Stress".to_string(),
                group: Some("Structures".to_string()),
            },
        ];
        let filtered = filter_options(&opts, "hoop");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "structures.hoop_stress");
    }
}
