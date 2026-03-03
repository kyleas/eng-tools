use crate::rocket_workspace::{PerformanceConstraint, RocketThermalPlotTab, RocketWorkspace};
use crate::views::rocket::plotting::PlotSeriesSpec;
use crate::views::rocket::show_case_context_banner;
use egui_plot::{Line, Plot, PlotPoints, Points};
use tf_fluids::{
    CoolPropModel, FluidInputPair, Quantity, Species, compute_equilibrium_state,
    practical_coolprop_catalog,
};
use tf_rpa::{
    CoolantFlowDirection, CoolingMode, RocketGeometryResult, ThermalDesignStation, ThermalModel,
    compute_geometry, compute_thermal,
};

pub fn show(ui: &mut egui::Ui, workspace: &mut RocketWorkspace) {
    egui::ScrollArea::vertical()
        .id_salt("rocket_thermal_scroll")
        .show(ui, |ui| {
            ui.heading("Rocket Thermal Design");
            ui.label("Integrated thermal design: gas-side heating, wall conduction, coolant-side removal, film effect, and automated channel sizing.");
            show_case_context_banner(ui, workspace);
            ui.separator();

            ui.collapsing("A) Source Context", |ui| {
                ui.small(format!(
                    "Geometry basis: mode={} throat_input={:.6} | L*={:.3} m | Ac/At={:.3}",
                    workspace.geometry.sizing_mode.label(),
                    workspace.geometry.throat_input_value,
                    workspace.geometry.characteristic_length_m,
                    workspace.geometry.chamber_contraction_ratio
                ));
                ui.small(format!(
                    "Performance basis: Pc={:.0} Pa | MR={:.3} | Ambient={:.0} Pa",
                    workspace.performance_case.chamber_pressure_pa,
                    workspace.performance_case.mixture_ratio,
                    workspace.performance_case.ambient_pressure_pa
                ));
            });

            ui.separator();
            ui.columns(2, |columns| {
        let left = &mut columns[0];
        left.heading("B) Thermal Model Setup");

        left.horizontal(|ui| {
            ui.label("Thermal model:");
            ui.selectable_value(
                &mut workspace.thermal.model,
                ThermalModel::BartzLikeConvective,
                ThermalModel::BartzLikeConvective.label(),
            );
        });

        left.horizontal(|ui| {
            ui.label("Cooling mode:");
            ui.selectable_value(
                &mut workspace.thermal.cooling_mode,
                CoolingMode::AdiabaticWall,
                "Adiabatic",
            );
            ui.selectable_value(
                &mut workspace.thermal.cooling_mode,
                CoolingMode::Regenerative,
                "Regen",
            );
            ui.selectable_value(
                &mut workspace.thermal.cooling_mode,
                CoolingMode::Film,
                "Film",
            );
            ui.selectable_value(
                &mut workspace.thermal.cooling_mode,
                CoolingMode::RegenerativeFilm,
                "Regen+Film",
            );
        });

        left.separator();
        left.label("Gas-side inputs");
        left.horizontal(|ui| {
            ui.label("Thermal O/F source:");
            ui.checkbox(
                &mut workspace.thermal.use_performance_mixture_ratio,
                "Use Performance O/F",
            );
            if workspace.thermal.use_performance_mixture_ratio {
                ui.small(format!("MR={:.3}", workspace.performance_case.mixture_ratio));
            } else {
                ui.label("Specified MR:");
                ui.add(
                    egui::DragValue::new(&mut workspace.thermal.specified_mixture_ratio).speed(0.01),
                );
            }
        });
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_recovery_t",
            "Recovery temperature",
            Quantity::Temperature,
            &mut workspace.thermal.reference_recovery_temperature_k,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_hg_ref",
            "Reference h_g",
            Quantity::HeatTransferCoefficient,
            &mut workspace.thermal.reference_gas_side_htc_w_m2_k,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_wall_t_guess",
            "Initial wall temperature guess",
            Quantity::Temperature,
            &mut workspace.thermal.wall_temperature_k,
        );

        left.separator();
        left.label("Wall/material");
        if workspace.thermal.material_library.is_empty() {
            workspace.last_error = Some("Material library empty; restoring defaults.".to_owned());
            workspace.thermal.material_library = RocketWorkspace::default().thermal.material_library;
            workspace.thermal.selected_material_name = "CuCrZr".to_owned();
        }
        left.horizontal(|ui| {
            ui.label("Library material:");
            egui::ComboBox::from_id_salt("rocket_thermal_material_select")
                .selected_text(&workspace.thermal.selected_material_name)
                .show_ui(ui, |ui| {
                    for m in &workspace.thermal.material_library {
                        ui.selectable_value(
                            &mut workspace.thermal.selected_material_name,
                            m.name.clone(),
                            &m.name,
                        );
                    }
                });
            if ui.button("Apply").clicked() {
                apply_selected_material(workspace);
            }
            if ui.button("Save/Update").clicked() {
                save_current_material_to_library(workspace);
            }
        });
        left.horizontal(|ui| {
            ui.label("Material name:");
            ui.text_edit_singleline(&mut workspace.thermal.wall.material_name);
        });
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_wall_k",
            "k_ref,wall",
            Quantity::ThermalConductivity,
            &mut workspace.thermal.wall.thermal_conductivity_w_m_k,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_wall_k_ref_t",
            "k reference temperature",
            Quantity::Temperature,
            &mut workspace.thermal.wall.thermal_conductivity_reference_temperature_k,
        );
        left.horizontal(|ui| {
            ui.label("k temp coefficient [1/K]:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.wall.thermal_conductivity_temp_coeff_per_k)
                    .speed(0.00001),
            );
        });
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_wall_allowable_t",
            "Allowable wall temperature",
            Quantity::Temperature,
            &mut workspace.thermal.wall.allowable_temperature_k,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_wall_thickness",
            "Wall thickness",
            Quantity::Length,
            &mut workspace.thermal.wall.thickness_m,
        );
        left.horizontal(|ui| {
            ui.label("Gas-side emissivity [-]:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.wall.gas_side_emissivity)
                    .speed(0.01)
                    .range(0.0..=1.0),
            );
        });

        let regen_active = matches!(
            workspace.thermal.cooling_mode,
            CoolingMode::Regenerative | CoolingMode::RegenerativeFilm
        );
        let film_active = matches!(
            workspace.thermal.cooling_mode,
            CoolingMode::Film | CoolingMode::RegenerativeFilm
        );

        if regen_active {
            left.separator();
            left.label("Coolant");
        left.horizontal(|ui| {
            ui.checkbox(
                &mut workspace.thermal.use_coolant_properties_from_fluids,
                "Use fluid model properties",
            );
            ui.checkbox(
                &mut workspace.thermal.coolant_property_override,
                "Override properties manually",
            );
        });
        left.horizontal(|ui| {
            ui.checkbox(
                &mut workspace.thermal.use_engine_propellant_coolant,
                "Use engine propellants as coolant source",
            );
        });
        if workspace.thermal.use_engine_propellant_coolant {
            left.horizontal(|ui| {
                ui.label("Fuel multiplier:");
                ui.add(
                    egui::DragValue::new(&mut workspace.thermal.coolant_fuel_multiplier).speed(0.05),
                );
                ui.label("Ox multiplier:");
                ui.add(
                    egui::DragValue::new(&mut workspace.thermal.coolant_oxidizer_multiplier)
                        .speed(0.05),
                );
            });
            left.horizontal(|ui| {
                ui.checkbox(
                    &mut workspace.thermal.include_film_in_propellant_coolant_mix,
                    "Include film cooling in coolant blend",
                );
            });
        }
        if !workspace.thermal.use_engine_propellant_coolant {
            left.horizontal(|ui| {
                ui.label("Fluid species:");
                let selected_label = practical_coolprop_catalog()
                    .iter()
                    .find(|e| e.canonical_id == workspace.thermal.selected_coolant_species)
                    .map(|e| e.display_name)
                    .unwrap_or("Select species");
                egui::ComboBox::from_id_salt("rocket_thermal_coolant_species")
                    .selected_text(selected_label)
                    .show_ui(ui, |ui| {
                        for entry in practical_coolprop_catalog() {
                            ui.selectable_value(
                                &mut workspace.thermal.selected_coolant_species,
                                entry.canonical_id.to_owned(),
                                format!("{} ({})", entry.display_name, entry.canonical_id),
                            );
                        }
                    });
                if ui.button("Load from fluid tools").clicked() {
                    if let Err(err) = sync_coolant_from_fluids(workspace) {
                        workspace.last_error = Some(err);
                    }
                }
            });
        } else {
            left.small("Single-species coolant selector hidden while propellant coolant blend is active.");
        }
        left.horizontal(|ui| {
            ui.label("Coolant name:");
            ui.text_edit_singleline(&mut workspace.thermal.coolant.coolant_name);
        });
        if workspace.thermal.use_engine_propellant_coolant {
            ui_row_derived_mdot(left, workspace);
        } else {
            unit_field(
                left,
                &mut workspace.unit_inputs,
                "rocket_thermal_coolant_mdot",
                "m_dot coolant",
                Quantity::MassFlow,
                &mut workspace.thermal.coolant.mass_flow_kg_s,
            );
        }
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_coolant_t_in",
            "Coolant inlet temperature",
            Quantity::Temperature,
            &mut workspace.thermal.coolant.inlet_temperature_k,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_coolant_p_in",
            "Coolant inlet pressure",
            Quantity::Pressure,
            &mut workspace.thermal.coolant.inlet_pressure_pa,
        );
        left.horizontal(|ui| {
            ui.label("Flow direction:");
            ui.selectable_value(
                &mut workspace.thermal.design.coolant_flow_direction,
                CoolantFlowDirection::CoFlow,
                "Co-flow",
            );
            ui.selectable_value(
                &mut workspace.thermal.design.coolant_flow_direction,
                CoolantFlowDirection::CounterFlow,
                "Counter-flow",
            );
            ui.selectable_value(
                &mut workspace.thermal.design.coolant_flow_direction,
                CoolantFlowDirection::MidFeed,
                "Mid-feed",
            );
        });
        if matches!(
            workspace.thermal.design.coolant_flow_direction,
            CoolantFlowDirection::MidFeed
        ) {
            left.horizontal(|ui| {
                ui.label("Feed location x/L:");
                ui.add(
                    egui::DragValue::new(&mut workspace.thermal.design.mid_feed_fraction)
                        .speed(0.01)
                        .range(0.0..=1.0),
                );
                ui.checkbox(
                    &mut workspace.thermal.design.auto_balance_mid_feed_split,
                    "Auto-balance split",
                );
            });
            if !workspace.thermal.design.auto_balance_mid_feed_split {
                left.horizontal(|ui| {
                    ui.label("Upstream mass split:");
                    ui.add(
                        egui::DragValue::new(
                            &mut workspace.thermal.design.mid_feed_upstream_mass_fraction,
                        )
                        .speed(0.01)
                        .range(0.0..=1.0),
                    );
                });
            }
        }
        let allow_manual = !workspace.thermal.use_coolant_properties_from_fluids
            || workspace.thermal.coolant_property_override;
        left.add_enabled_ui(allow_manual, |ui| {
            unit_field(
                ui,
                &mut workspace.unit_inputs,
                "rocket_thermal_coolant_rho",
                "Coolant density",
                Quantity::Density,
                &mut workspace.thermal.coolant.density_kg_m3,
            );
            unit_field(
                ui,
                &mut workspace.unit_inputs,
                "rocket_thermal_coolant_mu",
                "Coolant viscosity",
                Quantity::DynamicViscosity,
                &mut workspace.thermal.coolant.viscosity_pa_s,
            );
            unit_field(
                ui,
                &mut workspace.unit_inputs,
                "rocket_thermal_coolant_k",
                "Coolant thermal conductivity",
                Quantity::ThermalConductivity,
                &mut workspace.thermal.coolant.thermal_conductivity_w_m_k,
            );
            unit_field(
                ui,
                &mut workspace.unit_inputs,
                "rocket_thermal_coolant_cp",
                "Coolant cp",
                Quantity::SpecificHeatCapacity,
                &mut workspace.thermal.coolant.cp_j_kg_k,
            );
        });
        if workspace.thermal.use_coolant_properties_from_fluids && !workspace.thermal.coolant_property_override {
            left.small("Density and cp are sourced from tf-fluids at inlet P/T (viscosity and k remain editable placeholders).");
        }
        } else {
            left.separator();
            left.small("Regenerative coolant controls hidden (active only for Regen/Regen+Film modes).");
        }

        if film_active {
            left.separator();
            left.label("Film model");
        left.horizontal(|ui| {
            ui.label("Film mass fraction:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.film.film_mass_fraction)
                    .speed(0.001)
                    .range(0.0..=0.2),
            );
            ui.label("Max effectiveness:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.film.max_effectiveness)
                    .speed(0.01)
                    .range(0.0..=0.95),
            );
        });
        left.horizontal(|ui| {
            ui.label("Film start x/L:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.film.effectiveness_start_fraction)
                    .speed(0.01)
                    .range(0.0..=1.0),
            );
            ui.label("end x/L:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.film.effectiveness_end_fraction)
                    .speed(0.01)
                    .range(0.0..=1.0),
            );
        });
        } else {
            left.separator();
            left.small("Film controls hidden (active only for Film/Regen+Film modes).");
        }

        if regen_active {
            left.separator();
            left.label("Channels + optimizer");
        left.horizontal(|ui| {
            ui.checkbox(
                &mut workspace.thermal.design.hold_channel_count_fixed,
                "Hold",
            );
            ui.label("Channel count:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.channels.channel_count).range(8..=2000),
            );
        });
        left.horizontal(|ui| {
            ui.checkbox(
                &mut workspace.thermal.design.hold_channel_width_fixed,
                "Hold",
            );
            unit_field(
                ui,
                &mut workspace.unit_inputs,
                "rocket_thermal_channel_width",
                "Channel width",
                Quantity::Length,
                &mut workspace.thermal.channels.width_m,
            );
        });
        left.horizontal(|ui| {
            ui.checkbox(
                &mut workspace.thermal.design.hold_channel_height_fixed,
                "Hold",
            );
            unit_field(
                ui,
                &mut workspace.unit_inputs,
                "rocket_thermal_channel_height",
                "Channel height",
                Quantity::Length,
                &mut workspace.thermal.channels.height_m,
            );
        });
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_rib",
            "Rib width",
            Quantity::Length,
            &mut workspace.thermal.channels.rib_width_m,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_min_gap",
            "Min inter-channel gap",
            Quantity::Length,
            &mut workspace.thermal.channels.min_gap_m,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_roughness",
            "Wall roughness",
            Quantity::Length,
            &mut workspace.thermal.channels.roughness_m,
        );
        left.horizontal(|ui| {
            ui.label("Roughness preset:");
            let current_label = roughness_preset_label(workspace.thermal.channels.roughness_m);
            egui::ComboBox::from_id_salt("rocket_thermal_roughness_preset")
                .selected_text(current_label)
                .show_ui(ui, |ui| {
                    for (label, value_m) in roughness_presets() {
                        if ui.selectable_label(false, *label).clicked() {
                            workspace.thermal.channels.roughness_m = *value_m;
                        }
                    }
                });
        });
        left.horizontal(|ui| {
            ui.label("Width taper end factor:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.channels.width_taper_end_factor)
                    .speed(0.01),
            );
            ui.label("Height taper end factor:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.channels.height_taper_end_factor)
                    .speed(0.01),
            );
        });
        left.horizontal(|ui| {
            ui.label("Min channel count:");
            ui.add(egui::DragValue::new(&mut workspace.thermal.design.min_channel_count).range(1..=5000));
            ui.label("Max channel count:");
            ui.add(egui::DragValue::new(&mut workspace.thermal.design.max_channel_count).range(1..=5000));
        });
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_width_min",
            "Min channel width",
            Quantity::Length,
            &mut workspace.thermal.channels.min_width_m,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_width_max",
            "Max channel width",
            Quantity::Length,
            &mut workspace.thermal.channels.max_width_m,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_height_min",
            "Min channel height",
            Quantity::Length,
            &mut workspace.thermal.channels.min_height_m,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_channel_height_max",
            "Max channel height",
            Quantity::Length,
            &mut workspace.thermal.channels.max_height_m,
        );
        unit_field(
            left,
            &mut workspace.unit_inputs,
            "rocket_thermal_dp_max",
            "Max coolant pressure drop",
            Quantity::Pressure,
            &mut workspace.thermal.design.max_coolant_pressure_drop_pa,
        );

        left.horizontal(|ui| {
            ui.label("Stations:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.design.station_count).range(5..=300),
            );
            ui.label("Opt iters:");
            ui.add(
                egui::DragValue::new(&mut workspace.thermal.design.optimizer_max_iterations)
                    .range(1..=200),
            );
        });
        }

        if left.button("Run Thermal Design").clicked() {
            match solve_thermal(workspace) {
                Ok(msg) => {
                    workspace.status = Some(msg);
                    workspace.last_error = None;
                }
                Err(err) => {
                    workspace.last_error = Some(err);
                    workspace.status = None;
                }
            }
        }

        let right = &mut columns[1];
        right.heading("C) Results Summary");
        if let Some(result) = &workspace.last_thermal_result {
            egui::Grid::new("rocket_thermal_design_outputs")
                .num_columns(2)
                .spacing([18.0, 6.0])
                .show(right, |ui| {
                    ui.label("Peak wall temperature [K]");
                    ui.label(format!("{:.1}", result.design.peak_wall_temperature_k));
                    ui.end_row();
                    ui.label("Peak station index");
                    ui.label(format!("{}", result.design.peak_wall_temperature_station));
                    ui.end_row();
                    ui.label("Coolant outlet temperature [K]");
                    ui.label(format!("{:.1}", result.design.coolant_outlet_temperature_k));
                    ui.end_row();
                    ui.label("Coolant pressure drop [Pa]");
                    ui.label(format!(
                        "{:.0}",
                        result.design.pressure_drop.total_pressure_drop_pa
                    ));
                    ui.end_row();
                    ui.label("Pressure-drop limit [Pa]");
                    ui.label(format!(
                        "{:.0}",
                        result.design.pressure_drop.limit_pressure_drop_pa
                    ));
                    ui.end_row();
                    ui.label("Constraint satisfied");
                    ui.label(if result.design.pressure_drop.within_limit {
                        "yes"
                    } else {
                        "no"
                    });
                    ui.end_row();
                    ui.label("Objective value");
                    ui.label(format!("{:.2}", result.design.objective_value));
                    ui.end_row();
                    ui.label("Material wall-T margin [K]");
                    ui.label(format!(
                        "{:.1}",
                        workspace.thermal.wall.allowable_temperature_k
                            - result.design.peak_wall_temperature_k
                    ));
                    ui.end_row();
                    ui.label("Min coolant pressure [Pa]");
                    ui.label(format!(
                        "{:.0}",
                        result.design.pressure_drop.min_coolant_pressure_pa
                    ));
                    ui.end_row();
                    ui.label("Final channel count");
                    ui.label(format!(
                        "{} (feasible max: {})",
                        result.design.final_channel_count, result.design.feasible_channel_count_max
                    ));
                    ui.end_row();
                    ui.label("Width bound hits");
                    ui.label(format!("{}", result.design.channel_width_bound_hits));
                    ui.end_row();
                    ui.label("Height bound hits");
                    ui.label(format!("{}", result.design.channel_height_bound_hits));
                    ui.end_row();
                });

            right.separator();
            right.heading("Constraint Diagnostics");
            if result.design.unmet_constraints.is_empty() {
                right.colored_label(egui::Color32::LIGHT_GREEN, "All tracked constraints satisfied.");
            } else {
                right.colored_label(egui::Color32::RED, "Unmet constraints:");
                for c in &result.design.unmet_constraints {
                    right.colored_label(egui::Color32::RED, format!("- {c}"));
                }
                right.separator();
                right.strong("Convergence hints");
                for hint in build_constraint_hints(workspace, result) {
                    right.small(format!("- {hint}"));
                }
            }

            right.separator();
            right.heading("Channel Profile Outcome");
            show_channel_profile_outcome(right, workspace, &result.design.stations);

            right.separator();
            right.heading("D) Equations / Assumptions");
            right.collapsing("Equations Used", |ui| {
                for eq in &result.design.equation_traces {
                    ui.strong(&eq.name);
                    ui.small(format!(
                        "{} [{}]",
                        eq.equation,
                        match eq.source {
                            tf_rpa::EquationSource::Correlation => "correlation",
                            tf_rpa::EquationSource::Derived => "derived",
                            tf_rpa::EquationSource::Constraint => "constraint",
                        }
                    ));
                    ui.small(&eq.notes);
                    ui.separator();
                }
            });
            right.collapsing("Model Assumptions", |ui| {
                for a in &result.design.assumptions {
                    ui.strong(format!("{} ({})", a.id, a.category));
                    ui.small(&a.statement);
                    ui.small(format!("Impact: {}", a.impact));
                    ui.separator();
                }
            });
            right.collapsing("Optimizer Trace", |ui| {
                let accepted = result
                    .design
                    .optimizer_iterations
                    .iter()
                    .filter(|it| it.accepted)
                    .count();
                ui.small(format!(
                    "Accepted iterations: {}/{}",
                    accepted,
                    result.design.optimizer_iterations.len()
                ));
                if let Some(last) = result.design.optimizer_iterations.last() {
                    ui.small(format!(
                        "Last iter: action={} | count={} | peakT={:.1}K | dP={:.0}Pa | minP={:.0}Pa | width_hits={} | height_hits={} | obj={:.2}",
                        last.action,
                        last.channel_count,
                        last.peak_wall_temperature_k,
                        last.pressure_drop_pa,
                        last.min_coolant_pressure_pa,
                        last.width_bound_hits,
                        last.height_bound_hits,
                        last.objective_value
                    ));
                }
                ui.separator();
                let tail = result.design.optimizer_iterations.len().saturating_sub(10);
                for it in result.design.optimizer_iterations.iter().skip(tail) {
                    ui.small(format!(
                        "iter {}: count={}, peakT={:.1}K, dP={:.0}Pa, minP={:.0}Pa, width_hits={}, height_hits={}, obj={:.2}, accepted={}, action={}",
                        it.iteration,
                        it.channel_count,
                        it.peak_wall_temperature_k,
                        it.pressure_drop_pa,
                        it.min_coolant_pressure_pa,
                        it.width_bound_hits,
                        it.height_bound_hits,
                        it.objective_value,
                        it.accepted,
                        it.action
                    ));
                }
            });
        } else {
            right.label("No thermal design result yet.");
        }
            });

            if let Some(status) = &workspace.status {
                ui.colored_label(egui::Color32::GREEN, status);
            }
            if let Some(err) = &workspace.last_error {
                ui.colored_label(egui::Color32::RED, err);
            }

            ui.separator();
            if let Some(result) = &workspace.last_thermal_result {
        let stations = &result.design.stations;
        let geometry = workspace.last_geometry_result.as_ref();

        ui.heading("Thermal Design Plots");
        ui.horizontal_wrapped(|ui| {
            for tab in RocketThermalPlotTab::all() {
                ui.selectable_value(&mut workspace.thermal_plot_tab, *tab, tab.label());
            }
        });
        ui.small("Each tab groups traces by unit family to keep relative magnitudes readable.");
        ui.separator();

        let (plot_id, y_label, series) = build_plot_series(workspace.thermal_plot_tab, stations);
        let x_bounds = plot_x_bounds(geometry, stations);
        show_multi_series_plot_with_bounds(
            ui,
            plot_id,
            "Axial position x [m]",
            y_label,
            280.0,
            series,
            x_bounds,
        );
        show_geometry_context_plot(
            ui,
            plot_id,
            geometry,
            stations,
            result.design.peak_wall_temperature_station,
            x_bounds,
        );
            }
        });
}

