mod data;
mod geometry;
mod performance;
mod plotting;
mod propellants;
mod studies;
mod thermal;

use crate::rocket_workspace::{RocketSubtab, RocketWorkspace};
use tf_rpa::NozzleConstraint;

#[derive(Debug, Default)]
pub struct RocketView;

impl RocketView {
    pub fn show(&mut self, ui: &mut egui::Ui, workspace: &mut RocketWorkspace) {
        ui.horizontal_wrapped(|ui| {
            for tab in RocketSubtab::all() {
                ui.selectable_value(&mut workspace.selected_subtab, *tab, tab.label());
            }
        });
        ui.separator();

        match workspace.selected_subtab {
            RocketSubtab::Performance => performance::show(ui, workspace),
            RocketSubtab::Geometry => geometry::show(ui, workspace),
            RocketSubtab::Thermal => thermal::show(ui, workspace),
            RocketSubtab::Propellants => propellants::show(ui, workspace),
            RocketSubtab::Studies => studies::show(ui, workspace),
            RocketSubtab::Data => data::show(ui, workspace),
        }
    }
}

pub(super) fn show_case_context_banner(ui: &mut egui::Ui, workspace: &RocketWorkspace) {
    ui.group(|ui| {
        ui.label(format!(
            "Case: {} | Propellants: {} / {}",
            workspace.performance_case.case_name,
            workspace.performance_case.oxidizer_name,
            workspace.performance_case.fuel_name
        ));
        let nozzle_constraint = match workspace.performance_case.nozzle_constraint {
            NozzleConstraint::ExpansionRatio(value) => format!("Expansion ratio ε={value:.2}"),
            NozzleConstraint::ExitPressurePa(value) => format!("Exit pressure={value:.0} Pa"),
        };
        ui.small(format!(
            "Pc={:.0} Pa | MR={:.3} | Pa={:.0} Pa | {:?} / {:?} | {}",
            workspace.performance_case.chamber_pressure_pa,
            workspace.performance_case.mixture_ratio,
            workspace.performance_case.ambient_pressure_pa,
            workspace.performance_case.combustor_model,
            workspace.performance_case.nozzle_chemistry_model,
            nozzle_constraint
        ));
    });
}
