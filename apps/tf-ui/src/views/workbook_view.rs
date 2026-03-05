use std::collections::HashMap;
use std::path::Path;

use egui_plot::{Legend, Line, Plot, PlotPoints};
use tf_eng::{StudyTargetDescriptor, StudyTargetKind, list_study_targets};
use tf_workbook::{
    ConstantRowContent, EquationSolveRowContent, PlotRowContent, StudyRowContent, TextRowContent,
    WorkbookDocument, WorkbookRow, WorkbookRowExecution, WorkbookRowKind, WorkbookRunResult,
    WorkbookSweepAxis, WorkbookTab, create_workbook_skeleton, execute_workbook, load_workbook_dir,
    save_workbook_dir,
};
use uuid::Uuid;

pub struct WorkbookView {
    workbook_path: String,
    new_workbook_name: String,
    workbook: Option<WorkbookDocument>,
    selected_tab: usize,
    run_result: Option<WorkbookRunResult>,
    auto_run: bool,
    last_error: Option<String>,
    targets: Vec<StudyTargetDescriptor>,
}

#[derive(Default)]
struct EditOutcome {
    changed: bool,
    changed_unfrozen: bool,
}

#[derive(Clone)]
struct PlotSourceOption {
    ref_value: String,
    label: String,
}

impl Default for WorkbookView {
    fn default() -> Self {
        Self {
            workbook_path: String::new(),
            new_workbook_name: "engineering_workbook.engwb".to_string(),
            workbook: None,
            selected_tab: 0,
            run_result: None,
            auto_run: true,
            last_error: None,
            targets: list_study_targets().unwrap_or_default(),
        }
    }
}