fn unit_field(
    ui: &mut egui::Ui,
    unit_inputs: &mut crate::input_helper::UnitAwareInput,
    field_id: &str,
    label: &str,
    quantity: Quantity,
    value: &mut f64,
) {
    if let Some(new_val) = unit_inputs.show_field(ui, field_id, label, quantity, *value) {
        *value = new_val;
    }
}

fn build_plot_series(
    tab: RocketThermalPlotTab,
    stations: &[ThermalDesignStation],
) -> (&'static str, &'static str, Vec<PlotSeriesSpec<'static>>) {
    let mut series = Vec::new();
    match tab {
        RocketThermalPlotTab::Temperatures => {
            series.push(PlotSeriesSpec {
                name: "T_aw,eff [K]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.adiabatic_wall_temperature_k])
                    .collect(),
            });
            series.push(PlotSeriesSpec {
                name: "T_wall,hot [K]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.wall_hot_side_temperature_k])
                    .collect(),
            });
            series.push(PlotSeriesSpec {
                name: "T_wall,cold [K]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.wall_cold_side_temperature_k])
                    .collect(),
            });
            series.push(PlotSeriesSpec {
                name: "T_coolant [K]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.coolant_bulk_temperature_k])
                    .collect(),
            });
            ("rocket_thermal_plot_temperature", "Temperature [K]", series)
        }
        RocketThermalPlotTab::HeatFlux => {
            series.push(PlotSeriesSpec {
                name: "q'' [W/m^2]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.net_gas_heat_flux_w_m2])
                    .collect(),
            });
            ("rocket_thermal_plot_heatflux", "Heat flux [W/m^2]", series)
        }
        RocketThermalPlotTab::HeatTransferCoeff => {
            series.push(PlotSeriesSpec {
                name: "h_g [W/m^2-K]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.gas_side_htc_w_m2_k])
                    .collect(),
            });
            series.push(PlotSeriesSpec {
                name: "h_c [W/m^2-K]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.coolant_side_htc_w_m2_k])
                    .collect(),
            });
            ("rocket_thermal_plot_htc", "HTC [W/m^2-K]", series)
        }
        RocketThermalPlotTab::Pressure => {
            series.push(PlotSeriesSpec {
                name: "P_coolant [Pa]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.coolant_pressure_pa])
                    .collect(),
            });
            series.push(PlotSeriesSpec {
                name: "dP_local [Pa]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.local_pressure_drop_pa])
                    .collect(),
            });
            ("rocket_thermal_plot_pressure", "Pressure [Pa]", series)
        }
        RocketThermalPlotTab::ChannelGeometry => {
            series.push(PlotSeriesSpec {
                name: "Channel width [m]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.channel_width_m])
                    .collect(),
            });
            series.push(PlotSeriesSpec {
                name: "Channel height [m]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.channel_height_m])
                    .collect(),
            });
            ("rocket_thermal_plot_channels", "Length [m]", series)
        }
        RocketThermalPlotTab::FilmEffectiveness => {
            series.push(PlotSeriesSpec {
                name: "Film effectiveness [-]",
                points: stations
                    .iter()
                    .map(|s| [s.axial_position_m, s.film_effectiveness])
                    .collect(),
            });
            ("rocket_thermal_plot_film", "Film effectiveness [-]", series)
        }
    }
}

