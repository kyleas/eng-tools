use std::hash::Hash;

use egui::Ui;
use egui_dnd::{DragDropConfig, Handle, dnd};

pub fn show_reorderable_ids(
    ui: &mut Ui,
    id_source: impl Hash,
    row_ids: &mut [String],
    mut render_row: impl FnMut(&mut Ui, &str, Handle),
) {
    dnd(ui, id_source)
        .with_mouse_config(DragDropConfig::mouse())
        .show_vec(row_ids, |ui, row_id, handle, _| {
            render_row(ui, row_id.as_str(), handle);
        });
}
