use crate::rocket_workspace::RocketWorkspace;
use crate::views::rocket::show_case_context_banner;
use tf_rpa::{GeometrySizingMode, NozzleContourStyle, RocketGeometryResult, compute_geometry};

pub fn show(ui: &mut egui::Ui, workspace: &mut RocketWorkspace) {
    ui.heading("Rocket Geometry");
    ui.label("First-pass chamber/nozzle sizing derived from the active Performance case.");
    ui.label(format!(
        "Base case: '{}' | Propellants: {} / {}",
        workspace.performance_case.case_name,
        workspace.performance_case.oxidizer_name,
        workspace.performance_case.fuel_name
    ));
    show_case_context_banner(ui, workspace);

    ui.separator();
    ui.columns(2, |columns| {
        let left = &mut columns[0];
        left.heading("Sizing inputs");
        left.horizontal(|ui| {
            ui.label("Mode:");
            ui.selectable_value(
                &mut workspace.geometry.sizing_mode,
                GeometrySizingMode::GivenThroatDiameter,
                "Given throat diameter",
            );
            ui.selectable_value(
                &mut workspace.geometry.sizing_mode,
                GeometrySizingMode::GivenThroatArea,
                "Given throat area",
            );
        });

        left.horizontal(|ui| match workspace.geometry.sizing_mode {
            GeometrySizingMode::GivenThroatDiameter => {
                ui.label("Throat diameter [m]:");
                ui.add(
                    egui::DragValue::new(&mut workspace.geometry.throat_input_value).speed(0.001),
                );
            }
            GeometrySizingMode::GivenThroatArea => {
                ui.label("Throat area [m²]:");
                ui.add(
                    egui::DragValue::new(&mut workspace.geometry.throat_input_value).speed(0.00001),
                );
            }
        });

        left.horizontal(|ui| {
            ui.label("Contraction ratio Ac/At:");
            ui.add(
                egui::DragValue::new(&mut workspace.geometry.chamber_contraction_ratio).speed(0.05),
            );
        });
        left.horizontal(|ui| {
            ui.label("Characteristic length L* [m]:");
            ui.add(
                egui::DragValue::new(&mut workspace.geometry.characteristic_length_m).speed(0.01),
            );
        });
        left.horizontal(|ui| {
            ui.label("Nozzle half-angle [deg]:");
            ui.add(egui::DragValue::new(&mut workspace.geometry.nozzle_half_angle_deg).speed(0.2));
        });
        left.horizontal(|ui| {
            ui.label("Nozzle contour style:");
            egui::ComboBox::from_id_salt("rocket_geometry_nozzle_style")
                .selected_text(workspace.geometry.nozzle_contour_style.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut workspace.geometry.nozzle_contour_style,
                        NozzleContourStyle::Conical,
                        NozzleContourStyle::Conical.label(),
                    );
                    ui.selectable_value(
                        &mut workspace.geometry.nozzle_contour_style,
                        NozzleContourStyle::BellParabolic,
                        NozzleContourStyle::BellParabolic.label(),
                    );
                    ui.selectable_value(
                        &mut workspace.geometry.nozzle_contour_style,
                        NozzleContourStyle::TruncatedIdeal,
                        NozzleContourStyle::TruncatedIdeal.label(),
                    );
                });
        });
        left.horizontal(|ui| {
            ui.label("Nozzle truncation ratio:");
            ui.add(
                egui::DragValue::new(&mut workspace.geometry.nozzle_truncation_ratio)
                    .speed(0.01)
                    .range(0.4..=1.0),
            );
        });

        if left.button("Compute Geometry").clicked() {
            match compute_geometry(&workspace.geometry_problem()) {
                Ok(result) => {
                    workspace.last_geometry_result = Some(result);
                    workspace.status =
                        Some("Geometry estimate computed from active Performance case".to_owned());
                    workspace.last_error = None;
                }
                Err(err) => {
                    workspace.last_error = Some(format!("Geometry solve failed: {err}"));
                    workspace.status = None;
                }
            }
        }

        let right = &mut columns[1];
        right.heading("Key outputs");
        if let Some(result) = &workspace.last_geometry_result {
            show_output_grid(right, result);
        } else {
            right.label("No geometry result yet. Configure inputs and click Compute Geometry.");
        }
    });

    if let Some(status) = &workspace.status {
        ui.colored_label(egui::Color32::GREEN, status);
    }
    if let Some(err) = &workspace.last_error {
        ui.colored_label(egui::Color32::RED, err);
    }

    ui.separator();
    ui.heading("Geometry preview (schematic)");
    let size = egui::vec2(ui.available_width(), 220.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::DARK_GRAY));

    if let Some(result) = &workspace.last_geometry_result {
        draw_preview(&painter, rect.shrink2(egui::vec2(8.0, 8.0)), result);
    } else {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Run geometry sizing to render preview",
            egui::TextStyle::Body.resolve(ui.style()),
            egui::Color32::GRAY,
        );
    }
}

