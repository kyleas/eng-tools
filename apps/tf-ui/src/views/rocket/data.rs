use crate::rocket_workspace::RocketWorkspace;

pub fn show(ui: &mut egui::Ui, workspace: &RocketWorkspace) {
    let snapshot = workspace.data_snapshot();

    ui.heading("Rocket Data & Provenance");
    ui.label(
        "Read-only summary of active assumptions, result provenance, and current limitations.",
    );

    ui.separator();
    ui.heading("Active Case Summary");
    egui::Grid::new("rocket_data_case")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Case");
            ui.label(&snapshot.case_name);
            ui.end_row();
            ui.label("Backend chain");
            ui.label(&snapshot.backend_chain);
            ui.end_row();
            ui.label("Status");
            ui.label(snapshot.status.as_deref().unwrap_or("n/a"));
            ui.end_row();
            ui.label("Last error");
            ui.label(snapshot.last_error.as_deref().unwrap_or("none"));
            ui.end_row();
        });

    ui.separator();
    ui.heading("Propellants");
    ui.label(format!(
        "Active pair: {} / {}",
        snapshot.oxidizer, snapshot.fuel
    ));
    if let Some(p) = &snapshot.selected_preset {
        ui.small(format!("Preset: {} [{}]", p.display_name, p.category));
        if !p.notes.is_empty() {
            ui.small(format!("Notes: {}", p.notes));
        }
    } else {
        ui.small("Preset: custom/manual");
    }

    ui.separator();
    ui.heading("Assumptions");
    ui.collapsing("Performance assumptions", |ui| {
        ui.small(format!(
            "Chamber pressure: {:.0} Pa",
            snapshot.performance.chamber_pressure_pa
        ));
        ui.small(format!(
            "Mixture ratio (O/F): {:.4}",
            snapshot.performance.mixture_ratio
        ));
        ui.small(format!(
            "Ambient pressure: {:.0} Pa",
            snapshot.performance.ambient_pressure_pa
        ));
        ui.small(format!(
            "Oxidizer temperature: {:.2} K",
            snapshot.performance.oxidizer_temperature_k
        ));
        ui.small(format!(
            "Fuel temperature: {:.2} K",
            snapshot.performance.fuel_temperature_k
        ));
        ui.small(format!(
            "Combustor model: {}",
            snapshot.performance.combustor_model
        ));
        ui.small(format!(
            "Nozzle chemistry: {}",
            snapshot.performance.nozzle_chemistry
        ));
        ui.small(format!(
            "Nozzle constraint: {}",
            snapshot.performance.nozzle_constraint
        ));
    });
    ui.collapsing("Geometry assumptions", |ui| {
        ui.small(format!("Sizing mode: {}", snapshot.geometry.sizing_mode));
        ui.small(format!(
            "Throat input: {}",
            snapshot.geometry.throat_input_label
        ));
        ui.small(format!(
            "Contraction ratio Ac/At: {:.3}",
            snapshot.geometry.contraction_ratio
        ));
        ui.small(format!(
            "Characteristic length L*: {:.3} m",
            snapshot.geometry.characteristic_length_m
        ));
        ui.small(format!(
            "Nozzle half-angle: {:.2} deg",
            snapshot.geometry.nozzle_half_angle_deg
        ));
    });
    ui.collapsing("Thermal assumptions", |ui| {
        ui.small(format!("Thermal model: {}", snapshot.thermal.model));
        ui.small(format!("Cooling mode: {}", snapshot.thermal.cooling_mode));
        ui.small(format!(
            "Recovery temperature: {:.1} K",
            snapshot.thermal.recovery_temperature_k
        ));
        ui.small(format!(
            "Wall temperature: {:.1} K",
            snapshot.thermal.wall_temperature_k
        ));
        ui.small(format!(
            "Reference h_g: {:.0} W/m²-K",
            snapshot.thermal.reference_htc_w_m2_k
        ));
    });

    ui.separator();
    ui.heading("Result Provenance");
    for field in &snapshot.provenance_fields {
        ui.horizontal_wrapped(|ui| {
            ui.strong(&field.name);
            ui.small(format!("— {}", field.source.label()));
            ui.label(format!(": {}", field.value));
        });
    }

    ui.separator();
    ui.heading("Warnings / Unsupported / Deferred");
    if snapshot.warnings.is_empty() {
        ui.small("No active warnings.");
    } else {
        for w in &snapshot.warnings {
            ui.colored_label(egui::Color32::YELLOW, format!("• {w}"));
        }
    }
}
