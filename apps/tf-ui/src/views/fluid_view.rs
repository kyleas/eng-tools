use crate::curve_source::CurveData;
use crate::fluid_picker::SearchableFluidPicker;
use crate::fluid_workspace::{
    ComputeStatus, FluidWorkspace, SaturationMode, StateMode, StatePoint,
};
use crate::input_helper::UnitAwareInput;
use crate::plot_export::ExportContext;
use crate::property_metadata::FluidProperty;
use std::collections::HashMap;
use tf_fluids::{
    CoolPropModel, FluidInputPair, Quantity, compute_equilibrium_state,
    compute_saturated_liquid_at_pressure, compute_saturated_liquid_at_temperature,
    compute_state_with_quality,
};

pub struct FluidView {
    model: CoolPropModel,
    /// Fluid pickers for each state point (keyed by state ID)
    fluid_pickers: HashMap<String, SearchableFluidPicker>,
    /// Unit-aware input helper for managing input fields
    #[allow(dead_code)]
    unit_inputs: UnitAwareInput,
    /// Sweep configuration UI state
    sweep_config: SweepConfig,
    /// Fluid picker for sweep configuration
    sweep_fluid_picker: SearchableFluidPicker,
    /// Multiple sweep curves organized as tabs
    sweep_tabs: Vec<SweepTab>,
    /// Currently active sweep tab index
    active_sweep_tab: usize,
    /// Index of the trace selected in the inspector (None = no selection)
    selected_trace_index: Option<usize>,
    /// Export context for managing export dialogs
    export_ctx: ExportContext,
    /// Cached phase envelope data (keyed by species + x/y properties)
    phase_envelope_cache: std::collections::HashMap<String, PhaseEnvelopeData>,
}

/// A sweep tab containing curve data and metadata
#[derive(Debug, Clone)]
struct SweepTab {
    /// Tab label (e.g., "N2 Sweep 1")
    label: String,
    /// The curve data
    curve: CurveData,
    /// X-axis property
    #[allow(dead_code)]
    x_property: FluidProperty,
    /// Y-axis property
    #[allow(dead_code)]
    y_property: FluidProperty,
    /// Y-axis scaling factor (1.0 = normal, 2.0 = 2x, 0.5 = half)
    y_scale: f32,
    /// Which plot group this trace belongs to (0, 1, 2, etc.)
    plot_group: usize,
    /// Time created (for unique identification)
    #[allow(dead_code)]
    created_at: std::time::Instant,
    /// Sweep configuration used to generate this curve
    sweep_config: Box<SweepConfig>,
}

/// Phase envelope curve data for a specific species
#[derive(Debug, Clone)]
struct PhaseEnvelopeData {
    /// Species this envelope is for
    #[allow(dead_code)]
    species: tf_fluids::Species,
    /// X property used
    #[allow(dead_code)]
    x_property: FluidProperty,
    /// Y property used
    #[allow(dead_code)]
    y_property: FluidProperty,
    /// Liquid saturation curve points
    liquid_points: Vec<[f64; 2]>,
    /// Vapor saturation curve points
    vapor_points: Vec<[f64; 2]>,
}

/// Transient state for sweep configuration UI
#[derive(Debug, Clone)]
struct SweepConfig {
    /// Whether the sweep panel is expanded
    show_panel: bool,
    /// Selected species for sweep
    species: tf_fluids::Species,
    /// X property selector (using FluidProperty)
    x_property: FluidProperty,
    /// Y property selector (using FluidProperty)
    y_property: FluidProperty,
    /// Sweep variable (Temperature or Pressure)
    sweep_variable: String,
    /// Start value with units
    start_value: String,
    /// End value with units
    end_value: String,
    /// Number of points
    num_points: String,
    /// Sweep type (Linear or Logarithmic)
    sweep_type: String,
    /// Fixed property (currently unused in simple UI)
    fixed_property: String,
    /// Fixed property value
    fixed_property_value: String,
    /// Last generated sweep error message
    last_error: Option<String>,
    /// Show phase envelope overlay on plot
    show_phase_envelope: bool,
    /// Plot height in pixels
    plot_height: f32,
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            show_panel: false,
            species: tf_fluids::Species::N2,
            x_property: FluidProperty::Temperature,
            y_property: FluidProperty::Density,
            sweep_variable: "Temperature".to_string(),
            start_value: "300K".to_string(),
            end_value: "400K".to_string(),
            num_points: "50".to_string(),
            sweep_type: "Linear".to_string(),
            fixed_property: "Pressure".to_string(),
            fixed_property_value: "101325Pa".to_string(),
            last_error: None,
            show_phase_envelope: false,
            plot_height: 300.0,
        }
    }
}

impl Default for FluidView {
    fn default() -> Self {
        Self {
            model: CoolPropModel::new(),
            fluid_pickers: HashMap::new(),
            unit_inputs: UnitAwareInput::new(),
            sweep_config: SweepConfig::default(),
            sweep_fluid_picker: SearchableFluidPicker::default(),
            sweep_tabs: Vec::new(),
            active_sweep_tab: 0,
            selected_trace_index: None,
            export_ctx: ExportContext::default(),
            phase_envelope_cache: HashMap::new(),
        }
    }
}

impl FluidView {
    pub fn show(&mut self, ui: &mut egui::Ui, workspace: &mut FluidWorkspace) {
        // Wrap entire view in a scroll area to handle overflow content
        egui::ScrollArea::vertical()
            .id_salt("fluid_workspace_main_scroll")
            .show(ui, |ui| {
                ui.heading("Fluid Workspace");
                ui.label("Row-based fluid state comparison and property calculator");
                ui.separator();

                // Toolbar
                ui.horizontal(|ui| {
                    if ui.button("➕ Add State Point").clicked() {
                        workspace.add_state_point();
                    }

                    ui.label(format!("{} state points", workspace.state_points.len()));
                });

                ui.add_space(8.0);

                // Ensure we have pickers for all state points
                for state in &workspace.state_points {
                    self.fluid_pickers.entry(state.id.clone()).or_default();
                }

                // Remove pickers for deleted state points
                self.fluid_pickers
                    .retain(|id, _| workspace.state_points.iter().any(|s| &s.id == id));

                // Table view
                egui::ScrollArea::both()
                    .id_salt("fluid_workspace_table_scroll")
                    .show(ui, |ui| {
                        self.show_state_table(ui, workspace);
                    });

                // Auto-compute any state points that have complete inputs
                self.auto_compute_states(workspace);

                ui.add_space(12.0);

                // Sweep configuration panel
                self.show_sweep_panel(ui, workspace);

                ui.add_space(8.0);

                // Inline sweep plot (if any sweeps have been generated)
                if !self.sweep_tabs.is_empty() {
                    self.show_inline_sweep_plot(ui, workspace);
                }

                // Show export dialog if requested
                if self.export_ctx.show_export_dialog {
                    self.show_export_dialog(ui.ctx());
                }

                let remove_ids: Vec<String> = workspace
                    .state_points
                    .iter()
                    .filter(|s| s.error_message.as_deref() == Some("REMOVE_REQUESTED"))
                    .map(|s| s.id.clone())
                    .collect();

                for id in remove_ids {
                    workspace.remove_state_point(&id);
                }
            });
    }

