# UI Standards and Design Guidelines

This document defines the user interface standards and design patterns for the Thermoflow GUI application (`tf-ui`). Following these standards ensures a consistent, intuitive, and professional user experience across all views and features.

## Core Principles

1. **Consistency**: Similar actions should look and behave the same way across all views
2. **Clarity**: UI elements should clearly communicate their purpose
3. **Efficiency**: Users should be able to accomplish tasks with minimal friction
4. **Feedback**: The UI should provide clear feedback for user actions
5. **Accessibility**: Text should be readable, controls should be appropriately sized

## Button Standards

### Icon-Only Buttons

Icon-only buttons should use standardized symbols that are universally recognized:

| Action | Symbol | Usage |
|--------|--------|-------|
| Delete/Remove | **✕** | Remove items from lists, close tabs, delete entries |
| Add/Create | **➕** | Add new items, create entries |
| Edit/Rename | **✏** | Edit or rename existing items |
| Export/Save | **💾** | Export data, save files |
| Refresh/Reload | **🔄** | Refresh data, clear and reload |
| Search | **🔍** | Open search interfaces |
| Settings | **⚙** | Open settings or configuration |
| Charts/Plots | **📈** or **📊** | Chart-related actions |

#### Implementation Notes:
- Use `ui.small_button("✕")` for delete actions in compact layouts
- Add `.on_hover_text("Descriptive text")` to all icon-only buttons for accessibility
- Avoid using emoji that are ambiguous (e.g., 🗑 for delete, use ✕ instead)

**Examples:**
```rust
// Good: Delete button with tooltip
if ui.small_button("✕").on_hover_text("Remove state").clicked() {
    // handle deletion
}

// Bad: Using trash can emoji without clear meaning
if ui.small_button("🗑").clicked() {  // Don't use this
    // handle deletion
}
```

### Text Buttons

Text buttons should use clear, action-oriented language:

- Use verbs: "Add State Point", "Generate Sweep", "Export Data"
- Be specific: "Delete Component" is better than just "Delete"
- Keep text concise but descriptive

**Examples:**
```rust
// Good: Clear action verbs
if ui.button("➕ Add State Point").clicked() { ... }
if ui.button("📊 Generate Sweep").clicked() { ... }
if ui.button("💾 Export Active...").clicked() { ... }

// Acceptable: Text-only for complex actions
if ui.button("Delete Component 'Tank-01'").clicked() { ... }
```

### Button Sizing

- **Regular buttons**: Use `ui.button()` for primary actions
- **Small buttons**: Use `ui.small_button()` for compact layouts and repeated elements (like table rows)
- **Minimum touch target**: Ensure clickable areas are at least 24×24 pixels for usability

## Layout Standards

### Scrolling Regions

All views should handle overflow content gracefully:

```rust
// Wrap entire view content in a scroll area
egui::ScrollArea::vertical()
    .id_salt("unique_view_scroll")
    .show(ui, |ui| {
        // View content here
    });
```

- Use `vertical()` for vertically scrolling content (most common)
- Use `both()` for tables and content that may overflow in both directions
- Always provide a unique `id_salt()` to preserve scroll state

### Resizable Areas

For resizable panels and plots:

1. **Drag-to-resize handles**: Use visual separators that can be dragged
2. **Visual feedback**: Change cursor and color on hover/drag
3. **Sensible constraints**: Clamp sizes to reasonable min/max values

**Example: Draggable resize handle**
```rust
// Add visual and interactive resize handle
let resize_handle_rect = ui.allocate_space(egui::vec2(ui.available_width(), 8.0)).1;
let resize_response = ui.interact(
    resize_handle_rect, 
    ui.id().with("resize_handle"), 
    egui::Sense::drag()
);

// Visual feedback
let handle_color = if resize_response.hovered() || resize_response.dragged() {
    ui.visuals().widgets.active.bg_fill
} else {
    ui.visuals().widgets.inactive.bg_fill
};
ui.painter().rect_filled(resize_handle_rect, 2.0, handle_color);

// Cursor feedback
if resize_response.hovered() {
    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
}

// Handle drag
if resize_response.dragged() {
    let delta = resize_response.drag_delta().y;
    self.plot_height = (self.plot_height + delta).clamp(150.0, 1200.0);
}
```

**Guidelines:**
- Minimum plot/panel height: 150px
- Maximum reasonable height: 1200px
- Resize handles should be at least 8px tall for easy grabbing

### Spacing

Use consistent spacing between UI elements:

- `ui.separator()`: Between major sections
- `ui.add_space(12.0)`: Between section groups
- `ui.add_space(8.0)`: Between related control groups
- `ui.add_space(4.0)`: Between closely related items

