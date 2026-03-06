use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use crate::ui::drag_reorder::show_reorderable_ids;
use crate::ui::search_picker::{PickerOption, searchable_picker};
use tf_eng::{StudyTargetDescriptor, StudyTargetKind, list_study_targets};
use tf_workbook::{
    ConstantRowContent, EquationSolveRowContent, PlotRowContent, StudyRowContent, TextRenderMode,
    TextRowContent, WorkbookDocument, WorkbookRow, WorkbookRowExecution, WorkbookRowKind,
    WorkbookRowResult, WorkbookRowState, WorkbookRunResult, WorkbookSweepAxis, WorkbookTab,
    create_workbook_skeleton, execute_workbook, load_workbook_dir, row_requires_key,
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
    show_execution_results: bool,
    last_error: Option<String>,
    focus_row_id: Option<String>,
    selected_row_id: Option<String>,
    confirm_delete_selected: bool,
    picker_queries: HashMap<String, String>,
    targets: Vec<StudyTargetDescriptor>,
    highlighter: egui_demo_lib::easy_mark::MemoizedEasymarkHighlighter,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum RowVisualState {
    NotRun,
    Ok,
    Warning,
    Error,
    Incomplete,
}

struct RowVisualStatus {
    state: RowVisualState,
    message: String,
}

#[derive(Clone, Copy)]
enum TextCommand {
    Strong,
    Italic,
    Code,
    Heading,
    Quote,
    Bullet,
    Numbered,
    Rule,
    Link,
    CodeBlock,
}

struct HeaderOutcome {
    selected: bool,
    toggled: bool,
    kind_changed: bool,
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
            highlighter: Default::default(),
        }
    }
}

