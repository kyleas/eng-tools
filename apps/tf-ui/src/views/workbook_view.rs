use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use tf_eng::{StudyTargetDescriptor, StudyTargetKind, list_study_targets};
use tf_workbook::{
    ConstantRowContent, EquationSolveRowContent, MarkdownRowContent, PlotRowContent,
    StudyRowContent, TextRowContent, WorkbookDocument, WorkbookRow, WorkbookRowExecution,
    WorkbookRowKind, WorkbookRunResult, WorkbookSweepAxis, WorkbookTab, create_workbook_skeleton,
    execute_workbook, load_workbook_dir, save_workbook_dir,
};
use uuid::Uuid;

pub struct WorkbookView {
    workbook_path: String,
    new_workbook_name: String,
    workbook: Option<WorkbookDocument>,
    selected_tab: usize,
    run_result: Option<WorkbookRunResult>,
    auto_run: bool,
    show_execution_results: bool,
    last_error: Option<String>,
    focus_row_id: Option<String>,
    dragging_row_id: Option<String>,
    picker_queries: HashMap<String, String>,
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

#[derive(Clone)]
struct OptionItem {
    id: String,
    label: String,
    group: Option<String>,
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
            show_execution_results: false,
            last_error: None,
            focus_row_id: None,
            dragging_row_id: None,
            picker_queries: HashMap::new(),
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
            let focus_row_id = self.focus_row_id.clone();
            if let Some(tab) = doc.tabs.get_mut(self.selected_tab) {
                let outcome = Self::show_tab_editor(
                    ui,
                    tab,
                    run_tab,
                    &targets,
                    &mut self.dragging_row_id,
                    &mut self.picker_queries,
                    focus_row_id.as_deref(),
                );
                if outcome.changed {
                    self.last_error = None;
                    if self.auto_run && outcome.changed_unfrozen {
                        run_after = true;
                    }
                }
            }
            self.focus_row_id = None;
        }

        if run_after {
            self.run_now();
        }

        if let Some(run) = self.run_result.clone() {
            self.show_results(ui, &run);
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
        dragging_row_id: &mut Option<String>,
        picker_queries: &mut HashMap<String, String>,
        focus_row_id: Option<&str>,
    ) -> EditOutcome {
        let mut outcome = EditOutcome::default();

        ui.separator();
        ui.horizontal(|ui| {
            ui.heading(format!("Tab: {}", tab.name));
            if ui.button("Add text").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Text(TextRowContent {
                        text: String::new(),
                        header: false,
                        mono: false,
                    }),
                    false,
                ));
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            if ui.button("Add markdown").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Markdown(MarkdownRowContent {
                        markdown: String::new(),
                        preview: false,
                    }),
                    true,
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
                    true,
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
                    true,
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
                    true,
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
                    true,
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
        let mut drag_drop_insert_index: Option<usize> = None;
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

            let group_response = ui.push_id(row.id.clone(), |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let drag_resp = ui.add(
                            egui::Label::new(egui::RichText::new("⋮⋮").strong())
                                .sense(egui::Sense::drag()),
                        );
                        if drag_resp.drag_started() {
                            *dragging_row_id = Some(row.id.clone());
                        }
                        let chevron = if collapsed { "▸" } else { "▾" };
                        if ui.button(chevron).clicked() {
                            row.collapsed = !row.collapsed;
                            outcome.changed = true;
                            row_changed = true;
                        }
                        ui.label(egui::RichText::new(row_header_title(row, targets)).strong());
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
                            ui.checkbox(&mut row.freeze, "freeze");
                            if row.freeze != frozen_before {
                                outcome.changed = true;
                                row_changed = true;
                            }
                        });

