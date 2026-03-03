use egui_plot::{Legend, Line, Plot, PlotPoints};

pub struct PlotSeriesSpec<'a> {
    pub name: &'a str,
    pub points: Vec<[f64; 2]>,
}

pub fn show_multi_series_plot(
    ui: &mut egui::Ui,
    id: &str,
    x_label: &str,
    y_label: &str,
    height: f32,
    series: Vec<PlotSeriesSpec<'_>>,
) {
    Plot::new(id)
        .legend(Legend::default())
        .height(height)
        .view_aspect(2.4)
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