    fn show_state_table(&mut self, ui: &mut egui::Ui, workspace: &mut FluidWorkspace) {
        use egui_extras::{Column, TableBuilder};

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(30.0)) // Status indicator
            .column(Column::initial(80.0).at_least(60.0)) // State label
            .column(Column::initial(100.0).at_least(80.0)) // State Mode
            .column(Column::initial(120.0).at_least(100.0)) // Fluid
            .column(Column::initial(80.0).at_least(60.0)) // Input type
            .column(Column::initial(120.0).at_least(100.0)) // Input 1
            .column(Column::initial(120.0).at_least(100.0)) // Input 2
            .column(Column::initial(80.0).at_least(60.0)) // Quality (conditional)
            .column(Column::initial(110.0).at_least(90.0)) // Pressure
            .column(Column::initial(110.0).at_least(90.0)) // Temperature
            .column(Column::initial(110.0).at_least(90.0)) // Density
            .column(Column::initial(110.0).at_least(90.0)) // Enthalpy
            .column(Column::initial(110.0).at_least(90.0)) // Entropy
            .column(Column::initial(100.0).at_least(80.0)) // Cp
            .column(Column::initial(100.0).at_least(80.0)) // Cv
            .column(Column::initial(80.0).at_least(60.0)) // γ
            .column(Column::initial(100.0).at_least(80.0)) // Speed of sound
            .column(Column::initial(80.0).at_least(60.0)) // Phase
            .column(Column::exact(40.0)) // Remove button
            .header(22.0, |mut header| {
                header.col(|ui| {
                    ui.strong("●");
                });
                header.col(|ui| {
                    ui.strong("State");
                });
                header.col(|ui| {
                    ui.strong("Mode");
                });
                header.col(|ui| {
                    ui.strong("Fluid");
                });
                header.col(|ui| {
                    ui.strong("Input");
                });
                header.col(|ui| {
                    ui.strong("Value 1");
                });
                header.col(|ui| {
                    ui.strong("Value 2");
                });
                header.col(|ui| {
                    ui.strong("Quality");
                });
                header.col(|ui| {
                    ui.strong("P [Pa]");
                });
                header.col(|ui| {
                    ui.strong("T [K]");
                });
                header.col(|ui| {
                    ui.strong("ρ [kg/m³]");
                });
                header.col(|ui| {
                    ui.strong("h [J/kg]");
                });
                header.col(|ui| {
                    ui.strong("s [J/(kg·K)]");
                });
                header.col(|ui| {
                    ui.strong("cp");
                });
                header.col(|ui| {
                    ui.strong("cv");
                });
                header.col(|ui| {
                    ui.strong("γ");
                });
                header.col(|ui| {
                    ui.strong("a [m/s]");
                });
                header.col(|ui| {
                    ui.strong("Phase");
                });
                header.col(|ui| {
                    ui.strong("");
                });
            })
            .body(|mut body| {
                let state_ids: Vec<String> = workspace
                    .state_points
                    .iter()
                    .map(|s| s.id.clone())
                    .collect();

                for state_id in state_ids {
                    if let Some(state) =
                        workspace.state_points.iter_mut().find(|s| s.id == state_id)
                    {
                        body.row(28.0, |mut row| {
                            self.show_state_row(&mut row, state);
                        });
                    }
                }
            });
    }

    fn show_state_row(&mut self, row: &mut egui_extras::TableRow, state: &mut StatePoint) {
        // Status indicator
        row.col(|ui| {
            let (color, tooltip) = match state.status {
                ComputeStatus::Success => {
                    (egui::Color32::from_rgb(0, 200, 0), "Computed successfully")
                }
                ComputeStatus::Failed => (egui::Color32::from_rgb(200, 0, 0), "Computation failed"),
                ComputeStatus::Computing => (egui::Color32::from_rgb(255, 165, 0), "Computing..."),
                ComputeStatus::NotComputed => (egui::Color32::GRAY, "Not computed"),
            };
            ui.label(egui::RichText::new("●").color(color).size(16.0))
                .on_hover_text(tooltip);
        });

        // State label (editable)
        row.col(|ui| {
            ui.text_edit_singleline(&mut state.label);
        });

        // State Mode selector (Specified or Equilibrium)
        row.col(|ui| {
            let mut current_mode = state.state_mode;
            egui::ComboBox::from_id_salt(format!("state_mode_{}", state.id))
                .selected_text(current_mode.label())
                .width(90.0)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut current_mode, StateMode::Specified, "Specified")
                        .changed()
                    {
                        state.state_mode = current_mode;
                        state.clear_result();
                    }
                    if ui
                        .selectable_value(&mut current_mode, StateMode::Equilibrium, "Equilibrium")
                        .changed()
                    {
                        state.state_mode = current_mode;
                        state.clear_result();
                    }
                });
        });

        // Fluid picker
        row.col(|ui| {
            if let Some(picker) = self.fluid_pickers.get_mut(&state.id) {
                if picker.show(
                    ui,
                    format!("fluid_species_{}", state.id),
                    &mut state.species,
                ) {
                    state.clear_result();
                }
            }
        });

        // Input pair selector (or saturation mode selector for Equilibrium mode)
        row.col(|ui| {
            if state.state_mode == StateMode::Equilibrium {
                // Show saturation mode selector for Equilibrium mode
                let mut current_sat_mode = state.saturation_mode;
                egui::ComboBox::from_id_salt(format!("sat_mode_{}", state.id))
                    .selected_text(current_sat_mode.label())
                    .width(60.0)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut current_sat_mode,
                                SaturationMode::Temperature,
                                "T",
                            )
                            .changed()
                        {
                            state.saturation_mode = current_sat_mode;
                            state.clear_result();
                        }
                        if ui
                            .selectable_value(&mut current_sat_mode, SaturationMode::Pressure, "P")
                            .changed()
                        {
                            state.saturation_mode = current_sat_mode;
                            state.clear_result();
                        }
                        if ui
                            .selectable_value(&mut current_sat_mode, SaturationMode::Quality, "Q")
                            .changed()
                        {
                            state.saturation_mode = current_sat_mode;
                            state.clear_result();
                        }
                    });
            } else {
                // Show input pair selector for Specified mode
                egui::ComboBox::from_id_salt(format!("input_pair_{}", state.id))
                    .selected_text(short_pair_label(state.input_pair))
                    .width(60.0)
                    .show_ui(ui, |ui| {
                        for pair in [
                            FluidInputPair::PT,
                            FluidInputPair::PH,
                            FluidInputPair::RhoH,
                            FluidInputPair::PS,
                        ] {
                            if ui
                                .selectable_value(
                                    &mut state.input_pair,
                                    pair,
                                    short_pair_label(pair),
                                )
                                .changed()
                            {
                                state.clear_result();
                            }
                        }
                    });
            }
        });

        // Input 1 (unit-aware)
        row.col(|ui| {
            let quantity = if state.state_mode == StateMode::Equilibrium {
                match state.saturation_mode {
                    SaturationMode::Temperature => Quantity::Temperature,
                    SaturationMode::Pressure => Quantity::Pressure,
                    SaturationMode::Quality => Quantity::Pressure, // Input 1 is Pressure in Quality mode
                }
            } else {
                input_quantity_for_pair(state.input_pair, true)
            };
            if ui.text_edit_singleline(&mut state.input_1_text).changed() {
                if let Ok(value) = tf_fluids::parse_quantity(&state.input_1_text, quantity) {
                    state.input_1 = value;
                    state.clear_result();
                }
            }
        });

        // Input 2 (unit-aware) - shown conditionally
        row.col(|ui| {
            // Only show Input 2 if:
            // - In Specified mode, or
            // - In Equilibrium Quality mode (need both P and T)
            let should_show = if state.state_mode == StateMode::Equilibrium {
                state.saturation_mode == SaturationMode::Quality
            } else {
                true
            };

            if should_show {
                let quantity = if state.state_mode == StateMode::Equilibrium {
                    Quantity::Temperature
                } else {
                    input_quantity_for_pair(state.input_pair, false)
                };
                if ui.text_edit_singleline(&mut state.input_2_text).changed() {
                    if let Ok(value) = tf_fluids::parse_quantity(&state.input_2_text, quantity) {
                        state.input_2 = value;
                        state.clear_result();
                    }
                }
            } else {
                ui.label("-");
            }
        });

        // Quality (shown conditionally or always in Quality mode)
        row.col(|ui| {
            // Show quality input if:
            // - In Equilibrium Quality mode, or
            // - When two-phase ambiguity is detected in Specified mode
            let show_quality = (state.state_mode == StateMode::Equilibrium
                && state.saturation_mode == SaturationMode::Quality)
                || (state.state_mode == StateMode::Specified && state.needs_disambiguation);

            if show_quality {
                if ui
                    .add(
                        egui::DragValue::new(state.quality.get_or_insert(0.5))
                            .speed(0.01)
                            .range(0.0..=1.0)
                            .max_decimals(3),
                    )
                    .changed()
                {
                    state.clear_result();
                }
            } else {
                ui.label("-");
            }
        });

        // Computed properties
        if let Some(result) = &state.last_result {
            row.col(|ui| {
                ui.monospace(fmt_value(result.pressure_pa()));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.temperature_k()));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.density_kg_m3()));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.enthalpy_j_per_kg));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.entropy_j_per_kg_k));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.cp_j_per_kg_k));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.cv_j_per_kg_k));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.gamma));
            });
            row.col(|ui| {
                ui.monospace(fmt_value(result.speed_of_sound_m_s()));
            });
            row.col(|ui| {
                ui.label(result.phase.as_ref().unwrap_or(&"N/A".to_string()));
            });
        } else {
            // Empty cells for not-yet-computed properties
            for _ in 0..10 {
                row.col(|ui| {
                    ui.label("-");
                });
            }
        }

        // Remove button
        row.col(|ui| {
            if ui.small_button("×").on_hover_text("Remove state").clicked() {
                state.error_message = Some("REMOVE_REQUESTED".to_string());
            }
        });
    }

    fn auto_compute_states(&mut self, workspace: &mut FluidWorkspace) {
        for state in &mut workspace.state_points {
            // Check if inputs are complete based on mode
            let inputs_ready = match state.state_mode {
                StateMode::Specified => state.inputs_complete(),
                StateMode::Equilibrium => match state.saturation_mode {
                    SaturationMode::Temperature | SaturationMode::Pressure => {
                        !state.input_1_text.trim().is_empty()
                    }
                    SaturationMode::Quality => {
                        !state.input_1_text.trim().is_empty()
                            && !state.input_2_text.trim().is_empty()
                    }
                },
            };

            // Only auto-compute if inputs are ready and status is NotComputed
            if inputs_ready && state.status == ComputeStatus::NotComputed {
                let result = match state.state_mode {
                    StateMode::Specified => {
                        // Original Specified mode computation
                        compute_equilibrium_state(
                            &self.model,
                            state.species,
                            state.input_pair,
                            state.input_1,
                            state.input_2,
                        )
                    }
                    StateMode::Equilibrium => {
                        // Equilibrium mode: use saturation-aware functions
                        match state.saturation_mode {
                            SaturationMode::Temperature => {
                                // Compute saturated liquid at given temperature (default)
                                compute_saturated_liquid_at_temperature(
                                    &self.model,
                                    state.species,
                                    state.input_1,
                                )
                            }
                            SaturationMode::Pressure => {
                                // Compute saturated liquid at given pressure (default)
                                compute_saturated_liquid_at_pressure(
                                    &self.model,
                                    state.species,
                                    state.input_1,
                                )
                            }
                            SaturationMode::Quality => {
                                // Compute state at given P, T, and Q
                                compute_state_with_quality(
                                    &self.model,
                                    state.species,
                                    state.input_1, // Pressure
                                    state.input_2, // Temperature
                                    state.quality.unwrap_or(0.5),
                                )
                            }
                        }
                    }
                };

                match result {
                    Ok(result) => {
                        state.last_result = Some(result);
                        state.status = ComputeStatus::Success;
                        state.error_message = None;
                        state.needs_disambiguation = false;
                    }
                    Err(e) => {
                        let err_msg = format!("{}", e);
                        // Check if error suggests two-phase ambiguity (only for Specified mode)
                        if state.state_mode == StateMode::Specified
                            && (err_msg.contains("two-phase")
                                || err_msg.contains("quality")
                                || (state.input_pair == FluidInputPair::PT
                                    && err_msg.contains("invalid")))
                        {
                            state.needs_disambiguation = true;
                            state.status = ComputeStatus::Failed;
                            state.error_message = Some("Two-phase: specify quality".to_string());
                        } else {
                            state.last_result = None;
                            state.status = ComputeStatus::Failed;
                            state.error_message = Some(err_msg);
                            state.needs_disambiguation = false;
                        }
                    }
                }
            }
        }
    }

    fn show_sweep_panel(&mut self, ui: &mut egui::Ui, workspace: &mut FluidWorkspace) {
        // Collapse/expand button
        let expand_text = if self.sweep_config.show_panel {
            "▼ Fluid Property Sweep"
        } else {
            "▶ Fluid Property Sweep"
        };

        if ui.button(expand_text).clicked() {
            self.sweep_config.show_panel = !self.sweep_config.show_panel;
        }

        if !self.sweep_config.show_panel {
            return;
        }

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("Generate a fluid property sweep as a curve for plotting");
                ui.add_space(4.0);

                // Grid layout for configuration
                egui::Grid::new("sweep_config_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Species dropdown
                        ui.label("Species:");
                        self.sweep_fluid_picker.show(
                            ui,
                            "sweep_species_picker",
                            &mut self.sweep_config.species,
                        );
                        ui.end_row();

                        // X Property dropdown
                        ui.label("X Property:");
                        egui::ComboBox::from_id_salt("sweep_x_property")
                            .selected_text(self.sweep_config.x_property.label())
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                for prop in FluidProperty::all() {
                                    ui.selectable_value(
                                        &mut self.sweep_config.x_property,
                                        *prop,
                                        prop.label(),
                                    );
                                }
                            });
                        ui.end_row();

                        // Y Property dropdown
                        ui.label("Y Property:");
                        egui::ComboBox::from_id_salt("sweep_y_property")
                            .selected_text(self.sweep_config.y_property.label())
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                for prop in FluidProperty::all() {
                                    ui.selectable_value(
                                        &mut self.sweep_config.y_property,
                                        *prop,
                                        prop.label(),
                                    );
                                }
                            });
                        ui.end_row();

                        // Sweep Variable
                        ui.label("Sweep Variable:");
                        egui::ComboBox::from_id_salt("sweep_variable")
                            .selected_text(&self.sweep_config.sweep_variable)
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.sweep_config.sweep_variable,
                                    "Temperature".to_string(),
                                    "Temperature",
                                );
                                ui.selectable_value(
                                    &mut self.sweep_config.sweep_variable,
                                    "Pressure".to_string(),
                                    "Pressure",
                                );
                            });
                        ui.end_row();

                        // Start Value
                        ui.label("Start Value:")
                            .on_hover_text("e.g., 300K or 100 Pa");
                        ui.text_edit_singleline(&mut self.sweep_config.start_value);
                        ui.end_row();

                        // End Value
                        ui.label("End Value:").on_hover_text("e.g., 400K or 200 Pa");
                        ui.text_edit_singleline(&mut self.sweep_config.end_value);
                        ui.end_row();

                        // Number of Points
                        ui.label("Points:")
                            .on_hover_text("Number of points along the sweep");
                        ui.text_edit_singleline(&mut self.sweep_config.num_points);
                        ui.end_row();

                        // Sweep Type
                        ui.label("Type:")
                            .on_hover_text("Linear or Logarithmic spacing");
                        egui::ComboBox::from_id_salt("sweep_type")
                            .selected_text(&self.sweep_config.sweep_type)
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.sweep_config.sweep_type,
                                    "Linear".to_string(),
                                    "Linear",
                                );
                                ui.selectable_value(
                                    &mut self.sweep_config.sweep_type,
                                    "Logarithmic".to_string(),
                                    "Logarithmic",
                                );
                            });
                        ui.end_row();

                        // Fixed Property Value
                        ui.label("Fixed Value:").on_hover_text(
                            "Value for the fixed property (e.g., 101325Pa for non-swept pressure)",
                        );
                        ui.text_edit_singleline(&mut self.sweep_config.fixed_property_value);
                        ui.end_row();

                        // Phase Envelope Option
                        ui.label("Phase Envelope:")
                            .on_hover_text("Overlay saturation curve (vapor dome)");
                        ui.checkbox(&mut self.sweep_config.show_phase_envelope, "Show");
                        ui.end_row();
                    });

                ui.add_space(8.0);

                // Error message if any
                if let Some(ref error) = self.sweep_config.last_error {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                    ui.add_space(4.0);
                }

                // Generation buttons
                ui.horizontal(|ui| {
                    if ui.button("📊 Generate Sweep").clicked() {
                        self.generate_sweep(workspace);
                    }
                });
            });
        });
    }

    fn show_inline_sweep_plot(&mut self, ui: &mut egui::Ui, _workspace: &mut FluidWorkspace) {
        if self.sweep_tabs.is_empty() {
            return;
        }

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("📈 Sweep Results");

                // Group sweeps by plot_group
                use std::collections::HashMap;
                let mut plot_groups: HashMap<usize, Vec<usize>> = HashMap::new();

                for (idx, tab) in self.sweep_tabs.iter().enumerate() {
                    plot_groups.entry(tab.plot_group).or_default().push(idx);
                }

                // Create sorted list of plot group IDs
                let mut group_ids: Vec<usize> = plot_groups.keys().copied().collect();
                group_ids.sort_unstable();

                // Find which plot group the active trace belongs to
                let active_plot_group = if let Some(active_idx) = self.selected_trace_index {
                    if active_idx < self.sweep_tabs.len() {
                        self.sweep_tabs[active_idx].plot_group
                    } else {
                        0
                    }
                } else {
                    0
                };

                // Show plot group tabs if more than one group exists
                if group_ids.len() > 1 {
                    ui.horizontal(|ui| {
                        ui.label("Plots:");
                        for &group_id in &group_ids {
                            let is_active_group = group_id == active_plot_group;
                            let button = if is_active_group {
                                ui.button(format!("▼ Plot {}", group_id + 1))
                            } else {
                                ui.button(format!("  Plot {}", group_id + 1))
                            };

                            if button.clicked() {
                                // Switch to first trace in this group
                                if let Some(&first_idx) = plot_groups[&group_id].first() {
                                    self.selected_trace_index = Some(first_idx);
                                }
                            }
                        }
                    });
                    ui.add_space(4.0);
                }

                // Get traces for active plot group
                let traces_in_group = plot_groups
                    .get(&active_plot_group)
                    .cloned()
                    .unwrap_or_default();

                if !traces_in_group.is_empty() {
                    // Trace tabs with close buttons
                    let mut tabs_to_remove = Vec::new();

                    ui.horizontal(|ui| {
                        ui.label("Traces:");
                        for &idx in &traces_in_group {
                            if idx < self.sweep_tabs.len() {
                                let tab_label = self.sweep_tabs[idx].label.clone();
                                let is_selected = self.selected_trace_index == Some(idx);

                                // Indicator + label + delete button in one group
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        // Indicator: * for selected, space for unselected
                                        let indicator = if is_selected { "*" } else { " " };
                                        ui.label(indicator);

                                        // Trace label button
                                        if ui.button(&tab_label).clicked() {
                                            // Save current config to previous tab if any
                                            if let Some(prev_idx) = self.selected_trace_index {
                                                if prev_idx < self.sweep_tabs.len() {
                                                    *self.sweep_tabs[prev_idx].sweep_config =
                                                        self.sweep_config.clone();
                                                }
                                            }
                                            // Load new tab's config
                                            self.selected_trace_index = Some(idx);
                                            if idx < self.sweep_tabs.len() {
                                                self.sweep_config =
                                                    (*self.sweep_tabs[idx].sweep_config).clone();
                                            }
                                        }

                                        // Delete button directly after label (using × symbol)
                                        if ui.small_button("×").clicked() {
                                            tabs_to_remove.push(idx);
                                        }
                                    });
                                });
                            }
                        }
                    });

                    // Remove tabs (in reverse order to maintain indices)
                    for idx in tabs_to_remove.into_iter().rev() {
                        self.sweep_tabs.remove(idx);
                        if let Some(sel) = self.selected_trace_index {
                            if sel >= self.sweep_tabs.len() && !self.sweep_tabs.is_empty() {
                                self.selected_trace_index = Some(self.sweep_tabs.len() - 1);
                            } else if !self.sweep_tabs.is_empty() {
                                // Keep current selection if valid
                            } else {
                                self.selected_trace_index = None;
                            }
                        }
                    }

                    // Show plot if there are still traces
                    if !traces_in_group.is_empty() {
                        ui.separator();

                        // Collect curves for this plot group and apply y_scale
                        let mut lines = Vec::new();
                        for &idx in &traces_in_group {
                            if idx < self.sweep_tabs.len() {
                                let tab = &self.sweep_tabs[idx];
                                let points: Vec<[f64; 2]> = tab
                                    .curve
                                    .x_values
                                    .iter()
                                    .zip(tab.curve.y_values.iter())
                                    .map(|(x, y)| [*x, y * (tab.y_scale as f64)])
                                    .collect();

                                let plot_points = egui_plot::PlotPoints::from(points);
                                let line = egui_plot::Line::new(plot_points).name(&tab.curve.label);
                                lines.push(line);
                            }
                        }

                        // Add phase envelope overlay if enabled
                        if self.sweep_config.show_phase_envelope {
                            // Get properties from first trace in group
                            if let Some(&first_idx) = traces_in_group.first() {
                                if first_idx < self.sweep_tabs.len() {
                                    let tab = &self.sweep_tabs[first_idx];
                                    if let Some(envelope) = self.get_phase_envelope(
                                        self.sweep_config.species,
                                        tab.x_property,
                                        tab.y_property,
                                    ) {
                                        // Liquid saturation curve
                                        if !envelope.liquid_points.is_empty() {
                                            let liquid_line = egui_plot::Line::new(
                                                egui_plot::PlotPoints::from(envelope.liquid_points),
                                            )
                                            .name("Saturated Liquid")
                                            .color(egui::Color32::LIGHT_BLUE)
                                            .style(egui_plot::LineStyle::dashed_dense());
                                            lines.push(liquid_line);
                                        }

                                        // Vapor saturation curve
                                        if !envelope.vapor_points.is_empty() {
                                            let vapor_line = egui_plot::Line::new(
                                                egui_plot::PlotPoints::from(envelope.vapor_points),
                                            )
                                            .name("Saturated Vapor")
                                            .color(egui::Color32::LIGHT_RED)
                                            .style(egui_plot::LineStyle::dashed_dense());
                                            lines.push(vapor_line);
                                        }
                                    }
                                }
                            }
                        }

                        // Display the plot with a stable ID based on plot group
                        let plot_id = format!("inline_sweep_plot_{}", active_plot_group);
                        egui_plot::Plot::new(&plot_id)
                            .height(self.sweep_config.plot_height)
                            .legend(egui_plot::Legend::default())
                            .show(ui, |plot_ui| {
                                for line in lines {
                                    plot_ui.line(line);
                                }
                            });

                        // Add draggable resize handle
                        ui.add_space(2.0);
                        let resize_handle_rect =
                            ui.allocate_space(egui::vec2(ui.available_width(), 8.0)).1;
                        let resize_response = ui.interact(
                            resize_handle_rect,
                            ui.id().with("resize_handle"),
                            egui::Sense::drag(),
                        );

                        // Visual feedback for resize handle
                        let handle_color = if resize_response.hovered() || resize_response.dragged()
                        {
                            ui.visuals().widgets.active.bg_fill
                        } else {
                            ui.visuals().widgets.inactive.bg_fill
                        };
                        ui.painter()
                            .rect_filled(resize_handle_rect, 2.0, handle_color);

                        // Draw resize indicator
                        if resize_response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                        }

                        // Handle drag to resize
                        if resize_response.dragged() {
                            let delta = resize_response.drag_delta().y;
                            self.sweep_config.plot_height =
                                (self.sweep_config.plot_height + delta).clamp(150.0, 1200.0);
                        }

                        ui.add_space(2.0);

                        ui.add_space(4.0);

                        // Buttons for plot actions
                        let mut clear_all = false;
                        let mut export_active = false;

                        ui.horizontal(|ui| {
                            if ui.button("➕ Send All to Plots").clicked() {
                                // Curves are already in pending_curves from generate_sweep
                            }
                            if ui.button("🔄 Clear All Tabs").clicked() {
                                clear_all = true;
                            }
                            if ui.button("💾 Export Active...").clicked() {
                                export_active = true;
                            }
                        });

                        if clear_all {
                            self.sweep_tabs.clear();
                            self.selected_trace_index = None;
                        }

                        if export_active {
                            if let Some(sel_idx) = self.selected_trace_index {
                                if sel_idx < self.sweep_tabs.len() {
                                    self.export_ctx.show_export_dialog = true;
                                    let label = self.sweep_tabs[sel_idx].label.replace(' ', "_");
                                    self.export_ctx.plot_name = label;
                                    if self.export_ctx.export_directory.is_empty() {
                                        self.export_ctx.export_directory = "exports".to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            });
        });
    }

    /// Public method to render trace and sweep inspector content in the right-side inspector panel
    pub fn show_inspector_content(&mut self, ui: &mut egui::Ui) {
        if self.selected_trace_index.is_none() {
            ui.label("Select a trace to inspect");
            return;
        }

        let trace_idx = match self.selected_trace_index {
            Some(idx) if idx < self.sweep_tabs.len() => idx,
            _ => return,
        };

        ui.heading("📋 Trace Inspector");
        ui.separator();

        // Trace metadata - allow renaming
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.sweep_tabs[trace_idx].label);

        ui.separator();
        ui.label("Display Options:");

        // Y-scale multiplier control
        ui.horizontal(|ui| {
            ui.label("Y-Scale:")
                .on_hover_text("Multiply Y-values by this factor (e.g., 2.0 = twice as high)");
            let mut scale_str = format!("{:.2}x", self.sweep_tabs[trace_idx].y_scale);
            if ui.text_edit_singleline(&mut scale_str).changed() {
                let clean_str = scale_str.trim_end_matches('x').trim();
                if let Ok(new_scale) = clean_str.parse::<f32>() {
                    if new_scale > 0.0 {
                        self.sweep_tabs[trace_idx].y_scale = new_scale;
                    }
                }
            }
        });

        // Plot group assignment
        ui.horizontal(|ui| {
            ui.label("Plot:")
                .on_hover_text("Which plot (0 = Plot 1, 1 = Plot 2, etc.)");
            let mut group_str = (self.sweep_tabs[trace_idx].plot_group).to_string();
            if ui.text_edit_singleline(&mut group_str).changed() {
                if let Ok(new_group) = group_str.parse::<usize>() {
                    self.sweep_tabs[trace_idx].plot_group = new_group;
                }
            }
        });

        // Curve statistics
        ui.separator();
        ui.label("Statistics:");
        let tab = &self.sweep_tabs[trace_idx];
        let x_min = tab
            .curve
            .x_values
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let x_max = tab
            .curve
            .x_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let y_min_raw = tab
            .curve
            .y_values
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let y_max_raw = tab
            .curve
            .y_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let y_min = y_min_raw * (tab.y_scale as f64);
        let y_max = y_max_raw * (tab.y_scale as f64);

        ui.monospace(format!("X: {:.3e} to {:.3e}", x_min, x_max));
        ui.monospace(format!("Y: {:.3e} to {:.3e}", y_min, y_max));
        ui.monospace(format!("Points: {}", tab.curve.x_values.len()));

        // Sweep configuration
        ui.separator();
        ui.label("Sweep Configuration:");

        ui.horizontal(|ui| {
            ui.label("Species:");
            self.sweep_fluid_picker.show(
                ui,
                "inspector_species_picker",
                &mut self.sweep_config.species,
            );
        });

        let mut config_changed = false;

        ui.horizontal(|ui| {
            ui.label("X Property:");
            if egui::ComboBox::from_id_salt("inspector_x_property")
                .selected_text(self.sweep_config.x_property.label())
                .width(100.0)
                .show_ui(ui, |ui| {
                    for prop in FluidProperty::all() {
                        ui.selectable_value(&mut self.sweep_config.x_property, *prop, prop.label());
                    }
                })
                .response
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Y Property:");
            if egui::ComboBox::from_id_salt("inspector_y_property")
                .selected_text(self.sweep_config.y_property.label())
                .width(100.0)
                .show_ui(ui, |ui| {
                    for prop in FluidProperty::all() {
                        ui.selectable_value(&mut self.sweep_config.y_property, *prop, prop.label());
                    }
                })
                .response
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Sweep Var:");
            if egui::ComboBox::from_id_salt("inspector_sweep_variable")
                .selected_text(&self.sweep_config.sweep_variable)
                .width(80.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.sweep_config.sweep_variable,
                        "Temperature".to_string(),
                        "Temperature",
                    );
                    ui.selectable_value(
                        &mut self.sweep_config.sweep_variable,
                        "Pressure".to_string(),
                        "Pressure",
                    );
                })
                .response
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Start:");
            if ui
                .text_edit_singleline(&mut self.sweep_config.start_value)
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("End:");
            if ui
                .text_edit_singleline(&mut self.sweep_config.end_value)
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Points:");
            if ui
                .text_edit_singleline(&mut self.sweep_config.num_points)
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Type:");
            if egui::ComboBox::from_id_salt("inspector_sweep_type")
                .selected_text(&self.sweep_config.sweep_type)
                .width(80.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.sweep_config.sweep_type,
                        "Linear".to_string(),
                        "Linear",
                    );
                    ui.selectable_value(
                        &mut self.sweep_config.sweep_type,
                        "Logarithmic".to_string(),
                        "Logarithmic",
                    );
                })
                .response
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Var-Fixed:");
            if egui::ComboBox::from_id_salt("inspector_fixed_property")
                .selected_text(&self.sweep_config.fixed_property)
                .width(80.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.sweep_config.fixed_property,
                        "Pressure".to_string(),
                        "Pressure",
                    );
                    ui.selectable_value(
                        &mut self.sweep_config.fixed_property,
                        "Temperature".to_string(),
                        "Temperature",
                    );
                })
                .response
                .changed()
            {
                config_changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Fix Value:");
            if ui
                .text_edit_singleline(&mut self.sweep_config.fixed_property_value)
                .changed()
            {
                config_changed = true;
            }
        });

        // Auto re-sweep if configuration changed
        if config_changed {
            self.regenerate_sweep_for_trace(trace_idx);
        }
    }

    /// Regenerate the sweep curve for a specific trace using current sweep configuration
    fn regenerate_sweep_for_trace(&mut self, trace_idx: usize) {
        if trace_idx >= self.sweep_tabs.len() {
            return;
        }

        // Get species from sweep config
        let species = self.sweep_config.species;

        // Parse number of points
        let num_points: usize = match self.sweep_config.num_points.parse() {
            Ok(n) if n >= 2 => n,
            _ => {
                self.sweep_config.last_error = Some("Points must be >= 2".to_string());
                return;
            }
        };

        // Use curve_generator to create the sweep
        use crate::curve_source::{CurveSource, FluidSweepParameters};

        let params = FluidSweepParameters {
            species: species.key().to_string(),
            sweep_variable: self.sweep_config.sweep_variable.clone(),
            start_value: self.sweep_config.start_value.clone(),
            end_value: self.sweep_config.end_value.clone(),
            num_points,
            sweep_type: self.sweep_config.sweep_type.clone(),
            fixed_property_name: Some(self.sweep_config.fixed_property.clone()),
            fixed_property_value: Some(self.sweep_config.fixed_property_value.clone()),
        };

        let source = CurveSource::FluidPropertySweep {
            x_property: self.sweep_config.x_property.canonical_name().to_string(),
            y_property: self.sweep_config.y_property.canonical_name().to_string(),
            parameters: params,
        };

        // Try to generate the curve
        match crate::curve_generator::generate_curve_data(&source, None) {
            Some(curve_data) => {
                // Update the existing trace's curve data
                self.sweep_tabs[trace_idx].curve = curve_data;
                self.sweep_tabs[trace_idx].x_property = self.sweep_config.x_property;
                self.sweep_tabs[trace_idx].y_property = self.sweep_config.y_property;
                self.sweep_config.last_error = None;
            }
            None => {
                self.sweep_config.last_error = Some("Failed to regenerate sweep curve".to_string());
            }
        }
    }

    /// Generate or retrieve cached phase envelope for current species and properties
    fn get_phase_envelope(
        &mut self,
        species: tf_fluids::Species,
        x_property: FluidProperty,
        y_property: FluidProperty,
    ) -> Option<PhaseEnvelopeData> {
        // Create cache key
        let cache_key = format!(
            "{}_{}_{}",
            species.key(),
            x_property.canonical_name(),
            y_property.canonical_name()
        );

        // Check if cached
        if let Some(cached) = self.phase_envelope_cache.get(&cache_key) {
            return Some(cached.clone());
        }

        // Generate phase envelope
        // Determine if we should sweep by temperature or pressure based on X property
        let envelope = if matches!(x_property, FluidProperty::Temperature) {
            // Temperature-based sweep
            use tf_fluids::generate_phase_envelope_by_temperature;

            // Get temperature limits (rough estimates for common fluids)
            let t_min = 100.0; // K
            let t_max = 500.0; // K
            let num_points = 100;

            generate_phase_envelope_by_temperature(&self.model, species, t_min, t_max, num_points)
                .ok()?
        } else {
            // Pressure-based sweep
            use tf_fluids::generate_phase_envelope_by_pressure;

            // Get pressure limits
            let p_min = 1000.0; // Pa (0.01 bar)
            let p_max = 10_000_000.0; // Pa (100 bar)
            let num_points = 100;

            generate_phase_envelope_by_pressure(
                &self.model,
                species,
                p_min,
                p_max,
                num_points,
                true, // Use log spacing for pressure
            )
            .ok()?
        };

        // Extract properties
        use tf_fluids::extract_property;
        let x_prop_name = x_property.canonical_name();
        let y_prop_name = y_property.canonical_name();

        let liquid_x = extract_property(&envelope.liquid_states, x_prop_name);
        let liquid_y = extract_property(&envelope.liquid_states, y_prop_name);
        let vapor_x = extract_property(&envelope.vapor_states, x_prop_name);
        let vapor_y = extract_property(&envelope.vapor_states, y_prop_name);

        let liquid_points: Vec<[f64; 2]> = liquid_x
            .iter()
            .zip(liquid_y.iter())
            .map(|(x, y)| [*x, *y])
            .collect();

        let vapor_points: Vec<[f64; 2]> = vapor_x
            .iter()
            .zip(vapor_y.iter())
            .map(|(x, y)| [*x, *y])
            .collect();

        let envelope_data = PhaseEnvelopeData {
            species,
            x_property,
            y_property,
            liquid_points,
            vapor_points,
        };

        // Cache it
        self.phase_envelope_cache
            .insert(cache_key, envelope_data.clone());
        Some(envelope_data)
    }

    fn generate_sweep(&mut self, workspace: &mut FluidWorkspace) {
        // Get species from sweep config
        let species = self.sweep_config.species;
        let species_str = species.key().to_string();

        // Parse number of points
        let num_points: usize = match self.sweep_config.num_points.parse() {
            Ok(n) if n >= 2 => n,
            _ => {
                self.sweep_config.last_error = Some("Points must be >= 2".to_string());
                return;
            }
        };

        // Use curve_generator to create the sweep
        use crate::curve_source::{CurveSource, FluidSweepParameters};

        let params = FluidSweepParameters {
            species: species_str.clone(),
            sweep_variable: self.sweep_config.sweep_variable.clone(),
            start_value: self.sweep_config.start_value.clone(),
            end_value: self.sweep_config.end_value.clone(),
            num_points,
            sweep_type: self.sweep_config.sweep_type.clone(),
            fixed_property_name: Some(self.sweep_config.fixed_property.clone()),
            fixed_property_value: Some(self.sweep_config.fixed_property_value.clone()),
        };

        let source = CurveSource::FluidPropertySweep {
            x_property: self.sweep_config.x_property.canonical_name().to_string(),
            y_property: self.sweep_config.y_property.canonical_name().to_string(),
            parameters: params,
        };

        // Try to generate the curve
        match crate::curve_generator::generate_curve_data(&source, None) {
            Some(curve_data) => {
                // Successfully generated - add as a new tab
                let tab_label = format!("{} Sweep {}", species_str, self.sweep_tabs.len() + 1);
                let tab = SweepTab {
                    label: tab_label,
                    curve: curve_data,
                    x_property: self.sweep_config.x_property,
                    y_property: self.sweep_config.y_property,
                    y_scale: 1.0,
                    plot_group: 0,
                    created_at: std::time::Instant::now(),
                    sweep_config: Box::new(self.sweep_config.clone()),
                };
                self.sweep_tabs.push(tab);
                self.active_sweep_tab = self.sweep_tabs.len() - 1;

                // Also add to pending curves for the app to add to plotting workspace
                let curve_def = convert_curve_source_to_def(&source);
                workspace.pending_curves.push(curve_def);
                self.sweep_config.last_error = None;
            }
            None => {
                self.sweep_config.last_error = Some("Failed to generate sweep curve".to_string());
            }
        }
    }
    fn show_export_dialog(&mut self, ctx: &egui::Context) {
        use crate::plot_export::{ExportFormat, export_plot_with_curves, export_to_csv};

        egui::Window::new("Export Plot")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Format:");
                    egui::ComboBox::from_id_salt("export_format_fluid")
                        .selected_text(self.export_ctx.export_format.label())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.export_ctx.export_format,
                                ExportFormat::Csv,
                                ExportFormat::Csv.label(),
                            );
                            ui.selectable_value(
                                &mut self.export_ctx.export_format,
                                ExportFormat::Png,
                                ExportFormat::Png.label(),
                            );
                            ui.selectable_value(
                                &mut self.export_ctx.export_format,
                                ExportFormat::Jpg,
                                ExportFormat::Jpg.label(),
                            );
                            ui.selectable_value(
                                &mut self.export_ctx.export_format,
                                ExportFormat::Pdf,
                                ExportFormat::Pdf.label(),
                            );
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Plot Name:");
                    ui.text_edit_singleline(&mut self.export_ctx.plot_name);
                });

                ui.horizontal(|ui| {
                    ui.label("Directory:");
                    ui.text_edit_singleline(&mut self.export_ctx.export_directory)
                        .on_hover_text(
                            "Enter folder path (e.g., 'exports', './plots', 'D:\\\\exports')",
                        );
                });

                if matches!(
                    self.export_ctx.export_format,
                    ExportFormat::Png | ExportFormat::Jpg | ExportFormat::Pdf
                ) {
                    ui.label("⚠ Image export creates a data visualization");
                }

                if let Some(status) = &self.export_ctx.export_status {
                    if status.contains("successful") || status.contains("Exported to") {
                        ui.colored_label(egui::Color32::GREEN, status);
                    } else {
                        ui.colored_label(egui::Color32::RED, status);
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("Export").clicked() {
                        if self.export_ctx.plot_name.is_empty() {
                            self.export_ctx.export_status =
                                Some("Plot name cannot be empty".to_string());
                        } else if let Some(active_tab) = self.sweep_tabs.get(self.active_sweep_tab)
                        {
                            let path_str = self.export_ctx.get_export_path();
                            let path = std::path::Path::new(&path_str);

                            // Create directory if it doesn't exist
                            if let Some(parent) = path.parent() {
                                if !parent.as_os_str().is_empty() {
                                    let _ = std::fs::create_dir_all(parent);
                                }
                            }

                            match self.export_ctx.export_format {
                                ExportFormat::Csv => {
                                    match export_to_csv(
                                        path,
                                        &active_tab.curve.x_values,
                                        &active_tab.curve.y_values,
                                        "X",
                                        "Y",
                                    ) {
                                        Ok(_) => {
                                            self.export_ctx.export_status =
                                                Some(format!("Exported to: {}", path_str));
                                        }
                                        Err(e) => {
                                            self.export_ctx.export_status =
                                                Some(format!("Export failed: {}", e));
                                        }
                                    }
                                }
                                ExportFormat::Png | ExportFormat::Jpg | ExportFormat::Pdf => {
                                    // Collect all curves for export
                                    let curves: Vec<(String, Vec<[f64; 2]>)> = self
                                        .sweep_tabs
                                        .iter()
                                        .map(|tab| {
                                            let points: Vec<[f64; 2]> = tab
                                                .curve
                                                .x_values
                                                .iter()
                                                .zip(tab.curve.y_values.iter())
                                                .map(|(x, y)| [*x, *y])
                                                .collect();
                                            (tab.label.clone(), points)
                                        })
                                        .collect();

                                    match export_plot_with_curves(
                                        path,
                                        self.export_ctx.export_format,
                                        &curves,
                                        &active_tab.label,
                                        800,
                                        600,
                                    ) {
                                        Ok(_) => {
                                            self.export_ctx.export_status =
                                                Some(format!("Exported to: {}", path_str));
                                        }
                                        Err(e) => {
                                            self.export_ctx.export_status =
                                                Some(format!("Export failed: {}", e));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.export_ctx.show_export_dialog = false;
                        self.export_ctx.export_status = None;
                    }
                });
            });
    }
}

/// Convert a runtime CurveSource to a persistent ArbitraryCurveSourceDef
fn convert_curve_source_to_def(
    source: &crate::curve_source::CurveSource,
) -> tf_project::schema::ArbitraryCurveSourceDef {
    use crate::curve_source::CurveSource;
    use tf_project::schema::{ArbitraryCurveSourceDef, FixedPropertyDef, FluidSweepParametersDef};

    match source {
        CurveSource::FluidPropertySweep {
            x_property,
            y_property,
            parameters,
        } => {
            let fixed_property = if let (Some(prop_name), Some(prop_value)) = (
                &parameters.fixed_property_name,
                &parameters.fixed_property_value,
            ) {
                Some(FixedPropertyDef {
                    property_name: prop_name.clone(),
                    value: prop_value.clone(),
                })
            } else {
                None
            };

            ArbitraryCurveSourceDef::FluidPropertySweep {
                x_property: x_property.clone(),
                y_property: y_property.clone(),
                parameters: FluidSweepParametersDef {
                    sweep_variable: parameters.sweep_variable.clone(),
                    start_value: parameters.start_value.clone(),
                    end_value: parameters.end_value.clone(),
                    num_points: parameters.num_points,
                    sweep_type: parameters.sweep_type.clone(),
                    species: parameters.species.clone(),
                    fixed_property,
                },
            }
        }
        _ => {
            // For now, only handle FluidPropertySweep
            panic!("Unsupported curve source type")
        }
    }
}

fn short_pair_label(pair: FluidInputPair) -> &'static str {
    match pair {
        FluidInputPair::PT => "P-T",
        FluidInputPair::PH => "P-H",
        FluidInputPair::RhoH => "ρ-H",
        FluidInputPair::PS => "P-S",
    }
}

fn input_quantity_for_pair(pair: FluidInputPair, is_first: bool) -> Quantity {
    match (pair, is_first) {
        (FluidInputPair::PT, true) => Quantity::Pressure,
        (FluidInputPair::PT, false) => Quantity::Temperature,
        (FluidInputPair::PH, true) => Quantity::Pressure,
        (FluidInputPair::PH, false) => Quantity::SpecificEnthalpy,
        (FluidInputPair::RhoH, true) => Quantity::Density,
        (FluidInputPair::RhoH, false) => Quantity::SpecificEnthalpy,
        (FluidInputPair::PS, true) => Quantity::Pressure,
        (FluidInputPair::PS, false) => Quantity::SpecificEntropy,
    }
}

fn fmt_value(value: f64) -> String {
    if !value.is_finite() {
        return "NaN".to_string();
    }

    let abs = value.abs();
    if abs >= 1.0e5 || (abs > 0.0 && abs < 1.0e-3) {
        format!("{value:.4e}")
    } else {
        format!("{value:.4}")
    }
}