impl WorkbookView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Engineering Workbook");
        ui.label("Row-based engineering worksheet (.engwb) powered by tf-workbook + tf-eng.");

        self.toolbar(ui);

        if let Some(err) = &self.last_error {
            ui.colored_label(egui::Color32::LIGHT_RED, err);
        }

        let mut run_after = false;
        if let Some(doc) = &mut self.workbook {
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new(&doc.manifest.title).strong());
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
                let toolbar_outcome = Self::show_top_toolbar(
                    ui,
                    tab,
                    &targets,
                    &mut self.selected_row_id,
                    &mut self.confirm_delete_selected,
                );
                if toolbar_outcome.changed {
                    self.last_error = None;
                    if self.auto_run && toolbar_outcome.changed_unfrozen {
                        run_after = true;
                    }
                }

                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt(("workbook_rows_scroll", &tab.file))
                    .show(ui, |ui| {
                        let outcome = Self::show_tab_editor(
                            ui,
                            tab,
                            run_tab,
                            &targets,
                            &mut self.selected_row_id,
                            &mut self.picker_queries,
                            focus_row_id.as_deref(),
                            &mut self.highlighter,
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
        let selected_idx = selected_row_id
            .as_ref()
            .and_then(|id| tab.rows.iter().position(|row| &row.id == id));
        let has_selected = selected_idx.is_some();

        ui.horizontal_wrapped(|ui| {
            ui.heading(format!("Tab: {}", tab.name));
            ui.menu_button("Add row", |ui| {
                for (label, kind, collapsed) in [
                    (
                        "Text",
                        WorkbookRowKind::Text(TextRowContent {
                            content: String::new(),
                            render_mode: TextRenderMode::EasyMark,
                            style: Default::default(),
                            legacy_header: false,
                            legacy_mono: false,
                        }),
                        false,
                    ),
                    (
                        "Constant",
                        WorkbookRowKind::Constant(ConstantRowContent {
                            value: String::new(),
                            dimension_hint: None,
                        }),
                        true,
                    ),
                    (
                        "Equation",
                        WorkbookRowKind::EquationSolve(EquationSolveRowContent {
                            path_id: default_equation_id(targets),
                            target: None,
                            branch: None,
                            inputs: Default::default(),
                        }),
                        true,
                    ),
                    (
                        "Study",
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
                    ),
                    (
                        "Plot",
                        WorkbookRowKind::Plot(PlotRowContent {
                            source_row: String::new(),
                            x: String::new(),
                            y: String::new(),
                            title: Some("Plot".to_string()),
                            x_label: None,
                            y_label: None,
                        }),
                        true,
                    ),
                ] {
                    if ui.button(label).clicked() {
                        let row = new_row(kind, collapsed);
                        *selected_row_id = Some(row.id.clone());
                        tab.rows.push(row);
                        outcome.changed = true;
                        outcome.changed_unfrozen = true;
                        ui.close_menu();
                    }
                }
            });

            let selected_label = selected_idx
                .and_then(|idx| tab.rows.get(idx))
                .map(row_primary_name)
                .unwrap_or_else(|| "None selected".to_string());
            ui.small(format!("Selected: {}", selected_label));

            if ui
                .add_enabled(has_selected, egui::Button::new("Duplicate"))
                .clicked()
                && let Some(idx) = selected_idx
            {
                let mut copy = tab.rows[idx].clone();
                copy.id = Uuid::new_v4().to_string();
                copy.collapsed = true;
                copy.freeze = false;
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
        highlighter: &mut egui_demo_lib::easy_mark::MemoizedEasymarkHighlighter,
    ) -> EditOutcome {
        let mut outcome = EditOutcome::default();
        let row_exec_map = run_tab
            .map(|tab_exec| {
                tab_exec
                    .rows
                    .iter()
                    .map(|row| (row.id.clone(), row.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let duplicate_keys = duplicate_key_set(&tab.rows);
        let key_snapshot = tab
            .rows
            .iter()
            .map(|row| {
                (
                    row.id.clone(),
                    row.key.clone(),
                    matches!(row.kind, WorkbookRowKind::Study(_)),
                )
            })
            .collect::<Vec<_>>();

        let before_order = tab
            .rows
            .iter()
            .map(|row| row.id.clone())
            .collect::<Vec<_>>();
        let mut row_order = before_order.clone();
        let mut row_changed_ids = BTreeSet::new();

        show_reorderable_ids(
            ui,
            ("workbook_rows", &tab.file),
            &mut row_order,
            |ui, row_id, handle| {
                let Some(row_idx) = tab.rows.iter().position(|row| row.id == *row_id) else {
                    return;
                };
                let row = &mut tab.rows[row_idx];
                let exec = row_exec_map.get(&row.id);
                let is_selected = selected_row_id
                    .as_ref()
                    .map(|selected| selected == &row.id)
                    .unwrap_or(false);
                let visual_status = row_visual_status(row, exec, &duplicate_keys);
                let source_options = key_snapshot
                    .iter()
                    .filter(|(id, key, is_study)| id != &row.id && key.is_some() && *is_study)
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

                let row_scope_id = row.id.clone();
                ui.push_id(("workbook_row", row_scope_id), |ui| {
                    let frame = row_frame(ui, row, is_selected);
                    let frame_response = frame.show(ui, |ui| {
                        let header = Self::show_row_header(
                            ui,
                            row,
                            exec,
                            targets,
                            picker_queries,
                            handle,
                            is_selected,
                            &visual_status,
                        );
                        if header.selected {
                            *selected_row_id = Some(row.id.clone());
                        }
                        if header.toggled {
                            row.collapsed = !row.collapsed;
                            row_changed_ids.insert(row.id.clone());
                        }
                        if header.kind_changed {
                            row_changed_ids.insert(row.id.clone());
                        }

                        if row.collapsed {
                            if matches!(row.kind, WorkbookRowKind::Text(_)) {
                                ui.add_space(6.0);
                                if let WorkbookRowKind::Text(content) = &row.kind {
                                    render_text_document(ui, &content.content);
                                }
                            }
                        } else {
                            ui.add_space(8.0);
                            if !matches!(row.kind, WorkbookRowKind::Text(_)) {
                                if Self::render_row_utilities(ui, row) {
                                    row_changed_ids.insert(row.id.clone());
                                }
                                if let Some(message) = key_validation_message(row, &duplicate_keys)
                                {
                                    ui.colored_label(egui::Color32::LIGHT_RED, message);
                                }
                                ui.separator();
                            }

                            let body_changed = match &mut row.kind {
                                WorkbookRowKind::Text(content) => {
                                    Self::render_text_row(ui, row.id.as_str(), content, highlighter)
                                }
                                WorkbookRowKind::Constant(content) => {
                                    Self::render_constant_row(ui, row.id.as_str(), content)
                                }
                                WorkbookRowKind::EquationSolve(content) => {
                                    Self::render_equation_row(
                                        ui,
                                        row.id.as_str(),
                                        content,
                                        targets,
                                        picker_queries,
                                    )
                                }
                                WorkbookRowKind::Study(content) => Self::render_study_row(
                                    ui,
                                    row.id.as_str(),
                                    content,
                                    targets,
                                    picker_queries,
                                ),
                                WorkbookRowKind::Plot(content) => Self::render_plot_row(
                                    ui,
                                    row.id.as_str(),
                                    content,
                                    &source_options,
                                    &valid_keys,
                                    picker_queries,
                                ),
                                WorkbookRowKind::Markdown(content) => {
                                    let mut text =
                                        TextRowContent::from_markdown(content.markdown.clone());
                                    let changed = Self::render_text_row(
                                        ui,
                                        row.id.as_str(),
                                        &mut text,
                                        highlighter,
                                    );
                                    if changed {
                                        content.markdown = text.content;
                                    }
                                    changed
                                }
                            };
                            if body_changed {
                                row_changed_ids.insert(row.id.clone());
                            }

                            if !matches!(row.kind, WorkbookRowKind::Text(_)) {
                                ui.separator();
                                Self::render_row_result(ui, row, exec, &visual_status);
                            }
                        }
                    });

                    if focus_row_id == Some(row.id.as_str()) {
                        ui.scroll_to_rect(frame_response.response.rect, Some(egui::Align::Center));
                    }
                    ui.add_space(if matches!(row.kind, WorkbookRowKind::Text(_)) {
                        10.0
                    } else {
                        12.0
                    });
                });
            },
        );

        if row_order != before_order {
            let mut by_id = tab
                .rows
                .drain(..)
                .map(|row| (row.id.clone(), row))
                .collect::<HashMap<_, _>>();
            let mut reordered = Vec::with_capacity(row_order.len());
            for id in &row_order {
                if let Some(row) = by_id.remove(id) {
                    reordered.push(row);
                }
            }
            tab.rows = reordered;
            outcome.changed = true;
            outcome.changed_unfrozen = true;
        }

        if !row_changed_ids.is_empty() {
            outcome.changed = true;
            outcome.changed_unfrozen = tab
                .rows
                .iter()
                .filter(|row| row_changed_ids.contains(&row.id))
                .any(|row| !row.freeze);
        }

        outcome
    }

    fn show_row_header(
        ui: &mut egui::Ui,
        row: &mut WorkbookRow,
        exec: Option<&WorkbookRowExecution>,
        targets: &[StudyTargetDescriptor],
        picker_queries: &mut HashMap<String, String>,
        handle: egui_dnd::Handle,
        is_selected: bool,
        visual_status: &RowVisualStatus,
    ) -> HeaderOutcome {
        let mut selected = false;
        let mut toggled = false;
        let mut kind_changed = false;
        let summary = row_summary_preview(row, exec, targets);
        let output = output_preview(exec);

        ui.horizontal(|ui| {
            let drag_response = handle.ui(ui, |ui| {
                draw_drag_handle(ui);
            });

            let chevron_clicked = ui.button(if row.collapsed { ">" } else { "v" }).clicked();
            if chevron_clicked {
                toggled = true;
                selected = true;
            }

            draw_status_circle(ui, visual_status).on_hover_text(&visual_status.message);

            ui.vertical(|ui| {
                let title = if is_selected {
                    egui::RichText::new(summary.clone()).strong()
                } else {
                    egui::RichText::new(summary.clone())
                };
                ui.label(title.size(if matches!(row.kind, WorkbookRowKind::Text(_)) {
                    18.0
                } else {
                    15.0
                }));
                if let Some(output) = output.as_deref() {
                    ui.small(output);
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut current_kind = row_type_badge(&row.kind).to_string();
                if let Some(next) = searchable_picker(
                    ui,
                    egui::Id::new(("row_kind", row.id.as_str())),
                    &mut current_kind,
                    &row_kind_options(),
                    picker_queries,
                ) {
                    row.kind = default_row_kind_for_picker(&next, targets);
                    if matches!(row.kind, WorkbookRowKind::Text(_)) {
                        row.key = None;
                        row.title = None;
                    }
                    selected = true;
                    kind_changed = true;
                }
                ui.label(
                    egui::RichText::new(row_type_badge_label(&row.kind))
                        .small()
                        .color(ui.visuals().weak_text_color()),
                );
            });

            let click_rect = ui.min_rect();
            let click_response = ui.interact(
                click_rect,
                ui.id().with(("row_header", row.id.as_str())),
                egui::Sense::click(),
            );
            let suppress_toggle =
                drag_response.dragged() || drag_response.drag_started() || kind_changed;
            if click_response.clicked() && !suppress_toggle {
                toggled = true;
                selected = true;
            }
        });

        HeaderOutcome {
            selected,
            toggled,
            kind_changed,
        }
    }

    fn render_row_utilities(ui: &mut egui::Ui, row: &mut WorkbookRow) -> bool {
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            ui.label("Key");
            let key_entry = row.key.get_or_insert_with(String::new);
            if ui.text_edit_singleline(key_entry).changed() {
                changed = true;
            }
            if ui
                .checkbox(&mut row.freeze, "pause auto-run for this row")
                .changed()
            {
                changed = true;
            }
        });
        changed
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
            .filter(|target| target.kind == StudyTargetKind::Equation)
            .map(|target| PickerOption {
                id: target.id.clone(),
                label: target.name.clone(),
                group: target.category.clone(),
            })
            .collect::<Vec<_>>();

        ui.horizontal(|ui| {
            ui.label("Equation");
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
            .find(|target| target.kind == StudyTargetKind::Equation && target.id == c.path_id)
        else {
            ui.small("Unknown equation target");
            return changed;
        };

        ui.small(format!("{} [{}]", desc.name, desc.id));
        if let Some(display) = desc
            .display_unicode
            .as_ref()
            .or(desc.display_ascii.as_ref())
            .or(desc.display_latex.as_ref())
        {
            ui.label(egui::RichText::new(display).italics());
        }

        let mut target_value = c.target.clone().unwrap_or_default();
        ui.horizontal(|ui| {
            ui.label("Output");
            let output_items = desc
                .outputs
                .iter()
                .map(|output| PickerOption {
                    id: output.key.clone(),
                    label: output.label.clone(),
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
            if ui.small_button("Infer").clicked() {
                target_value.clear();
                changed = true;
            }
        });
        c.target = non_empty_option(target_value);

        if !desc.branch_options.is_empty() {
            ui.horizontal(|ui| {
                ui.label("Branch");
                let mut branch = c.branch.clone().unwrap_or_default();
                let branch_items = desc
                    .branch_options
                    .iter()
                    .map(|branch| PickerOption {
                        id: branch.clone(),
                        label: branch.clone(),
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
                if ui.small_button("Clear").clicked() {
                    branch.clear();
                    changed = true;
                }
                c.branch = non_empty_option(branch);
            });
        }

        ui.separator();
        ui.label(egui::RichText::new("Inputs").strong());
        let mut blank_fields = Vec::new();
        for field in desc
            .input_fields
            .iter()
            .filter(|field| field.key != "branch")
        {
            ui.push_id(row_field_id(row_id, &field.key), |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&field.key).strong());
                    let entry = c.inputs.entry(field.key.clone()).or_default();
                    if ui
                        .add(
                            egui::TextEdit::singleline(entry)
                                .desired_width(ui.available_width() * 0.55),
                        )
                        .changed()
                    {
                        changed = true;
                    }
                    if let Some(unit) = &field.unit {
                        ui.small(unit);
                    }
                    if entry.trim().is_empty() {
                        blank_fields.push(field.key.clone());
                    }
                });
                if !field.description.is_empty() {
                    ui.small(field.description.clone());
                }
            });
        }

        if c.target.is_none() {
            match blank_fields.as_slice() {
                [field] => ui.small(format!("Inferred output: {}", field)),
                [] => ui.colored_label(
                    egui::Color32::YELLOW,
                    "Choose an output explicitly when no variable is blank.",
                ),
                _ => ui.colored_label(
                    egui::Color32::YELLOW,
                    format!("Ambiguous: {} blank inputs remain.", blank_fields.len()),
                ),
            };
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
            changed |= ui
                .selectable_value(&mut c.kind, StudyTargetKind::Equation, "equation")
                .changed();
            changed |= ui
                .selectable_value(&mut c.kind, StudyTargetKind::Device, "device")
                .changed();
            changed |= ui
                .selectable_value(&mut c.kind, StudyTargetKind::Workflow, "workflow")
                .changed();
        });

        let target_options = targets
            .iter()
            .filter(|target| target.kind == c.kind)
            .map(|target| PickerOption {
                id: target.id.clone(),
                label: target.name.clone(),
                group: target.category.clone(),
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
            .find(|target| target.kind == c.kind && target.id == c.target_id)
        else {
            ui.small("Unknown study target");
            return changed;
        };

        ui.horizontal(|ui| {
            ui.label("Sweep field");
            let sweep_items = desc
                .sweepable_fields
                .iter()
                .map(|field| PickerOption {
                    id: field.clone(),
                    label: field.clone(),
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
            .filter(|output| output.numeric || output.plottable)
            .map(|output| PickerOption {
                id: output.key.clone(),
                label: output.label.clone(),
                group: None,
            })
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            ui.label("Output");
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
            for field in desc
                .input_fields
                .iter()
                .filter(|field| field.key != c.sweep_field)
            {
                ui.push_id(row_field_id(row_id, &field.key), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&field.key);
                        let entry = c.fixed_inputs.entry(field.key.clone()).or_default();
                        changed |= ui
                            .add(
                                egui::TextEdit::singleline(entry)
                                    .desired_width(ui.available_width() * 0.55),
                            )
                            .changed();
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
                    ui.label("Values");
                    let mut text = values
                        .iter()
                        .map(|value| value.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    if ui.text_edit_singleline(&mut text).changed() {
                        let parsed = text
                            .split(',')
                            .filter_map(|chunk| chunk.trim().parse::<f64>().ok())
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
                .map(|source| PickerOption {
                    id: source.ref_value.clone(),
                    label: source.label.clone(),
                    group: Some("Study rows".to_string()),
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
            changed |= ui
                .add(egui::TextEdit::singleline(&mut c.source_row).desired_width(180.0))
                .changed();
        });

        if let Some(key) = parse_ref_key(&c.source_row) {
            if !valid_keys.iter().any(|candidate| candidate == key) {
                ui.colored_label(
                    egui::Color32::LIGHT_RED,
                    format!("unknown source key '{}'", key),
                );
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

    fn render_text_row(
        ui: &mut egui::Ui,
        row_id: &str,
        c: &mut TextRowContent,
        highlighter: &mut egui_demo_lib::easy_mark::MemoizedEasymarkHighlighter,
    ) -> bool {
        let mut changed = false;
        c.render_mode = TextRenderMode::EasyMark;

        let editor_id = egui::Id::new(("workbook_text_editor", row_id));
        ui.horizontal_wrapped(|ui| {
            for (label, command) in [
                ("B", TextCommand::Strong),
                ("I", TextCommand::Italic),
                ("code", TextCommand::Code),
                ("H", TextCommand::Heading),
                ("quote", TextCommand::Quote),
                ("bullet", TextCommand::Bullet),
                ("1.", TextCommand::Numbered),
                ("rule", TextCommand::Rule),
                ("link", TextCommand::Link),
                ("block", TextCommand::CodeBlock),
            ] {
                if ui.small_button(label).clicked()
                    && apply_text_command(ui.ctx(), editor_id, &mut c.content, command)
                {
                    changed = true;
                }
            }
        });
        ui.add_space(6.0);

        ui.columns(2, |columns| {
            egui::ScrollArea::vertical()
                .id_salt(("text_editor_source", row_id))
                .max_height(280.0)
                .show(&mut columns[0], |ui| {
                    let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                        let mut job = highlighter.highlight(ui.style(), text);
                        job.wrap.max_width = wrap_width;
                        ui.fonts(|fonts| fonts.layout_job(job))
                    };
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut c.content)
                            .id(editor_id)
                            .desired_width(f32::INFINITY)
                            .desired_rows(14)
                            .font(egui::TextStyle::Monospace)
                            .layouter(&mut layouter),
                    );
                    changed |= response.changed();
                    changed |= apply_text_shortcuts(ui, editor_id, &mut c.content);
                });
            egui::ScrollArea::vertical()
                .id_salt(("text_rendered", row_id))
                .max_height(280.0)
                .show(&mut columns[1], |ui| {
                    render_text_document(ui, &c.content);
                });
        });

        changed
    }

    fn render_constant_row(ui: &mut egui::Ui, row_id: &str, c: &mut ConstantRowContent) -> bool {
        let mut changed = false;
        ui.label(egui::RichText::new("Value").strong());
        let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
            let mut job = constant_layout_job(ui, text);
            job.wrap.max_width = wrap_width;
            ui.fonts(|fonts| fonts.layout_job(job))
        };
        changed |= ui
            .add(
                egui::TextEdit::multiline(&mut c.value)
                    .id_source(("constant_editor", row_id))
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace)
                    .layouter(&mut layouter)
                    .code_editor(),
            )
            .changed();
        ui.small("Number or any-unit string parsed by eng-core.");
        ui.horizontal(|ui| {
            ui.label("Dimension hint");
            changed |= ui
                .text_edit_singleline(c.dimension_hint.get_or_insert_with(String::new))
                .changed();
        });
        changed
    }

    fn render_row_result(
        ui: &mut egui::Ui,
        row: &WorkbookRow,
        exec: Option<&WorkbookRowExecution>,
        visual_status: &RowVisualStatus,
    ) {
        ui.label(egui::RichText::new("Result").strong());
        if let Some(message) = key_validation_message(row, &BTreeSet::new()) {
            ui.colored_label(egui::Color32::LIGHT_RED, message);
        }
        let Some(exec) = exec else {
            ui.small(&visual_status.message);
            return;
        };

        match exec.state {
            WorkbookRowState::Ok => {
                ui.colored_label(egui::Color32::LIGHT_GREEN, "OK");
                if let Some(result) = &exec.result {
                    match result {
                        WorkbookRowResult::Equation(result) => {
                            ui.horizontal(|ui| {
                                ui.label(format!(
                                    "{} = {}",
                                    result.solve.output_key, result.solve.value
                                ));
                                if ui.small_button("Copy").clicked() {
                                    ui.ctx().copy_text(result.solve.value.to_string());
                                }
                            });
                        }
                        WorkbookRowResult::Constant(result) => {
                            ui.horizontal(|ui| {
                                ui.label(result.value.to_string());
                                if ui.small_button("Copy").clicked() {
                                    ui.ctx().copy_text(result.value.to_string());
                                }
                            });
                        }
                        WorkbookRowResult::Study(result) => {
                            ui.label(format!(
                                "Study rows: {} ({} ok / {} fail)",
                                result.table.rows.len(),
                                result.meta.n_ok,
                                result.meta.n_fail
                            ));
                        }
                        WorkbookRowResult::Plot(result) => {
                            ui.label(format!("Series: {}", result.series.len()));
                        }
                        WorkbookRowResult::Text { .. } => {}
                    }
                }
            }
            WorkbookRowState::Invalid | WorkbookRowState::Error => {
                ui.colored_label(egui::Color32::LIGHT_RED, format!("{:?}", exec.state));
                if let Some(first) = exec.messages.first() {
                    ui.label(first);
                }
                ui.collapsing("Details", |ui| {
                    for message in &exec.messages {
                        ui.small(message);
                    }
                });
            }
            _ => {
                ui.colored_label(egui::Color32::YELLOW, format!("{:?}", exec.state));
                for message in &exec.messages {
                    ui.small(message);
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
                    WorkbookRowState::Ok => ok += 1,
                    WorkbookRowState::Invalid | WorkbookRowState::Error => invalid += 1,
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
                            let label = row.key.clone().unwrap_or_else(|| "row".to_string());
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

fn row_frame(ui: &egui::Ui, row: &WorkbookRow, is_selected: bool) -> egui::Frame {
    if matches!(row.kind, WorkbookRowKind::Text(_)) {
        egui::Frame::none()
            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
            .fill(if is_selected {
                ui.visuals().extreme_bg_color.linear_multiply(0.2)
            } else {
                egui::Color32::TRANSPARENT
            })
    } else {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
            .fill(if is_selected {
                ui.visuals().faint_bg_color
            } else {
                ui.visuals().panel_fill
            })
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

fn row_kind_options() -> Vec<PickerOption> {
    vec![
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
    ]
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

fn row_type_badge_label(kind: &WorkbookRowKind) -> &'static str {
    match kind {
        WorkbookRowKind::Text(_) | WorkbookRowKind::Markdown(_) => "Text",
        WorkbookRowKind::Constant(_) => "Constant",
        WorkbookRowKind::EquationSolve(_) => "Equation",
        WorkbookRowKind::Study(_) => "Study",
        WorkbookRowKind::Plot(_) => "Plot",
    }
}

fn default_row_kind_for_picker(next: &str, targets: &[StudyTargetDescriptor]) -> WorkbookRowKind {
    match next {
        "text" => WorkbookRowKind::Text(TextRowContent {
            content: String::new(),
            render_mode: TextRenderMode::EasyMark,
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
            render_mode: TextRenderMode::EasyMark,
            style: Default::default(),
            legacy_header: false,
            legacy_mono: false,
        }),
    }
}

fn non_empty_option(text: String) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn default_equation_id(targets: &[StudyTargetDescriptor]) -> String {
    targets
        .iter()
        .find(|target| target.kind == StudyTargetKind::Equation)
        .map(|target| target.id.clone())
        .unwrap_or_else(|| "structures.hoop_stress".to_string())
}

fn default_target_id(targets: &[StudyTargetDescriptor], kind: StudyTargetKind) -> String {
    targets
        .iter()
        .find(|target| target.kind == kind)
        .map(|target| target.id.clone())
        .unwrap_or_default()
}

fn draw_drag_handle(ui: &mut egui::Ui) {
    let size = egui::vec2(16.0, 20.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    let color = ui.visuals().weak_text_color();
    let radius = 1.4;
    let x0 = rect.left() + 5.0;
    let x1 = rect.left() + 10.0;
    let y0 = rect.top() + 5.0;
    for row in 0..3 {
        let y = y0 + row as f32 * 5.0;
        ui.painter().circle_filled(egui::pos2(x0, y), radius, color);
        ui.painter().circle_filled(egui::pos2(x1, y), radius, color);
    }
    if response.hovered() {
        ui.output_mut(|out| out.cursor_icon = egui::CursorIcon::Grab);
    }
}

fn draw_status_circle(ui: &mut egui::Ui, status: &RowVisualStatus) -> egui::Response {
    let color = match status.state {
        RowVisualState::Ok => egui::Color32::from_rgb(72, 192, 110),
        RowVisualState::Warning => egui::Color32::from_rgb(230, 191, 76),
        RowVisualState::Error => egui::Color32::from_rgb(220, 92, 92),
        RowVisualState::Incomplete | RowVisualState::NotRun => egui::Color32::from_gray(120),
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), 4.0, color);
    response
}

fn row_visual_status(
    row: &WorkbookRow,
    exec: Option<&WorkbookRowExecution>,
    duplicate_keys: &BTreeSet<String>,
) -> RowVisualStatus {
    if let Some(message) = key_validation_message(row, duplicate_keys) {
        return RowVisualStatus {
            state: RowVisualState::Error,
            message,
        };
    }
    if let Some(exec) = exec {
        let message = exec
            .messages
            .first()
            .cloned()
            .unwrap_or_else(|| format!("{:?}", exec.state));
        let state = match exec.state {
            WorkbookRowState::Ok => RowVisualState::Ok,
            WorkbookRowState::Invalid | WorkbookRowState::Error => RowVisualState::Error,
            WorkbookRowState::Ambiguous | WorkbookRowState::Ready => RowVisualState::Warning,
            WorkbookRowState::Incomplete => RowVisualState::Incomplete,
        };
        return RowVisualStatus { state, message };
    }
    RowVisualStatus {
        state: RowVisualState::NotRun,
        message: "Not run yet".to_string(),
    }
}

fn key_validation_message(row: &WorkbookRow, duplicate_keys: &BTreeSet<String>) -> Option<String> {
    let trimmed = row.key.as_deref().map(str::trim).unwrap_or_default();
    if row_requires_key(row) && trimmed.is_empty() {
        return Some("Key is required because this row can be referenced.".to_string());
    }
    if !trimmed.is_empty() && duplicate_keys.contains(trimmed) {
        return Some(format!("Duplicate key '{}'.", trimmed));
    }
    None
}

fn duplicate_key_set(rows: &[WorkbookRow]) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for row in rows {
        let trimmed = row.key.as_deref().map(str::trim).unwrap_or_default();
        if trimmed.is_empty() {
            continue;
        }
        if !seen.insert(trimmed.to_string()) {
            duplicates.insert(trimmed.to_string());
        }
    }
    duplicates
}

fn row_primary_name(row: &WorkbookRow) -> String {
    row.key
        .as_deref()
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| row_type_badge_label(&row.kind).to_string())
}

fn row_summary_preview(
    row: &WorkbookRow,
    exec: Option<&WorkbookRowExecution>,
    targets: &[StudyTargetDescriptor],
) -> String {
    let key = row
        .key
        .as_deref()
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .unwrap_or("row");
    if let Some(exec) = exec {
        if let Some(result) = &exec.result {
            match result {
                WorkbookRowResult::Equation(result) => {
                    return format!(
                        "{} = {} {}",
                        key, result.solve.output_key, result.solve.value
                    );
                }
                WorkbookRowResult::Constant(result) => {
                    return format!("{} = {}", key, result.value);
                }
                WorkbookRowResult::Study(result) => {
                    return format!(
                        "{} = Study(n={}, ok={}, err={})",
                        key,
                        result.table.rows.len(),
                        result.meta.n_ok,
                        result.meta.n_fail
                    );
                }
                WorkbookRowResult::Plot(result) => {
                    let label = result
                        .series
                        .first()
                        .map(|series| series.name.clone())
                        .unwrap_or_else(|| "plot".to_string());
                    return format!("{} = Plot({})", key, label);
                }
                WorkbookRowResult::Text { content, .. } => return preview_text_line(content),
            }
        }
    }

    match &row.kind {
        WorkbookRowKind::Text(content) => preview_text_line(&content.content),
        WorkbookRowKind::Constant(content) => format!("{} = {}", key, content.value),
        WorkbookRowKind::EquationSolve(content) => targets
            .iter()
            .find(|target| target.id == content.path_id)
            .map(|target| format!("{} ({})", target.name, content.path_id))
            .unwrap_or_else(|| content.path_id.clone()),
        WorkbookRowKind::Study(content) => format!("Study {}", content.target_id),
        WorkbookRowKind::Plot(content) => format!(
            "Plot {} vs {} from {}",
            content.y,
            content.x,
            preview_plot_source(&content.source_row)
        ),
        WorkbookRowKind::Markdown(content) => preview_text_line(&content.markdown),
    }
}

fn output_preview(exec: Option<&WorkbookRowExecution>) -> Option<String> {
    let exec = exec?;
    match exec.result.as_ref()? {
        WorkbookRowResult::Equation(result) => Some(format!(
            "{} = {}",
            result.solve.output_key, result.solve.value
        )),
        WorkbookRowResult::Constant(result) => Some(result.value.to_string()),
        WorkbookRowResult::Study(result) => Some(format!("{} samples", result.table.rows.len())),
        WorkbookRowResult::Plot(result) => Some(format!("{} series", result.series.len())),
        WorkbookRowResult::Text { .. } => None,
    }
}

fn render_text_document(ui: &mut egui::Ui, content: &str) {
    egui_demo_lib::easy_mark::easy_mark(ui, content);
}

fn preview_text_line(text: &str) -> String {
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return trimmed.chars().take(72).collect::<String>();
        }
    }
    "Text".to_string()
}

fn preview_plot_source(source: &str) -> String {
    parse_ref_key(source).unwrap_or(source).to_string()
}

fn constant_layout_job(ui: &egui::Ui, text: &str) -> egui::text::LayoutJob {
    let monospace = egui::TextFormat {
        font_id: egui::TextStyle::Monospace.resolve(ui.style()),
        color: ui.visuals().text_color(),
        ..Default::default()
    };
    let comment = egui::TextFormat {
        font_id: egui::TextStyle::Monospace.resolve(ui.style()),
        color: ui.visuals().weak_text_color(),
        ..Default::default()
    };
    let number = egui::TextFormat {
        font_id: egui::TextStyle::Monospace.resolve(ui.style()),
        color: egui::Color32::from_rgb(110, 190, 255),
        ..Default::default()
    };
    let unit = egui::TextFormat {
        font_id: egui::TextStyle::Monospace.resolve(ui.style()),
        color: egui::Color32::from_rgb(170, 215, 120),
        ..Default::default()
    };

    let mut job = egui::text::LayoutJob::default();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            job.append(line, 0.0, comment.clone());
        } else {
            let mut token = String::new();
            let mut in_number = false;
            for ch in line.chars() {
                let is_number_char =
                    ch.is_ascii_digit() || matches!(ch, '.' | '-' | '+' | 'e' | 'E');
                if is_number_char {
                    if !token.is_empty() && !in_number {
                        job.append(&token, 0.0, monospace.clone());
                        token.clear();
                    }
                    token.push(ch);
                    in_number = true;
                } else {
                    if !token.is_empty() {
                        job.append(
                            &token,
                            0.0,
                            if in_number {
                                number.clone()
                            } else {
                                monospace.clone()
                            },
                        );
                        token.clear();
                    }
                    in_number = false;
                    if ch.is_ascii_alphabetic() || ch == '/' || ch == '^' || ch == '%' {
                        job.append(&ch.to_string(), 0.0, unit.clone());
                    } else {
                        job.append(&ch.to_string(), 0.0, monospace.clone());
                    }
                }
            }
            if !token.is_empty() {
                job.append(
                    &token,
                    0.0,
                    if in_number {
                        number.clone()
                    } else {
                        monospace.clone()
                    },
                );
            }
        }
        job.append("\n", 0.0, monospace.clone());
    }
    job
}

fn apply_text_shortcuts(ui: &egui::Ui, editor_id: egui::Id, content: &mut String) -> bool {
    if !ui.memory(|memory| memory.has_focus(editor_id)) {
        return false;
    }

    let mut changed = false;
    if ui.input_mut(|input| {
        input.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::B,
        ))
    }) {
        changed |= apply_text_command(ui.ctx(), editor_id, content, TextCommand::Strong);
    }
    if ui.input_mut(|input| {
        input.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::I,
        ))
    }) {
        changed |= apply_text_command(ui.ctx(), editor_id, content, TextCommand::Italic);
    }
    if ui.input_mut(|input| {
        input.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::COMMAND,
            egui::Key::E,
        ))
    }) {
        changed |= apply_text_command(ui.ctx(), editor_id, content, TextCommand::Code);
    }
    changed
}

fn apply_text_command(
    ctx: &egui::Context,
    editor_id: egui::Id,
    content: &mut String,
    command: TextCommand,
) -> bool {
    let Some(mut state) = egui::TextEdit::load_state(ctx, editor_id) else {
        return insert_text_command(content, command);
    };
    let Some(mut range) = state.cursor.char_range() else {
        return insert_text_command(content, command);
    };

    let changed = match command {
        TextCommand::Strong => {
            toggle_surrounding(content, &mut range, "*");
            true
        }
        TextCommand::Italic => {
            toggle_surrounding(content, &mut range, "/");
            true
        }
        TextCommand::Code => {
            toggle_surrounding(content, &mut range, "`");
            true
        }
        TextCommand::Heading => insert_line_prefix(content, &mut range, "# "),
        TextCommand::Quote => insert_line_prefix(content, &mut range, "> "),
        TextCommand::Bullet => insert_line_prefix(content, &mut range, "- "),
        TextCommand::Numbered => insert_line_prefix(content, &mut range, "1. "),
        TextCommand::Rule => insert_block(content, &mut range, "\n---\n"),
        TextCommand::Link => wrap_or_insert(content, &mut range, "[", "](https://example.com)"),
        TextCommand::CodeBlock => insert_block(content, &mut range, "\n```\ncode\n```\n"),
    };

    if changed {
        state.cursor.set_char_range(Some(range));
        state.store(ctx, editor_id);
    }
    changed
}

fn insert_text_command(content: &mut String, command: TextCommand) -> bool {
    let snippet = match command {
        TextCommand::Strong => "*strong*",
        TextCommand::Italic => "/italics/",
        TextCommand::Code => "`code`",
        TextCommand::Heading => "# heading",
        TextCommand::Quote => "> quote",
        TextCommand::Bullet => "- item",
        TextCommand::Numbered => "1. item",
        TextCommand::Rule => "\n---\n",
        TextCommand::Link => "[label](https://example.com)",
        TextCommand::CodeBlock => "\n```\ncode\n```\n",
    };
    content.push_str(snippet);
    true
}

fn wrap_or_insert(
    content: &mut String,
    range: &mut egui::text_selection::CCursorRange,
    prefix: &str,
    suffix: &str,
) -> bool {
    let [primary, secondary] = range.sorted();
    content.insert_str(secondary.index, suffix);
    let advance = prefix.chars().count();
    content.insert_str(primary.index, prefix);
    range.primary.index += advance;
    range.secondary.index += advance;
    true
}

fn insert_block(
    content: &mut String,
    range: &mut egui::text_selection::CCursorRange,
    block: &str,
) -> bool {
    let [primary, secondary] = range.sorted();
    let insert_at = secondary.index.max(primary.index);
    content.insert_str(insert_at, block);
    let new_index = insert_at + block.chars().count();
    range.primary.index = new_index;
    range.secondary.index = new_index;
    true
}

fn insert_line_prefix(
    content: &mut String,
    range: &mut egui::text_selection::CCursorRange,
    prefix: &str,
) -> bool {
    let [primary, _] = range.sorted();
    let line_start = content[..primary.index]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0);
    content.insert_str(line_start, prefix);
    let advance = prefix.chars().count();
    range.primary.index += advance;
    range.secondary.index += advance;
    true
}

fn toggle_surrounding(
    content: &mut String,
    range: &mut egui::text_selection::CCursorRange,
    surrounding: &str,
) {
    let [primary, secondary] = range.sorted();
    let surrounding_len = surrounding.chars().count();
    let prefix_start = primary.index.saturating_sub(surrounding_len);
    let suffix_end = secondary.index.saturating_add(surrounding_len);
    let prefix_matches = content
        .get(prefix_start..primary.index)
        .map(|text| text == surrounding)
        .unwrap_or(false);
    let suffix_matches = content
        .get(secondary.index..suffix_end)
        .map(|text| text == surrounding)
        .unwrap_or(false);

    if prefix_matches && suffix_matches {
        content.replace_range(secondary.index..suffix_end, "");
        content.replace_range(prefix_start..primary.index, "");
        range.primary.index = range.primary.index.saturating_sub(surrounding_len);
        range.secondary.index = range.secondary.index.saturating_sub(surrounding_len);
    } else {
        content.insert_str(secondary.index, surrounding);
        content.insert_str(primary.index, surrounding);
        range.primary.index += surrounding_len;
        range.secondary.index += surrounding_len;
    }
}

fn parse_ref_key(expr: &str) -> Option<&str> {
    let trimmed = expr.trim();
    if let Some(key) = trimmed.strip_prefix("ref:") {
        let key = key.trim();
        if !key.is_empty() {
            return Some(key);
        }
    }
    if let Some(key) = trimmed.strip_prefix('@') {
        let key = key.trim();
        if !key.is_empty() {
            return Some(key);
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
        let options = vec![
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
        let filtered = crate::ui::search_picker::filter_options(&options, "hoop");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "structures.hoop_stress");
    }

    #[test]
    fn duplicate_rows_must_receive_new_ids() {
        let row_a = new_row(
            WorkbookRowKind::Text(TextRowContent {
                content: "a".to_string(),
                render_mode: TextRenderMode::EasyMark,
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

    #[test]
    fn duplicate_key_detection_ignores_blank_keys() {
        let rows = vec![
            WorkbookRow {
                id: "1".to_string(),
                key: None,
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Text(TextRowContent {
                    content: String::new(),
                    render_mode: TextRenderMode::EasyMark,
                    style: Default::default(),
                    legacy_header: false,
                    legacy_mono: false,
                }),
            },
            WorkbookRow {
                id: "2".to_string(),
                key: Some("gamma".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Constant(ConstantRowContent {
                    value: "1.4".to_string(),
                    dimension_hint: None,
                }),
            },
            WorkbookRow {
                id: "3".to_string(),
                key: Some("gamma".to_string()),
                title: None,
                collapsed: false,
                freeze: false,
                kind: WorkbookRowKind::Constant(ConstantRowContent {
                    value: "1.3".to_string(),
                    dimension_hint: None,
                }),
            },
        ];
        let duplicates = duplicate_key_set(&rows);
        assert!(duplicates.contains("gamma"));
        assert_eq!(duplicates.len(), 1);
    }

    #[test]
    fn missing_required_key_message_only_applies_to_referenceable_rows() {
        let text_row = new_row(
            WorkbookRowKind::Text(TextRowContent {
                content: "notes".to_string(),
                render_mode: TextRenderMode::EasyMark,
                style: Default::default(),
                legacy_header: false,
                legacy_mono: false,
            }),
            false,
        );
        let constant_row = new_row(
            WorkbookRowKind::Constant(ConstantRowContent {
                value: "1.4".to_string(),
                dimension_hint: None,
            }),
            true,
        );
        assert!(key_validation_message(&text_row, &BTreeSet::new()).is_none());
        assert!(key_validation_message(&constant_row, &BTreeSet::new()).is_some());
    }

    #[test]
    fn toggle_surrounding_wraps_selection() {
        let mut content = "hello".to_string();
        let mut range = egui::text_selection::CCursorRange::two(
            egui::text::CCursor::new(1),
            egui::text::CCursor::new(4),
        );
        toggle_surrounding(&mut content, &mut range, "*");
        assert_eq!(content, "h*ell*o");
    }
}