fn show_output_grid(ui: &mut egui::Ui, result: &RocketGeometryResult) {
    egui::Grid::new("rocket_geometry_outputs")
        .num_columns(2)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            ui.label("Throat area [m²]");
            ui.label(format!("{:.6}", result.throat_area_m2));
            ui.end_row();
            ui.label("Throat diameter [m]");
            ui.label(format!("{:.5}", result.throat_diameter_m));
            ui.end_row();
            ui.label("Exit area [m²]");
            ui.label(format!("{:.6}", result.exit_area_m2));
            ui.end_row();
            ui.label("Exit diameter [m]");
            ui.label(format!("{:.5}", result.exit_diameter_m));
            ui.end_row();
            ui.label("Expansion ratio ε");
            ui.label(format!("{:.3}", result.expansion_ratio));
            ui.end_row();
            ui.label("Estimated chamber area [m²]");
            ui.label(format!("{:.6}", result.chamber_area_m2_estimate));
            ui.end_row();
            ui.label("Estimated chamber diameter [m]");
            ui.label(format!("{:.5}", result.chamber_diameter_m_estimate));
            ui.end_row();
            ui.label("Estimated chamber length [m]");
            ui.label(format!("{:.5}", result.chamber_length_m_estimate));
            ui.end_row();
            ui.label("Estimated nozzle length [m]");
            ui.label(format!("{:.5}", result.nozzle_length_m_estimate));
            ui.end_row();
            ui.label("Reference chamber pressure [Pa]");
            ui.label(format!("{:.0}", result.chamber_pressure_pa_reference));
            ui.end_row();
        });

    ui.separator();
    ui.small(format!(
        "Assumptions: {:?}, {:?}, L*={:.3} m, Ac/At={:.2}, half-angle={:.1}°, style={}, trunc={:.2}",
        result.assumptions.combustor_model,
        result.assumptions.nozzle_chemistry_model,
        result.assumptions.characteristic_length_m,
        result.assumptions.chamber_contraction_ratio,
        result.assumptions.nozzle_half_angle_deg,
        result.assumptions.nozzle_contour_style.label(),
        result.assumptions.nozzle_truncation_ratio
    ));
    for note in &result.notes {
        ui.small(format!("• {note}"));
    }
}

fn draw_preview(painter: &egui::Painter, rect: egui::Rect, result: &RocketGeometryResult) {
    let model = &result.canonical_model;
    let chamber_r = 0.5 * model.chamber_diameter_m;
    let throat_r = 0.5 * model.throat_diameter_m;
    let exit_r = 0.5 * model.exit_diameter_m;

    let max_r = chamber_r.max(exit_r).max(throat_r);
    let total_l = model.exit_axial_m;
    if max_r <= 0.0 || total_l <= 0.0 {
        return;
    }

    let sx = f64::from(rect.width()) / total_l;
    let sy = f64::from(rect.height()) * 0.45 / max_r;
    let scale = sx.min(sy);

    let x0 = f64::from(rect.left()) + (f64::from(rect.width()) - total_l * scale) * 0.5;
    let yc = f64::from(rect.center().y);

    let x_throat = x0 + model.throat_axial_m * scale;
    let x_exit = x0 + model.exit_axial_m * scale;

    let y_th_t = yc - throat_r * scale;
    let y_th_b = yc + throat_r * scale;
    let y_ex_t = yc - exit_r * scale;
    let y_ex_b = yc + exit_r * scale;

    let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 160, 250));
    // Draw contour using canonical model points (upper/lower mirrored).
    let mut prev: Option<egui::Pos2> = None;
    for p in &model.wall_contour_upper {
        let x = (x0 + p[0] * scale) as f32;
        let y = (yc - p[1] * scale) as f32;
        let pt = egui::pos2(x, y);
        if let Some(pr) = prev {
            painter.line_segment([pr, pt], stroke);
        }
        prev = Some(pt);
    }
    let mut prev: Option<egui::Pos2> = None;
    for p in &model.wall_contour_upper {
        let x = (x0 + p[0] * scale) as f32;
        let y = (yc + p[1] * scale) as f32;
        let pt = egui::pos2(x, y);
        if let Some(pr) = prev {
            painter.line_segment([pr, pt], stroke);
        }
        prev = Some(pt);
    }

    painter.line_segment(
        [
            egui::pos2(x_throat as f32, y_th_t as f32),
            egui::pos2(x_throat as f32, y_th_b as f32),
        ],
        egui::Stroke::new(1.0, egui::Color32::LIGHT_RED),
    );
    painter.line_segment(
        [
            egui::pos2(x_exit as f32, y_ex_t as f32),
            egui::pos2(x_exit as f32, y_ex_b as f32),
        ],
        egui::Stroke::new(1.0, egui::Color32::LIGHT_GREEN),
    );

    painter.text(
        egui::pos2(x_throat as f32, rect.top() + 4.0),
        egui::Align2::CENTER_TOP,
        "Throat",
        egui::FontId::default(),
        egui::Color32::LIGHT_RED,
    );
    painter.text(
        egui::pos2(x_exit as f32, rect.top() + 4.0),
        egui::Align2::CENTER_TOP,
        "Exit",
        egui::FontId::default(),
        egui::Color32::LIGHT_GREEN,
    );
}
