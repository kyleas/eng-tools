use egui_plot::{Legend, Line, Plot, PlotPoints};
use tf_eng::{
    DeviceStudyRequest, EquationStudyRequest, SweepAxisSpec, WorkflowStudyRequest,
    run_device_study, run_equation_study, run_workflow_study, studyable_device_keys,
    studyable_workflow_keys,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StudyMode {
    Equation,
    Device,
    Workflow,
}

pub struct EngStudyView {
    mode: StudyMode,
    axis_start: f64,
    axis_end: f64,
    axis_count: usize,

    equation_path_id: String,
    equation_target: String,
    equation_sweep_variable: String,
    equation_fixed_inputs_json: String,
    equation_branch: String,

    device_keys: Vec<String>,
    device_key_index: usize,
    device_sweep_arg: String,
    device_fixed_args_json: String,
    device_outputs_csv: String,
    device_output_key: String,

    workflow_keys: Vec<String>,
    workflow_key_index: usize,
    workflow_sweep_arg: String,
    workflow_fixed_args_json: String,
    workflow_output_key: String,

    result: Option<tf_eng::StudyResult>,
    last_error: Option<String>,
}

impl Default for EngStudyView {
    fn default() -> Self {
        let device_keys = studyable_device_keys();
        let workflow_keys = studyable_workflow_keys();
        Self {
            mode: StudyMode::Equation,
            axis_start: 0.2,
            axis_end: 3.0,
            axis_count: 20,

            equation_path_id: "compressible.isentropic_pressure_ratio".to_string(),
            equation_target: "p_p0".to_string(),
            equation_sweep_variable: "M".to_string(),
            equation_fixed_inputs_json: "{\"gamma\":1.4}".to_string(),
            equation_branch: String::new(),

            device_key_index: 0,
            device_keys,
            device_sweep_arg: "input_value".to_string(),
            device_fixed_args_json:
                "{\"input_kind\":\"mach\",\"target_kind\":\"pressure_ratio\",\"gamma\":1.4}"
                    .to_string(),
            device_outputs_csv: "value,pivot,path_text".to_string(),
            device_output_key: "value".to_string(),

            workflow_key_index: 0,
            workflow_keys,
            workflow_sweep_arg: "area_ratio".to_string(),
            workflow_fixed_args_json: "{\"gamma\":1.4,\"branch\":\"supersonic\"}".to_string(),
            workflow_output_key: "pre_shock_mach".to_string(),

            result: None,
            last_error: None,
        }
    }
}

impl EngStudyView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Eng Studies");
            ui.label(
                "Generic bridge to eng equation/device/workflow studies with plot-ready outputs.",
            );
            ui.separator();

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, StudyMode::Equation, "Equation");
                ui.selectable_value(&mut self.mode, StudyMode::Device, "Device");
                ui.selectable_value(&mut self.mode, StudyMode::Workflow, "Workflow");
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Axis start");
                ui.add(egui::DragValue::new(&mut self.axis_start));
                ui.label("end");
                ui.add(egui::DragValue::new(&mut self.axis_end));
                ui.label("count");
                ui.add(egui::DragValue::new(&mut self.axis_count).range(2..=2000));
            });

            match self.mode {
                StudyMode::Equation => self.show_equation_fields(ui),
                StudyMode::Device => self.show_device_fields(ui),
                StudyMode::Workflow => self.show_workflow_fields(ui),
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

    fn show_equation_fields(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("path_id");
            ui.text_edit_singleline(&mut self.equation_path_id);
        });
        ui.horizontal(|ui| {
            ui.label("target");
            ui.text_edit_singleline(&mut self.equation_target);
            ui.label("sweep variable");
            ui.text_edit_singleline(&mut self.equation_sweep_variable);
        });
        ui.label("fixed inputs JSON (numeric object)");
        ui.text_edit_singleline(&mut self.equation_fixed_inputs_json);
        ui.horizontal(|ui| {
            ui.label("branch (optional)");
            ui.text_edit_singleline(&mut self.equation_branch);
        });
    }

    fn show_device_fields(&mut self, ui: &mut egui::Ui) {
        if self.device_keys.is_empty() {
            ui.label("No studyable devices registered.");
            return;
        }
        self.device_key_index = self
            .device_key_index
            .min(self.device_keys.len().saturating_sub(1));
        egui::ComboBox::from_label("device")
            .selected_text(self.device_keys[self.device_key_index].clone())
            .show_ui(ui, |ui| {
                for (idx, key) in self.device_keys.iter().enumerate() {
                    ui.selectable_value(&mut self.device_key_index, idx, key);
                }
            });
        ui.horizontal(|ui| {
            ui.label("sweep arg");
            ui.text_edit_singleline(&mut self.device_sweep_arg);
        });
        ui.label("fixed args JSON (object)");
        ui.text_edit_singleline(&mut self.device_fixed_args_json);
        ui.horizontal(|ui| {
            ui.label("outputs csv");
            ui.text_edit_singleline(&mut self.device_outputs_csv);
            ui.label("plot output key");
            ui.text_edit_singleline(&mut self.device_output_key);
        });
    }

    fn show_workflow_fields(&mut self, ui: &mut egui::Ui) {
        if self.workflow_keys.is_empty() {
            ui.label("No studyable workflows registered.");
            return;
        }
        self.workflow_key_index = self
            .workflow_key_index
            .min(self.workflow_keys.len().saturating_sub(1));
        egui::ComboBox::from_label("workflow")
            .selected_text(self.workflow_keys[self.workflow_key_index].clone())
            .show_ui(ui, |ui| {
                for (idx, key) in self.workflow_keys.iter().enumerate() {
                    ui.selectable_value(&mut self.workflow_key_index, idx, key);
                }
            });
        ui.horizontal(|ui| {
            ui.label("sweep arg");
            ui.text_edit_singleline(&mut self.workflow_sweep_arg);
            ui.label("plot output key");
            ui.text_edit_singleline(&mut self.workflow_output_key);
        });
        ui.label("fixed args JSON (object)");
        ui.text_edit_singleline(&mut self.workflow_fixed_args_json);
    }

    fn run_selected(&self) -> Result<tf_eng::StudyResult, String> {
        let axis = SweepAxisSpec::Linspace {
            start: self.axis_start,
            end: self.axis_end,
            count: self.axis_count,
        };
        match self.mode {
            StudyMode::Equation => {
                let fixed_inputs = parse_numeric_map(&self.equation_fixed_inputs_json)?;
                run_equation_study(EquationStudyRequest {
                    path_id: self.equation_path_id.clone(),
                    target: self.equation_target.clone(),
                    sweep_variable: self.equation_sweep_variable.clone(),
                    axis,
                    fixed_inputs,
                    branch: (!self.equation_branch.trim().is_empty())
                        .then(|| self.equation_branch.trim().to_string()),
                    output_key: None,
                })
                .map_err(|e| e.to_string())
            }
            StudyMode::Device => {
                let fixed_args = parse_json_object(&self.device_fixed_args_json)?;
                let requested_outputs = self
                    .device_outputs_csv
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                run_device_study(DeviceStudyRequest {
                    device_key: self.device_keys[self.device_key_index].clone(),
                    sweep_arg: self.device_sweep_arg.clone(),
                    axis,
                    fixed_args,
                    requested_outputs,
                    output_key: (!self.device_output_key.trim().is_empty())
                        .then(|| self.device_output_key.trim().to_string()),
                })
                .map_err(|e| e.to_string())
            }
            StudyMode::Workflow => {
                let fixed_args = parse_json_object(&self.workflow_fixed_args_json)?;
                run_workflow_study(WorkflowStudyRequest {
                    workflow_key: self.workflow_keys[self.workflow_key_index].clone(),
                    sweep_arg: self.workflow_sweep_arg.clone(),
                    axis,
                    fixed_args,
                    output_key: (!self.workflow_output_key.trim().is_empty())
                        .then(|| self.workflow_output_key.trim().to_string()),
                })
                .map_err(|e| e.to_string())
            }
        }
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
}

fn parse_json_object(text: &str) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    match serde_json::from_str::<serde_json::Value>(text).map_err(|e| e.to_string())? {
        serde_json::Value::Object(obj) => Ok(obj),
        _ => Err("JSON must be an object".to_string()),
    }
}

fn parse_numeric_map(text: &str) -> Result<std::collections::BTreeMap<String, f64>, String> {
    let obj = parse_json_object(text)?;
    let mut out = std::collections::BTreeMap::new();
    for (k, v) in obj {
        let Some(n) = v.as_f64() else {
            return Err(format!("fixed input '{k}' must be numeric"));
        };
        out.insert(k, n);
    }
    Ok(out)
}
