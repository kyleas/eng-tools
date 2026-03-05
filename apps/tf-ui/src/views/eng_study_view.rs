use std::collections::BTreeMap;

use egui_plot::{Legend, Line, Plot, PlotPoints};
use serde_json::{Map, Value};
use tf_eng::{
    StudyFieldType, StudyPresetDescriptor, StudyRunRequest, StudyTargetDescriptor, StudyTargetKind,
    SweepAxisSpec, list_study_presets, list_study_targets, run_study_from_form,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisMode {
    Values,
    Linspace,
    Logspace,
}

pub struct EngStudyView {
    targets: Vec<StudyTargetDescriptor>,
    presets: Vec<StudyPresetDescriptor>,

    kind: StudyTargetKind,
    selected_target_idx: usize,
    selected_preset_idx: usize,

    axis_mode: AxisMode,
    axis_start: f64,
    axis_end: f64,
    axis_count: usize,
    axis_values_csv: String,

    sweep_field: String,
    output_key: String,
    field_values: BTreeMap<String, Value>,

    result: Option<tf_eng::StudyResult>,
    last_error: Option<String>,
}

impl Default for EngStudyView {
    fn default() -> Self {
        let targets = list_study_targets().unwrap_or_default();
        let presets = list_study_presets().unwrap_or_default();
        let mut s = Self {
            targets,
            presets,
            kind: StudyTargetKind::Equation,
            selected_target_idx: 0,
            selected_preset_idx: 0,
            axis_mode: AxisMode::Linspace,
            axis_start: 0.2,
            axis_end: 3.0,
            axis_count: 20,
            axis_values_csv: "0.2,0.5,1.0,2.0,3.0".to_string(),
            sweep_field: String::new(),
            output_key: String::new(),
            field_values: BTreeMap::new(),
            result: None,
            last_error: None,
        };
        s.reset_for_target();
        s
    }
}

impl EngStudyView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Eng Study Explorer");
            ui.label("Schema-driven study forms from tf-eng target descriptors.");

            ui.separator();
            self.render_kind_selector(ui);

            ui.separator();
            self.render_target_selector(ui);
            self.render_preset_selector(ui);

            if let Some(target) = self.current_target() {
                ui.label(format!("{}", target.description));
                ui.separator();
                self.render_sweep_controls(ui, &target);
                self.render_field_controls(ui, &target);
                self.render_output_selector(ui, &target);
            }

            if ui.button("Run Study").clicked() {
                match self.run_selected() {
                    Ok(result) => {
                        self.last_error = None;
                        self.result = Some(result);
                    }
                    Err(e) => {
                        self.result = None;
                        self.last_error = Some(e);
                    }
                }
            }

            if let Some(err) = &self.last_error {
                ui.separator();
                ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
            }

            if let Some(result) = &self.result {
                self.show_result(ui, result);
            }
        });
    }

    fn render_kind_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.kind, StudyTargetKind::Equation, "Equation");
            ui.selectable_value(&mut self.kind, StudyTargetKind::Device, "Device");
            ui.selectable_value(&mut self.kind, StudyTargetKind::Workflow, "Workflow");
        });
    }

    fn render_target_selector(&mut self, ui: &mut egui::Ui) {
        let filtered = self.filtered_targets();
        if filtered.is_empty() {
            ui.label("No studyable targets discovered.");
            return;
        }
        self.selected_target_idx = self
            .selected_target_idx
            .min(filtered.len().saturating_sub(1));
        let current_name = filtered[self.selected_target_idx].name.clone();
        egui::ComboBox::from_label("Target")
            .selected_text(current_name)
            .show_ui(ui, |ui| {
                for (i, t) in filtered.iter().enumerate() {
                    if ui
                        .selectable_value(
                            &mut self.selected_target_idx,
                            i,
                            format!("{} ({})", t.name, t.id),
                        )
                        .changed()
                    {
                        self.reset_for_target();
                    }
                }
            });
    }

    fn render_preset_selector(&mut self, ui: &mut egui::Ui) {
        let Some(target) = self.current_target() else {
            return;
        };
        let matching = self
            .presets
            .iter()
            .filter(|p| p.target_kind == target.kind && p.target_id == target.id)
            .cloned()
            .collect::<Vec<_>>();
        if matching.is_empty() {
            return;
        }
        self.selected_preset_idx = self
            .selected_preset_idx
            .min(matching.len().saturating_sub(1));
        egui::ComboBox::from_label("Preset")
            .selected_text(matching[self.selected_preset_idx].name.clone())
            .show_ui(ui, |ui| {
                for (i, p) in matching.iter().enumerate() {
                    if ui
                        .selectable_value(&mut self.selected_preset_idx, i, p.name.clone())
                        .changed()
                    {
                        self.apply_preset(p);
                    }
                }
            });
        if ui.button("Apply preset").clicked() {
            let preset = matching[self.selected_preset_idx].clone();
            self.apply_preset(&preset);
        }
    }

    fn render_sweep_controls(&mut self, ui: &mut egui::Ui, target: &StudyTargetDescriptor) {
        ui.label("Sweep");
        if !target.sweepable_fields.is_empty() {
            if self.sweep_field.is_empty() {
                self.sweep_field = target.sweepable_fields[0].clone();
            }
            egui::ComboBox::from_label("Sweep field")
                .selected_text(self.sweep_field.clone())
                .show_ui(ui, |ui| {
                    for f in &target.sweepable_fields {
                        ui.selectable_value(&mut self.sweep_field, f.clone(), f);
                    }
                });
        }

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.axis_mode, AxisMode::Linspace, "linspace");
            ui.selectable_value(&mut self.axis_mode, AxisMode::Logspace, "logspace");
            ui.selectable_value(&mut self.axis_mode, AxisMode::Values, "values");
        });
        match self.axis_mode {
            AxisMode::Values => {
                ui.text_edit_singleline(&mut self.axis_values_csv);
            }
            AxisMode::Linspace | AxisMode::Logspace => {
                ui.horizontal(|ui| {
                    ui.label("start");
                    ui.add(egui::DragValue::new(&mut self.axis_start));
                    ui.label("end");
                    ui.add(egui::DragValue::new(&mut self.axis_end));
                    ui.label("count");
                    ui.add(egui::DragValue::new(&mut self.axis_count).range(2..=5000));
                });
            }
        }
    }

    fn render_field_controls(&mut self, ui: &mut egui::Ui, target: &StudyTargetDescriptor) {
        ui.separator();
        ui.label("Inputs");
        for field in &target.input_fields {
            let key = field.key.clone();
            if !self.field_values.contains_key(&key) {
                if let Some(v) = &field.default_value {
                    self.field_values.insert(key.clone(), v.clone());
                }
            }
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{}{}",
                    field.label,
                    if field.required { " *" } else { "" }
                ));
                match field.field_type {
                    StudyFieldType::Enum => {
                        let current = self
                            .field_values
                            .get(&key)
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string();
                        let mut next = current.clone();
                        egui::ComboBox::from_id_salt(format!("field_{key}"))
                            .selected_text(if current.is_empty() {
                                "(select)"
                            } else {
                                &current
                            })
                            .show_ui(ui, |ui| {
                                for opt in &field.enum_options {
                                    ui.selectable_value(&mut next, opt.key.clone(), &opt.label);
                                }
                            });
                        self.field_values.insert(key.clone(), Value::from(next));
                    }
                    StudyFieldType::Bool => {
                        let mut b = self
                            .field_values
                            .get(&key)
                            .and_then(Value::as_bool)
                            .unwrap_or(false);
                        ui.checkbox(&mut b, "");
                        self.field_values.insert(key.clone(), Value::from(b));
                    }
                    StudyFieldType::Float => {
                        let mut text = self
                            .field_values
                            .get(&key)
                            .and_then(Value::as_f64)
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        if text.is_empty()
                            && let Some(Value::Number(n)) = field.default_value.as_ref()
                        {
                            text = n.to_string();
                        }
                        if ui.text_edit_singleline(&mut text).changed() {
                            if let Ok(v) = text.trim().parse::<f64>() {
                                self.field_values.insert(key.clone(), Value::from(v));
                            } else if text.trim().is_empty() {
                                self.field_values.remove(&key);
                            }
                        }
                    }
                    StudyFieldType::Int => {
                        let mut text = self
                            .field_values
                            .get(&key)
                            .and_then(Value::as_i64)
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        if ui.text_edit_singleline(&mut text).changed() {
                            if let Ok(v) = text.trim().parse::<i64>() {
                                self.field_values.insert(key.clone(), Value::from(v));
                            } else if text.trim().is_empty() {
                                self.field_values.remove(&key);
                            }
                        }
                    }
                    StudyFieldType::String => {
                        let mut text = self
                            .field_values
                            .get(&key)
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string();
                        if ui.text_edit_singleline(&mut text).changed() {
                            self.field_values.insert(key.clone(), Value::from(text));
                        }
                    }
                }
            });
            ui.small(&field.description);
        }
    }

    fn render_output_selector(&mut self, ui: &mut egui::Ui, target: &StudyTargetDescriptor) {
        ui.separator();
        let plot_outputs = target
            .outputs
            .iter()
            .filter(|o| o.plottable)
            .collect::<Vec<_>>();
        if plot_outputs.is_empty() {
            return;
        }
        if self.output_key.is_empty() {
            self.output_key = target
                .default_output
                .clone()
                .unwrap_or_else(|| plot_outputs[0].key.clone());
        }
        egui::ComboBox::from_label("Output")
            .selected_text(self.output_key.clone())
            .show_ui(ui, |ui| {
                for out in plot_outputs {
                    ui.selectable_value(&mut self.output_key, out.key.clone(), out.label.clone());
                }
            });
    }

    fn run_selected(&self) -> Result<tf_eng::StudyResult, String> {
        let Some(target) = self.current_target() else {
            return Err("No target selected".to_string());
        };

        let axis = match self.axis_mode {
            AxisMode::Linspace => SweepAxisSpec::Linspace {
                start: self.axis_start,
                end: self.axis_end,
                count: self.axis_count,
            },
            AxisMode::Logspace => SweepAxisSpec::Logspace {
                start: self.axis_start,
                end: self.axis_end,
                count: self.axis_count,
            },
            AxisMode::Values => {
                let values = self
                    .axis_values_csv
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        s.parse::<f64>()
                            .map_err(|_| format!("invalid axis value '{s}'"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                SweepAxisSpec::Values(values)
            }
        };

        let inputs = Map::from_iter(
            self.field_values
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        run_study_from_form(StudyRunRequest {
            target_kind: target.kind.clone(),
            target_id: target.id.clone(),
            sweep_field: self.sweep_field.clone(),
            axis,
            inputs,
            output_key: Some(self.output_key.clone()),
        })
        .map_err(|e| e.to_string())
    }

    fn show_result(&self, ui: &mut egui::Ui, result: &tf_eng::StudyResult) {
        ui.separator();
        ui.label(format!(
            "Study {} | ok={} fail={} | output={}",
            result.meta.study_id,
            result.meta.n_ok,
            result.meta.n_fail,
            result.meta.selected_output_key
        ));
        if !result.meta.warnings_summary.is_empty() {
            ui.label(format!(
                "Warnings: {}",
                result.meta.warnings_summary.join(" | ")
            ));
        }

        if let Some(series) = result.series.first() {
            let points = series
                .x
                .iter()
                .zip(series.y.iter())
                .filter_map(|(x, y)| y.is_finite().then_some([*x, *y]))
                .collect::<Vec<[f64; 2]>>();
            let line = Line::new(PlotPoints::from(points)).name(series.name.clone());
            Plot::new("eng_study_plot")
                .height(320.0)
                .legend(Legend::default())
                .x_axis_label(series.x_label.clone())
                .y_axis_label(series.y_label.clone())
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        }

        ui.separator();
        ui.collapsing("Table preview", |ui| {
            let max_rows = result.table.rows.len().min(12);
            for row in result.table.rows.iter().take(max_rows) {
                ui.monospace(row.join(" | "));
            }
            if result.table.rows.len() > max_rows {
                ui.small(format!(
                    "... {} more rows",
                    result.table.rows.len() - max_rows
                ));
            }
        });
    }

    fn filtered_targets(&self) -> Vec<StudyTargetDescriptor> {
        self.targets
            .iter()
            .filter(|t| t.kind == self.kind)
            .cloned()
            .collect()
    }

    fn current_target(&self) -> Option<StudyTargetDescriptor> {
        let filtered = self.filtered_targets();
        filtered.get(self.selected_target_idx).cloned()
    }

    fn reset_for_target(&mut self) {
        self.field_values.clear();
        if let Some(target) = self.current_target() {
            self.sweep_field = target
                .plot_default
                .as_ref()
                .map(|p| p.x_field.clone())
                .or_else(|| target.sweepable_fields.first().cloned())
                .unwrap_or_default();
            self.output_key = target
                .plot_default
                .as_ref()
                .map(|p| p.y_output.clone())
                .or_else(|| target.default_output.clone())
                .unwrap_or_default();
            for f in &target.input_fields {
                if let Some(v) = &f.default_value {
                    self.field_values.insert(f.key.clone(), v.clone());
                }
            }
        }
    }

    fn apply_preset(&mut self, preset: &StudyPresetDescriptor) {
        self.sweep_field = preset.sweep_field.clone();
        self.output_key = preset.output_key.clone();
        self.field_values = preset
            .input_overrides
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        match preset.axis {
            SweepAxisSpec::Linspace { start, end, count } => {
                self.axis_mode = AxisMode::Linspace;
                self.axis_start = start;
                self.axis_end = end;
                self.axis_count = count;
            }
            SweepAxisSpec::Logspace { start, end, count } => {
                self.axis_mode = AxisMode::Logspace;
                self.axis_start = start;
                self.axis_end = end;
                self.axis_count = count;
            }
            SweepAxisSpec::Values(ref values) => {
                self.axis_mode = AxisMode::Values;
                self.axis_values_csv = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
            }
        }
    }
}
