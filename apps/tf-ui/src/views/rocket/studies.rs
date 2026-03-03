use crate::rocket_workspace::{RocketStudyConfig, RocketWorkspace};
use crate::views::rocket::plotting::show_multi_series_plot;
use crate::views::rocket::{plotting::PlotSeriesSpec, show_case_context_banner};
use tf_cea::NativeCeaBackend;
use tf_rpa::{
    RocketAnalysisSolver, RocketStudyProblem, StudyOutputMetric, StudyRange, StudyVariable,
    run_single_variable_study,
};

pub fn show(ui: &mut egui::Ui, workspace: &mut RocketWorkspace) {
    ui.heading("Rocket Studies");
    ui.label("Single-variable studies derived from the current Performance case.");
    ui.label("Base case assumptions and propellant setup are inherited directly from Performance.");
    show_case_context_banner(ui, workspace);
    ui.separator();

    show_study_controls(ui, &mut workspace.studies);

    ui.separator();
    if ui.button("Run Study").clicked() {
        match run_study(workspace) {
            Ok(msg) => {
                workspace.status = Some(msg);
                workspace.last_error = None;
            }
            Err(err) => {
                workspace.last_error = Some(err);
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
    if let Some(result) = &workspace.last_study_result {
        ui.heading("Study Warnings");
        if result.warning_messages.is_empty() {
            ui.label("No warnings.");
        } else {
            for warning in &result.warning_messages {
                ui.colored_label(egui::Color32::YELLOW, format!("• {warning}"));
            }
        }

        ui.separator();
        ui.heading("Results Table");
        egui::ScrollArea::both().max_height(260.0).show(ui, |ui| {
            egui::Grid::new("rocket_study_table")
                .striped(true)
                .spacing([14.0, 6.0])
                .show(ui, |ui| {
                    ui.strong(result.variable.label());
                    for metric in &result.outputs {
                        ui.strong(metric.label());
                    }
                    ui.strong("Status");
                    ui.end_row();

                    for point in &result.points {
                        ui.label(format!("{:.6}", point.sweep_value));
                        for metric in &result.outputs {
                            let value = point
                                .outputs
                                .iter()
                                .find(|m| m.metric == *metric)
                                .map(|m| format!("{:.6}", m.value))
                                .unwrap_or_else(|| "n/a".to_owned());
                            ui.label(value);
                        }
                        ui.label(point.error.as_deref().unwrap_or("ok"));
                        ui.end_row();
                    }
                });
        });

        ui.separator();
        ui.heading("Inline Plot");
        let series: Vec<PlotSeriesSpec<'_>> = result
            .outputs
            .iter()
            .map(|metric| PlotSeriesSpec {
                name: metric.label(),
                points: result
                    .points
                    .iter()
                    .filter_map(|p| {
                        p.outputs
                            .iter()
                            .find(|m| m.metric == *metric)
                            .map(|m| [p.sweep_value, m.value])
                    })
                    .collect(),
            })
            .collect();

        show_multi_series_plot(
            ui,
            "rocket_study_plot",
            result.variable.label(),
            "Selected output",
            260.0,
            series,
        );
    } else {
        ui.label("No study results yet. Configure and run a study.");
    }
}

fn show_study_controls(ui: &mut egui::Ui, config: &mut RocketStudyConfig) {
    egui::Grid::new("rocket_study_controls")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("Sweep variable");
            egui::ComboBox::from_id_salt("study_variable")
                .selected_text(config.variable.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut config.variable,
                        StudyVariable::ChamberPressurePa,
                        StudyVariable::ChamberPressurePa.label(),
                    );
                    ui.selectable_value(
                        &mut config.variable,
                        StudyVariable::MixtureRatio,
                        StudyVariable::MixtureRatio.label(),
                    );
                    ui.selectable_value(
                        &mut config.variable,
                        StudyVariable::AmbientPressurePa,
                        StudyVariable::AmbientPressurePa.label(),
                    );
                    ui.selectable_value(
                        &mut config.variable,
                        StudyVariable::ExpansionRatio,
                        StudyVariable::ExpansionRatio.label(),
                    );
                });
            ui.end_row();

            ui.label("Sweep min");
            ui.add(egui::DragValue::new(&mut config.min).speed(0.1));
            ui.end_row();

            ui.label("Sweep max");
            ui.add(egui::DragValue::new(&mut config.max).speed(0.1));
            ui.end_row();

            ui.label("Point count");
            ui.add(egui::DragValue::new(&mut config.point_count).range(2..=200));
            ui.end_row();
        });

    ui.separator();
    ui.label("Output metrics to include:");
    for metric in all_metrics() {
        let mut selected = config.selected_metrics.contains(metric);
        if ui.checkbox(&mut selected, metric.label()).changed() {
            if selected {
                if !config.selected_metrics.contains(metric) {
                    config.selected_metrics.push(*metric);
                }
            } else {
                config.selected_metrics.retain(|m| m != metric);
            }
        }
    }
}

fn all_metrics() -> &'static [StudyOutputMetric] {
    const METRICS: &[StudyOutputMetric] = &[
        StudyOutputMetric::ChamberTemperatureK,
        StudyOutputMetric::ChamberGamma,
        StudyOutputMetric::ChamberMolecularWeightKgPerKmol,
        StudyOutputMetric::CharacteristicVelocityMPerS,
        StudyOutputMetric::ThrustCoefficientVac,
        StudyOutputMetric::SpecificImpulseVacS,
        StudyOutputMetric::SpecificImpulseAmbS,
        StudyOutputMetric::EffectiveExhaustVelocityVacMPerS,
        StudyOutputMetric::EffectiveExhaustVelocityAmbMPerS,
        StudyOutputMetric::ChamberToAmbientPressureRatio,
    ];
    METRICS
}

fn run_study(workspace: &mut RocketWorkspace) -> Result<String, String> {
    let backend = NativeCeaBackend::new();
    let solver = RocketAnalysisSolver::new(backend);

    let study = RocketStudyProblem {
        base_problem: workspace.performance_case.to_analysis_problem(),
        variable: workspace.studies.variable,
        range: StudyRange {
            min: workspace.studies.min,
            max: workspace.studies.max,
            point_count: workspace.studies.point_count,
        },
        outputs: workspace.studies.selected_metrics.clone(),
    };

    let result = run_single_variable_study(&solver, &study)
        .map_err(|e| format!("Study execution failed: {e}"))?;

    let success_count = result.points.iter().filter(|p| p.error.is_none()).count();
    workspace.last_study_result = Some(result);

    Ok(format!(
        "Study complete: {} successful points",
        success_count
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_selection_toggles() {
        let mut cfg = RocketStudyConfig::default();
        let metric = StudyOutputMetric::ChamberGamma;
        assert!(!cfg.selected_metrics.contains(&metric));
        cfg.selected_metrics.push(metric);
        assert!(cfg.selected_metrics.contains(&metric));
        cfg.selected_metrics.retain(|m| *m != metric);
        assert!(!cfg.selected_metrics.contains(&metric));
    }
}