impl WorkbookView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Engineering Workbook");
        ui.label("Row-based text-first worksheet (.engwb) powered by tf-workbook + tf-eng.");

        self.toolbar(ui);

        if let Some(err) = &self.last_error {
            ui.colored_label(egui::Color32::RED, err);
        }

        let mut run_after = false;
        if let Some(doc) = &mut self.workbook {
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Workbook: {}", doc.manifest.title));
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

            let run_tab = self
                .run_result
                .as_ref()
                .and_then(|r| r.tabs.get(self.selected_tab));

            let targets = self.targets.clone();
            if let Some(tab) = doc.tabs.get_mut(self.selected_tab) {
                let outcome = Self::show_tab_editor(ui, tab, run_tab, &targets);
                if outcome.changed {
                    self.last_error = None;
                    if self.auto_run && outcome.changed_unfrozen {
                        run_after = true;
                    }
                }
            }
        }

        if run_after {
            self.run_now();
        }

        if let Some(run) = &self.run_result {
            self.show_results(ui, run);
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Path");
            ui.text_edit_singleline(&mut self.workbook_path);

            if ui.button("Open").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.open_workbook(&path);
                }
            }

            if ui.button("Create").clicked()
                && let Some(parent) = rfd::FileDialog::new().pick_folder()
            {
                let name = if self.new_workbook_name.trim().is_empty() {
                    "engineering_workbook.engwb".to_string()
                } else {
                    self.new_workbook_name.clone()
                };
                let folder = parent.join(name);
                match create_workbook_skeleton(&folder, "Engineering Workbook") {
                    Ok(doc) => {
                        self.workbook_path = folder.to_string_lossy().to_string();
                        self.workbook = Some(doc);
                        self.selected_tab = 0;
                        self.last_error = None;
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

        ui.horizontal(|ui| {
            ui.label("New workbook folder name");
            ui.text_edit_singleline(&mut self.new_workbook_name);
        });
    }

    fn open_workbook(&mut self, dir: &Path) {
        if !dir.join("workbook.yaml").exists() {
            self.last_error = Some(format!("{} is not a workbook directory", dir.display()));
            return;
        }
        match load_workbook_dir(dir) {
            Ok(doc) => {
                self.workbook_path = dir.to_string_lossy().to_string();
                self.workbook = Some(doc);
                self.selected_tab = 0;
                self.last_error = None;
            }
            Err(e) => self.last_error = Some(e.to_string()),
        }
    }

    fn show_tab_editor(
        ui: &mut egui::Ui,
        tab: &mut WorkbookTab,
        run_tab: Option<&tf_workbook::WorkbookTabExecution>,
        targets: &[StudyTargetDescriptor],
    ) -> EditOutcome {
        let mut outcome = EditOutcome::default();

        ui.separator();
        ui.horizontal(|ui| {
            ui.heading(format!("Tab: {}", tab.name));
            if ui.button("Add text").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Text(TextRowContent {
                        text: String::new(),
                    }),
                    !tab.rows.is_empty(),
                ));
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            if ui.button("Add constant").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Constant(ConstantRowContent {
                        value: String::new(),
                        dimension_hint: None,
                    }),
                    !tab.rows.is_empty(),
                ));
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            if ui.button("Add equation").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                        path_id: default_equation_id(targets),
                        target: None,
                        branch: None,
                        inputs: Default::default(),
                    }),
                    !tab.rows.is_empty(),
                ));
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            if ui.button("Add study").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Study(StudyRowContent {
                        kind: StudyTargetKind::Equation,
                        target_id: default_target_id(targets, StudyTargetKind::Equation),
                        sweep_field: "M".to_string(),
                        sweep: WorkbookSweepAxis::Linspace {
                            start: 0.5,
                            end: 3.0,
                            count: 25,
                        },
                        fixed_inputs: Default::default(),
                        output_key: String::new(),
                    }),
                    !tab.rows.is_empty(),
                ));
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            if ui.button("Add plot").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Plot(PlotRowContent {
                        source_row: String::new(),
                        x: String::new(),
                        y: String::new(),
                        title: Some("Plot".to_string()),
                        x_label: None,
                        y_label: None,
                    }),
                    !tab.rows.is_empty(),
                ));
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
        });

        let row_exec_map = run_tab
            .map(|t| {
                t.rows
                    .iter()
                    .map(|r| (r.id.clone(), r.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let mut remove_idx = None;
        let mut reorder: Option<(usize, usize)> = None;
        for i in 0..tab.rows.len() {
            let can_move_down = i + 1 < tab.rows.len();
            let mut duplicate = false;
            let mut row_changed = false;
            let plot_source_options = tab
                .rows
                .iter()
                .take(i)
                .filter(|r| matches!(r.kind, WorkbookRowKind::Study(_)) && r.key.is_some())
                .map(|r| {
                    let key = r.key.as_deref().unwrap_or_default();
                    PlotSourceOption {
                        ref_value: format!("ref:{}", key),
                        label: key.to_string(),
                    }
                })
                .collect::<Vec<_>>();
            let valid_keys = tab
                .rows
                .iter()
                .filter_map(|r| r.key.clone())
                .collect::<Vec<_>>();
            let row = &mut tab.rows[i];
            let exec = row_exec_map.get(&row.id);
            let collapsed = row.collapsed;
            let frozen_before = row.freeze;

            ui.push_id(row.id.clone(), |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let chevron = if collapsed { ">" } else { "v" };
                        if ui.button(chevron).clicked() {
                            row.collapsed = !row.collapsed;
                            outcome.changed = true;
                            row_changed = true;
                        }
                        ui.label(primary_row_label(row));
                        ui.small(row_type_badge(&row.kind));
                        ui.colored_label(
                            status_color(exec),
                            exec.map(|e| format!("{:?}", e.state))
                                .unwrap_or_else(|| "not-run".to_string()),
                        );
                        if let Some(preview) = output_preview(exec) {
                            ui.small(preview);
                        }
                        ui.separator();
                        ui.label("⋮⋮");
                        if i > 0 && ui.small_button("up").clicked() {
                            reorder = Some((i, i - 1));
                        }
                        if can_move_down && ui.small_button("dn").clicked() {
                            reorder = Some((i, i + 1));
                        }
                        if ui.small_button("dup").clicked() {
                            duplicate = true;
                        }
                        if ui.small_button("del").clicked() {
                            remove_idx = Some(i);
                        }
                    });

                    if !row.collapsed {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Key");
                            row_changed |= ui
                                .text_edit_singleline(row.key.get_or_insert_with(String::new))
                                .changed();
                            ui.checkbox(&mut row.freeze, "freeze");
                            if row.freeze != frozen_before {
                                outcome.changed = true;
                                row_changed = true;
                            }
                        });
                        ui.collapsing("Details", |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Title");
                                row_changed |= ui
                                    .text_edit_singleline(row.title.get_or_insert_with(String::new))
                                    .changed();
                            });
                            ui.small(format!("row_id: {}", row.id));
                        });

                        match &mut row.kind {
                            WorkbookRowKind::Text(c) => {
                                row_changed |= ui.text_edit_multiline(&mut c.text).changed();
                            }
                            WorkbookRowKind::Markdown(c) => {
                                row_changed |= ui.text_edit_multiline(&mut c.markdown).changed();
                            }
                            WorkbookRowKind::Constant(c) => {
                                ui.horizontal(|ui| {
                                    ui.label("Value");
                                    row_changed |= ui.text_edit_singleline(&mut c.value).changed();
                                    ui.small("number or any-unit string");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Dimension hint");
                                    row_changed |= ui
                                        .text_edit_singleline(
                                            c.dimension_hint.get_or_insert_with(String::new),
                                        )
                                        .changed();
                                });
                            }
                            WorkbookRowKind::EquationSolve(c) => {
                                row_changed |=
                                    Self::render_equation_row(ui, row.id.as_str(), c, targets);
                            }
                            WorkbookRowKind::Study(c) => {
                                row_changed |=
                                    Self::render_study_row(ui, row.id.as_str(), c, targets);
                            }
                            WorkbookRowKind::Plot(c) => {
                                row_changed |= Self::render_plot_row(
                                    ui,
                                    row.id.as_str(),
                                    c,
                                    &plot_source_options,
                                    &valid_keys,
                                );
                            }
                        }
                    }
                });
            });
            if row_changed {
                outcome.changed = true;
                if !row.freeze {
                    outcome.changed_unfrozen = true;
                }
            }
            if duplicate {
                let mut copy = tab.rows[i].clone();
                copy.id = Uuid::new_v4().to_string();
                copy.collapsed = true;
                tab.rows.insert(i + 1, copy);
                outcome.changed = true;
                outcome.changed_unfrozen = true;
                break;
            }
            ui.add_space(6.0);
        }

        if let Some((a, b)) = reorder {
            tab.rows.swap(a, b);
            outcome.changed = true;
            outcome.changed_unfrozen = true;
        }
        if let Some(i) = remove_idx {
            tab.rows.remove(i);
            outcome.changed = true;
            outcome.changed_unfrozen = true;
        }

        outcome
    }

    fn render_equation_row(
        ui: &mut egui::Ui,
        row_id: &str,
        c: &mut EquationSolveRowContent,
        targets: &[StudyTargetDescriptor],
    ) -> bool {
        let mut changed = false;
        let equations = targets
            .iter()
            .filter(|t| t.kind == StudyTargetKind::Equation)
            .collect::<Vec<_>>();

        ui.horizontal(|ui| {
            ui.label("Target");
            egui::ComboBox::from_id_salt(row_field_id(row_id, "equation_target"))
                .selected_text(display_target_name(&equations, &c.path_id))
                .show_ui(ui, |ui| {
                    for t in &equations {
                        if ui
                            .selectable_label(c.path_id == t.id, format!("{} ({})", t.name, t.id))
                            .clicked()
                        {
                            c.path_id = t.id.clone();
                            changed = true;
                        }
                    }
                });
        });

        let Some(desc) = targets
            .iter()
            .find(|t| t.kind == StudyTargetKind::Equation && t.id == c.path_id)
        else {
            ui.small("Unknown equation target");
            return changed;
        };

        let mut target_value = c.target.clone().unwrap_or_default();
        ui.horizontal(|ui| {
            ui.label("Output");
            egui::ComboBox::from_id_salt(row_field_id(row_id, "equation_output"))
                .selected_text(if target_value.is_empty() {
                    "<infer from one blank>".to_string()
                } else {
                    target_value.clone()
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(target_value.is_empty(), "<infer from one blank>")
                        .clicked()
                    {
                        target_value.clear();
                        changed = true;
                    }
                    for out in &desc.outputs {
                        if ui
                            .selectable_label(target_value == out.key, out.key.clone())
                            .clicked()
                        {
                            target_value = out.key.clone();
                            changed = true;
                        }
                    }
                });
        });
        c.target = if target_value.trim().is_empty() {
            None
        } else {
            Some(target_value)
        };

        if !desc.branch_options.is_empty() {
            ui.horizontal(|ui| {
                ui.label("Branch");
                let mut branch = c.branch.clone().unwrap_or_default();
                egui::ComboBox::from_id_salt(row_field_id(row_id, "equation_branch"))
                    .selected_text(if branch.is_empty() {
                        "<none>".to_string()
                    } else {
                        branch.clone()
                    })
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(branch.is_empty(), "<none>").clicked() {
                            branch.clear();
                            changed = true;
                        }
                        for b in &desc.branch_options {
                            if ui.selectable_label(branch == *b, b).clicked() {
                                branch = b.clone();
                                changed = true;
                            }
                        }
                    });
                c.branch = if branch.is_empty() {
                    None
                } else {
                    Some(branch)
                };
            });
        }

        ui.separator();
        ui.label("Inputs");
        let mut blank_count = 0usize;
        let mut blank_name = String::new();
        for field in desc.input_fields.iter().filter(|f| f.key != "branch") {
            ui.push_id(row_field_id(row_id, &field.key), |ui| {
                ui.horizontal(|ui| {
                    ui.label(&field.key);
                    let entry = c.inputs.entry(field.key.clone()).or_default();
                    changed |= ui.text_edit_singleline(entry).changed();
                    if let Some(unit) = &field.unit {
                        ui.small(unit);
                    }
                    if entry.trim().is_empty() {
                        blank_count += 1;
                        blank_name = field.key.clone();
                    }
                });
            });
        }

        if c.target.as_deref().unwrap_or_default().is_empty() {
            if blank_count == 1 {
                ui.small(format!("Inferred target: {}", blank_name));
            } else {
                ui.colored_label(
                    egui::Color32::YELLOW,
                    "Target inference needs exactly one blank input",
                );
            }
        }

        changed
    }

    fn render_study_row(
        ui: &mut egui::Ui,
        row_id: &str,
        c: &mut StudyRowContent,
        targets: &[StudyTargetDescriptor],
    ) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Kind");
            ui.selectable_value(&mut c.kind, StudyTargetKind::Equation, "equation");
            ui.selectable_value(&mut c.kind, StudyTargetKind::Device, "device");
            ui.selectable_value(&mut c.kind, StudyTargetKind::Workflow, "workflow");
        });

        let target_options = targets
            .iter()
            .filter(|t| t.kind == c.kind)
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            ui.label("Target");
            egui::ComboBox::from_id_salt(row_field_id(row_id, "study_target"))
                .selected_text(display_target_name(&target_options, &c.target_id))
                .show_ui(ui, |ui| {
                    for t in &target_options {
                        if ui
                            .selectable_label(c.target_id == t.id, format!("{} ({})", t.name, t.id))
                            .clicked()
                        {
                            c.target_id = t.id.clone();
                            changed = true;
                        }
                    }
                });
        });

        let Some(desc) = targets
            .iter()
            .find(|t| t.kind == c.kind && t.id == c.target_id)
        else {
            ui.small("Unknown study target");
            return changed;
        };

        ui.horizontal(|ui| {
            ui.label("Sweep field");
            egui::ComboBox::from_id_salt(row_field_id(row_id, "study_sweep"))
                .selected_text(c.sweep_field.clone())
                .show_ui(ui, |ui| {
                    for f in &desc.sweepable_fields {
                        if ui.selectable_label(c.sweep_field == *f, f).clicked() {
                            c.sweep_field = f.clone();
                            changed = true;
                        }
                    }
                });
        });

        let outputs = desc
            .outputs
            .iter()
            .filter(|o| o.numeric || o.plottable)
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            ui.label("Output key");
            egui::ComboBox::from_id_salt(row_field_id(row_id, "study_output"))
                .selected_text(c.output_key.clone())
                .show_ui(ui, |ui| {
                    for o in &outputs {
                        if ui
                            .selectable_label(c.output_key == o.key, o.key.clone())
                            .clicked()
                        {
                            c.output_key = o.key.clone();
                            changed = true;
                        }
                    }
                });
        });

        ui.collapsing("Fixed inputs", |ui| {
            for field in desc.input_fields.iter().filter(|f| f.key != c.sweep_field) {
                ui.push_id(row_field_id(row_id, &field.key), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&field.key);
                        let entry = c.fixed_inputs.entry(field.key.clone()).or_default();
                        changed |= ui.text_edit_singleline(entry).changed();
                        if let Some(unit) = &field.unit {
                            ui.small(unit);
                        }
                    });
                });
            }
        });

        ui.collapsing("Sweep", |ui| match &mut c.sweep {
            WorkbookSweepAxis::Values { values } => {
                ui.horizontal(|ui| {
                    ui.label("values (comma)");
                    let mut text = values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    if ui.text_edit_singleline(&mut text).changed() {
                        let parsed = text
                            .split(',')
                            .filter_map(|s| s.trim().parse::<f64>().ok())
                            .collect::<Vec<_>>();
                        if !parsed.is_empty() {
                            *values = parsed;
                            changed = true;
                        }
                    }
                });
            }
            WorkbookSweepAxis::Linspace { start, end, count }
            | WorkbookSweepAxis::Logspace { start, end, count } => {
                ui.horizontal(|ui| {
                    ui.label("start");
                    changed |= ui.add(egui::DragValue::new(start)).changed();
                    ui.label("end");
                    changed |= ui.add(egui::DragValue::new(end)).changed();
                    ui.label("count");
                    changed |= ui.add(egui::DragValue::new(count).range(2..=500)).changed();
                });
            }
        });

        changed
    }

    fn render_plot_row(
        ui: &mut egui::Ui,
        row_id: &str,
        c: &mut PlotRowContent,
        source_options: &[PlotSourceOption],
        valid_keys: &[String],
    ) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Source");
            egui::ComboBox::from_id_salt(row_field_id(row_id, "plot_source"))
                .selected_text(if c.source_row.is_empty() {
                    "<select source>".to_string()
                } else {
                    c.source_row.clone()
                })
                .show_ui(ui, |ui| {
                    for src in source_options {
                        if ui
                            .selectable_label(c.source_row == src.ref_value, src.label.clone())
                            .clicked()
                        {
                            c.source_row = src.ref_value.clone();
                            changed = true;
                        }
                    }
                });
            changed |= ui.text_edit_singleline(&mut c.source_row).changed();
        });

        if let Some(k) = parse_ref_key(&c.source_row)
            && !valid_keys.iter().any(|existing| existing == k)
        {
            ui.colored_label(egui::Color32::RED, format!("unknown source key '{}'", k));
        }

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
        ui.heading("Execution results");
        for tab in &run.tabs {
            ui.collapsing(format!("{} ({})", tab.name, tab.file), |ui| {
                for row in &tab.rows {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            if let Some(key) = &row.key {
                                ui.label(key);
                            }
                            ui.small(format!("state: {:?}", row.state));
                        });
                        for m in &row.messages {
                            ui.colored_label(egui::Color32::YELLOW, m);
                        }
                        if let Some(result) = &row.result {
                            match result {
                                tf_workbook::WorkbookRowResult::Equation(e) => {
                                    ui.label(format!("{} = {}", e.solve.output_key, e.solve.value));
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
                                tf_workbook::WorkbookRowResult::Constant(c) => {
                                    ui.label(format!("{}", c.value));
                                }
                                tf_workbook::WorkbookRowResult::Study(s) => {
                                    ui.label(format!("study rows: {}", s.table.rows.len()));
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

fn new_row(kind: WorkbookRowKind, collapsed: bool) -> WorkbookRow {
    WorkbookRow {
        id: Uuid::new_v4().to_string(),
        key: None,
        title: None,
        collapsed,
        freeze: false,
        kind,
    }
}

fn row_type_badge(kind: &WorkbookRowKind) -> &'static str {
    match kind {
        WorkbookRowKind::Text(_) => "text",
        WorkbookRowKind::Markdown(_) => "markdown",
        WorkbookRowKind::Constant(_) => "constant",
        WorkbookRowKind::EquationSolve(_) => "equation",
        WorkbookRowKind::Study(_) => "study",
        WorkbookRowKind::Plot(_) => "plot",
    }
}

fn primary_row_label(row: &WorkbookRow) -> String {
    if let Some(k) = &row.key
        && !k.trim().is_empty()
    {
        return k.clone();
    }
    "Untitled".to_string()
}

fn status_color(exec: Option<&WorkbookRowExecution>) -> egui::Color32 {
    match exec.map(|e| &e.state) {
        Some(tf_workbook::WorkbookRowState::Ok) => egui::Color32::LIGHT_GREEN,
        Some(tf_workbook::WorkbookRowState::Invalid)
        | Some(tf_workbook::WorkbookRowState::Error) => egui::Color32::LIGHT_RED,
        Some(tf_workbook::WorkbookRowState::Ambiguous) => egui::Color32::YELLOW,
        _ => egui::Color32::LIGHT_GRAY,
    }
}

fn output_preview(exec: Option<&WorkbookRowExecution>) -> Option<String> {
    let exec = exec?;
    match exec.result.as_ref()? {
        tf_workbook::WorkbookRowResult::Equation(e) => {
            Some(format!("{}={}", e.solve.output_key, e.solve.value))
        }
        tf_workbook::WorkbookRowResult::Constant(c) => Some(format!("{}", c.value)),
        _ => None,
    }
}

fn default_equation_id(targets: &[StudyTargetDescriptor]) -> String {
    targets
        .iter()
        .find(|t| t.kind == StudyTargetKind::Equation)
        .map(|t| t.id.clone())
        .unwrap_or_else(|| "structures.hoop_stress".to_string())
}

fn default_target_id(targets: &[StudyTargetDescriptor], kind: StudyTargetKind) -> String {
    targets
        .iter()
        .find(|t| t.kind == kind)
        .map(|t| t.id.clone())
        .unwrap_or_default()
}

fn display_target_name(targets: &[&StudyTargetDescriptor], id: &str) -> String {
    targets
        .iter()
        .find(|t| t.id == id)
        .map(|t| format!("{} ({})", t.name, t.id))
        .unwrap_or_else(|| id.to_string())
}

fn parse_ref_key(expr: &str) -> Option<&str> {
    let t = expr.trim();
    if let Some(k) = t.strip_prefix("ref:")
        && !k.trim().is_empty()
    {
        return Some(k.trim());
    }
    if let Some(k) = t.strip_prefix('@')
        && !k.trim().is_empty()
    {
        return Some(k.trim());
    }
    None
}

fn row_field_id(row_id: &str, field_key: &str) -> egui::Id {
    egui::Id::new(("workbook_row_field", row_id, field_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_field_id_is_stable_and_unique() {
        let a1 = row_field_id("row_a", "P");
        let a2 = row_field_id("row_a", "P");
        let b1 = row_field_id("row_b", "P");
        let a_other = row_field_id("row_a", "t");
        assert_eq!(a1, a2);
        assert_ne!(a1, b1);
        assert_ne!(a1, a_other);
    }
}
