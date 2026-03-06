use std::collections::HashMap;
use std::path::Path;

use crate::ui::drag_reorder::show_reorderable_ids;
use crate::ui::search_picker::{PickerOption, searchable_picker};
use tf_eng::{StudyTargetDescriptor, StudyTargetKind, list_study_targets};
use tf_workbook::{
    ConstantRowContent, EquationSolveRowContent, PlotRowContent, StudyRowContent, TextRenderMode,
    TextRowContent, WorkbookDocument, WorkbookRow, WorkbookRowExecution, WorkbookRowKind,
    WorkbookRunResult, WorkbookSweepAxis, WorkbookTab, create_workbook_skeleton, execute_workbook,
    load_workbook_dir, save_workbook_dir,
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
    selected_row_id: Option<String>,
    confirm_delete_selected: bool,
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
            selected_row_id: None,
            confirm_delete_selected: false,
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
                let add_outcome = Self::show_top_toolbar(
                    ui,
                    tab,
                    &targets,
                    &mut self.selected_row_id,
                    &mut self.confirm_delete_selected,
                );
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
                            &mut self.selected_row_id,
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

    fn show_top_toolbar(
        ui: &mut egui::Ui,
        tab: &mut WorkbookTab,
        targets: &[StudyTargetDescriptor],
        selected_row_id: &mut Option<String>,
        confirm_delete_selected: &mut bool,
    ) -> EditOutcome {
        let mut outcome = EditOutcome::default();
        ui.horizontal_wrapped(|ui| {
            ui.heading(format!("Tab: {}", tab.name));
            ui.menu_button("Add row", |ui| {
                if ui.button("Text").clicked() {
                    let row = new_row(
                        WorkbookRowKind::Text(TextRowContent {
                            content: String::new(),
                            render_mode: TextRenderMode::Plain,
                            style: Default::default(),
                            legacy_header: false,
                            legacy_mono: false,
                        }),
                        false,
                    );
                    *selected_row_id = Some(row.id.clone());
                    tab.rows.push(row);
                    outcome.changed = true;
                    outcome.changed_unfrozen = true;
                    ui.close_menu();
                }
                if ui.button("Constant").clicked() {
                    let row = new_row(
                        WorkbookRowKind::Constant(ConstantRowContent {
                            value: String::new(),
                            dimension_hint: None,
                        }),
                        true,
                    );
                    *selected_row_id = Some(row.id.clone());
                    tab.rows.push(row);
                    outcome.changed = true;
                    outcome.changed_unfrozen = true;
                    ui.close_menu();
                }
                if ui.button("Equation").clicked() {
                    let row = new_row(
                        WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                            path_id: default_equation_id(targets),
                            target: None,
                            branch: None,
                            inputs: Default::default(),
                        }),
                        true,
                    );
                    *selected_row_id = Some(row.id.clone());
                    tab.rows.push(row);
                    outcome.changed = true;
                    outcome.changed_unfrozen = true;
                    ui.close_menu();
                }
                if ui.button("Study").clicked() {
                    let row = new_row(
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
                    );
                    *selected_row_id = Some(row.id.clone());
                    tab.rows.push(row);
                    outcome.changed = true;
                    outcome.changed_unfrozen = true;
                    ui.close_menu();
                }
                if ui.button("Plot").clicked() {
                    let row = new_row(
                        WorkbookRowKind::Plot(PlotRowContent {
                            source_row: String::new(),
                            x: String::new(),
                            y: String::new(),
                            title: Some("Plot".to_string()),
                            x_label: None,
                            y_label: None,
                        }),
                        true,
                    );
                    *selected_row_id = Some(row.id.clone());
                    tab.rows.push(row);
                    outcome.changed = true;
                    outcome.changed_unfrozen = true;
                    ui.close_menu();
                }
            });

            let selected_idx = selected_row_id
                .as_ref()
                .and_then(|id| tab.rows.iter().position(|r| &r.id == id));
            let has_selected = selected_idx.is_some();

            if ui
                .add_enabled(has_selected, egui::Button::new("Duplicate"))
                .clicked()
                && let Some(idx) = selected_idx
            {
                let mut copy = tab.rows[idx].clone();
                copy.id = Uuid::new_v4().to_string();
                copy.collapsed = true;
                *selected_row_id = Some(copy.id.clone());
                tab.rows.insert(idx + 1, copy);
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }

            if ui
                .add_enabled(has_selected, egui::Button::new("Delete"))
                .clicked()
            {
                *confirm_delete_selected = true;
            }
            if *confirm_delete_selected {
                ui.colored_label(egui::Color32::YELLOW, "Delete selected row?");
                if ui.button("Confirm").clicked()
                    && let Some(idx) = selected_idx
                {
                    tab.rows.remove(idx);
                    *selected_row_id = None;
                    *confirm_delete_selected = false;
                    outcome.changed = true;
                    outcome.changed_unfrozen = true;
                }
                if ui.button("Cancel").clicked() {
                    *confirm_delete_selected = false;
                }
            }

            if ui
                .add_enabled(has_selected, egui::Button::new("Move Up"))
                .clicked()
                && let Some(idx) = selected_idx
                && idx > 0
            {
                tab.rows.swap(idx, idx - 1);
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }
            if ui
                .add_enabled(has_selected, egui::Button::new("Move Down"))
                .clicked()
                && let Some(idx) = selected_idx
                && idx + 1 < tab.rows.len()
            {
                tab.rows.swap(idx, idx + 1);
                outcome.changed = true;
                outcome.changed_unfrozen = true;
            }

            if ui.button("Collapse all").clicked() {
                for row in &mut tab.rows {
                    row.collapsed = true;
                }
                outcome.changed = true;
            }
            if ui.button("Expand all").clicked() {
                for row in &mut tab.rows {
                    row.collapsed = false;
                }
                outcome.changed = true;
            }
        });
        outcome
    }

    fn show_tab_editor(
        ui: &mut egui::Ui,
        tab: &mut WorkbookTab,
        run_tab: Option<&tf_workbook::WorkbookTabExecution>,
        targets: &[StudyTargetDescriptor],
        selected_row_id: &mut Option<String>,
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
        show_reorderable_ids(
            ui,
            ("workbook_rows", &tab.file),
            &mut row_order,
            |ui, row_id, handle| {
                let Some(row_idx) = tab.rows.iter().position(|r| r.id == *row_id) else {
                    return;
                };
                let row = &mut tab.rows[row_idx];
                let exec = row_exec_map.get(&row.id);
                let frozen_before = row.freeze;
                let row_scope_id = row.id.clone();
                let is_selected = selected_row_id
                    .as_ref()
                    .map(|id| id == &row.id)
                    .unwrap_or(false);
                ui.push_id(row_scope_id, |ui| {
                    let frame = egui::Frame::group(ui.style().as_ref());
                    let row_response = handle.ui(ui, |ui| {
                        frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if is_selected {
                                    ui.label(egui::RichText::new("[selected]").small());
                                }
                                let status = status_color(exec);
                                let status_dot = egui::RichText::new("●").color(status);
                                let resp = ui.label(status_dot);
                                let hover_text = exec
                                    .map(|e| {
                                        let first = e.messages.first().cloned().unwrap_or_default();
                                        if first.is_empty() {
                                            format!("{:?}", e.state)
                                        } else {
                                            format!("{:?}: {}", e.state, first)
                                        }
                                    })
                                    .unwrap_or_else(|| "Not run".to_string());
                                resp.on_hover_text(hover_text);
                                let chevron = if row.collapsed { ">" } else { "v" };
                                if ui.button(chevron).clicked() {
                                    row.collapsed = !row.collapsed;
                                    row_changed_ids.push(row.id.clone());
                                }
                                ui.label(row_summary_preview(row, exec, targets));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let mut current_kind =
                                            row_type_badge(&row.kind).to_string();
                                        let kind_items = vec![
                                            PickerOption {
                                                id: "text".to_string(),
                                                label: "Text".to_string(),
                                                group: None,
                                            },
                                            PickerOption {
                                                id: "constant".to_string(),
                                                label: "Constant".to_string(),
                                                group: None,
                                            },
                                            PickerOption {
                                                id: "equation".to_string(),
                                                label: "Equation".to_string(),
                                                group: None,
                                            },
                                            PickerOption {
                                                id: "study".to_string(),
                                                label: "Study".to_string(),
                                                group: None,
                                            },
                                            PickerOption {
                                                id: "plot".to_string(),
                                                label: "Plot".to_string(),
                                                group: None,
                                            },
                                        ];
                                        if let Some(next) = searchable_picker(
                                            ui,
                                            egui::Id::new(("row_kind", row.id.as_str())),
                                            &mut current_kind,
                                            &kind_items,
                                            picker_queries,
                                        ) {
                                            row.kind = default_row_kind_for_picker(&next, targets);
                                            row_changed_ids.push(row.id.clone());
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
                                    WorkbookRowKind::Text(c) => Self::render_text_row(ui, c),
                                    WorkbookRowKind::Constant(c) => {
                                        Self::render_constant_row(ui, c)
                                    }
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
                                    WorkbookRowKind::Markdown(c) => {
                                        let mut t =
                                            TextRowContent::from_markdown(c.markdown.clone());
                                        let changed = Self::render_text_row(ui, &mut t);
                                        if changed {
                                            c.markdown = t.content;
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
                    });

                    if row_response.clicked() {
                        *selected_row_id = Some(row.id.clone());
                        if row.collapsed {
                            row.collapsed = false;
                            row_changed_ids.push(row.id.clone());
                        }
                    }
                    ui.add_space(8.0);
                });
            },
        );

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
            .map(|t| PickerOption {
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
                .map(|out| PickerOption {
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
                    .map(|b| PickerOption {
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
            .map(|t| PickerOption {
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
                .map(|f| PickerOption {
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
            .map(|o| PickerOption {
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
                .map(|src| PickerOption {
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

    fn render_text_row(ui: &mut egui::Ui, c: &mut TextRowContent) -> bool {
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            changed |= ui
                .selectable_value(&mut c.render_mode, TextRenderMode::Plain, "plain")
                .changed();
            changed |= ui
                .selectable_value(&mut c.render_mode, TextRenderMode::EasyMark, "easymark")
                .changed();
            changed |= ui.checkbox(&mut c.style.header, "Header").changed();
            changed |= ui.checkbox(&mut c.style.mono, "Mono").changed();
            changed |= ui.checkbox(&mut c.style.muted, "Muted").changed();
        });
        let editor_id = egui::Id::new(("workbook_text_editor", c.content.as_ptr() as usize));
        let mut edit = egui::TextEdit::multiline(&mut c.content)
            .desired_rows(8)
            .id(editor_id)
            .lock_focus(true);
        if c.style.mono {
            edit = edit.code_editor();
        }
        let response = ui.add(edit);
        changed |= response.changed();
        if response.has_focus() && apply_text_shortcuts(ui, &mut c.content, response.id) {
            changed = true;
        }
        if c.render_mode == TextRenderMode::EasyMark {
            ui.separator();
            ui.label(egui::RichText::new("Preview").italics());
            render_easymark_preview(ui, &c.content, c.style.header, c.style.muted);
        }
        changed
    }

    fn render_constant_row(ui: &mut egui::Ui, c: &mut ConstantRowContent) -> bool {
        let mut changed = false;
        ui.label("Value");
        changed |= ui
            .add(
                egui::TextEdit::multiline(&mut c.value)
                    .desired_rows(3)
                    .code_editor()
                    .lock_focus(true),
            )
            .changed();
        ui.small("number or any-unit string");
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
                        tf_workbook::WorkbookRowResult::Text { content, .. } => {
                            ui.label(content);
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

fn render_easymark_preview(ui: &mut egui::Ui, markdown: &str, header: bool, muted: bool) {
    // EasyMark provides the closest "live script" rich text experience in egui without custom renderers.
    if header || muted {
        let mut style = (*ui.ctx().style()).clone();
        if header {
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(18.0, egui::FontFamily::Proportional),
            );
        }
        if muted {
            style.visuals.override_text_color = Some(egui::Color32::GRAY);
        }
        ui.scope(|ui| {
            ui.set_style(style);
            egui_demo_lib::easy_mark::easy_mark(ui, markdown);
        });
        return;
    }
    egui_demo_lib::easy_mark::easy_mark(ui, markdown);
}

fn apply_text_shortcuts(ui: &egui::Ui, content: &mut String, editor_id: egui::Id) -> bool {
    if !ui.memory(|m| m.has_focus(editor_id)) {
        return false;
    }
    let mut changed = false;
    let commands = [
        (egui::Key::B, "**"),
        (egui::Key::I, "*"),
        (egui::Key::Backtick, "`"),
    ];
    for (key, marker) in commands {
        let pressed = ui.input(|i| i.modifiers.command && i.key_pressed(key));
        if pressed {
            content.push_str(marker);
            content.push_str(marker);
            changed = true;
        }
    }
    changed
}

fn preview_text_line(text: &str) -> String {
    for line in text.lines() {
        let t = line.trim();
        if !t.is_empty() {
            return t.chars().take(70).collect::<String>();
        }
    }
    "Text".to_string()
}

fn preview_plot_source(source: &str) -> String {
    parse_ref_key(source).unwrap_or(source).to_string()
}

fn row_summary_preview(
    row: &WorkbookRow,
    exec: Option<&WorkbookRowExecution>,
    targets: &[StudyTargetDescriptor],
) -> String {
    let key = row.key.clone().unwrap_or_else(|| "row".to_string());
    if let Some(exec) = exec {
        if let Some(result) = &exec.result {
            match result {
                tf_workbook::WorkbookRowResult::Equation(e) => {
                    return format!("{} = {} {}", key, e.solve.output_key, e.solve.value);
                }
                tf_workbook::WorkbookRowResult::Constant(c) => {
                    return format!("{} = {}", key, c.value);
                }
                tf_workbook::WorkbookRowResult::Study(s) => {
                    let ok = s.meta.n_ok;
                    let err = s.meta.n_fail;
                    return format!(
                        "{} = Study(n={}, ok={}, err={})",
                        key,
                        s.table.rows.len(),
                        ok,
                        err
                    );
                }
                tf_workbook::WorkbookRowResult::Plot(p) => {
                    if let Some(first) = p.series.first() {
                        return format!("{} = Plot({})", key, first.name);
                    }
                    return format!("{} = Plot", key);
                }
                tf_workbook::WorkbookRowResult::Text { content, .. } => {
                    return preview_text_line(content);
                }
            }
        }
    }
    match &row.kind {
        WorkbookRowKind::Text(c) => preview_text_line(&c.content),
        WorkbookRowKind::Constant(c) => format!("{} = {}", key, c.value),
        WorkbookRowKind::EquationSolve(c) => targets
            .iter()
            .find(|t| t.id == c.path_id)
            .map(|t| format!("{} ({})", t.name, c.path_id))
            .unwrap_or_else(|| c.path_id.clone()),
        WorkbookRowKind::Study(c) => format!("Study {}", c.target_id),
        WorkbookRowKind::Plot(c) => format!(
            "Plot {} vs {} from {}",
            c.y,
            c.x,
            preview_plot_source(&c.source_row)
        ),
        WorkbookRowKind::Markdown(c) => preview_text_line(&c.markdown),
    }
}

fn default_row_kind_for_picker(next: &str, targets: &[StudyTargetDescriptor]) -> WorkbookRowKind {
    match next {
        "text" => WorkbookRowKind::Text(TextRowContent {
            content: String::new(),
            render_mode: TextRenderMode::Plain,
            style: Default::default(),
            legacy_header: false,
            legacy_mono: false,
        }),
        "constant" => WorkbookRowKind::Constant(ConstantRowContent {
            value: String::new(),
            dimension_hint: None,
        }),
        "equation" => WorkbookRowKind::EquationSolve(EquationSolveRowContent {
            path_id: default_equation_id(targets),
            target: None,
            branch: None,
            inputs: Default::default(),
        }),
        "study" => WorkbookRowKind::Study(StudyRowContent {
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
        "plot" => WorkbookRowKind::Plot(PlotRowContent {
            source_row: String::new(),
            x: String::new(),
            y: String::new(),
            title: Some("Plot".to_string()),
            x_label: None,
            y_label: None,
        }),
        _ => WorkbookRowKind::Text(TextRowContent {
            content: String::new(),
            render_mode: TextRenderMode::Plain,
            style: Default::default(),
            legacy_header: false,
            legacy_mono: false,
        }),
    }
}

fn row_type_badge(kind: &WorkbookRowKind) -> &'static str {
    match kind {
        WorkbookRowKind::Text(_) | WorkbookRowKind::Markdown(_) => "text",
        WorkbookRowKind::Constant(_) => "constant",
        WorkbookRowKind::EquationSolve(_) => "equation",
        WorkbookRowKind::Study(_) => "study",
        WorkbookRowKind::Plot(_) => "plot",
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
    use crate::ui::search_picker::filter_options;

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
            PickerOption {
                id: "compressible.isentropic_pressure_ratio".to_string(),
                label: "Isentropic Pressure Ratio".to_string(),
                group: Some("Compressible".to_string()),
            },
            PickerOption {
                id: "structures.hoop_stress".to_string(),
                label: "Thin-Wall Hoop Stress".to_string(),
                group: Some("Structures".to_string()),
            },
        ];
        let filtered = filter_options(&opts, "hoop");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "structures.hoop_stress");
    }

    #[test]
    fn duplicate_rows_must_receive_new_ids() {
        let row_a = new_row(
            WorkbookRowKind::Text(TextRowContent {
                content: "a".to_string(),
                render_mode: TextRenderMode::Plain,
                style: Default::default(),
                legacy_header: false,
                legacy_mono: false,
            }),
            false,
        );
        let mut row_b = row_a.clone();
        row_b.id = Uuid::new_v4().to_string();
        assert_ne!(row_a.id, row_b.id);
    }
}