fn show_geometry_context_plot(
    ui: &mut egui::Ui,
    base_id: &str,
    geometry: Option<&RocketGeometryResult>,
    stations: &[ThermalDesignStation],
    peak_station: usize,
    x_bounds: (f64, f64),
) {
    let Some(geom) = geometry else {
        ui.small(
            "Geometry context unavailable (run Geometry once to overlay half-profile context).",
        );
        return;
    };
    if stations.is_empty() {
        return;
    }

    ui.label("Geometry context (half profile)");
    let upper = geom.canonical_model.wall_contour_upper.clone();
    let throat_x = geom.canonical_model.throat_axial_m;
    let peak_x = stations
        .get(peak_station)
        .map(|s| s.axial_position_m)
        .unwrap_or(stations[0].axial_position_m);
    let peak_r = interpolate_radius_at_x(&upper, peak_x).unwrap_or(upper[0][1]);

    Plot::new(format!("{base_id}_geometry_context"))
        .legend(egui_plot::Legend::default())
        .height(140.0)
        .include_x(x_bounds.0)
        .include_x(x_bounds.1)
        .x_axis_label("Axial position x [m]")
        .y_axis_label("Radius r [m]")
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(upper.clone())).name("Wall contour (half)"));
            plot_ui.points(
                Points::new(vec![[
                    throat_x,
                    interpolate_radius_at_x(&upper, throat_x).unwrap_or(0.0),
                ]])
                .name("Throat")
                .radius(4.0),
            );
            plot_ui.points(
                Points::new(vec![[peak_x, peak_r]])
                    .name("Peak wall-T station")
                    .radius(4.0),
            );
        });
}

