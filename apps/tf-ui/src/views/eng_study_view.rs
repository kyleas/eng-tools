use std::collections::BTreeMap;

use egui_plot::{Legend, Line, Plot, PlotPoints};
use serde_json::{Map, Value};
use tf_eng::{
    SingleSolveResult, SolveRowState, SolveValidation, StudyFieldType, StudyPresetDescriptor,
    StudyResult, StudyRunRequest, StudyTargetDescriptor, StudyTargetKind, SweepAxisSpec,
    describe_device_target, describe_equation_target, describe_workflow_target,
    evaluate_single_solve, list_study_presets, list_study_targets, run_study_from_form,
    validate_single_solve,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkbenchMode {
    Solve,
    Study,
    Reference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisMode {
    Values,
    Linspace,
    Logspace,
}

#[derive(Debug, Clone)]
struct SolveRow {
    kind: StudyTargetKind,
    expanded: bool,
    freeze: bool,
    target_search: String,
    target_id: String,
    output_search: String,
    output_key: String,
    input_text: BTreeMap<String, String>,
    validation: Option<SolveValidation>,
    last_signature: Option<String>,
    result: Option<SingleSolveResult>,
    last_error: Option<String>,
}

impl SolveRow {
    fn new(kind: StudyTargetKind) -> Self {
        Self {
            kind,
            expanded: true,
            freeze: false,
            target_search: String::new(),
            target_id: String::new(),
            output_search: String::new(),
            output_key: String::new(),
            input_text: BTreeMap::new(),
            validation: None,
            last_signature: None,
            result: None,
            last_error: None,
        }
    }
}

pub struct EngStudyView {
    targets: Vec<StudyTargetDescriptor>,
    presets: Vec<StudyPresetDescriptor>,
    mode: WorkbenchMode,

    solve_rows: Vec<SolveRow>,

    study_kind: StudyTargetKind,
    study_target_search: String,
    study_target_id: String,
    study_preset_id: String,
    axis_mode: AxisMode,
    axis_start: f64,
    axis_end: f64,
    axis_count: usize,
    axis_values_csv: String,
    sweep_field_search: String,
    sweep_field: String,
    output_search: String,
    output_key: String,
    field_values: BTreeMap<String, Value>,
    result: Option<StudyResult>,
    last_error: Option<String>,

    reference_kind: StudyTargetKind,
    reference_target_search: String,
    reference_target_id: String,

    recents: Vec<String>,
    favorites: Vec<String>,
}

impl Default for EngStudyView {
    fn default() -> Self {
        let targets = list_study_targets().unwrap_or_default();
        let presets = list_study_presets().unwrap_or_default();
        let mut s = Self {
            targets,
            presets,
            mode: WorkbenchMode::Solve,
            solve_rows: vec![SolveRow::new(StudyTargetKind::Equation)],
            study_kind: StudyTargetKind::Equation,
            study_target_search: String::new(),
            study_target_id: String::new(),
            study_preset_id: String::new(),
            axis_mode: AxisMode::Linspace,
            axis_start: 0.2,
            axis_end: 3.0,
            axis_count: 20,
            axis_values_csv: "0.2,0.5,1.0,2.0,3.0".to_string(),
            sweep_field_search: String::new(),
            sweep_field: String::new(),
            output_search: String::new(),
            output_key: String::new(),
            field_values: BTreeMap::new(),
            result: None,
            last_error: None,
            reference_kind: StudyTargetKind::Equation,
            reference_target_search: String::new(),
            reference_target_id: String::new(),
            recents: Vec::new(),
            favorites: Vec::new(),
        };
        s.ensure_study_target();
        s.ensure_reference_target();
        s.ensure_solve_rows();
        s
    }
}

impl EngStudyView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Eng Workbench");
            ui.label(
                "Metadata-driven solve, study, and reference over eng equations/devices/workflows.",
            );

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, WorkbenchMode::Solve, "Solve");
                ui.selectable_value(&mut self.mode, WorkbenchMode::Study, "Study");
                ui.selectable_value(&mut self.mode, WorkbenchMode::Reference, "Reference");
            });
            ui.separator();

            match self.mode {
                WorkbenchMode::Solve => self.show_solve_mode(ui),
                WorkbenchMode::Study => self.show_study_mode(ui),
                WorkbenchMode::Reference => self.show_reference_mode(ui),
            }
        });
    }

    fn show_solve_mode(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Add row").clicked() {
                self.solve_rows
                    .push(SolveRow::new(StudyTargetKind::Equation));
                self.ensure_solve_rows();
            }
            if ui.button("Clear all").clicked() {
                self.solve_rows = vec![SolveRow::new(StudyTargetKind::Equation)];
                self.ensure_solve_rows();
            }
        });

        let mut delete_idx = None;
        let mut duplicate_idx = None;

        for i in 0..self.solve_rows.len() {
            let mut row = self.solve_rows[i].clone();
            let mut recent_to_add: Option<String> = None;
            ui.separator();
            ui.group(|ui| {
                let mut changed_target = false;
                ui.horizontal(|ui| {
                    ui.heading(format!("Solve Row {}", i + 1));
                    ui.checkbox(&mut row.expanded, "expanded");
                    ui.checkbox(&mut row.freeze, "freeze");
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut row.kind, StudyTargetKind::Equation, "Equation");
                    ui.selectable_value(&mut row.kind, StudyTargetKind::Device, "Device");
                    ui.selectable_value(&mut row.kind, StudyTargetKind::Workflow, "Workflow");
                });

                let all_targets = self.targets_of_kind(&row.kind, "");
                let (selected_id, was_changed) = Self::render_target_picker(
                    ui,
                    &all_targets,
                    &mut row.target_search,
                    &row.target_id,
                    "solve_row_target",
                    i,
                );
                if was_changed {
                    row.target_id = selected_id;
                    row.input_text.clear();
                    row.output_key.clear();
                    row.validation = None;
                    row.last_signature = None;
                    row.result = None;
                    row.last_error = None;
                    changed_target = true;
                } else if row.target_id.is_empty() {
                    row.target_id = selected_id;
                    changed_target = true;
                }

                let descriptor = self.descriptor_for(row.kind.clone(), &row.target_id);
                if let Some(target) = descriptor {
                    if changed_target {
                        Self::seed_solve_defaults(
                            &target,
                            &mut row.input_text,
                            &mut row.output_key,
                        );
                    }
                    Self::render_reference_panel(ui, &target, false);
                    if row.expanded {
                        Self::render_solve_input_controls(ui, &target, &mut row.input_text);
                    }

                    if target.kind == StudyTargetKind::Equation {
                        if let Some(implied) =
                            Self::implied_equation_target_text(&target, &row.input_text)
                        {
                            ui.small(format!("Implied target (one unknown): {implied}"));
                        } else {
                            ui.small(
                                "Implied target available when exactly one variable is left blank.",
                            );
                        }
                    }

                    Self::render_output_picker(
                        ui,
                        &target,
                        &mut row.output_search,
                        &mut row.output_key,
                        true,
                        i,
                        false,
                    );

                    let evaluation = if row.freeze {
                        validate_single_solve(
                            target.kind.clone(),
                            &target.id,
                            &row.input_text,
                            if row.output_key.is_empty() {
                                None
                            } else {
                                Some(row.output_key.as_str())
                            },
                        )
                        .map(|v| (v, None))
                    } else {
                        evaluate_single_solve(
                            target.kind.clone(),
                            &target.id,
                            &row.input_text,
                            if row.output_key.is_empty() {
                                None
                            } else {
                                Some(row.output_key.as_str())
                            },
                        )
                    };

                    if let Ok((validation, maybe_result)) = evaluation {
                        row.validation = Some(validation.clone());
                        if let Some(res) = maybe_result {
                            let signature = format!(
                                "{}|{}|{}|{}",
                                target.kind.kind_name(),
                                target.id,
                                validation.output_key.clone().unwrap_or_default(),
                                serde_json::to_string(&validation.normalized_inputs)
                                    .unwrap_or_default()
                            );
                            row.last_error = None;
                            row.result = Some(res);
                            row.last_signature = Some(signature);
                            recent_to_add =
                                Some(format!("{}:{}", target.kind.kind_name(), target.id));
                        } else {
                            row.result = None;
                        }
                    } else if let Err(e) = evaluation {
                        row.validation = None;
                        row.result = None;
                        row.last_error = Some(e.to_string());
                    }

                    if let Some(v) = &row.validation {
                        Self::render_row_validation(ui, v);
                        ui.collapsing("Debug request", |ui| {
                            if let Some(preview) = &v.request_preview {
                                ui.monospace(
                                    serde_json::to_string_pretty(preview)
                                        .unwrap_or_else(|_| preview.to_string()),
                                );
                            } else {
                                ui.small("No executable request yet.");
                            }
                        });
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Duplicate").clicked() {
                            duplicate_idx = Some(i);
                        }
                        if ui.button("Clear row").clicked() {
                            row.input_text.clear();
                            row.validation = None;
                            row.last_signature = None;
                            row.result = None;
                            row.last_error = None;
                        }
                        if ui.button("Delete row").clicked() {
                            delete_idx = Some(i);
                        }
                    });

                    if let Some(err) = &row.last_error {
                        ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
                    }
                    if let Some(res) = &row.result {
                        Self::render_single_result(ui, res);
                        Self::render_copy_actions(ui, &target, res, row.validation.as_ref());
                    }
                } else {
                    ui.label("No target selected.");
                }
            });
            self.solve_rows[i] = row;
            if let Some(recent) = recent_to_add {
                self.push_recent(&recent);
            }
        }

        if let Some(i) = duplicate_idx
            && let Some(row) = self.solve_rows.get(i).cloned()
        {
            self.solve_rows.insert(i + 1, row);
        }
        if let Some(i) = delete_idx {
            if self.solve_rows.len() > 1 {
                self.solve_rows.remove(i);
            } else {
                self.solve_rows[0] = SolveRow::new(StudyTargetKind::Equation);
            }
        }
        self.ensure_solve_rows();
    }

    fn show_study_mode(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.study_kind, StudyTargetKind::Equation, "Equation");
            ui.selectable_value(&mut self.study_kind, StudyTargetKind::Device, "Device");
            ui.selectable_value(&mut self.study_kind, StudyTargetKind::Workflow, "Workflow");
        });

        let all_targets = self.targets_of_kind(&self.study_kind, "");
        let (selected, changed) = Self::render_target_picker(
            ui,
            &all_targets,
            &mut self.study_target_search,
            &self.study_target_id,
            "study_target",
            0,
        );
        if changed || self.study_target_id.is_empty() {
            self.study_target_id = selected;
            self.reset_for_study_target();
        }

        self.render_study_preset_selector(ui);

        if let Some(target) = self.current_study_target() {
            Self::render_reference_panel(ui, &target, true);
            self.render_sweep_controls(ui, &target);
            Self::render_field_controls(ui, &target, &mut self.field_values);
            Self::render_output_picker(
                ui,
                &target,
                &mut self.output_search,
                &mut self.output_key,
                false,
                0,
                true,
            );

            if ui.button("Run Study").clicked() {
                match self.run_selected() {
                    Ok(result) => {
                        self.last_error = None;
                        self.result = Some(result);
                        self.push_recent(&format!("{}:{}", target.kind.kind_name(), target.id));
                    }
                    Err(e) => {
                        self.result = None;
                        self.last_error = Some(e);
                    }
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
    }

    fn show_reference_mode(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.reference_kind,
                StudyTargetKind::Equation,
                "Equation",
            );
            ui.selectable_value(&mut self.reference_kind, StudyTargetKind::Device, "Device");
            ui.selectable_value(
                &mut self.reference_kind,
                StudyTargetKind::Workflow,
                "Workflow",
            );
        });

        let all_targets = self.targets_of_kind(&self.reference_kind, "");
        let (selected, changed) = Self::render_target_picker(
            ui,
            &all_targets,
            &mut self.reference_target_search,
            &self.reference_target_id,
            "reference_target",
            0,
        );
        if changed || self.reference_target_id.is_empty() {
            self.reference_target_id = selected;
        }

        if let Some(target) = self.current_reference_target() {
            Self::render_reference_panel(ui, &target, false);
            ui.separator();
            ui.label("Quick actions");
            ui.horizontal(|ui| {
                if ui.button("Pin favorite").clicked()
                    && !self.favorites.iter().any(|f| f == &target.id)
                {
                    self.favorites.push(target.id.clone());
                }
                if ui.button("Copy key").clicked() {
                    ui.ctx().copy_text(target.id.clone());
                }
            });

            if !self.favorites.is_empty() {
                ui.separator();
                ui.label("Favorites");
                for f in self.favorites.clone() {
                    ui.horizontal(|ui| {
                        ui.monospace(f.clone());
                        if ui.button("Use").clicked() {
                            self.reference_target_id = f.clone();
                        }
                    });
                }
            }
            if !self.recents.is_empty() {
                ui.separator();
                ui.label("Recent");
                for r in self.recents.iter().rev().take(10) {
                    ui.monospace(r);
                }
            }
        }
    }

    fn render_target_picker(
        ui: &mut egui::Ui,
        all_targets: &[StudyTargetDescriptor],
        search: &mut String,
        selected_id: &str,
        id_prefix: &str,
        row_idx: usize,
    ) -> (String, bool) {
        let mut choices = filter_targets(all_targets, search);
        if choices.is_empty() {
            ui.label("No targets discovered.");
            return (String::new(), false);
        }

        let mut selected = selected_id.to_string();
        if selected.is_empty() || !choices.iter().any(|t| t.id == selected) {
            selected = choices[0].id.clone();
        }

        ui.horizontal(|ui| {
            ui.label("Find target");
            ui.text_edit_singleline(search);
        });

        choices = filter_targets(all_targets, search);
        if !choices.iter().any(|t| t.id == selected) {
            selected = choices
                .first()
                .map(|t| t.id.clone())
                .unwrap_or_else(String::new);
        }

        let mut changed = false;
        let selected_text = choices
            .iter()
            .find(|t| t.id == selected)
            .map(|t| format!("{} ({})", t.name, t.id))
            .unwrap_or_else(|| "(select)".to_string());

        egui::ComboBox::from_id_salt(format!("{id_prefix}_{row_idx}"))
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for t in &choices {
                    if ui
                        .selectable_label(selected == t.id, format!("{} ({})", t.name, t.id))
                        .clicked()
                    {
                        selected = t.id.clone();
                        changed = true;
                    }
                }
            });
        (selected, changed)
    }

    fn render_study_preset_selector(&mut self, ui: &mut egui::Ui) {
        let Some(target) = self.current_study_target() else {
            return;
        };
        let matching = self
            .presets
            .iter()
            .filter(|p| p.target_kind == target.kind && p.target_id == target.id)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            return;
        }

        if self.study_preset_id.is_empty() || !matching.iter().any(|p| p.id == self.study_preset_id)
        {
            self.study_preset_id = matching[0].id.clone();
        }

        egui::ComboBox::from_label("Preset")
            .selected_text(
                matching
                    .iter()
                    .find(|p| p.id == self.study_preset_id)
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "(select)".to_string()),
            )
            .show_ui(ui, |ui| {
                for p in &matching {
                    ui.selectable_value(&mut self.study_preset_id, p.id.clone(), &p.name);
                }
            });
        if ui.button("Apply preset").clicked()
            && let Some(p) = matching.iter().find(|p| p.id == self.study_preset_id)
        {
            let preset = (*p).clone();
            self.apply_preset(&preset);
        }
    }

    fn render_sweep_controls(&mut self, ui: &mut egui::Ui, target: &StudyTargetDescriptor) {
        ui.separator();
        ui.label("Sweep");
        if !target.sweepable_fields.is_empty() {
            ui.horizontal(|ui| {
                ui.label("Find sweep field");
                ui.text_edit_singleline(&mut self.sweep_field_search);
            });
            let filtered = target
                .sweepable_fields
                .iter()
                .filter(|f| {
                    self.sweep_field_search.trim().is_empty()
                        || contains_ci(f, &self.sweep_field_search)
                })
                .cloned()
                .collect::<Vec<_>>();
            if self.sweep_field.is_empty() {
                self.sweep_field = target
                    .plot_default
                    .as_ref()
                    .map(|p| p.x_field.clone())
                    .or_else(|| filtered.first().cloned())
                    .unwrap_or_default();
            }
            egui::ComboBox::from_label("Sweep field")
                .selected_text(self.sweep_field.clone())
                .show_ui(ui, |ui| {
                    for f in filtered {
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

    fn render_field_controls(
        ui: &mut egui::Ui,
        target: &StudyTargetDescriptor,
        fields: &mut BTreeMap<String, Value>,
    ) {
        ui.separator();
        ui.label("Inputs");
        for field in &target.input_fields {
            let key = field.key.clone();
            if !fields.contains_key(&key)
                && let Some(v) = &field.default_value
            {
                fields.insert(key.clone(), v.clone());
            }

            let has_value = fields.contains_key(&key);
            let color = if has_value {
                egui::Color32::LIGHT_GREEN
            } else if field.required {
                egui::Color32::LIGHT_RED
            } else {
                egui::Color32::GRAY
            };

            ui.horizontal(|ui| {
                ui.colored_label(
                    color,
                    format!("{}{}", field.label, if field.required { " *" } else { "" }),
                );
                match field.field_type {
                    StudyFieldType::Enum => {
                        let current = fields
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
                        if next.trim().is_empty() {
                            fields.remove(&key);
                        } else {
                            fields.insert(key.clone(), Value::from(next));
                        }
                    }
                    StudyFieldType::Bool => {
                        let mut b = fields.get(&key).and_then(Value::as_bool).unwrap_or(false);
                        ui.checkbox(&mut b, "");
                        fields.insert(key.clone(), Value::from(b));
                    }
                    StudyFieldType::Float => {
                        let mut text = fields
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
                                fields.insert(key.clone(), Value::from(v));
                            } else if text.trim().is_empty() {
                                fields.remove(&key);
                            }
                        }
                    }
                    StudyFieldType::Int => {
                        let mut text = fields
                            .get(&key)
                            .and_then(Value::as_i64)
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        if ui.text_edit_singleline(&mut text).changed() {
                            if let Ok(v) = text.trim().parse::<i64>() {
                                fields.insert(key.clone(), Value::from(v));
                            } else if text.trim().is_empty() {
                                fields.remove(&key);
                            }
                        }
                    }
                    StudyFieldType::String => {
                        let mut text = fields
                            .get(&key)
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string();
                        if ui.text_edit_singleline(&mut text).changed() {
                            if text.trim().is_empty() {
                                fields.remove(&key);
                            } else {
                                fields.insert(key.clone(), Value::from(text));
                            }
                        }
                    }
                }
            });

            let mut meta = field.description.clone();
            if let Some(unit) = &field.unit {
                meta.push_str(&format!(" | unit: {unit}"));
            }
            if !field.enum_options.is_empty() {
                meta.push_str(" | enum");
            }
            ui.small(meta);
        }
    }

    fn render_solve_input_controls(
        ui: &mut egui::Ui,
        target: &StudyTargetDescriptor,
        fields: &mut BTreeMap<String, String>,
    ) {
        ui.separator();
        ui.label("Inputs (number or eng unit string)");
        for field in &target.input_fields {
            let entry = fields.entry(field.key.clone()).or_insert_with(|| {
                field
                    .default_value
                    .as_ref()
                    .map(value_to_display)
                    .unwrap_or_default()
            });
            ui.horizontal(|ui| {
                let mut label = field.label.clone();
                if field.required {
                    label.push_str(" *");
                }
                ui.label(label);
                ui.text_edit_singleline(entry);
            });
            let mut meta = field.description.clone();
            if let Some(u) = &field.unit {
                meta.push_str(&format!(" | default unit: {u}"));
            }
            ui.small(meta);
        }
    }

    fn render_row_validation(ui: &mut egui::Ui, validation: &SolveValidation) {
        let (label, color) = match validation.state {
            SolveRowState::Validating => ("validating", egui::Color32::LIGHT_BLUE),
            SolveRowState::Ready => ("ready", egui::Color32::LIGHT_GREEN),
            SolveRowState::Success => ("success", egui::Color32::LIGHT_GREEN),
            SolveRowState::Invalid | SolveRowState::Error => ("invalid", egui::Color32::LIGHT_RED),
            SolveRowState::Ambiguous => ("ambiguous", egui::Color32::YELLOW),
            SolveRowState::Unsupported => ("unsupported", egui::Color32::YELLOW),
            SolveRowState::Incomplete => ("incomplete", egui::Color32::GRAY),
        };
        ui.colored_label(color, format!("State: {label}"));
        if let Some(out) = &validation.output_key {
            ui.small(format!("Output: {out}"));
        }
        if !validation.missing_required.is_empty() {
            ui.small(format!(
                "Missing required: {}",
                validation.missing_required.join(", ")
            ));
        }
        for field in &validation.fields {
            if !field.valid && !field.empty {
                ui.colored_label(
                    egui::Color32::LIGHT_RED,
                    format!(
                        "{}: {}",
                        field.key,
                        field.message.clone().unwrap_or_default()
                    ),
                );
            }
        }
        for reason in &validation.blocking_reasons {
            ui.colored_label(egui::Color32::LIGHT_RED, reason);
        }
    }

    fn render_output_picker(
        ui: &mut egui::Ui,
        target: &StudyTargetDescriptor,
        output_search: &mut String,
        output_key: &mut String,
        allow_auto_equation_target: bool,
        row_idx: usize,
        study_mode: bool,
    ) {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Find output");
            ui.text_edit_singleline(output_search);
        });

        let mut outputs = target.outputs.clone();
        if study_mode {
            outputs.retain(|o| o.plottable);
        }
        outputs.retain(|o| {
            output_search.trim().is_empty()
                || contains_ci(&o.key, output_search)
                || contains_ci(&o.label, output_search)
        });
        if outputs.is_empty() {
            ui.label("No outputs available.");
            return;
        }

        if output_key.is_empty() {
            *output_key = target
                .default_output
                .clone()
                .unwrap_or_else(|| outputs[0].key.clone());
        }

        let selected_text = if allow_auto_equation_target
            && target.kind == StudyTargetKind::Equation
            && output_key.is_empty()
        {
            "(auto missing variable)".to_string()
        } else {
            outputs
                .iter()
                .find(|o| o.key == *output_key)
                .map(|o| format!("{} ({})", o.label, o.key))
                .unwrap_or_else(|| output_key.clone())
        };

        egui::ComboBox::from_id_salt(format!("output_picker_{}_{}", target.id, row_idx))
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                if allow_auto_equation_target && target.kind == StudyTargetKind::Equation {
                    if ui
                        .selectable_label(output_key.is_empty(), "(auto missing variable)")
                        .clicked()
                    {
                        output_key.clear();
                    }
                }
                for out in outputs {
                    if ui
                        .selectable_label(
                            *output_key == out.key,
                            format!("{} ({})", out.label, out.key),
                        )
                        .clicked()
                    {
                        *output_key = out.key;
                    }
                }
            });
    }

    fn render_reference_panel(ui: &mut egui::Ui, target: &StudyTargetDescriptor, compact: bool) {
        ui.separator();
        ui.heading(&target.name);
        ui.small(format!("id: {}", target.id));
        ui.label(&target.description);
        if let Some(category) = &target.category {
            ui.small(format!("Category: {category}"));
        }
        // Egui does not provide stable native MathJax/LaTeX rendering. Prefer readable
        // unicode/ascii equation text by default and keep latex available in details.
        if let Some(display) = &target.display_unicode {
            ui.monospace(format!("Equation: {display}"));
        } else if let Some(display) = &target.display_ascii {
            ui.monospace(format!("Equation: {display}"));
        }
        if let Some(latex) = &target.display_latex {
            ui.collapsing("LaTeX", |ui| {
                ui.monospace(latex);
            });
        }
        if !target.branch_options.is_empty() {
            ui.small(format!("Branches: {}", target.branch_options.join(", ")));
        }
        if let Some(default_output) = &target.default_output {
            ui.small(format!("Default output: {default_output}"));
        }
        if compact {
            return;
        }

        ui.collapsing("Inputs", |ui| {
            for f in &target.input_fields {
                let req = if f.required { "required" } else { "optional" };
                let unit = f.unit.clone().unwrap_or_else(|| "-".to_string());
                ui.monospace(format!(
                    "{} | {} | {} | unit: {}",
                    f.key, f.label, req, unit
                ));
            }
        });
        ui.collapsing("Outputs", |ui| {
            for o in &target.outputs {
                let plot = if o.plottable { "plot" } else { "meta" };
                ui.monospace(format!("{} | {} | {}", o.key, o.label, plot));
            }
        });
    }

    fn render_single_result(ui: &mut egui::Ui, res: &SingleSolveResult) {
        ui.separator();
        ui.label(format!("Output: {}", res.output_key));
        let mut line = format!("Value: {}", value_to_display(&res.value));
        if let Some(unit) = &res.unit {
            line.push_str(&format!(" {unit}"));
        }
        ui.monospace(line);
        if let Some(path) = &res.path_text {
            ui.collapsing("Path / diagnostics", |ui| {
                ui.monospace(path);
            });
        }
        if !res.warnings.is_empty() {
            ui.label(format!("Warnings: {}", res.warnings.join(" | ")));
        }
    }

    fn render_copy_actions(
        ui: &mut egui::Ui,
        target: &StudyTargetDescriptor,
        res: &SingleSolveResult,
        validation: Option<&SolveValidation>,
    ) {
        ui.horizontal(|ui| {
            if ui.button("Copy target key").clicked() {
                ui.ctx().copy_text(target.id.clone());
            }
            if ui.button("Copy output value").clicked() {
                let mut out = value_to_display(&res.value);
                if let Some(unit) = &res.unit {
                    out.push_str(&format!(" {unit}"));
                }
                ui.ctx().copy_text(out);
            }
            if ui.button("Copy display").clicked() {
                let display = target
                    .display_unicode
                    .clone()
                    .or(target.display_ascii.clone())
                    .or(target.display_latex.clone())
                    .unwrap_or_default();
                ui.ctx().copy_text(display);
            }
            if ui.button("Copy normalized inputs").clicked()
                && let Some(v) = validation
            {
                let text = serde_json::to_string_pretty(&v.normalized_inputs)
                    .unwrap_or_else(|_| "{}".to_string());
                ui.ctx().copy_text(text);
            }
        });
    }

    fn run_selected(&self) -> Result<StudyResult, String> {
        let Some(target) = self.current_study_target() else {
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

    fn show_result(&self, ui: &mut egui::Ui, result: &StudyResult) {
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

    fn targets_of_kind(&self, kind: &StudyTargetKind, search: &str) -> Vec<StudyTargetDescriptor> {
        self.targets
            .iter()
            .filter(|t| &t.kind == kind)
            .filter(|t| {
                search.trim().is_empty()
                    || contains_ci(&t.name, search)
                    || contains_ci(&t.id, search)
                    || t.category
                        .as_ref()
                        .map(|c| contains_ci(c, search))
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    fn descriptor_for(&self, kind: StudyTargetKind, id: &str) -> Option<StudyTargetDescriptor> {
        self.targets
            .iter()
            .find(|t| t.kind == kind && t.id == id)
            .cloned()
            .or_else(|| match kind {
                StudyTargetKind::Equation => describe_equation_target(id).ok(),
                StudyTargetKind::Device => describe_device_target(id).ok(),
                StudyTargetKind::Workflow => describe_workflow_target(id).ok(),
            })
    }

    fn current_study_target(&self) -> Option<StudyTargetDescriptor> {
        self.descriptor_for(self.study_kind.clone(), &self.study_target_id)
    }

    fn current_reference_target(&self) -> Option<StudyTargetDescriptor> {
        self.descriptor_for(self.reference_kind.clone(), &self.reference_target_id)
    }

    fn ensure_study_target(&mut self) {
        if self.study_target_id.is_empty()
            && let Some(t) = self.targets_of_kind(&self.study_kind, "").first()
        {
            self.study_target_id = t.id.clone();
        }
    }

    fn ensure_reference_target(&mut self) {
        if self.reference_target_id.is_empty()
            && let Some(t) = self.targets_of_kind(&self.reference_kind, "").first()
        {
            self.reference_target_id = t.id.clone();
        }
    }

    fn ensure_solve_rows(&mut self) {
        for i in 0..self.solve_rows.len() {
            if self.solve_rows[i].target_id.is_empty()
                && let Some(t) = self.targets_of_kind(&self.solve_rows[i].kind, "").first()
            {
                self.solve_rows[i].target_id = t.id.clone();
                if let Some(d) = self.descriptor_for(
                    self.solve_rows[i].kind.clone(),
                    &self.solve_rows[i].target_id,
                ) {
                    let row = &mut self.solve_rows[i];
                    Self::seed_solve_defaults(&d, &mut row.input_text, &mut row.output_key);
                }
            }
        }
    }

    fn seed_solve_defaults(
        target: &StudyTargetDescriptor,
        input_text: &mut BTreeMap<String, String>,
        output_key: &mut String,
    ) {
        input_text.clear();
        for f in &target.input_fields {
            if let Some(v) = &f.default_value {
                input_text.insert(f.key.clone(), value_to_display(v));
            }
        }
        *output_key = target.default_output.clone().unwrap_or_default();
    }

    fn reset_for_study_target(&mut self) {
        self.field_values.clear();
        self.sweep_field_search.clear();
        self.output_search.clear();
        if let Some(target) = self.current_study_target() {
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

    fn implied_equation_target_text(
        target: &StudyTargetDescriptor,
        values: &BTreeMap<String, String>,
    ) -> Option<String> {
        if target.kind != StudyTargetKind::Equation {
            return None;
        }
        let missing = target
            .input_fields
            .iter()
            .filter(|f| f.key != "branch")
            .filter(|f| {
                values
                    .get(&f.key)
                    .map(|v| v.trim().is_empty())
                    .unwrap_or(true)
            })
            .map(|f| f.key.clone())
            .collect::<Vec<_>>();
        (missing.len() == 1).then(|| missing[0].clone())
    }

    fn push_recent(&mut self, item: &str) {
        if let Some(idx) = self.recents.iter().position(|r| r == item) {
            self.recents.remove(idx);
        }
        self.recents.push(item.to_string());
        if self.recents.len() > 25 {
            self.recents.remove(0);
        }
    }
}

trait KindName {
    fn kind_name(&self) -> &'static str;
}

impl KindName for StudyTargetKind {
    fn kind_name(&self) -> &'static str {
        match self {
            StudyTargetKind::Equation => "equation",
            StudyTargetKind::Device => "device",
            StudyTargetKind::Workflow => "workflow",
        }
    }
}

fn contains_ci(hay: &str, needle: &str) -> bool {
    hay.to_ascii_lowercase()
        .contains(&needle.trim().to_ascii_lowercase())
}

fn filter_targets(
    all_targets: &[StudyTargetDescriptor],
    search: &str,
) -> Vec<StudyTargetDescriptor> {
    all_targets
        .iter()
        .filter(|t| {
            search.trim().is_empty()
                || contains_ci(&t.name, search)
                || contains_ci(&t.id, search)
                || t.category
                    .as_ref()
                    .map(|c| contains_ci(c, search))
                    .unwrap_or(false)
        })
        .cloned()
        .collect()
}

fn value_to_display(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_solve_row_is_expanded_by_default() {
        let row = SolveRow::new(StudyTargetKind::Equation);
        assert!(row.expanded);
        assert!(!row.freeze);
    }

    #[test]
    fn search_filter_matches_case_insensitive_substrings() {
        let targets = vec![StudyTargetDescriptor {
            id: "compressible.isentropic_pressure_ratio".to_string(),
            kind: StudyTargetKind::Equation,
            name: "Isentropic Pressure Ratio".to_string(),
            description: String::new(),
            category: Some("compressible".to_string()),
            studyable: true,
            input_fields: vec![],
            sweepable_fields: vec![],
            outputs: vec![],
            default_output: None,
            plot_default: None,
            display_latex: None,
            display_unicode: None,
            display_ascii: None,
            branch_options: vec![],
        }];
        let out = filter_targets(&targets, "PRESSURE");
        assert_eq!(out.len(), 1);
        let out = filter_targets(&targets, "compressible");
        assert_eq!(out.len(), 1);
    }
}
