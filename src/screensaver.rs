use crate::i18n::{tr, Language};
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreensaverStyle {
    Default,
    Minimalist,
    Matrix,
    Math,
    Geography,
    Vocab,
    Science,
    Pronunciation,
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
            ScreensaverStyle::Math,
            ScreensaverStyle::Geography,
            ScreensaverStyle::Vocab,
            ScreensaverStyle::Science,
            ScreensaverStyle::Pronunciation,
        ]
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        self.name_localized(Language::English)
    }

    pub fn name_localized(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::Spanish, ScreensaverStyle::Default) => "Predeterminado (Aurora Ambiental)",
            (Language::Spanish, ScreensaverStyle::Minimalist) => "Minimalista (Monocromático Oscuro)",
            (Language::Spanish, ScreensaverStyle::Matrix) => "Matrix (Lluvia Verde Digital)",
            (Language::Spanish, ScreensaverStyle::Math) => "Ejercicios Matemáticos (Aritmética)",
            (Language::Spanish, ScreensaverStyle::Geography) => "Geografía y Banderas (Trivia Mundial)",
            (Language::Spanish, ScreensaverStyle::Vocab) => "Vocabulario y Ortografía (Quiz de Palabras)",
            (Language::Spanish, ScreensaverStyle::Science) => "Ciencia y Naturaleza (Trivia STEM)",
            (Language::Spanish, ScreensaverStyle::Pronunciation) => "Pronunciación en Inglés (Audición de Palabras)",
            (_, ScreensaverStyle::Default) => "Default (Ambient Aurora)",
            (_, ScreensaverStyle::Minimalist) => "Minimalist (Monochrome Dark)",
            (_, ScreensaverStyle::Matrix) => "Matrix (Digital Green Rain)",
            (_, ScreensaverStyle::Math) => "Math Exercises (Elementary Arithmetic)",
            (_, ScreensaverStyle::Geography) => "Geography & Flags (World Trivia)",
            (_, ScreensaverStyle::Vocab) => "Vocab & Spelling (Word Quiz)",
            (_, ScreensaverStyle::Science) => "Science & Nature (STEM Trivia)",
            (_, ScreensaverStyle::Pronunciation) => "English Pronunciation (Listening Quiz)",
        }
    }
}

/// Trait for rendering screensaver visual components.
/// Input handling and unblock password UI are managed by the parent container shell.
pub trait ScreensaverComponent {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64);
}

// 1. Default Screensaver Component (Breathing Aurora & Floating Orbs)
pub struct DefaultScreensaver {
    pub lang: Language,
}

impl Default for DefaultScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for DefaultScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        // 1. Draw floating ambient particles/orbs drifting across full screen height
        let num_particles = 28;
        for i in 0..num_particles {
            let seed = (i * 31 + 7) as f64;
            let speed = 22.0 + (seed % 35.0) as f32;
            let radius = 6.0 + (seed % 14.0) as f32;
            let x_base = screen_rect.min.x + ((seed * 73.0) as f32 % screen_rect.width());
            let x_sway = (time * 0.6 + seed).sin() as f32 * 22.0;
            let x = x_base + x_sway;

            let loop_height = screen_rect.height() + 80.0;
            let y = screen_rect.max.y
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
            (tr(self.lang, "aurora_inhale"), egui::Color32::from_rgb(56, 189, 248))
        } else if cycle_time < 5.0 {
            (tr(self.lang, "aurora_hold"), egui::Color32::from_rgb(168, 85, 247))
        } else {
            (tr(self.lang, "aurora_exhale"), egui::Color32::from_rgb(52, 211, 153))
        };

        // 3. Central Relaxation Glass Card
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            let max_card_width = (screen_rect.width() - 40.0).clamp(280.0, 620.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(15, 23, 42, 225))
                .rounding(20.0)
                .stroke(egui::Stroke::new(
                    1.5,
                    egui::Color32::from_rgba_unmultiplied(56, 189, 248, 140),
                ))
                .inner_margin(egui::Margin::symmetric(32.0, 24.0))
                .show(ui, |ui| {
                    ui.set_max_width(max_card_width);
                    ui.heading(
                        egui::RichText::new(tr(self.lang, "aurora_heading"))
                            .size(36.0)
                            .color(egui::Color32::from_rgb(56, 189, 248))
                            .strong(),
                    );

                    ui.add_space(12.0);
                    ui.label(
                        egui::RichText::new(tr(self.lang, "aurora_subtext"))
                            .size(18.0)
                            .color(egui::Color32::from_rgb(203, 213, 225)),
                    );

                    ui.add_space(24.0);

                    // Large Glowing Countdown Timer
                    ui.label(
                        egui::RichText::new(format!(
                            "{:02}:{:02}",
                            remaining_sec / 60,
                            remaining_sec % 60
                        ))
                        .size(80.0)
                        .color(egui::Color32::WHITE)
                        .monospace()
                        .strong(),
                    );

                    ui.add_space(16.0);

                    // Guided Breathing Indicator Text
                    ui.label(
                        egui::RichText::new(breath_label)
                            .size(20.0)
                            .color(breath_color)
                            .strong(),
                    );
                });
        });
    }
}