## Tab and Panel Standards

### Tab Organization

When organizing content into tabs:

1. **Grouping by property**: Group items with the same X-axis or fundamental property
2. **Active indication**: Use visual indicator (▼) for active tab
3. **Tab labels**: Be descriptive but concise (e.g., "Temperature (X-axis)", "N2 Sweep 1")

**Tab Button Pattern:**
```rust
for (idx, item) in items.iter().enumerate() {
    let is_active = idx == self.active_tab;
    let button_text = if is_active {
        format!("▼ {}", item.label)
    } else {
        format!("  {}", item.label)
    };
    
    if ui.button(button_text).clicked() {
        self.active_tab = idx;
    }
    
    // Close button
    if ui.small_button("✕").clicked() {
        // Handle removal
    }
}
```

### Panel Headers

Use clear, emoji-enhanced headers for major sections:

```rust
ui.label("📈 Sweep Results");
ui.label("⚙ Configuration");
ui.label("📊 Plot Settings");
```

**Emoji Guidelines:**
- Use sparingly and consistently
- Choose universally recognized symbols
- Limit to one emoji per header

## Table Standards

### Column Configuration

Use `egui_extras::TableBuilder` with appropriate column sizing:

```rust
TableBuilder::new(ui)
    .striped(true)                          // Alternate row colors
    .resizable(true)                        // Allow column resizing
    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
    .column(Column::exact(30.0))            // Fixed width for icons/status
    .column(Column::initial(120.0).at_least(80.0))  // Resizable with minimum
    // ...
```

**Column Width Guidelines:**
- Status indicators: 30-40px (exact)
- Short labels/IDs: 60-80px minimum
- Input fields: 100-120px minimum
- Numeric data: 80-110px minimum
- Buttons: 40-60px (exact)

### Table Headers

```rust
.header(22.0, |mut header| {
    header.col(|ui| { ui.strong("Status"); });
    header.col(|ui| { ui.strong("Label"); });
    // ...
});
```

- Use `ui.strong()` for header text
- Keep headers short and clear
- Include units in brackets: "T [K]", "P [Pa]", "ρ [kg/m³]"

## Plot Standards

### Plot Configuration

Basic plot setup:

```rust
egui_plot::Plot::new("unique_plot_id")
    .height(self.plot_height)               // Configurable height
    .legend(egui_plot::Legend::default())   // Show legend
    .show(ui, |plot_ui| {
        for line in lines {
            plot_ui.line(line);
        }
    });
```

### Multi-Trace Plots

When plotting multiple traces:

1. **Same X-axis**: Group on same plot
2. **Different X-axes**: Create separate plot tabs
3. **Different Y-axis scales**: Warn user, consider grouping options

**Warning Pattern:**
```rust
if has_mixed_scales {
    ui.colored_label(
        egui::Color32::from_rgb(255, 200, 100),
        "⚠ Note: Traces use different Y-axis assignments. Consider grouping similar properties."
    );
}
```

### Plot Legends

- Always include legends when showing multiple traces
- Use descriptive labels: Include species, property, and differentiator
- Example: "N2 Density Sweep 1", "H2O Temperature vs Pressure"

## Color Standards

### Status Indicators

Use consistent colors for status indication:

| Status | Color | RGB | Usage |
|--------|-------|-----|-------|
| Success | Green | `(100, 200, 100)` | Successful computation |
| Error | Red | `(255, 100, 100)` | Errors, failures |
| Warning | Orange | `(255, 200, 100)` | Warnings, notes |
| Pending | Gray | `(180, 180, 180)` | Not yet computed |
| Active | Theme | `ui.visuals().widgets.active` | Active selections |

**Example:**
```rust
// Success
ui.colored_label(egui::Color32::from_rgb(100, 200, 100), "✓");

// Error
ui.colored_label(egui::Color32::RED, format!("Error: {}", msg));

// Warning
ui.colored_label(egui::Color32::from_rgb(255, 200, 100), "⚠ Warning message");
```

### Interactive Elements

Use egui's theme colors for interactive elements:

```rust
// Hover state
let color = if hovered {
    ui.visuals().widgets.hovered.bg_fill
} else {
    ui.visuals().widgets.inactive.bg_fill
};

// Active/pressed state
let color = if active {
    ui.visuals().widgets.active.bg_fill
} else {
    ui.visuals().widgets.inactive.bg_fill
};
```

## Input Standards

### Numeric Inputs

For property values with units:

```rust
ui.label("Temperature:");
ui.text_edit_singleline(&mut self.temperature_input);
```

- Accept unit suffixes: "300K", "101325Pa", "1.5bar"
- Provide clear error messages for invalid inputs
- Use `.on_hover_text()` to show expected format

