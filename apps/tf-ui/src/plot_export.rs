//! Plot export functionality for various formats (CSV, PNG, JPG, PDF).
//!
//! This module handles exporting plots to multiple formats with proper rendering of curve data.

use image::{RgbImage, RgbaImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Export data to CSV format
pub fn export_to_csv(
    path: &Path,
    x_values: &[f64],
    y_values: &[f64],
    x_label: &str,
    y_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "{},{}", x_label, y_label)?;

    // Write data rows
    for (x, y) in x_values.iter().zip(y_values.iter()) {
        writeln!(file, "{},{}", x, y)?;
    }

    Ok(())
}

/// Export a color image to PNG file
pub fn export_plot_to_png(
    path: &Path,
    image: &egui::ColorImage,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert egui ColorImage to RGBA
    let pixels: Vec<[u8; 4]> = image
        .pixels
        .iter()
        .map(|color| [color.r(), color.g(), color.b(), color.a()])
        .collect();

    let rgba_image =
        RgbaImage::from_raw(image.size[0] as u32, image.size[1] as u32, pixels.concat())
            .ok_or("Failed to create image")?;

    rgba_image.save(path)?;
    Ok(())
}

/// Export a color image to JPG file
pub fn export_plot_to_jpg(
    path: &Path,
    image: &egui::ColorImage,
    _quality: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert to RGB (JPG doesn't support alpha)
    let pixels: Vec<[u8; 3]> = image
        .pixels
        .iter()
        .map(|color| [color.r(), color.g(), color.b()])
        .collect();

    let rgb_image = RgbImage::from_raw(image.size[0] as u32, image.size[1] as u32, pixels.concat())
        .ok_or("Failed to create image")?;

    image::DynamicImage::ImageRgb8(rgb_image).save_with_format(path, image::ImageFormat::Jpeg)?;
    Ok(())
}

/// Export a color image to PDF file
pub fn export_plot_to_pdf(
    path: &Path,
    _image: &egui::ColorImage,
    title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use printpdf::*;

    // Create PDF with standard dimensions
    let (document, page1, layer1) =
        PdfDocument::new("Plot Export", Mm(210.0), Mm(297.0), "Layer 1");
    let font = document.add_builtin_font(BuiltinFont::Helvetica)?;

    let current_layer = document.get_page(page1).get_layer(layer1);

    // Add title
    current_layer.use_text(title, 24.0, Mm(10.0), Mm(280.0), &font);
    current_layer.use_text("Data visualization plot", 12.0, Mm(10.0), Mm(270.0), &font);

    // Note: printpdf doesn't support embedding images easily without external dependencies
    // For now, we create a valid PDF with the title
    // A full implementation would embed the plot image

    document.save(&mut std::io::BufWriter::new(File::create(path)?))?;
    Ok(())
}

/// Export context for managing export state
#[derive(Default)]
pub struct ExportContext {
    pub show_export_dialog: bool,
    pub export_format: ExportFormat,
    /// Plot name without extension (user input)
    pub plot_name: String,
    /// Directory to export to
    pub export_directory: String,
    pub export_status: Option<String>,
    /// Rectangle to capture for image export (in screen coordinates)
    #[allow(dead_code)]
    pub capture_rect: Option<egui::Rect>,
    /// Captured color image ready for export
    #[allow(dead_code)]
    pub captured_image: Option<egui::ColorImage>,
}

impl ExportContext {
    /// Get the full file path with appropriate extension
    pub fn get_export_path(&self) -> String {
        let ext = match self.export_format {
            ExportFormat::Png => "png",
            ExportFormat::Jpg => "jpg",
            ExportFormat::Pdf => "pdf",
            ExportFormat::Csv => "csv",
        };

        let dir = if self.export_directory.is_empty() {
            "exports".to_string()
        } else {
            self.export_directory.clone()
        };

        std::path::PathBuf::from(&dir)
            .join(format!("{}.{}", self.plot_name, ext))
            .to_string_lossy()
            .to_string()
    }

    /// Request screenshot capture of a specific region
    #[allow(dead_code)]
    pub fn request_capture(&mut self, rect: egui::Rect) {
        self.capture_rect = Some(rect);
        self.captured_image = None;
    }

    /// Process screenshot if available and export to file
    #[allow(dead_code)]
    pub fn process_screenshot(
        &mut self,
        ctx: &egui::Context,
        plot_title: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Request screenshot if we need it
        if let Some(rect) = self.capture_rect {
            if self.captured_image.is_none() {
                // Convert egui rect to pixel coordinates
                let pixels_per_point = ctx.pixels_per_point();
                let pixel_rect = egui::Rect::from_min_max(
                    (rect.min.to_vec2() * pixels_per_point).to_pos2(),
                    (rect.max.to_vec2() * pixels_per_point).to_pos2(),
                );

                // Request screenshot from context
                ctx.request_repaint(); // Ensure frame is rendered

                // Try to capture the region (note: this is a simplified approach)
                // In practice, we'll capture on the next frame when the image is available
                if let Some(image) = try_capture_region(ctx, pixel_rect) {
                    self.captured_image = Some(image);
                }
            }

            // If we have the captured image, export it
            if let Some(ref image) = self.captured_image {
                let path_str = self.get_export_path();
                let path = std::path::Path::new(&path_str);
                match self.export_format {
                    ExportFormat::Png => export_plot_to_png(path, image)?,
                    ExportFormat::Jpg => export_plot_to_jpg(path, image, 95)?,
                    ExportFormat::Pdf => export_plot_to_pdf(path, image, plot_title)?,
                    ExportFormat::Csv => {
                        return Err("Cannot export image capture as CSV".into());
                    }
                }
                self.captured_image = None;
                self.capture_rect = None;
                self.export_status = Some("Export successful!".to_string());
                return Ok(());
            }
        }
        Ok(())
    }
}

/// Try to capture a region from the screen (simplified placeholder)
/// In a real implementation, this would use platform-specific screenshot APIs
/// or egui's texture system to capture rendered content
#[allow(dead_code)]
fn try_capture_region(_ctx: &egui::Context, _rect: egui::Rect) -> Option<egui::ColorImage> {
    // Note: egui doesn't provide direct screenshot API for specific regions
    // This is a placeholder that would need platform-specific implementation
    // or rendering to a texture first, then capturing that texture

    // For now, return None to indicate we need a better implementation
    // The actual implementation would require:
    // 1. Render plot to an egui::TextureHandle
    // 2. Read back the texture data
    // 3. Convert to ColorImage

    None
}

/// Render plot directly to an image buffer (alternative to screenshot capture)
/// This creates an offscreen egui context and renders the plot to it
#[allow(dead_code)]
pub fn render_plot_to_image(
    width: u32,
    height: u32,
    _plot_builder: impl FnOnce(&mut egui::Ui),
) -> Result<egui::ColorImage, Box<dyn std::error::Error>> {
    // Create a simple pixel buffer filled with background color
    let pixels = vec![egui::Color32::from_rgb(32, 32, 32); (width * height) as usize];

    // Note: This is a simplified implementation. A full implementation would:
    // 1. Create an offscreen rendering context
    // 2. Call plot_builder with a proper UI context
    // 3. Capture the rendered output

    // For now, return a placeholder image
    // Real implementation would need egui rendering backend support

    Ok(egui::ColorImage {
        size: [width as usize, height as usize],
        pixels,
    })
}

/// Simplified plot export that works with current data
/// Instead of trying to capture rendered plots, this regenerates the plot with data
pub fn export_plot_with_curves(
    path: &Path,
    format: ExportFormat,
    curves: &[(String, Vec<[f64; 2]>)], // (label, points)
    title: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        ExportFormat::Csv => {
            // For CSV, export all curves together
            let mut file = File::create(path)?;
            writeln!(file, "Curve,X,Y")?;
            for (label, points) in curves {
                for point in points {
                    writeln!(file, "{},{},{}", label, point[0], point[1])?;
                }
            }
            Ok(())
        }
        ExportFormat::Png | ExportFormat::Jpg | ExportFormat::Pdf => {
            // For image formats, create a rendered plot with actual curve data
            let plot_image = create_plot_with_curves(width, height, title, curves)?;

            match format {
                ExportFormat::Png => export_plot_to_png(path, &plot_image),
                ExportFormat::Jpg => export_plot_to_jpg(path, &plot_image, 95),
                ExportFormat::Pdf => export_plot_to_pdf(path, &plot_image, title),
                _ => unreachable!(),
            }
        }
    }
}