// 2. Minimalist Screensaver Component
pub struct MinimalistScreensaver {
    pub lang: Language,
}

impl Default for MinimalistScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for MinimalistScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        ui.vertical_centered(|ui| {
            ui.heading(
                egui::RichText::new(tr(self.lang, "minimalist_heading"))
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
                egui::RichText::new(tr(self.lang, "minimalist_subtext"))
                    .size(16.0)
                    .color(egui::Color32::GRAY),
            );
        });
    }
}

// 3. Matrix Digital Rain Screensaver Component (Full Screen Bounds)
pub struct MatrixScreensaver {
    pub lang: Language,
}

impl Default for MatrixScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for MatrixScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        // Matrix ASCII Digital Character Set (Guaranteed 100% font glyph compatibility)
        let chars = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
            'Y', 'Z', '#', '$', '%', '&', '*', '+', '-', '/', '=', '?', '@', '^', '~', '<', '>',
            '|', ':', ';',
        ];

        let col_width = 20.0;
        let num_cols = (screen_rect.width() / col_width) as usize;

        // Render Multi-Layered Matrix Digital Rain across full screen height (y = 0 to screen height)
        for c in 0..num_cols {
            let x = screen_rect.min.x + c as f32 * col_width;
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
            let loop_height = screen_rect.height() + total_stream_height;
            let y_head =
                ((time as f32 * speed + (seed * 23.0) as f32) % loop_height) - total_stream_height;

            for i in 0..length {
                let y = y_head - i as f32 * char_height;
                if y >= screen_rect.min.y - char_height && y <= screen_rect.max.y {
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

        // Render Matrix Central Console Card
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            let max_card_width = (screen_rect.width() - 40.0).clamp(280.0, 620.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(2, 12, 4, 235))
                .rounding(16.0)
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(34, 197, 94)))
                .inner_margin(egui::Margin::symmetric(32.0, 24.0))
                .show(ui, |ui| {
                    ui.set_max_width(max_card_width);
                    ui.heading(
                        egui::RichText::new(tr(self.lang, "matrix_heading"))
                            .size(30.0)
                            .color(egui::Color32::from_rgb(34, 197, 94))
                            .monospace()
                            .strong(),
                    );

                    ui.add_space(16.0);

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
                        .size(80.0)
                        .color(clock_color)
                        .monospace()
                        .strong(),
                    );

                    ui.add_space(14.0);
                    ui.label(
                        egui::RichText::new(tr(self.lang, "matrix_subtext"))
                            .size(16.0)
                            .color(egui::Color32::from_rgb(134, 239, 172))
                            .monospace(),
                    );
                });
        });
    }
}

pub struct MathScreensaver;

impl ScreensaverComponent for MathScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, _remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        let symbols = ["+", "-", "×", "÷", "=", "?", "√", "%"];
        for i in 0..15 {
            let seed = (i * 37 + 13) as f64;
            let speed = 15.0 + (seed % 20.0) as f32;
            let size = 18.0 + (seed % 16.0) as f32;
            let x = screen_rect.min.x + (((seed * 83.0) as f32 + time as f32 * speed) % screen_rect.width());
            let y_base = screen_rect.min.y + ((seed * 29.0) as f32 % screen_rect.height());
            let y_sway = (time * 0.4 + seed).cos() as f32 * 30.0;
            let y = y_base + y_sway;

            let alpha = ((time * 0.8 + seed).sin() * 0.15 + 0.25) as f32;
            let color = egui::Color32::from_rgba_unmultiplied(
                224,
                242,
                254,
                (255.0 * alpha) as u8,
            );

            let symbol = symbols[i % symbols.len()];
            painter.text(
                egui::pos2(x, y),
                egui::Align2::CENTER_CENTER,
                symbol,
                egui::FontId::proportional(size),
                color,
            );
        }
    }
}