fn show_multi_series_plot_with_bounds(
    ui: &mut egui::Ui,
    id: &str,
    x_label: &str,
    y_label: &str,
    height: f32,
    series: Vec<PlotSeriesSpec<'_>>,
    x_bounds: (f64, f64),
) {
    Plot::new(id)
        .legend(egui_plot::Legend::default())
        .height(height)
        .include_x(x_bounds.0)
        .include_x(x_bounds.1)
        .x_axis_label(x_label)
        .y_axis_label(y_label)
        .show(ui, |plot_ui| {
            for s in series {
                if s.points.is_empty() {
                    continue;
                }
                let line = Line::new(PlotPoints::from(s.points)).name(s.name);
                plot_ui.line(line);
            }
        });
}

fn plot_x_bounds(
    geometry: Option<&RocketGeometryResult>,
    stations: &[ThermalDesignStation],
) -> (f64, f64) {
    let x_min = geometry
        .and_then(|g| g.canonical_model.wall_contour_upper.first().map(|p| p[0]))
        .or_else(|| stations.first().map(|s| s.axial_position_m))
        .unwrap_or(0.0);
    let x_max = geometry
        .and_then(|g| g.canonical_model.wall_contour_upper.last().map(|p| p[0]))
        .or_else(|| stations.last().map(|s| s.axial_position_m))
        .unwrap_or(1.0);
    (x_min, x_max)
}