/// Create a plot image with curve data rendered
fn create_plot_with_curves(
    width: u32,
    height: u32,
    _title: &str,
    curves: &[(String, Vec<[f64; 2]>)],
) -> Result<egui::ColorImage, Box<dyn std::error::Error>> {
    let width = width as usize;
    let height = height as usize;
    // Enhanced background color
    let mut pixels = vec![egui::Color32::from_rgb(35, 35, 38); width * height];

    if curves.is_empty() {
        return Ok(egui::ColorImage {
            size: [width, height],
            pixels,
        });
    }

    // Find data bounds
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    for (_label, points) in curves {
        for point in points {
            x_min = x_min.min(point[0]);
            x_max = x_max.max(point[0]);
            y_min = y_min.min(point[1]);
            y_max = y_max.max(point[1]);
        }
    }

    if !x_min.is_finite() || !x_max.is_finite() || !y_min.is_finite() || !y_max.is_finite() {
        return Ok(egui::ColorImage {
            size: [width, height],
            pixels,
        });
    }

    // Add 10% margins
    let x_range = (x_max - x_min).max(f64::MIN_POSITIVE);
    let y_range = (y_max - y_min).max(f64::MIN_POSITIVE);
    x_min -= x_range * 0.1;
    x_max += x_range * 0.1;
    y_min -= y_range * 0.1;
    y_max += y_range * 0.1;

    // Enhanced margins for better plot area
    let left_margin = 70;
    let bottom_margin = 50;
    let right_margin = 30;
    let top_margin = 60;

    let plot_width = (width as i32 - left_margin - right_margin).max(1) as f64;
    let plot_height = (height as i32 - top_margin - bottom_margin).max(1) as f64;

    // Enhanced color palette for curves
    let colors = [
        egui::Color32::from_rgb(100, 180, 255), // Bright Blue
        egui::Color32::from_rgb(255, 140, 80),  // Bright Orange
        egui::Color32::from_rgb(100, 255, 140), // Bright Green
        egui::Color32::from_rgb(255, 100, 140), // Bright Pink
        egui::Color32::from_rgb(220, 100, 255), // Bright Purple
    ];

    // Enhanced gridlines - 10x10 grid for better readability
    let grid_color = egui::Color32::from_rgb(70, 70, 75);
    for i in 0..=10 {
        let norm = i as f64 / 10.0;
        // Vertical gridlines
        let x_px = (left_margin as f64 + plot_width * norm) as i32;
        if x_px >= left_margin && x_px < (width as i32 - right_margin) {
            for y in top_margin..(height as i32 - bottom_margin) {
                let idx = (y as usize * width + x_px as usize).min(pixels.len() - 1);
                pixels[idx] = grid_color;
            }
        }
        // Horizontal gridlines
        let y_px = (top_margin as f64 + plot_height * (1.0 - norm)) as i32;
        if y_px >= top_margin && y_px < (height as i32 - bottom_margin) {
            for x in left_margin..(width as i32 - right_margin) {
                let idx = (y_px as usize * width + x as usize).min(pixels.len() - 1);
                pixels[idx] = grid_color;
            }
        }
    }

    // Draw curves with thicker lines for better visibility
    for (curve_idx, (_label, points)) in curves.iter().enumerate() {
        let color = colors[curve_idx % colors.len()];

        for i in 0..points.len().saturating_sub(1) {
            let p1 = points[i];
            let p2 = points[i + 1];

            // Convert to pixel coordinates
            let x1 = left_margin as f64 + ((p1[0] - x_min) / (x_max - x_min)) * plot_width;
            let y1 = top_margin as f64 + plot_height * (1.0 - (p1[1] - y_min) / (y_max - y_min));

            let x2 = left_margin as f64 + ((p2[0] - x_min) / (x_max - x_min)) * plot_width;
            let y2 = top_margin as f64 + plot_height * (1.0 - (p2[1] - y_min) / (y_max - y_min));

            // Draw thicker lines (2 pixels wide)
            draw_line_thick(&mut pixels, width, height, x1, y1, x2, y2, color, 2);
        }
    }

    // Draw enhanced axes
    let axis_color = egui::Color32::from_rgb(220, 220, 220);
    // Y axis (thicker)
    for y in top_margin..(height as i32 - bottom_margin) {
        for thickness in 0..2 {
            let x = left_margin + thickness;
            let idx = (y as usize * width + x as usize).min(pixels.len() - 1);
            pixels[idx] = axis_color;
        }
    }
    // X axis (thicker)
    for x in left_margin..(width as i32 - right_margin) {
        for thickness in 0..2 {
            let y = height as i32 - bottom_margin + thickness;
            let idx = (y as usize * width + x as usize).min(pixels.len() - 1);
            pixels[idx] = axis_color;
        }
    }

    Ok(egui::ColorImage {
        size: [width, height],
        pixels,
    })
}