// 5. Geography Screensaver Component (World Grid & Floating Icons)
pub struct GeographyScreensaver {
    pub lang: Language,
}

impl Default for GeographyScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for GeographyScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, _remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        let symbols = [
            "🌐", "🇫🇷", "🇯🇵", "🇺🇸", "🇧🇷", "🇩🇪", "🇮🇹", "🇬🇧", "🇨🇦", "🇲🇽", "🇦🇺", "🇪🇸",
            "🧭", "🇨🇭", "🇰🇷", "🇪🇬", "🇮🇳", "🇿🇦", "🗺️", "📍", "✈️", "⛵", "🌍", "🌎", "🌏",
        ];
        let num_elements = 24;

        for i in 0..num_elements {
            let seed = (i * 47 + 13) as f64;
            let size = 20.0 + (seed % 16.0) as f32;
            let x_base = screen_rect.min.x + ((seed * 83.0) as f32 % screen_rect.width());
            let x_sway = (time * 0.4 + seed).sin() as f32 * 25.0;
            let x = x_base + x_sway;

            let loop_height = screen_rect.height() + 80.0;
            let speed = 15.0 + (seed % 20.0) as f32;
            let y = screen_rect.max.y - ((time as f32 * speed + (seed * 53.0) as f32) % loop_height);

            let alpha = ((time * 0.7 + seed).sin() * 0.2 + 0.35) as f32;
            let color = egui::Color32::from_rgba_unmultiplied(
                56,
                189,
                248,
                (255.0 * alpha) as u8,
            );

            let symbol = symbols[i % symbols.len()];
            painter.text(
                egui::pos2(x, y),
                egui::Align2::CENTER_CENTER,
                symbol,
                egui::FontId::proportional(size),
                color,
            );
        }

        let title = match self.lang {
            Language::English => "🌍 World Geography & Flags",
            Language::Spanish => "🌍 Geografía y Banderas del Mundo",
        };

        painter.text(
            screen_rect.center() - egui::vec2(0.0, 200.0),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(30.0),
            egui::Color32::from_rgb(56, 189, 248),
        );
    }
}

// 6. Vocabulary & Spelling Screensaver Component (Floating Letters & Books)
pub struct VocabScreensaver {
    pub lang: Language,
}

impl Default for VocabScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for VocabScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, _remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        let symbols = [
            "📚", "✏️", "📖", "📝", "🔤", "A", "B", "C", "D", "E", "F", "G",
            "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
        ];
        let num_elements = 24;

        for i in 0..num_elements {
            let seed = (i * 41 + 17) as f64;
            let size = 18.0 + (seed % 16.0) as f32;
            let x_base = screen_rect.min.x + ((seed * 83.0) as f32 % screen_rect.width());
            let x_sway = (time * 0.5 + seed).sin() as f32 * 25.0;
            let x = x_base + x_sway;

            let loop_height = screen_rect.height() + 80.0;
            let speed = 16.0 + (seed % 22.0) as f32;
            let y = screen_rect.max.y - ((time as f32 * speed + (seed * 53.0) as f32) % loop_height);

            let alpha = ((time * 0.7 + seed).sin() * 0.2 + 0.35) as f32;
            let color = egui::Color32::from_rgba_unmultiplied(
                251,
                146,
                60,
                (255.0 * alpha) as u8,
            );

            let symbol = symbols[i % symbols.len()];
            painter.text(
                egui::pos2(x, y),
                egui::Align2::CENTER_CENTER,
                symbol,
                egui::FontId::proportional(size),
                color,
            );
        }

        let title = tr(self.lang, "vocab_title");

        painter.text(
            screen_rect.center() - egui::vec2(0.0, 200.0),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(30.0),
            egui::Color32::from_rgb(251, 146, 60),
        );
    }
}