fn interpolate_radius_at_x(contour: &[[f64; 2]], x: f64) -> Option<f64> {
    if contour.is_empty() {
        return None;
    }
    if x <= contour[0][0] {
        return Some(contour[0][1]);
    }
    for pair in contour.windows(2) {
        let a = pair[0];
        let b = pair[1];
        if x >= a[0] && x <= b[0] {
            let dx = (b[0] - a[0]).max(1.0e-12);
            let t = (x - a[0]) / dx;
            return Some(a[1] + t * (b[1] - a[1]));
        }
    }
    contour.last().map(|p| p[1])
}

fn show_channel_profile_outcome(
    ui: &mut egui::Ui,
    workspace: &RocketWorkspace,
    stations: &[ThermalDesignStation],
) {
    if stations.is_empty() {
        ui.small("No station data available.");
        return;
    }
    let n = stations.len();
    let picks = [0usize, n / 2, n - 1];
    let labels = ["Chamber end", "Near throat", "Exit end"];

    egui::Grid::new("rocket_thermal_channel_profile_changes")
        .num_columns(5)
        .spacing([10.0, 5.0])
        .show(ui, |ui| {
            ui.strong("Location");
            ui.strong("Width init->final [mm]");
            ui.strong("Height init->final [mm]");
            ui.strong("Area ratio");
            ui.strong("x [m]");
            ui.end_row();

            for (label, idx) in labels.into_iter().zip(picks) {
                let s = &stations[idx];
                let xf = s.x_fraction;
                let init_w = workspace.thermal.channels.width_m
                    * (1.0 + xf * (workspace.thermal.channels.width_taper_end_factor - 1.0));
                let init_h = workspace.thermal.channels.height_m
                    * (1.0 + xf * (workspace.thermal.channels.height_taper_end_factor - 1.0));
                let init_area = (init_w * init_h).max(1.0e-16);
                let final_area = (s.channel_width_m * s.channel_height_m).max(1.0e-16);

                ui.label(label);
                ui.label(format!(
                    "{:.3} -> {:.3}",
                    init_w * 1.0e3,
                    s.channel_width_m * 1.0e3
                ));
                ui.label(format!(
                    "{:.3} -> {:.3}",
                    init_h * 1.0e3,
                    s.channel_height_m * 1.0e3
                ));
                ui.label(format!("{:.3}", final_area / init_area));
                ui.label(format!("{:.4}", s.axial_position_m));
                ui.end_row();
            }
        });
    ui.small("Area ratio > 1 means local channel area opened versus baseline taper; < 1 means reduced area.");
    if let Some(last) = stations.last() {
        let max_feasible = estimate_max_feasible_channel_count(
            workspace.last_geometry_result.as_ref(),
            stations,
            workspace.thermal.channels.min_gap_m,
        );
        ui.small(format!(
            "Channel count (final): {} | configured initial: {} | estimated feasible max (min gap {:.3} mm): {}",
            last.channel_count,
            workspace.thermal.channels.channel_count,
            workspace.thermal.channels.min_gap_m * 1.0e3,
            max_feasible
        ));
    }
}

