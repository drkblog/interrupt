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
            ScreensaverStyle::Default => "Default (Ambient Aurora)",
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

// 1. Default Screensaver Component (Breathing Aurora & Floating Orbs)
pub struct DefaultScreensaver;

impl ScreensaverComponent for DefaultScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        let available_rect = ui.available_rect_before_wrap();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        // 1. Draw floating ambient particles/orbs drifting upwards softly
        let num_particles = 28;
        for i in 0..num_particles {
            let seed = (i * 31 + 7) as f64;
            let speed = 22.0 + (seed % 35.0) as f32;
            let radius = 6.0 + (seed % 14.0) as f32;
            let x_base = available_rect.min.x + ((seed * 73.0) as f32 % available_rect.width());
            let x_sway = (time * 0.6 + seed).sin() as f32 * 22.0;
            let x = x_base + x_sway;

            let loop_height = available_rect.height() + 80.0;
            let y = available_rect.max.y
                - ((time as f32 * speed + (seed * 47.0) as f32) % loop_height);

            let pulse_alpha = ((time * 1.5 + seed).sin() * 0.35 + 0.55) as f32;
            let color = match i % 3 {
                0 => egui::Color32::from_rgba_unmultiplied(
                    56,
                    189,
                    248,
                    (120.0 * pulse_alpha) as u8,
                ), // Sky Blue
                1 => egui::Color32::from_rgba_unmultiplied(
                    168,
                    85,
                    247,
                    (100.0 * pulse_alpha) as u8,
                ), // Purple
                _ => egui::Color32::from_rgba_unmultiplied(
                    52,
                    211,
                    153,
                    (110.0 * pulse_alpha) as u8,
                ), // Emerald
            };

            painter.circle_filled(egui::pos2(x, y), radius, color);
        }

        // 2. Guided Breathing Cycle State (3s Inhale -> 2s Hold -> 6s Exhale)
        let cycle_period = 11.0;
        let cycle_time = (time % cycle_period) as f32;
        let (breath_label, breath_color) = if cycle_time < 3.0 {
            ("🫁 Inhale deeply...", egui::Color32::from_rgb(56, 189, 248))
        } else if cycle_time < 5.0 {
            ("⏸️ Hold...", egui::Color32::from_rgb(168, 85, 247))
        } else {
            ("😮‍💨 Exhale slowly...", egui::Color32::from_rgb(52, 211, 153))
        };

        // 3. Central Relaxation Glass Card
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(15, 23, 42, 225))
                .rounding(20.0)
                .stroke(egui::Stroke::new(
                    1.5,
                    egui::Color32::from_rgba_unmultiplied(56, 189, 248, 140),
                ))
                .inner_margin(egui::Margin::symmetric(40.0, 28.0))
                .show(ui, |ui| {
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

                    ui.add_space(28.0);

                    // Large Glowing Countdown Timer
                    ui.label(
                        egui::RichText::new(format!(
                            "{:02}:{:02}",
                            remaining_sec / 60,
                            remaining_sec % 60
                        ))
                        .size(88.0)
                        .color(egui::Color32::WHITE)
                        .monospace()
                        .strong(),
                    );

                    ui.add_space(16.0);

                    // Guided Breathing Indicator Text
                    ui.label(
                        egui::RichText::new(breath_label)
                            .size(22.0)
                            .color(breath_color)
                            .strong(),
                    );
                });
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

// 3. Matrix Digital Rain Screensaver Component (Enhanced Visuals)
pub struct MatrixScreensaver;

impl ScreensaverComponent for MatrixScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        let available_rect = ui.available_rect_before_wrap();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        // Matrix Half-Width Katakana + ASCII Character Set
        let chars = [
            'ｦ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ',
            'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ',
            'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ',
            'ﾚ', 'ﾛ', 'ﾜ', 'ﾝ', '0', '1', '2', '3', '4', '5', '7', '8', '9', 'X', 'Z', 'K', 'M',
        ];

        let col_width = 20.0;
        let num_cols = (available_rect.width() / col_width) as usize;

        // Render Multi-Layered Matrix Digital Rain
        for c in 0..num_cols {
            let x = available_rect.min.x + c as f32 * col_width;
            let seed = (c * 43 + 19) as f64;

            // Determine if column is foreground (bright/fast) or background (dim/slow)
            let is_foreground = (c % 2) == 0;
            let font_size = if is_foreground { 17.0 } else { 13.0 };
            let char_height = if is_foreground { 19.0 } else { 15.0 };
            let speed = if is_foreground {
                130.0 + (seed % 100.0) as f32
            } else {
                70.0 + (seed % 60.0) as f32
            };
            let length = if is_foreground {
                12 + (seed as usize % 14)
            } else {
                7 + (seed as usize % 8)
            };

            let total_stream_height = length as f32 * char_height;
            let loop_height = available_rect.height() + total_stream_height;
            let y_head =
                ((time as f32 * speed + (seed * 23.0) as f32) % loop_height) - total_stream_height;

            for i in 0..length {
                let y = y_head - i as f32 * char_height;
                if y >= available_rect.min.y - char_height && y <= available_rect.max.y {
                    // Dynamically flicker characters over time
                    let char_idx = (seed as usize + i * 11 + (time * 9.0) as usize) % chars.len();
                    let ch = chars[char_idx];

                    let color = if i == 0 {
                        // Glowing pure white-green head character
                        egui::Color32::from_rgb(240, 255, 240)
                    } else if i == 1 {
                        // Bright neon lead trail
                        egui::Color32::from_rgb(134, 239, 172)
                    } else {
                        // Fading matrix green body
                        let fade = 1.0 - (i as f32 / length as f32);
                        let alpha = (if is_foreground { 230.0 } else { 140.0 } * fade) as u8;
                        let green = (220.0 * fade) as u8;
                        egui::Color32::from_rgba_unmultiplied(10, green.max(40), 50, alpha)
                    };

                    painter.text(
                        egui::pos2(x, y),
                        egui::Align2::LEFT_TOP,
                        ch.to_string(),
                        egui::FontId::monospace(font_size),
                        color,
                    );
                }
            }
        }

        // Render Matrix Central Console Card over digital rain
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(2, 12, 4, 235))
                .rounding(16.0)
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(34, 197, 94)))
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

                    // Pulse glow effect on timer clock
                    let pulse = (time * 2.5).sin().abs() as f32;
                    let clock_color = egui::Color32::from_rgb(
                        (74.0 + pulse * 40.0) as u8,
                        (222.0 + pulse * 33.0) as u8,
                        (128.0 + pulse * 60.0) as u8,
                    );

                    ui.label(
                        egui::RichText::new(format!(
                            "[{:02}:{:02}]",
                            remaining_sec / 60,
                            remaining_sec % 60
                        ))
                        .size(88.0)
                        .color(clock_color)
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
        ScreensaverStyle::Matrix => egui::Color32::from_rgb(2, 8, 2),
    }
}