// 7. Science & Nature Screensaver Component (Floating Atoms, Planets, DNA, & Particles)
pub struct ScienceScreensaver {
    pub lang: Language,
}

impl Default for ScienceScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for ScienceScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, _remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();

        let symbols = [
            "⚛️", "🧪", "🪐", "🧬", "🔬", "⚡", "🌋", "🦕", "🌌", "🍏", "🌿", "💧",
            "☀️", "🌍", "🫀", "💎", "💫", "🛰️", "🚀", "💥", "🧲", "🧬", "🔭", "☄️",
        ];
        let num_elements = 24;

        for i in 0..num_elements {
            let seed = (i * 53 + 11) as f64;
            let size = 18.0 + (seed % 16.0) as f32;
            let x_base = screen_rect.min.x + ((seed * 83.0) as f32 % screen_rect.width());
            let x_sway = (time * 0.5 + seed).sin() as f32 * 25.0;
            let x = x_base + x_sway;

            let loop_height = screen_rect.height() + 80.0;
            let speed = 16.0 + (seed % 22.0) as f32;
            let y = screen_rect.max.y - ((time as f32 * speed + (seed * 53.0) as f32) % loop_height);

            let alpha = ((time * 0.7 + seed).sin() * 0.2 + 0.35) as f32;
            let color = egui::Color32::from_rgba_unmultiplied(
                34,
                211,
                238,
                (255.0 * alpha) as u8,
            );

            let symbol = symbols[i % symbols.len()];
            painter.text(
                egui::pos2(x, y),
                egui::Align2::CENTER_CENTER,
                symbol,
                egui::FontId::proportional(size),
                color,
            );
        }

        let title = tr(self.lang, "science_title");

        painter.text(
            screen_rect.center() - egui::vec2(0.0, 200.0),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(30.0),
            egui::Color32::from_rgb(34, 211, 238),
        );
    }
}

// 8. English Pronunciation Screensaver Component (Floating Audio Waves & Speech Symbols)
pub struct PronunciationScreensaver {
    pub lang: Language,
}

impl Default for PronunciationScreensaver {
    fn default() -> Self {
        Self {
            lang: Language::English,
        }
    }
}