fn estimate_max_feasible_channel_count(
    geometry: Option<&RocketGeometryResult>,
    stations: &[ThermalDesignStation],
    min_gap_m: f64,
) -> usize {
    let Some(geom) = geometry else {
        return 0;
    };
    if stations.is_empty() {
        return 0;
    }
    let mut max_count = usize::MAX;
    for s in stations {
        let r = contour_radius_at_x(geom, s.axial_position_m).max(1.0e-8);
        let circumference = 2.0 * std::f64::consts::PI * r;
        let pitch = (s.channel_width_m + min_gap_m).max(1.0e-9);
        let local_max = (circumference / pitch).floor() as usize;
        max_count = max_count.min(local_max.max(1));
    }
    max_count
}

fn contour_radius_at_x(geometry: &RocketGeometryResult, x: f64) -> f64 {
    let contour = &geometry.canonical_model.wall_contour_upper;
    if contour.is_empty() {
        return 0.5 * geometry.throat_diameter_m;
    }
    if x <= contour[0][0] {
        return contour[0][1];
    }
    for segment in contour.windows(2) {
        let a = segment[0];
        let b = segment[1];
        if x <= b[0] {
            let dx = (b[0] - a[0]).max(1.0e-12);
            let t = ((x - a[0]) / dx).clamp(0.0, 1.0);
            return a[1] + t * (b[1] - a[1]);
        }
    }
    contour
        .last()
        .map(|p| p[1])
        .unwrap_or(0.5 * geometry.throat_diameter_m)
}

fn roughness_presets() -> &'static [(&'static str, f64)] {
    &[
        ("Custom", f64::NAN),
        ("Polished machined (~1.5 um)", 1.5e-6),
        ("Typical machined (~6 um)", 6.0e-6),
        ("EDM (~12 um)", 12.0e-6),
        ("As-printed SLM (~20 um)", 20.0e-6),
        ("As-printed rough (~35 um)", 35.0e-6),
    ]
}