                        match &mut row.kind {
                            WorkbookRowKind::Text(c) => {
                                row_changed |= Self::render_text_row(ui, c);
                            }
                            WorkbookRowKind::Markdown(c) => {
                                row_changed |= Self::render_markdown_row(ui, c);
                            }
                            WorkbookRowKind::Constant(c) => {
                                row_changed |= Self::render_constant_row(ui, c);
                            }
                            WorkbookRowKind::EquationSolve(c) => {
                                row_changed |= Self::render_equation_row(
                                    ui,
                                    row.id.as_str(),
                                    c,
                                    targets,
                                    picker_queries,
                                );
                            }
                            WorkbookRowKind::Study(c) => {
                                row_changed |= Self::render_study_row(
                                    ui,
                                    row.id.as_str(),
                                    c,
                                    targets,
                                    picker_queries,
                                );
                            }
                            WorkbookRowKind::Plot(c) => {
                                row_changed |= Self::render_plot_row(
                                    ui,
                                    row.id.as_str(),
                                    c,
                                    &plot_source_options,
                                    &valid_keys,
                                    picker_queries,
                                );
                            }
                        }
                        if matches!(
                            row.kind,
                            WorkbookRowKind::Text(_)
                                | WorkbookRowKind::Markdown(_)
                                | WorkbookRowKind::Constant(_)
                        ) {
                            ui.collapsing("Advanced", |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Key (optional)");
                                    row_changed |= ui
                                        .text_edit_singleline(
                                            row.key.get_or_insert_with(String::new),
                                        )
                                        .changed();
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Title (optional)");
                                    row_changed |= ui
                                        .text_edit_singleline(
                                            row.title.get_or_insert_with(String::new),
                                        )
                                        .changed();
                                });
                            });
                        }
                        Self::render_row_result(ui, exec);
                    }
                });
            });
            if focus_row_id == Some(row.id.as_str()) {
                ui.scroll_to_rect(group_response.response.rect, None);
            }
            if let Some(drag_row) = dragging_row_id.as_deref()
                && drag_row != row.id
                && group_response.response.hovered()
            {
                let pointer_y = ui
                    .input(|inp| inp.pointer.hover_pos().map(|p| p.y))
                    .unwrap_or(group_response.response.rect.center().y);
                let insert_idx = if pointer_y > group_response.response.rect.center().y {
                    i + 1
                } else {
                    i
                };
                drag_drop_insert_index = Some(insert_idx);
                let y = if pointer_y > group_response.response.rect.center().y {
                    group_response.response.rect.bottom()
                } else {
                    group_response.response.rect.top()
                };
                ui.painter().line_segment(
                    [
                        egui::pos2(group_response.response.rect.left(), y),
                        egui::pos2(group_response.response.rect.right(), y),
                    ],
                    egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                );
            }
            if row_changed {
                outcome.changed = true;
                if !row.freeze {
                    outcome.changed_unfrozen = true;
                }
            }
            if row.freeze != frozen_before {
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

        if ui.input(|inp| inp.pointer.any_released()) {
            if let Some(dragged) = dragging_row_id.clone()
                && let Some(insert_idx) = drag_drop_insert_index
                && reorder_rows_by_id(&mut tab.rows, &dragged, insert_idx)
            {
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            *dragging_row_id = None;
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
        picker_queries: &mut HashMap<String, String>,
    ) -> bool {
        let mut changed = false;
        let equations = targets
            .iter()
            .filter(|t| t.kind == StudyTargetKind::Equation)
            .map(|t| OptionItem {
                id: t.id.clone(),
                label: t.name.clone(),
                group: t.category.clone(),
            })
            .collect::<Vec<_>>();

        ui.horizontal(|ui| {
            ui.label("Target");
            if let Some(next) = searchable_picker(
                ui,
                egui::Id::new(("eq_target", row_id)),
                &mut c.path_id,
                &equations,
                picker_queries,
            ) {
                c.path_id = next;
                changed = true;
            }
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
            let output_items = desc
                .outputs
                .iter()
                .map(|out| OptionItem {
                    id: out.key.clone(),
                    label: out.label.clone(),
                    group: None,
                })
                .collect::<Vec<_>>();
            if let Some(next) = searchable_picker(
                ui,
                egui::Id::new(("eq_output", row_id)),
                &mut target_value,
                &output_items,
                picker_queries,
            ) {
                target_value = next;
                changed = true;
            }
            if ui.small_button("infer").clicked() {
                target_value.clear();
                changed = true;
            }
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
                let branch_items = desc
                    .branch_options
                    .iter()
                    .map(|b| OptionItem {
                        id: b.clone(),
                        label: b.clone(),
                        group: None,
                    })
                    .collect::<Vec<_>>();
                if let Some(next) = searchable_picker(
                    ui,
                    egui::Id::new(("eq_branch", row_id)),
                    &mut branch,
                    &branch_items,
                    picker_queries,
                ) {
                    branch = next;
                    changed = true;
                }
                if ui.small_button("none").clicked() {
                    branch.clear();
                    changed = true;
                }
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
        picker_queries: &mut HashMap<String, String>,
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
            .map(|t| OptionItem {
                id: t.id.clone(),
                label: t.name.clone(),
                group: t.category.clone(),
            })
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            ui.label("Target");
            if let Some(next) = searchable_picker(
                ui,
                egui::Id::new(("study_target", row_id)),
                &mut c.target_id,
                &target_options,
                picker_queries,
            ) {
                c.target_id = next;
                changed = true;
            }
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
            let sweep_items = desc
                .sweepable_fields
                .iter()
                .map(|f| OptionItem {
                    id: f.clone(),
                    label: f.clone(),
                    group: None,
                })
                .collect::<Vec<_>>();
            if let Some(next) = searchable_picker(
                ui,
                egui::Id::new(("study_sweep", row_id)),
                &mut c.sweep_field,
                &sweep_items,
                picker_queries,
            ) {
                c.sweep_field = next;
                changed = true;
            }
        });

        let outputs = desc
            .outputs
            .iter()
            .filter(|o| o.numeric || o.plottable)
            .map(|o| OptionItem {
                id: o.key.clone(),
                label: o.label.clone(),
                group: None,
            })
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            ui.label("Output key");
            if let Some(next) = searchable_picker(
                ui,
                egui::Id::new(("study_output", row_id)),
                &mut c.output_key,
                &outputs,
                picker_queries,
            ) {
                c.output_key = next;
                changed = true;
            }
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
        picker_queries: &mut HashMap<String, String>,
    ) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Source");
            let source_items = source_options
                .iter()
                .map(|src| OptionItem {
                    id: src.ref_value.clone(),
                    label: src.label.clone(),
                    group: Some("Study Rows".to_string()),
                })
                .collect::<Vec<_>>();
            if let Some(next) = searchable_picker(
                ui,
                egui::Id::new(("plot_source", row_id)),
                &mut c.source_row,
                &source_items,
                picker_queries,
            ) {
                c.source_row = next;
                changed = true;
            }
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

    fn render_text_row(ui: &mut egui::Ui, c: &mut TextRowContent) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.checkbox(&mut c.header, "Header");
            ui.checkbox(&mut c.mono, "Mono");
        });
        let mut edit = egui::TextEdit::multiline(&mut c.text)
            .desired_rows(8)
            .lock_focus(true);
        if c.mono {
            edit = edit.code_editor();
        }
        changed |= ui.add(edit).changed();
        if c.header && !c.text.trim().is_empty() {
            ui.label(egui::RichText::new(c.text.trim()).heading());
        }
        changed
    }

    fn render_markdown_row(ui: &mut egui::Ui, c: &mut MarkdownRowContent) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.checkbox(&mut c.preview, "Preview");
        });
        if c.preview {
            ui.label(c.markdown.clone());
        } else {
            changed |= ui.text_edit_multiline(&mut c.markdown).changed();
        }
        changed
    }

    fn render_constant_row(ui: &mut egui::Ui, c: &mut ConstantRowContent) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Value");
            changed |= ui.text_edit_singleline(&mut c.value).changed();
            ui.small("number or any-unit string");
        });
        ui.horizontal(|ui| {
            ui.label("Dimension hint");
            changed |= ui
                .text_edit_singleline(c.dimension_hint.get_or_insert_with(String::new))
                .changed();
        });
        changed
    }

    fn render_row_result(ui: &mut egui::Ui, exec: Option<&WorkbookRowExecution>) {
        ui.separator();
        ui.label(egui::RichText::new("Result").strong());
        let Some(exec) = exec else {
            ui.small("No execution yet");
            return;
        };

        match exec.state {
            tf_workbook::WorkbookRowState::Ok => {
                ui.colored_label(egui::Color32::LIGHT_GREEN, "OK");
                if let Some(result) = &exec.result {
                    match result {
                        tf_workbook::WorkbookRowResult::Equation(e) => {
                            ui.horizontal(|ui| {
                                ui.label(format!("{} = {}", e.solve.output_key, e.solve.value));
                                if ui.small_button("Copy").clicked() {
                                    ui.ctx().copy_text(e.solve.value.to_string());
                                }
                            });
                        }
                        tf_workbook::WorkbookRowResult::Constant(c) => {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", c.value));
                                if ui.small_button("Copy").clicked() {
                                    ui.ctx().copy_text(c.value.to_string());
                                }
                            });
                        }
                        tf_workbook::WorkbookRowResult::Study(s) => {
                            ui.label(format!("study rows: {}", s.table.rows.len()));
                        }
                        tf_workbook::WorkbookRowResult::Plot(p) => {
                            ui.label(format!("series: {}", p.series.len()));
                        }
                        tf_workbook::WorkbookRowResult::Text { text } => {
                            ui.label(text);
                        }
                        tf_workbook::WorkbookRowResult::Markdown { markdown } => {
                            ui.label(markdown);
                        }
                    }
                }
            }
            tf_workbook::WorkbookRowState::Invalid | tf_workbook::WorkbookRowState::Error => {
                ui.colored_label(egui::Color32::LIGHT_RED, format!("{:?}", exec.state));
                if let Some(first) = exec.messages.first() {
                    ui.label(first);
                }
                ui.collapsing("Details", |ui| {
                    for m in &exec.messages {
                        ui.small(m);
                    }
                });
            }
            _ => {
                ui.colored_label(egui::Color32::YELLOW, format!("{:?}", exec.state));
                for m in &exec.messages {
                    ui.small(m);
                }
            }
        }
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

    fn show_results(&mut self, ui: &mut egui::Ui, run: &WorkbookRunResult) {
        ui.separator();
        let mut ok = 0usize;
        let mut invalid = 0usize;
        let mut other = 0usize;
        for tab in &run.tabs {
            for row in &tab.rows {
                match row.state {
                    tf_workbook::WorkbookRowState::Ok => ok += 1,
                    tf_workbook::WorkbookRowState::Invalid
                    | tf_workbook::WorkbookRowState::Error => invalid += 1,
                    _ => other += 1,
                }
            }
        }
        egui::CollapsingHeader::new(format!(
            "Execution results ({} ok / {} invalid / {} other)",
            ok, invalid, other
        ))
        .default_open(self.show_execution_results)
        .show(ui, |ui| {
            for tab in &run.tabs {
                ui.collapsing(format!("{} ({})", tab.name, tab.file), |ui| {
                    for row in &tab.rows {
                        ui.horizontal(|ui| {
                            let label = row
                                .key
                                .clone()
                                .unwrap_or_else(|| row_type_badge_for_state(row));
                            if ui.button(label).clicked() {
                                self.focus_row_id = Some(row.id.clone());
                            }
                            ui.small(format!("{:?}", row.state));
                            if let Some(preview) = output_preview(Some(row)) {
                                ui.small(preview);
                            }
                        });
                    }
                });
            }
        });
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

fn row_header_title(row: &WorkbookRow, targets: &[StudyTargetDescriptor]) -> String {
    if let Some(title) = &row.title
        && !title.trim().is_empty()
    {
        return title.clone();
    }
    match &row.kind {
        WorkbookRowKind::Text(_) => "Text".to_string(),
        WorkbookRowKind::Markdown(_) => "Markdown".to_string(),
        WorkbookRowKind::Constant(_) => row
            .key
            .clone()
            .filter(|k| !k.trim().is_empty())
            .unwrap_or_else(|| "Constant".to_string()),
        WorkbookRowKind::EquationSolve(c) => targets
            .iter()
            .find(|t| t.kind == StudyTargetKind::Equation && t.id == c.path_id)
            .map(|t| format!("Equation: {}", t.name))
            .unwrap_or_else(|| format!("Equation: {}", c.path_id)),
        WorkbookRowKind::Study(c) => format!("Study: {}", c.target_id),
        WorkbookRowKind::Plot(_) => "Plot".to_string(),
    }
}

fn row_type_badge_for_state(row: &WorkbookRowExecution) -> String {
    row.key.clone().unwrap_or_else(|| "row".to_string())
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
        tf_workbook::WorkbookRowResult::Study(s) => Some(format!("{} rows", s.table.rows.len())),
        tf_workbook::WorkbookRowResult::Plot(p) => Some(format!("{} series", p.series.len())),
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

fn searchable_picker(
    ui: &mut egui::Ui,
    id: egui::Id,
    selected_id: &mut String,
    options: &[OptionItem],
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
            let filtered = filter_option_items(options, query);
            let mut grouped: BTreeMap<String, Vec<&OptionItem>> = BTreeMap::new();
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

fn filter_option_items<'a>(items: &'a [OptionItem], query: &str) -> Vec<&'a OptionItem> {
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

fn reorder_rows_by_id(rows: &mut Vec<WorkbookRow>, row_id: &str, insert_index: usize) -> bool {
    let Some(src) = rows.iter().position(|r| r.id == row_id) else {
        return false;
    };
    let row = rows.remove(src);
    let idx = insert_index.min(rows.len());
    rows.insert(idx, row);
    true
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

    #[test]
    fn option_filter_matches_case_insensitive_substrings() {
        let opts = vec![
            OptionItem {
                id: "compressible.isentropic_pressure_ratio".to_string(),
                label: "Isentropic Pressure Ratio".to_string(),
                group: Some("Compressible".to_string()),
            },
            OptionItem {
                id: "structures.hoop_stress".to_string(),
                label: "Thin-Wall Hoop Stress".to_string(),
                group: Some("Structures".to_string()),
            },
        ];
        let filtered = filter_option_items(&opts, "hoop");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "structures.hoop_stress");
    }

    #[test]
    fn reorder_rows_by_id_moves_row_to_insert_position() {
        let mut rows = vec![
            WorkbookRow {
                id: "a".to_string(),
                key: None,
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    text: "a".to_string(),
                    header: false,
                    mono: false,
                }),
            },
            WorkbookRow {
                id: "b".to_string(),
                key: None,
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    text: "b".to_string(),
                    header: false,
                    mono: false,
                }),
            },
            WorkbookRow {
                id: "c".to_string(),
                key: None,
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    text: "c".to_string(),
                    header: false,
                    mono: false,
                }),
            },
        ];
        assert!(reorder_rows_by_id(&mut rows, "a", 3));
        let ids = rows.iter().map(|r| r.id.clone()).collect::<Vec<_>>();
        assert_eq!(ids, vec!["b", "c", "a"]);
    }
}