### Dropdowns and Selection

Use consistent patterns for dropdowns:

```rust
egui::ComboBox::from_label("Property")
    .selected_text(selected.display_name())
    .show_ui(ui, |ui| {
        for option in options {
            ui.selectable_value(&mut selected, option, option.display_name());
        }
    });
```

## Dialog Standards

### Export Dialogs

Standard export dialog pattern:

```rust
egui::Window::new("Export Data")
    .collapsible(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.label("Export path:");
        ui.text_edit_singleline(&mut self.export_path);
        
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                self.show_dialog = false;
            }
            if ui.button("Export").clicked() {
                // Perform export
                self.show_dialog = false;
            }
        });
    });
```

**Guidelines:**
- Keep dialogs focused on one task
- Provide Cancel and Confirm options
- Close dialog after action completes
- Show progress for long operations

## Error Handling in UI

### Error Display

Display errors clearly and helpfully:

```rust
if let Some(ref error) = self.last_error {
    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
    ui.add_space(4.0);
}
```

**Error Message Guidelines:**
- Be specific: "Failed to parse temperature value '300X'" not "Invalid input"
- Suggest fixes: "Points must be >= 2" instead of "Invalid points"
- Use red color (`egui::Color32::RED`) for errors
- Clear error after successful operation

### Validation

Validate inputs before processing:

```rust
let num_points: usize = match input.parse() {
    Ok(n) if n >= 2 => n,
    _ => {
        self.error = Some("Points must be >= 2".to_string());
        return;
    }
};
```

## Accessibility Guidelines

### Tooltips

Always provide tooltips for:
- Icon-only buttons
- Abbreviated labels
- Complex controls
- Input format requirements

```rust
ui.button("✕")
    .on_hover_text("Remove this sweep trace");

ui.label("T:")
    .on_hover_text("Temperature in Kelvin (K)");
```

### Text Readability

- Use `ui.strong()` for headers and important text
- Use `ui.label()` for regular text
- Use `ui.monospace()` for numeric data that should align
- Ensure sufficient contrast for all text

## Performance Considerations

### Expensive Operations

- Don't recompute in every frame
- Cache results when possible
- Use `ui.ctx().request_repaint()` for animations
- Move heavy computation to background threads

### UI Responsiveness

```rust
// Bad: Blocking operation in UI thread
if ui.button("Compute").clicked() {
    let result = expensive_computation();  // UI freezes
}

// Good: Trigger async operation
if ui.button("Compute").clicked() {
    self.computation_pending = true;
    // Trigger background computation
}
```

## View-Specific Standards

### Fluid Workspace

- Use table for state point comparison
- Group sweeps by X-axis property
- Provide inline plots with drag-to-resize
- Show statistics for active curve

### Plot Workspace

- Support multiple plot windows
- Allow custom axis labels
- Provide template system for common plots
- Export to standard formats (CSV, PNG)

### System Editor

- Visual node-based editing
- Clear connection lines
- Context-sensitive inspector
- Validation feedback

## Code Organization

### View Structure

All views should follow consistent structure:

```rust
pub struct MyView {
    // State
    // ...
}

impl MyView {
    pub fn new() -> Self { ... }
    
    pub fn show(&mut self, ui: &mut egui::Ui, workspace: &mut Workspace) {
        // Main UI rendering
    }
    
    // Private helper methods
    fn show_section_1(&mut self, ui: &mut egui::Ui) { ... }
    fn show_section_2(&mut self, ui: &mut egui::Ui) { ... }
}
```

### Naming Conventions

- View files: `*_view.rs`
- Helper components: `*_picker.rs`, `*_panel.rs`
- Show methods: `show()`, `show_section()`, `show_dialog()`
- State fields: Use full descriptive names, not abbreviations

## Testing and Validation

Before committing UI changes:

1. **Visual review**: Check layout at different window sizes
2. **Functionality**: Test all buttons and interactions
3. **Error cases**: Verify error messages display correctly
4. **Consistency**: Compare with other views for style consistency
5. **Accessibility**: Ensure tooltips and hover text are present

## Future Considerations

Areas for potential enhancement:

- Keyboard shortcuts for common actions
- Customizable themes (light/dark)
- User-configurable layouts
- Accessibility features (screen reader support, high contrast mode)
- Internationalization (i18n) support

## References

- [egui documentation](https://docs.rs/egui/)
- [egui_plot documentation](https://docs.rs/egui_plot/)
- [Material Design guidelines](https://material.io/design) (for inspiration)
- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/) (for inspiration)

---

**Document Version**: 1.0  
**Last Updated**: March 1, 2026  
**Contributors**: Development Team