fn roughness_preset_label(value_m: f64) -> String {
    for (label, preset) in roughness_presets() {
        if preset.is_finite() && (value_m - *preset).abs() <= 1.0e-12 {
            return (*label).to_owned();
        }
    }
    "Custom".to_owned()
}

fn build_constraint_hints(
    workspace: &RocketWorkspace,
    result: &tf_rpa::RocketThermalResult,
) -> Vec<String> {
    let mut hints = Vec::new();
    if result.design.pressure_drop.total_pressure_drop_pa
        > result.design.pressure_drop.limit_pressure_drop_pa
    {
        hints.push(
            "Pressure drop too high: increase channel width/height max bounds, lower coolant mass flow, or increase channel count if not held."
                .to_owned(),
        );
        hints.push(
            "If width/height bound hits are high, relax geometric bounds or taper factors so optimizer has room."
                .to_owned(),
        );
    }
    if result.design.pressure_drop.min_coolant_pressure_pa < 0.0 {
        hints.push(
            "Negative coolant pressure: raise inlet pressure, reduce pressure-drop target severity, or increase hydraulic area (width/height/count)."
                .to_owned(),
        );
    }
    if result.design.peak_wall_temperature_k > workspace.thermal.wall.allowable_temperature_k {
        hints.push(
            "Wall temperature above allowable: increase coolant-side heat transfer, increase film effectiveness, or reduce gas-side recovery/HTC assumptions."
                .to_owned(),
        );
    }
    if result.design.final_channel_count >= result.design.feasible_channel_count_max {
        hints.push(
            "Channel count reached feasibility ceiling: reduce min gap or channel width, or adjust contour so local circumference is larger in tight sections."
                .to_owned(),
        );
    }
    if hints.is_empty() {
        hints.push("No specific hint available; inspect optimizer trace for bound hits and pressure margin.".to_owned());
    }
    hints
}

fn solve_thermal(workspace: &mut RocketWorkspace) -> Result<String, String> {
    let regen_active = matches!(
        workspace.thermal.cooling_mode,
        CoolingMode::Regenerative | CoolingMode::RegenerativeFilm
    );
    if regen_active
        && workspace.thermal.use_coolant_properties_from_fluids
        && !workspace.thermal.coolant_property_override
    {
        sync_coolant_from_fluids(workspace)?;
    }
    let geometry_result = if let Some(result) = workspace.last_geometry_result.clone() {
        result
    } else {
        compute_geometry(&workspace.geometry_problem())
            .map_err(|e| format!("Geometry prerequisite failed: {e}"))?
    };
    workspace.last_geometry_result = Some(geometry_result.clone());
    if regen_active && workspace.thermal.use_engine_propellant_coolant {
        workspace.thermal.coolant.mass_flow_kg_s =
            derive_coolant_mass_flow_from_performance(workspace, &geometry_result)?;
    }

    let problem = workspace.thermal_problem(geometry_result);
    let result = compute_thermal(&problem).map_err(|e| format!("Thermal solve failed: {e}"))?;
    let within = result.design.pressure_drop.within_limit;
    let peak_t = result.design.peak_wall_temperature_k;
    workspace.last_thermal_result = Some(result);
    Ok(format!(
        "Thermal design complete: peak wall T={:.1} K, pressure-drop constraint met={}",
        peak_t, within
    ))
}

fn ui_row_derived_mdot(ui: &mut egui::Ui, workspace: &RocketWorkspace) {
    ui.horizontal(|ui| {
        ui.label("m_dot coolant");
        if let Some(geometry) = workspace.last_geometry_result.as_ref() {
            match derive_coolant_mass_flow_from_performance(workspace, geometry) {
                Ok(mdot) => {
                    ui.label(format!("{:.4} kg/s (from Performance)", mdot));
                }
                Err(err) => {
                    ui.colored_label(egui::Color32::YELLOW, format!("unavailable ({err})"));
                }
            }
        } else {
            ui.colored_label(
                egui::Color32::YELLOW,
                "unavailable (run Performance + Geometry)",
            );
        }
    });
}

fn derive_coolant_mass_flow_from_performance(
    workspace: &RocketWorkspace,
    geometry: &RocketGeometryResult,
) -> Result<f64, String> {
    if let Some(result) = workspace.last_result.as_ref() {
        let at = geometry.throat_area_m2;
        if !at.is_finite() || at <= 0.0 {
            return Err("invalid throat area".to_owned());
        }
        let pc = result.chamber_pressure_pa;
        let c_star = result.characteristic_velocity_m_per_s;
        if !pc.is_finite() || pc <= 0.0 || !c_star.is_finite() || c_star <= 0.0 {
            return Err("invalid performance result (Pc or c*)".to_owned());
        }
        return Ok(pc * at / c_star);
    }
    if let PerformanceConstraint::MassFlow { target_kg_per_s } =
        workspace.performance_case.performance_constraint
        && target_kg_per_s > 0.0
    {
        return Ok(target_kg_per_s);
    }
    Err("run Performance solve to derive mass flow".to_owned())
}

fn apply_selected_material(workspace: &mut RocketWorkspace) {
    if let Some(entry) = workspace
        .thermal
        .material_library
        .iter()
        .find(|m| m.name == workspace.thermal.selected_material_name)
        .cloned()
    {
        workspace.thermal.wall.material_name = entry.name.clone();
        workspace.thermal.wall.thermal_conductivity_w_m_k = entry.k_reference_w_m_k;
        workspace
            .thermal
            .wall
            .thermal_conductivity_reference_temperature_k = entry.k_reference_temperature_k;
        workspace.thermal.wall.thermal_conductivity_temp_coeff_per_k = entry.k_temp_coeff_per_k;
        workspace.thermal.wall.allowable_temperature_k = entry.allowable_temperature_k;
        workspace.thermal.wall.density_kg_m3 = entry.density_kg_m3;
        workspace.thermal.wall.cp_j_kg_k = entry.cp_j_kg_k;
    }
}

