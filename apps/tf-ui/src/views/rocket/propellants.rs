use crate::rocket_workspace::{RocketWorkspace, species_display_name};

pub fn show(ui: &mut egui::Ui, workspace: &mut RocketWorkspace) {
    ui.heading("Rocket Propellants");
    ui.label("Select a propellant preset and apply it to the active Performance case.");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Search presets:");
        ui.text_edit_singleline(&mut workspace.propellants.search_query);
    });

    ui.separator();
    ui.heading("Preset Library");
    egui::ScrollArea::vertical()
        .max_height(220.0)
        .show(ui, |ui| {
            for preset in workspace.filtered_propellant_presets() {
                let is_selected = workspace.propellants.selected_preset_key == preset.key;
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(is_selected, preset.display_name)
                            .clicked()
                        {
                            workspace.propellants.selected_preset_key = preset.key.to_owned();
                        }
                        ui.label(format!("[{}]", preset.category));
                    });
                    ui.small(format!(
                        "Oxidizer: {} ({}) | Fuel: {} ({}) | Notes: {}",
                        preset.oxidizer_name,
                        species_display_name(preset.oxidizer_name),
                        preset.fuel_name,
                        species_display_name(preset.fuel_name),
                        preset.notes
                    ));
                    if let Some(mr) = preset.recommended_mixture_ratio {
                        ui.small(format!("Recommended O/F: {:.3}", mr));
                    }
                });
            }
        });

    ui.separator();
    ui.heading("Current Selection");
    if let Some(preset) = workspace.selected_propellant_preset() {
        ui.label(format!("Preset: {}", preset.display_name));
        ui.label(format!("Category: {}", preset.category));
        ui.label(format!(
            "Performance mapping: oxidizer='{}' ({}), fuel='{}' ({})",
            preset.oxidizer_name,
            species_display_name(preset.oxidizer_name),
            preset.fuel_name,
            species_display_name(preset.fuel_name),
        ));
        ui.label(format!("Notes: {}", preset.notes));
    } else {
        ui.label("No preset selected.");
    }

    ui.separator();
    if ui.button("Apply to Performance").clicked() {
        match workspace.apply_selected_propellant_to_performance() {
            Some(message) => {
                workspace.status = Some(message);
                workspace.last_error = None;
            }
            None => {
                workspace.last_error = Some("No propellant preset selected.".to_owned());
                workspace.status = None;
            }
        }
    }

    if let Some(status) = &workspace.status {
        ui.colored_label(egui::Color32::GREEN, status);
    }
    if let Some(err) = &workspace.last_error {
        ui.colored_label(egui::Color32::RED, err);
    }

    ui.separator();
    ui.heading("Performance Case Preview");
    ui.label(format!("Case: {}", workspace.performance_case.case_name));
    ui.label(format!(
        "Current performance propellants: {} ({}) / {} ({})",
        workspace.performance_case.oxidizer_name,
        species_display_name(&workspace.performance_case.oxidizer_name),
        workspace.performance_case.fuel_name,
        species_display_name(&workspace.performance_case.fuel_name)
    ));
    ui.label(format!(
        "Current O/F: {:.4}",
        workspace.performance_case.mixture_ratio
    ));
}