/// Draw a thicker line between two points with specified thickness
#[allow(clippy::too_many_arguments)]
fn draw_line_thick(
    pixels: &mut [egui::Color32],
    width: usize,
    height: usize,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: egui::Color32,
    thickness: i32,
) {
    let x1 = x1.clamp(0.0, width as f64 - 1.0);
    let y1 = y1.clamp(0.0, height as f64 - 1.0);
    let x2 = x2.clamp(0.0, width as f64 - 1.0);
    let y2 = y2.clamp(0.0, height as f64 - 1.0);

    let dx = x2 - x1;
    let dy = y2 - y1;
    let steps = (dx.abs().max(dy.abs()) as i32).max(1);

    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let x = x1 + dx * t;
        let y = y1 + dy * t;

        // Draw a circle of pixels around the line point
        for dx_offset in -(thickness / 2)..=(thickness / 2) {
            for dy_offset in -(thickness / 2)..=(thickness / 2) {
                let px = (x + dx_offset as f64) as usize;
                let py = (y + dy_offset as f64) as usize;

                if px < width && py < height {
                    let idx = py * width + px;
                    if idx < pixels.len() {
                        pixels[idx] = color;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    #[default]
    Csv,
    Png,
    Jpg,
    Pdf,
}

impl ExportFormat {
    #[allow(dead_code)]
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Png => "png",
            Self::Jpg => "jpg",
            Self::Pdf => "pdf",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Csv => "CSV (Data)",
            Self::Png => "PNG (Image)",
            Self::Jpg => "JPG (Image)",
            Self::Pdf => "PDF (Document)",
        }
    }
}
