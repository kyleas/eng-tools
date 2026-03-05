use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use egui_dnd::{DragDropConfig, dnd};
use tf_eng::{StudyTargetDescriptor, StudyTargetKind, list_study_targets};
use tf_workbook::{
    ConstantRowContent, EquationSolveRowContent, NarrativeRenderMode, NarrativeRowContent,
    PlotRowContent, StudyRowContent, WorkbookDocument, WorkbookRow, WorkbookRowExecution,
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
                let add_outcome = Self::show_add_toolbar(ui, tab, &targets);
                if add_outcome.changed {
                    self.last_error = None;
                    if self.auto_run && add_outcome.changed_unfrozen {
                        run_after = true;
                    }
                }
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt("workbook_rows_scroll")
                    .show(ui, |ui| {
                        let outcome = Self::show_tab_editor(
                            ui,
                            tab,
                            run_tab,
                            &targets,
                            &mut self.picker_queries,
                            focus_row_id.as_deref(),
                        );
                        if outcome.changed {
                            self.last_error = None;
                            if self.auto_run && outcome.changed_unfrozen {
                                run_after = true;
                            }
                        }
                    });
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

            if ui.button("Create").clicked() {
                if let Some(parent) = rfd::FileDialog::new().pick_folder() {
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
            }

            if ui.button("Save").clicked() {
                if let Some(doc) = &self.workbook {
                    if let Err(e) = save_workbook_dir(doc) {
                        self.last_error = Some(e.to_string());
                    }
                }
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

    fn show_add_toolbar(
        ui: &mut egui::Ui,
        tab: &mut WorkbookTab,
        targets: &[StudyTargetDescriptor],
    ) -> EditOutcome {
        let mut outcome = EditOutcome::default();
        ui.horizontal_wrapped(|ui| {
            ui.heading(format!("Tab: {}", tab.name));
            if ui.button("Add narrative").clicked() {
                tab.rows.push(new_row(
                    WorkbookRowKind::Narrative(NarrativeRowContent {
                        content: String::new(),
                        render_mode: NarrativeRenderMode::Plain,
                        style: Default::default(),
                    }),
                    false,
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
        outcome
    }

    fn show_tab_editor(
        ui: &mut egui::Ui,
        tab: &mut WorkbookTab,
        run_tab: Option<&tf_workbook::WorkbookTabExecution>,
        targets: &[StudyTargetDescriptor],
        picker_queries: &mut HashMap<String, String>,
        focus_row_id: Option<&str>,
    ) -> EditOutcome {
        let mut outcome = EditOutcome::default();
        let row_exec_map = run_tab
            .map(|t| {
                t.rows
                    .iter()
                    .map(|r| (r.id.clone(), r.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let mut remove_id: Option<String> = None;
        let mut duplicate_id: Option<String> = None;
        let mut row_changed_ids: Vec<String> = Vec::new();
        let before_order = tab.rows.iter().map(|r| r.id.clone()).collect::<Vec<_>>();
        let key_snapshot = tab
            .rows
            .iter()
            .map(|r| {
                (
                    r.id.clone(),
                    r.key.clone(),
                    matches!(r.kind, WorkbookRowKind::Study(_)),
                )
            })
            .collect::<Vec<_>>();

        let mut row_order = before_order.clone();
        dnd(ui, ("workbook_rows", &tab.file))
            .with_mouse_config(DragDropConfig::mouse())
            .show_vec(&mut row_order, |ui, row_id, handle, _state| {
                let Some(row_idx) = tab.rows.iter().position(|r| r.id == *row_id) else {
                    return;
                };
                let row = &mut tab.rows[row_idx];
                let exec = row_exec_map.get(&row.id);
                let frozen_before = row.freeze;
                let row_scope_id = row.id.clone();
                ui.push_id(row_scope_id, |ui| {
                    egui::Frame::group(ui.style().as_ref()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            handle.ui(ui, |ui| {
                                ui.label("⋮⋮");
                            });
                            let chevron = if row.collapsed { "▸" } else { "▾" };
                            if ui.button(chevron).clicked() {
                                row.collapsed = !row.collapsed;
                                row_changed_ids.push(row.id.clone());
                            }
                            ui.label(row_header_title(row, targets));
                            ui.small(row_type_badge(&row.kind));
                            ui.colored_label(
                                status_color(exec),
                                exec.map(|e| format!("{:?}", e.state))
                                    .unwrap_or_else(|| "not-run".to_string()),
                            );
                            if let Some(preview) = output_preview(exec) {
                                ui.small(preview);
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("del").clicked() {
                                        remove_id = Some(row.id.clone());
                                    }
                                    if ui.small_button("dup").clicked() {
                                        duplicate_id = Some(row.id.clone());
                                    }
                                    ui.checkbox(&mut row.freeze, "freeze");
                                },
                            );
                        });

                        if focus_row_id == Some(row.id.as_str()) {
                            ui.scroll_to_cursor(Some(egui::Align::Center));
                        }

                        if !row.collapsed {
                            ui.separator();
                            let mut row_changed = match &mut row.kind {
                                WorkbookRowKind::Narrative(c) => Self::render_narrative_row(ui, c),
                                WorkbookRowKind::Constant(c) => Self::render_constant_row(ui, c),
                                WorkbookRowKind::EquationSolve(c) => Self::render_equation_row(
                                    ui,
                                    row.id.as_str(),
                                    c,
                                    targets,
                                    picker_queries,
                                ),
                                WorkbookRowKind::Study(c) => Self::render_study_row(
                                    ui,
                                    row.id.as_str(),
                                    c,
                                    targets,
                                    picker_queries,
                                ),
                                WorkbookRowKind::Plot(c) => {
                                    let source_options = key_snapshot
                                        .iter()
                                        .filter(|(id, key, is_study)| {
                                            id != &row.id && key.is_some() && *is_study
                                        })
                                        .map(|(_, key, _)| {
                                            let key = key.as_deref().unwrap_or_default();
                                            PlotSourceOption {
                                                ref_value: format!("ref:{}", key),
                                                label: key.to_string(),
                                            }
                                        })
                                        .collect::<Vec<_>>();
                                    let valid_keys = key_snapshot
                                        .iter()
                                        .filter_map(|(_, key, _)| key.clone())
                                        .collect::<Vec<_>>();
                                    Self::render_plot_row(
                                        ui,
                                        row.id.as_str(),
                                        c,
                                        &source_options,
                                        &valid_keys,
                                        picker_queries,
                                    )
                                }
                                WorkbookRowKind::Text(c) => {
                                    let mut n = NarrativeRowContent {
                                        content: c.text.clone(),
                                        render_mode: NarrativeRenderMode::Plain,
                                        style: Default::default(),
                                    };
                                    let changed = Self::render_narrative_row(ui, &mut n);
                                    if changed {
                                        c.text = n.content;
                                    }
                                    changed
                                }
                                WorkbookRowKind::Markdown(c) => {
                                    let mut n = NarrativeRowContent {
                                        content: c.markdown.clone(),
                                        render_mode: NarrativeRenderMode::Markdown,
                                        style: Default::default(),
                                    };
                                    let changed = Self::render_narrative_row(ui, &mut n);
                                    if changed {
                                        c.markdown = n.content;
                                    }
                                    changed
                                }
                            };
                            if row.freeze != frozen_before {
                                row_changed = true;
                            }
                            if row_changed {
                                row_changed_ids.push(row.id.clone());
                            }
                            ui.collapsing("Advanced", |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Key (optional)");
                                    if ui
                                        .text_edit_singleline(
                                            row.key.get_or_insert_with(String::new),
                                        )
                                        .changed()
                                    {
                                        row_changed_ids.push(row.id.clone());
                                    }
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Title (optional)");
                                    if ui
                                        .text_edit_singleline(
                                            row.title.get_or_insert_with(String::new),
                                        )
                                        .changed()
                                    {
                                        row_changed_ids.push(row.id.clone());
                                    }
                                });
                            });
                            Self::render_row_result(ui, exec);
                        }
                    });
                    ui.add_space(8.0);
                });
            });

        if row_order != before_order {
            let mut by_id = tab
                .rows
                .drain(..)
                .map(|r| (r.id.clone(), r))
                .collect::<HashMap<_, _>>();
            let mut reordered = Vec::with_capacity(row_order.len());
            for id in &row_order {
                if let Some(row) = by_id.remove(id) {
                    reordered.push(row);
                }
            }
            tab.rows = reordered;
        }
        let after_order = row_order;
        if after_order != before_order {
            outcome.changed = true;
            outcome.changed_unfrozen = true;
        }

        if let Some(id) = duplicate_id {
            if let Some(idx) = tab.rows.iter().position(|r| r.id == id) {
                let mut copy = tab.rows[idx].clone();
                copy.id = Uuid::new_v4().to_string();
                copy.collapsed = true;
                tab.rows.insert(idx + 1, copy);
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
        }
        if let Some(id) = remove_id {
            if let Some(idx) = tab.rows.iter().position(|r| r.id == id) {
                tab.rows.remove(idx);
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
        }
        if !row_changed_ids.is_empty() {
            outcome.changed = true;
            if tab.rows.iter().any(|r| !r.freeze) {
                outcome.changed_unfrozen = true;
            }
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

        if let Some(k) = parse_ref_key(&c.source_row) {
            if !valid_keys.iter().any(|existing| existing == k) {
                ui.colored_label(egui::Color32::RED, format!("unknown source key '{}'", k));
            }
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

    fn render_narrative_row(ui: &mut egui::Ui, c: &mut NarrativeRowContent) -> bool {
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            ui.label("Mode");
            changed |= ui
                .selectable_value(&mut c.render_mode, NarrativeRenderMode::Plain, "plain")
                .changed();
            changed |= ui
                .selectable_value(
                    &mut c.render_mode,
                    NarrativeRenderMode::Markdown,
                    "markdown",
                )
                .changed();
            ui.separator();
            changed |= ui.checkbox(&mut c.style.header, "Header").changed();
            changed |= ui.checkbox(&mut c.style.mono, "Mono").changed();
            changed |= ui.checkbox(&mut c.style.muted, "Muted").changed();
        });
        let mut edit = egui::TextEdit::multiline(&mut c.content)
            .desired_rows(10)
            .lock_focus(true);
        if c.style.mono {
            edit = edit.code_editor();
        }
        changed |= ui.add(edit).changed();
        if c.render_mode == NarrativeRenderMode::Markdown {
            ui.separator();
            ui.label(egui::RichText::new("Preview").italics());
            render_markdown_preview(ui, &c.content, c.style.header, c.style.muted);
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
                        tf_workbook::WorkbookRowResult::Narrative { content, .. } => {
                            ui.small(content);
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

fn render_markdown_preview(ui: &mut egui::Ui, markdown: &str, header: bool, muted: bool) {
    let mut in_code = false;
    for line in markdown.lines() {
        let trimmed = line.trim_end();
        if trimmed.starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            ui.label(egui::RichText::new(trimmed).monospace());
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("### ") {
            ui.label(egui::RichText::new(rest).strong());
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("## ") {
            ui.label(egui::RichText::new(rest).size(18.0).strong());
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("# ") {
            ui.label(egui::RichText::new(rest).size(20.0).strong());
            continue;
        }
        let mut rich = egui::RichText::new(trimmed);
        if header {
            rich = rich.size(18.0).strong();
        }
        if muted {
            rich = rich.color(egui::Color32::GRAY);
        }
        ui.label(rich);
    }
}

fn row_type_badge(kind: &WorkbookRowKind) -> &'static str {
    match kind {
        WorkbookRowKind::Narrative(_) | WorkbookRowKind::Text(_) | WorkbookRowKind::Markdown(_) => {
            "narrative"
        }
        WorkbookRowKind::Constant(_) => "constant",
        WorkbookRowKind::EquationSolve(_) => "equation",
        WorkbookRowKind::Study(_) => "study",
        WorkbookRowKind::Plot(_) => "plot",
    }
}

fn row_header_title(row: &WorkbookRow, targets: &[StudyTargetDescriptor]) -> String {
    if let Some(key) = &row.key {
        if !key.trim().is_empty() {
            return key.clone();
        }
    }
    if let Some(title) = &row.title {
        if !title.trim().is_empty() {
            return title.clone();
        }
    }
    match &row.kind {
        WorkbookRowKind::Narrative(_) | WorkbookRowKind::Text(_) | WorkbookRowKind::Markdown(_) => {
            "Narrative".to_string()
        }
        WorkbookRowKind::Constant(_) => "Constant".to_string(),
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
    if let Some(k) = t.strip_prefix("ref:") {
        if !k.trim().is_empty() {
            return Some(k.trim());
        }
    }
    if let Some(k) = t.strip_prefix('@') {
        if !k.trim().is_empty() {
            return Some(k.trim());
        }
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
    fn duplicate_rows_must_receive_new_ids() {
        let row_a = new_row(
            WorkbookRowKind::Narrative(NarrativeRowContent {
                content: "a".to_string(),
                render_mode: NarrativeRenderMode::Plain,
                style: Default::default(),
            }),
            false,
        );
        let mut row_b = row_a.clone();
        row_b.id = Uuid::new_v4().to_string();
        assert_ne!(row_a.id, row_b.id);
    }
}
