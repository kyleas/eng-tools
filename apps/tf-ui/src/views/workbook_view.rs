use std::path::PathBuf;

use egui_plot::{Legend, Line, Plot, PlotPoints};
use tf_workbook::{
    ConstantRowContent, EquationSolveRowContent, PlotRowContent, StudyRowContent, TextRowContent,
    WorkbookDocument, WorkbookRow, WorkbookRowKind, WorkbookRunResult, WorkbookSweepAxis,
    WorkbookTab, create_workbook_skeleton, execute_workbook, load_workbook_dir, save_workbook_dir,
};
use uuid::Uuid;

pub struct WorkbookView {
    workbook_path: String,
    workbook: Option<WorkbookDocument>,
    selected_tab: usize,
    run_result: Option<WorkbookRunResult>,
    auto_run: bool,
    last_error: Option<String>,
}

impl Default for WorkbookView {
    fn default() -> Self {
        Self {
            workbook_path: "examples/workbook_v1.engwb".to_string(),
            workbook: None,
            selected_tab: 0,
            run_result: None,
            auto_run: true,
            last_error: None,
        }
    }
}

impl WorkbookView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Engineering Workbook v1");
        ui.label("Row-based text-first workbook (.engwb), executed by tf-workbook + tf-eng.");

        ui.horizontal(|ui| {
            ui.label("Workbook dir");
            ui.text_edit_singleline(&mut self.workbook_path);
            if ui.button("Create").clicked() {
                match create_workbook_skeleton(
                    PathBuf::from(&self.workbook_path).as_path(),
                    "Engineering Workbook",
                ) {
                    Ok(doc) => {
                        self.workbook = Some(doc);
                        self.last_error = None;
                        self.selected_tab = 0;
                    }
                    Err(e) => self.last_error = Some(e.to_string()),
                }
            }
            if ui.button("Open").clicked() {
                match load_workbook_dir(PathBuf::from(&self.workbook_path).as_path()) {
                    Ok(doc) => {
                        self.workbook = Some(doc);
                        self.last_error = None;
                        self.selected_tab = 0;
                    }
                    Err(e) => self.last_error = Some(e.to_string()),
                }
            }
            if ui.button("Save").clicked()
                && let Some(doc) = &self.workbook
                && let Err(e) = save_workbook_dir(doc)
            {
                self.last_error = Some(e.to_string());
            }
            if ui.button("Run").clicked() {
                self.run_now();
            }
            ui.checkbox(&mut self.auto_run, "auto run");
        });

        if let Some(err) = &self.last_error {
            ui.colored_label(egui::Color32::RED, err);
        }

        let mut changed = false;
        let mut should_auto_run = false;
        if let Some(doc) = &mut self.workbook {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Title: {}", doc.manifest.title));
            });

            ui.horizontal_wrapped(|ui| {
                for (i, tab) in doc.tabs.iter().enumerate() {
                    if ui
                        .selectable_label(
                            self.selected_tab == i,
                            format!("{} ({})", tab.name, tab.file),
                        )
                        .clicked()
                    {
                        self.selected_tab = i;
                    }
                }
            });

            if let Some(tab) = doc.tabs.get_mut(self.selected_tab) {
                changed |= Self::show_tab_editor(ui, tab);
            }

            if changed {
                self.last_error = None;
                if self.auto_run {
                    should_auto_run = true;
                }
            }
        }

        if should_auto_run {
            self.run_now();
        }

        if let Some(run) = &self.run_result {
            self.show_results(ui, run);
        }
    }

    fn show_tab_editor(ui: &mut egui::Ui, tab: &mut WorkbookTab) -> bool {
        let mut changed = false;

        ui.separator();
        ui.horizontal(|ui| {
            ui.heading(format!("Tab: {}", tab.name));
            if ui.button("Add text").clicked() {
                tab.rows.push(new_row(WorkbookRowKind::Text(TextRowContent {
                    text: "".to_string(),
                })));
                changed = true;
            }
            if ui.button("Add constant").clicked() {
                tab.rows
                    .push(new_row(WorkbookRowKind::Constant(ConstantRowContent {
                        value: "".to_string(),
                        dimension_hint: None,
                    })));
                changed = true;
            }
            if ui.button("Add equation").clicked() {
                tab.rows.push(new_row(WorkbookRowKind::EquationSolve(
                    EquationSolveRowContent {
                        path_id: "structures.hoop_stress".to_string(),
                        target: None,
                        branch: None,
                        inputs: Default::default(),
                    },
                )));
                changed = true;
            }
            if ui.button("Add study").clicked() {
                tab.rows
                    .push(new_row(WorkbookRowKind::Study(StudyRowContent {
                        kind: tf_eng::StudyTargetKind::Equation,
                        target_id: "compressible.isentropic_pressure_ratio".to_string(),
                        sweep_field: "M".to_string(),
                        sweep: WorkbookSweepAxis::Linspace {
                            start: 0.5,
                            end: 3.0,
                            count: 25,
                        },
                        fixed_inputs: Default::default(),
                        output_key: "p_p0".to_string(),
                    })));
                changed = true;
            }
            if ui.button("Add plot").clicked() {
                tab.rows.push(new_row(WorkbookRowKind::Plot(PlotRowContent {
                    source_row: "ref:study".to_string(),
                    x: "M".to_string(),
                    y: "p_p0".to_string(),
                    title: Some("Plot".to_string()),
                    x_label: None,
                    y_label: None,
                })));
                changed = true;
            }
        });

        let mut remove_idx = None;
        for i in 0..tab.rows.len() {
            let can_move_down = i + 1 < tab.rows.len();
            let mut duplicate = false;
            let mut move_up = false;
            let mut move_down = false;
            let row = &mut tab.rows[i];
            ui.separator();
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Row {}", i + 1));
                    ui.monospace(&row.id);
                    if ui.button("dup").clicked() {
                        duplicate = true;
                    }
                    if ui.button("del").clicked() {
                        remove_idx = Some(i);
                    }
                    if i > 0 && ui.button("up").clicked() {
                        move_up = true;
                    }
                    if can_move_down && ui.button("down").clicked() {
                        move_down = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("key");
                    changed |= ui
                        .text_edit_singleline(row.key.get_or_insert_with(String::new))
                        .changed();
                    ui.label("title");
                    changed |= ui
                        .text_edit_singleline(row.title.get_or_insert_with(String::new))
                        .changed();
                });

                match &mut row.kind {
                    WorkbookRowKind::Text(c) => {
                        changed |= ui.text_edit_multiline(&mut c.text).changed();
                    }
                    WorkbookRowKind::Markdown(c) => {
                        changed |= ui.text_edit_multiline(&mut c.markdown).changed();
                    }
                    WorkbookRowKind::Constant(c) => {
                        ui.horizontal(|ui| {
                            ui.label("value");
                            changed |= ui.text_edit_singleline(&mut c.value).changed();
                        });
                        ui.horizontal(|ui| {
                            ui.label("dimension_hint");
                            changed |= ui
                                .text_edit_singleline(
                                    c.dimension_hint.get_or_insert_with(String::new),
                                )
                                .changed();
                        });
                    }
                    WorkbookRowKind::EquationSolve(c) => {
                        ui.horizontal(|ui| {
                            ui.label("path_id");
                            changed |= ui.text_edit_singleline(&mut c.path_id).changed();
                        });
                        ui.horizontal(|ui| {
                            ui.label("target");
                            changed |= ui
                                .text_edit_singleline(c.target.get_or_insert_with(String::new))
                                .changed();
                            ui.label("branch");
                            changed |= ui
                                .text_edit_singleline(c.branch.get_or_insert_with(String::new))
                                .changed();
                        });
                        changed |= map_editor(ui, &mut c.inputs, "inputs");
                    }
                    WorkbookRowKind::Study(c) => {
                        ui.horizontal(|ui| {
                            ui.label("kind");
                            ui.selectable_value(
                                &mut c.kind,
                                tf_eng::StudyTargetKind::Equation,
                                "equation",
                            );
                            ui.selectable_value(
                                &mut c.kind,
                                tf_eng::StudyTargetKind::Device,
                                "device",
                            );
                            ui.selectable_value(
                                &mut c.kind,
                                tf_eng::StudyTargetKind::Workflow,
                                "workflow",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("target_id");
                            changed |= ui.text_edit_singleline(&mut c.target_id).changed();
                        });
                        ui.horizontal(|ui| {
                            ui.label("sweep_field");
                            changed |= ui.text_edit_singleline(&mut c.sweep_field).changed();
                            ui.label("output_key");
                            changed |= ui.text_edit_singleline(&mut c.output_key).changed();
                        });
                        changed |= map_editor(ui, &mut c.fixed_inputs, "fixed_inputs");
                    }
                    WorkbookRowKind::Plot(c) => {
                        ui.horizontal(|ui| {
                            ui.label("source_row");
                            changed |= ui.text_edit_singleline(&mut c.source_row).changed();
                        });
                        ui.horizontal(|ui| {
                            ui.label("x");
                            changed |= ui.text_edit_singleline(&mut c.x).changed();
                            ui.label("y");
                            changed |= ui.text_edit_singleline(&mut c.y).changed();
                        });
                        ui.horizontal(|ui| {
                            ui.label("title");
                            changed |= ui
                                .text_edit_singleline(c.title.get_or_insert_with(String::new))
                                .changed();
                        });
                    }
                }
            });
            if duplicate {
                let mut copy = tab.rows[i].clone();
                copy.id = Uuid::new_v4().to_string();
                tab.rows.insert(i + 1, copy);
                changed = true;
                break;
            }
            if move_up {
                tab.rows.swap(i, i - 1);
                changed = true;
                break;
            }
            if move_down {
                tab.rows.swap(i, i + 1);
                changed = true;
                break;
            }
        }

        if let Some(i) = remove_idx {
            tab.rows.remove(i);
            changed = true;
        }

        changed
    }

    fn run_now(&mut self) {
        let Some(doc) = &self.workbook else {
            return;
        };
        match execute_workbook(doc, None) {
            Ok(run) => {
                self.run_result = Some(run);
                self.last_error = None;
            }
            Err(e) => {
                self.run_result = None;
                self.last_error = Some(e.to_string());
            }
        }
    }

    fn show_results(&self, ui: &mut egui::Ui, run: &WorkbookRunResult) {
        ui.separator();
        ui.heading("Results");
        for tab in &run.tabs {
            ui.collapsing(format!("{} ({})", tab.name, tab.file), |ui| {
                for row in &tab.rows {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.monospace(&row.id);
                            if let Some(key) = &row.key {
                                ui.label(format!("key: {}", key));
                            }
                            ui.label(format!("state: {:?}", row.state));
                        });
                        if !row.messages.is_empty() {
                            for m in &row.messages {
                                ui.colored_label(egui::Color32::YELLOW, m);
                            }
                        }
                        if let Some(result) = &row.result {
                            match result {
                                tf_workbook::WorkbookRowResult::Constant(c) => {
                                    ui.label(format!("value: {}", c.value));
                                }
                                tf_workbook::WorkbookRowResult::Equation(e) => {
                                    ui.label(format!("{} = {}", e.solve.output_key, e.solve.value));
                                }
                                tf_workbook::WorkbookRowResult::Study(s) => {
                                    ui.label(format!(
                                        "study rows: {} cols: {}",
                                        s.table.rows.len(),
                                        s.table.columns.len()
                                    ));
                                }
                                tf_workbook::WorkbookRowResult::Plot(p) => {
                                    for series in &p.series {
                                        let pts = series
                                            .x
                                            .iter()
                                            .zip(series.y.iter())
                                            .map(|(x, y)| [*x, *y])
                                            .collect::<Vec<_>>();
                                        Plot::new(format!("plot_{}_{}", row.id, series.name))
                                            .legend(Legend::default())
                                            .show(ui, |plot_ui| {
                                                plot_ui.line(
                                                    Line::new(PlotPoints::from(pts))
                                                        .name(series.name.clone()),
                                                );
                                            });
                                    }
                                }
                                tf_workbook::WorkbookRowResult::Text { text } => {
                                    ui.label(text);
                                }
                                tf_workbook::WorkbookRowResult::Markdown { markdown } => {
                                    ui.label(markdown);
                                }
                            }
                        }
                    });
                }
            });
        }
    }
}

fn new_row(kind: WorkbookRowKind) -> WorkbookRow {
    WorkbookRow {
        id: Uuid::new_v4().to_string(),
        key: None,
        title: None,
        collapsed: false,
        kind,
    }
}

fn map_editor(
    ui: &mut egui::Ui,
    map: &mut std::collections::BTreeMap<String, String>,
    label: &str,
) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        let mut entries = map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        let mut remove_idx = None;
        for (i, (k, v)) in entries.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                changed |= ui.text_edit_singleline(k).changed();
                changed |= ui.text_edit_singleline(v).changed();
                if ui.button("x").clicked() {
                    remove_idx = Some(i);
                }
            });
        }
        if let Some(i) = remove_idx {
            entries.remove(i);
            changed = true;
        }
        if changed {
            map.clear();
            for (k, v) in entries {
                if !k.trim().is_empty() {
                    map.insert(k, v);
                }
            }
        }
        if ui.button("+ field").clicked() {
            map.insert("key".to_string(), "".to_string());
            changed = true;
        }
    });
    changed
}