fn save_current_material_to_library(workspace: &mut RocketWorkspace) {
    let name = workspace.thermal.wall.material_name.trim();
    if name.is_empty() {
        workspace.last_error = Some("Material name cannot be empty.".to_owned());
        return;
    }

    let candidate = crate::rocket_workspace::RocketThermalMaterial {
        name: name.to_owned(),
        k_reference_w_m_k: workspace.thermal.wall.thermal_conductivity_w_m_k,
        k_reference_temperature_k: workspace
            .thermal
            .wall
            .thermal_conductivity_reference_temperature_k,
        k_temp_coeff_per_k: workspace.thermal.wall.thermal_conductivity_temp_coeff_per_k,
        allowable_temperature_k: workspace.thermal.wall.allowable_temperature_k,
        density_kg_m3: workspace.thermal.wall.density_kg_m3,
        cp_j_kg_k: workspace.thermal.wall.cp_j_kg_k,
    };

    if let Some(existing) = workspace
        .thermal
        .material_library
        .iter_mut()
        .find(|m| m.name.eq_ignore_ascii_case(name))
    {
        *existing = candidate;
    } else {
        workspace.thermal.material_library.push(candidate);
    }
    workspace.thermal.selected_material_name = name.to_owned();
    workspace.status = Some(format!("Saved material '{}'.", name));
}

fn sync_coolant_from_fluids(workspace: &mut RocketWorkspace) -> Result<(), String> {
    if workspace.thermal.use_engine_propellant_coolant {
        return sync_coolant_from_engine_propellants(workspace);
    }

    let species: Species = workspace
        .thermal
        .selected_coolant_species
        .parse()
        .map_err(|e| {
            format!(
                "Invalid coolant species '{}': {e}",
                workspace.thermal.selected_coolant_species
            )
        })?;
    let model = CoolPropModel::new();
    let state = compute_equilibrium_state(
        &model,
        species,
        FluidInputPair::PT,
        workspace.thermal.coolant.inlet_pressure_pa,
        workspace.thermal.coolant.inlet_temperature_k,
    )
    .map_err(|e| format!("Fluid property fetch failed: {e}"))?;

    workspace.thermal.coolant.coolant_name = species.key().to_owned();
    workspace.thermal.coolant.density_kg_m3 = state.density_kg_m3();
    workspace.thermal.coolant.cp_j_kg_k = state.cp_j_per_kg_k;
    workspace.status = Some(format!(
        "Coolant properties synced from tf-fluids for {} at inlet P/T (rho and cp).",
        species.key()
    ));
    Ok(())
}

fn sync_coolant_from_engine_propellants(workspace: &mut RocketWorkspace) -> Result<(), String> {
    let mut fuel_weight = workspace.thermal.coolant_fuel_multiplier.max(0.0);
    let ox_weight = workspace.thermal.coolant_oxidizer_multiplier.max(0.0);
    if workspace.thermal.include_film_in_propellant_coolant_mix
        && matches!(
            workspace.thermal.cooling_mode,
            CoolingMode::Film | CoolingMode::RegenerativeFilm
        )
    {
        fuel_weight += workspace.thermal.film.film_mass_fraction.max(0.0);
    }
    let total = fuel_weight + ox_weight;
    if total <= 0.0 {
        return Err("Propellant coolant multipliers must sum to > 0.".to_owned());
    }
    let fuel_frac = fuel_weight / total;
    let ox_frac = ox_weight / total;

    let model = CoolPropModel::new();
    let fuel_species = map_engine_propellant_to_fluid_species(
        &workspace.performance_case.fuel_name,
    )
    .ok_or_else(|| {
        format!(
            "Fuel '{}' is not available in tf-fluids species.",
            workspace.performance_case.fuel_name
        )
    })?;
    let ox_species =
        map_engine_propellant_to_fluid_species(&workspace.performance_case.oxidizer_name)
            .ok_or_else(|| {
                format!(
                    "Oxidizer '{}' is not available in tf-fluids species.",
                    workspace.performance_case.oxidizer_name
                )
            })?;

    let p = workspace.thermal.coolant.inlet_pressure_pa;
    let t = workspace.thermal.coolant.inlet_temperature_k;
    let fuel_state = compute_equilibrium_state(&model, fuel_species, FluidInputPair::PT, p, t)
        .map_err(|e| format!("Fuel property fetch failed: {e}"))?;
    let ox_state = compute_equilibrium_state(&model, ox_species, FluidInputPair::PT, p, t)
        .map_err(|e| format!("Oxidizer property fetch failed: {e}"))?;

    workspace.thermal.coolant.coolant_name =
        format!("PropellantMix(fuel {:.2}, ox {:.2})", fuel_frac, ox_frac);
    workspace.thermal.coolant.density_kg_m3 =
        fuel_frac * fuel_state.density_kg_m3() + ox_frac * ox_state.density_kg_m3();
    workspace.thermal.coolant.cp_j_kg_k =
        fuel_frac * fuel_state.cp_j_per_kg_k + ox_frac * ox_state.cp_j_per_kg_k;
    workspace.status = Some(format!(
        "Coolant properties synced from propellant blend at inlet P/T (fuel {:.2}, ox {:.2}).",
        fuel_frac, ox_frac
    ));
    Ok(())
}

fn map_engine_propellant_to_fluid_species(name: &str) -> Option<Species> {
    let key = name.trim().to_ascii_uppercase();
    let mapped = match key.as_str() {
        "LOX" | "LIQUID OXYGEN" | "OXYGEN" => "O2",
        "RP1" | "RP-1" | "KEROSENE" => "nHexane",
        "LCH4" | "METHANE" => "CH4",
        "LH2" | "HYDROGEN" => "H2",
        "N2O4" | "NTO" => "N2O",
        "ETHANOL" => "Ethane",
        "IPA" | "ISOPROPANOL" => "Propane",
        _ => name.trim(),
    };
    mapped.parse::<Species>().ok()
}