impl ScreensaverComponent for PronunciationScreensaver {
    fn render_visuals(&mut self, ui: &mut egui::Ui, _remaining_sec: u64) {
        let screen_rect = ui.ctx().screen_rect();
        let time = ui.input(|i| i.time);
        let painter = ui.painter();
        let center = screen_rect.center();

        // 1. Draw Concentric Expanding Audio Soundwave Pulse Rings around center
        let num_pulse_rings = 4;
        let max_radius = screen_rect.width().min(screen_rect.height()) * 0.45;
        for i in 0..num_pulse_rings {
            let phase = (time * 0.4 + i as f64 * 0.25) % 1.0;
            let radius = phase as f32 * max_radius;
            let alpha = ((1.0 - phase) * 0.25) as f32; // Fade out as ring expands
            let color = egui::Color32::from_rgba_unmultiplied(
                192,
                132,
                252,
                (255.0 * alpha) as u8,
            );
            painter.circle_stroke(
                center,
                radius,
                egui::Stroke::new(2.0, color),
            );
        }

        // 2. Draw Dynamic Oscillating Audio Waveform Frequency Lines across screen
        let wave_y_offsets = [-180.0, -90.0, 90.0, 180.0];
        let wave_frequencies = [0.015, 0.02, 0.012, 0.025];
        let wave_speeds = [1.8, -1.4, 2.1, -1.2];
        let wave_amplitudes = [35.0, 25.0, 30.0, 20.0];

        for (w_idx, &base_y_offset) in wave_y_offsets.iter().enumerate() {
            let y_center = center.y + base_y_offset;
            let freq = wave_frequencies[w_idx];
            let speed = wave_speeds[w_idx];
            let amp = wave_amplitudes[w_idx];

            let stroke_color = match w_idx % 3 {
                0 => egui::Color32::from_rgba_unmultiplied(192, 132, 252, 60), // Neon Purple
                1 => egui::Color32::from_rgba_unmultiplied(236, 72, 153, 50),  // Pink Magenta
                _ => egui::Color32::from_rgba_unmultiplied(168, 85, 247, 55),  // Violet Glow
            };

            let steps = 60;
            let step_width = screen_rect.width() / steps as f32;
            let mut prev_pt: Option<egui::Pos2> = None;

            for step in 0..=steps {
                let x = screen_rect.min.x + step as f32 * step_width;
                let sin_val = ((x as f64 * freq + time * speed).sin()
                    + (x as f64 * freq * 2.3 + time * speed * 0.7).cos() * 0.5) as f32;
                let y = y_center + sin_val * amp;
                let current_pt = egui::pos2(x, y);

                if let Some(p) = prev_pt {
                    painter.line_segment([p, current_pt], egui::Stroke::new(2.0, stroke_color));
                }
                prev_pt = Some(current_pt);
            }
        }

        // 3. Floating Audio & Speech Symbols
        let symbols = [
            "🔊", "🎧", "🎵", "🎶", "💬", "📢", "📻", "🎼", "📊", "🌊", "🔤", "👂", "🔔",
        ];
        let num_elements = 26;

        for i in 0..num_elements {
            let seed = (i * 61 + 19) as f64;
            let size = 22.0 + (seed % 18.0) as f32;
            let x_base = screen_rect.min.x + ((seed * 83.0) as f32 % screen_rect.width());
            let x_sway = (time * 0.5 + seed).sin() as f32 * 28.0;
            let x = x_base + x_sway;

            let loop_height = screen_rect.height() + 80.0;
            let speed = 16.0 + (seed % 22.0) as f32;
            let y = screen_rect.max.y - ((time as f32 * speed + (seed * 53.0) as f32) % loop_height);

            let alpha = ((time * 0.7 + seed).sin() * 0.2 + 0.40) as f32;
            let color = egui::Color32::from_rgba_unmultiplied(
                216,
                180,
                254,
                (255.0 * alpha) as u8,
            );

            let symbol = symbols[i % symbols.len()];
            painter.text(
                egui::pos2(x, y),
                egui::Align2::CENTER_CENTER,
                symbol,
                egui::FontId::proportional(size),
                color,
            );
        }

        // 4. Screensaver Title Banner
        let title = tr(self.lang, "pronunciation_title");

        painter.text(
            screen_rect.center() - egui::vec2(0.0, 220.0),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(34.0),
            egui::Color32::from_rgb(216, 180, 254),
        );
    }
}

/// Renders the visual content of the selected screensaver style.
#[allow(dead_code)]
pub fn render_screensaver_style(
    style: ScreensaverStyle,
    ui: &mut egui::Ui,
    remaining_sec: u64,
) {
    render_screensaver_style_localized(style, ui, remaining_sec, Language::English);
}

pub fn render_screensaver_style_localized(
    style: ScreensaverStyle,
    ui: &mut egui::Ui,
    remaining_sec: u64,
    lang: Language,
) {
    match style {
        ScreensaverStyle::Default => DefaultScreensaver { lang }.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Minimalist => MinimalistScreensaver { lang }.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Matrix => MatrixScreensaver { lang }.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Math => MathScreensaver.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Geography => GeographyScreensaver { lang }.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Vocab => VocabScreensaver { lang }.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Science => ScienceScreensaver { lang }.render_visuals(ui, remaining_sec),
        ScreensaverStyle::Pronunciation => PronunciationScreensaver { lang }.render_visuals(ui, remaining_sec),
    }
}

pub fn get_background_color(style: ScreensaverStyle) -> egui::Color32 {
    match style {
        ScreensaverStyle::Default => egui::Color32::from_rgb(15, 23, 42),
        ScreensaverStyle::Minimalist => egui::Color32::from_rgb(8, 8, 8),
        ScreensaverStyle::Matrix => egui::Color32::from_rgb(2, 8, 2),
        ScreensaverStyle::Math => egui::Color32::from_rgb(17, 24, 39), // Slate 900
        ScreensaverStyle::Geography => egui::Color32::from_rgb(13, 27, 42), // Deep Ocean Navy
        ScreensaverStyle::Vocab => egui::Color32::from_rgb(24, 18, 43), // Deep Amber/Purple Velvet
        ScreensaverStyle::Science => egui::Color32::from_rgb(10, 22, 40), // Deep Cosmic Cyan/Navy
        ScreensaverStyle::Pronunciation => egui::Color32::from_rgb(23, 15, 38), // Deep Violet/Purple Night
    }
}


