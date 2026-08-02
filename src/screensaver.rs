use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreensaverStyle {
    Default,
    Minimalist,
    Matrix,
}

impl Default for ScreensaverStyle {
    fn default() -> Self {
        ScreensaverStyle::Default
    }
}

impl ScreensaverStyle {
    pub fn all() -> &'static [ScreensaverStyle] {
        &[
            ScreensaverStyle::Default,
            ScreensaverStyle::Minimalist,
            ScreensaverStyle::Matrix,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ScreensaverStyle::Default => "Default (Ambient Slate)",
            ScreensaverStyle::Minimalist => "Minimalist (Monochrome Dark)",
            ScreensaverStyle::Matrix => "Matrix (Digital Green Rain)",
        }
    }
}

/// Trait for rendering screensaver visual components.
/// Input handling and unblock password UI are managed by the parent container shell.
pub trait ScreensaverComponent {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64);
}

// 1. Default Screensaver Component
pub struct DefaultScreensaver;

impl ScreensaverComponent for DefaultScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        ui.vertical_centered(|ui| {
            ui.heading(
                egui::RichText::new("🌿 TIME TO TAKE A BREAK")
                    .size(42.0)
                    .color(egui::Color32::from_rgb(56, 189, 248))
                    .strong(),
            );

            ui.add_space(12.0);
            ui.label(
                egui::RichText::new("Step away, stretch, drink water, and rest your eyes.")
                    .size(20.0)
                    .color(egui::Color32::from_rgb(203, 213, 225)),
            );

            ui.add_space(32.0);

            ui.label(
                egui::RichText::new(format!(
                    "{:02}:{:02}",
                    remaining_sec / 60,
                    remaining_sec % 60
                ))
                .size(86.0)
                .color(egui::Color32::WHITE)
                .monospace()
                .strong(),
            );
        });
    }
}

// 2. Minimalist Screensaver Component
pub struct MinimalistScreensaver;

impl ScreensaverComponent for MinimalistScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        ui.vertical_centered(|ui| {
            ui.heading(
                egui::RichText::new("PAUSE")
                    .size(36.0)
                    .color(egui::Color32::from_rgb(160, 160, 160))
                    .strong(),
            );

            ui.add_space(20.0);

            ui.label(
                egui::RichText::new(format!(
                    "{:02}:{:02}",
                    remaining_sec / 60,
                    remaining_sec % 60
                ))
                .size(96.0)
                .color(egui::Color32::WHITE)
                .monospace(),
            );

            ui.add_space(12.0);
            ui.label(
                egui::RichText::new("Resting computer screen...")
                    .size(16.0)
                    .color(egui::Color32::GRAY),
            );
        });
    }
}

// 3. Matrix Digital Rain Screensaver Component
pub struct MatrixScreensaver;

impl ScreensaverComponent for MatrixScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        let available_rect = ui.available_rect_before_wrap();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        let chars = [
            '0', '1', 'X', 'Y', 'Z', 'A', '7', '9', 'B', 'K', 'M', '#', '%', '*', '&', '$', '@',
            '!',
        ];
        let col_width = 22.0;
        let char_height = 18.0;
        let num_cols = (available_rect.width() / col_width) as usize;

        // Draw animated digital rain streams
        for c in 0..num_cols {
            let x = available_rect.min.x + c as f32 * col_width;

            let seed = (c * 37 + 17) as f64;
            let speed = 110.0 + (seed % 120.0) as f32;
            let length = 10 + (seed as usize % 12);

            let y_head = ((time as f32 * speed + (seed * 19.0) as f32)
                % (available_rect.height() + length as f32 * char_height))
                - length as f32 * char_height;

            for i in 0..length {
                let y = y_head - i as f32 * char_height;
                if y >= available_rect.min.y && y <= available_rect.max.y {
                    let char_idx = (seed as usize + i + (y_head / 24.0) as usize) % chars.len();
                    let ch = chars[char_idx];

                    let color = if i == 0 {
                        egui::Color32::from_rgb(220, 255, 220) // Glowing white-green head
                    } else {
                        let alpha = (255.0 * (1.0 - (i as f32 / length as f32))) as u8;
                        egui::Color32::from_rgba_unmultiplied(34, 197, 94, alpha) // Fading tail
                    };

                    painter.text(
                        egui::pos2(x, y),
                        egui::Align2::LEFT_TOP,
                        ch.to_string(),
                        egui::FontId::monospace(15.0),
                        color,
                    );
                }
            }
        }

        // Render Matrix Central Console Card over digital rain
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(5, 20, 5, 220))
                .rounding(16.0)
                .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(34, 197, 94)))
                .inner_margin(egui::Margin::symmetric(36.0, 24.0))
                .show(ui, |ui| {
                    ui.heading(
                        egui::RichText::new("SYSTEM PAUSED // SCREEN BREAK")
                            .size(36.0)
                            .color(egui::Color32::from_rgb(34, 197, 94))
                            .monospace()
                            .strong(),
                    );

                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new(format!(
                            "[{:02}:{:02}]",
                            remaining_sec / 60,
                            remaining_sec % 60
                        ))
                        .size(88.0)
                        .color(egui::Color32::from_rgb(74, 222, 128))
                        .monospace()
                        .strong(),
                    );

                    ui.add_space(16.0);
                    ui.label(
                        egui::RichText::new("> Stand up and stretch before returning to console.")
                            .size(18.0)
                            .color(egui::Color32::from_rgb(134, 239, 172))
                            .monospace(),
                    );
                });
        });
    }
}

/// Renders the visual content of the selected screensaver style.
pub fn render_screensaver_style(
    style: ScreensaverStyle,
    ui: &mut egui::Ui,
    remaining_sec: u64,
) {
    match style {
        ScreensaverStyle::Default => DefaultScreensaver.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Minimalist => MinimalistScreensaver.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Matrix => MatrixScreensaver.render_visuals(ui, remaining_sec),
    }
}

pub fn get_background_color(style: ScreensaverStyle) -> egui::Color32 {
    match style {
        ScreensaverStyle::Default => egui::Color32::from_rgb(15, 23, 42),
        ScreensaverStyle::Minimalist => egui::Color32::from_rgb(8, 8, 8),
        ScreensaverStyle::Matrix => egui::Color32::from_rgb(4, 12, 4),
    }
}
