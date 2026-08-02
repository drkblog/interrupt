mod config;
mod win32;

use config::AppSettings;
use eframe::egui;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    Play,
    Warning,
    Pause,
}

pub struct InterruptApp {
    settings: AppSettings,
    state: AppState,
    state_start: Instant,
    password_input: String,
    settings_password_input: String,
    password_error: Option<String>,
    settings_unlocked: bool,
    show_settings: bool,
    new_play_time: u32,
    new_pause_time: u32,
    new_password_input: String,
    settings_message: Option<String>,
    focus_password_field: bool,
}

impl InterruptApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load();
        let new_play_time = settings.play_time_minutes;
        let new_pause_time = settings.pause_time_minutes;

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            settings,
            state: AppState::Play,
            state_start: Instant::now(),
            password_input: String::new(),
            settings_password_input: String::new(),
            password_error: None,
            settings_unlocked: false,
            show_settings: false,
            new_play_time,
            new_pause_time,
            new_password_input: String::new(),
            settings_message: None,
            focus_password_field: false,
        }
    }

    fn play_duration(&self) -> Duration {
        Duration::from_secs((self.settings.play_time_minutes as u64) * 60)
    }

    fn pause_duration(&self) -> Duration {
        if cfg!(feature = "debug") {
            Duration::from_secs(20)
        } else {
            Duration::from_secs((self.settings.pause_time_minutes as u64) * 60)
        }
    }

    fn warning_duration(&self) -> Duration {
        Duration::from_secs(60)
    }

    fn update_cycle_state(&mut self, ctx: &egui::Context) {
        let elapsed = self.state_start.elapsed();

        match self.state {
            AppState::Play => {
                let play_dur = self.play_duration();
                let warning_threshold = if play_dur > self.warning_duration() {
                    play_dur - self.warning_duration()
                } else {
                    Duration::from_secs(0)
                };

                if elapsed >= play_dur {
                    self.transition_to_pause(ctx);
                } else if elapsed >= warning_threshold {
                    self.state = AppState::Warning;
                }
            }
            AppState::Warning => {
                let play_dur = self.play_duration();
                if elapsed >= play_dur {
                    self.transition_to_pause(ctx);
                }
            }
            AppState::Pause => {
                let pause_dur = self.pause_duration();
                if elapsed >= pause_dur {
                    self.transition_to_play(ctx);
                }
            }
        }
    }

    fn transition_to_pause(&mut self, ctx: &egui::Context) {
        win32::capture_foreground_window();
        win32::enable_keyboard_hook();
        self.state = AppState::Pause;
        self.state_start = Instant::now();
        self.password_input.clear();
        self.password_error = None;
        self.focus_password_field = true;

        let rect = win32::get_virtual_screen_rect();
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(rect.width as f32, rect.height as f32)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        win32::make_app_window_fullscreen_topmost();
    }

    fn transition_to_play(&mut self, ctx: &egui::Context) {
        win32::disable_keyboard_hook();
        self.state = AppState::Play;
        self.state_start = Instant::now();
        self.password_input.clear();
        self.password_error = None;

        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(540.0, 360.0)));
        win32::restore_app_window_normal();
        win32::restore_foreground_window();
    }

    fn try_unblock(&mut self, ctx: &egui::Context) {
        if self.settings.verify_password(&self.password_input) {
            self.transition_to_play(ctx);
        } else {
            self.password_error = Some("Incorrect password. Please try again.".to_string());
        }
    }

    fn render_warning_banner(&mut self, ctx: &egui::Context) {
        let play_dur = self.play_duration();
        let elapsed = self.state_start.elapsed();
        let remaining_sec = if play_dur > elapsed {
            (play_dur - elapsed).as_secs()
        } else {
            0
        };

        egui::Area::new(egui::Id::new("warning_banner"))
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 20.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(220, 38, 38))
                    .rounding(8.0)
                    .inner_margin(egui::Margin::symmetric(24.0, 12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(
                                egui::RichText::new("⚠️ SCREEN LOCK WARNING")
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            );
                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "Screen will lock in {:02}:{:02} minutes",
                                    remaining_sec / 60,
                                    remaining_sec % 60
                                ))
                                .color(egui::Color32::YELLOW)
                                .size(18.0)
                                .strong(),
                            );
                        });
                    });
            });
    }

    fn render_pause_screen(&mut self, ctx: &egui::Context) {
        let pause_dur = self.pause_duration();
        let elapsed = self.state_start.elapsed();
        let remaining_sec = if pause_dur > elapsed {
            (pause_dur - elapsed).as_secs()
        } else {
            0
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 23, 42)))
            .show(ctx, |ui| {
                let available_height = ui.available_height();

                ui.vertical_centered(|ui| {
                    ui.add_space(available_height * 0.18);

                    ui.heading(
                        egui::RichText::new("🌿 TIME TO TAKE A BREAK")
                            .size(42.0)
                            .color(egui::Color32::from_rgb(56, 189, 248))
                            .strong(),
                    );

                    ui.add_space(12.0);
                    let subtitle = if cfg!(feature = "debug") {
                        "DEBUG MODE: Auto-unlocking in 20 seconds."
                    } else {
                        "Step away, stretch, drink water, and rest your eyes."
                    };
                    ui.label(
                        egui::RichText::new(subtitle)
                            .size(20.0)
                            .color(if cfg!(feature = "debug") {
                                egui::Color32::YELLOW
                            } else {
                                egui::Color32::from_rgb(203, 213, 225)
                            }),
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

                    ui.add_space(40.0);

                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(30, 41, 59))
                        .rounding(12.0)
                        .inner_margin(egui::Margin::same(24.0))
                        .show(ui, |ui| {
                            ui.set_max_width(360.0);
                            ui.label(
                                egui::RichText::new("Enter Password to Unblock Early:")
                                    .size(16.0)
                                    .color(egui::Color32::LIGHT_GRAY),
                            );
                            ui.add_space(8.0);

                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.password_input)
                                    .password(true)
                                    .hint_text("Password...")
                                    .desired_width(320.0),
                            );

                            let user_interacted = ctx.input(|i| {
                                i.pointer.any_click()
                                    || i.pointer.any_down()
                                    || !i.events.is_empty()
                            });

                            if self.focus_password_field
                                || !response.has_focus()
                                || user_interacted
                            {
                                response.request_focus();
                                self.focus_password_field = false;
                                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                            }

                            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                self.try_unblock(ctx);
                            }

                            ui.add_space(12.0);

                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("🔓 Unblock Screen")
                                            .size(16.0)
                                            .strong(),
                                    )
                                    .min_size(egui::vec2(200.0, 40.0)),
                                )
                                .clicked()
                            {
                                self.try_unblock(ctx);
                            }

                            if let Some(ref err) = self.password_error {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(err)
                                        .color(egui::Color32::LIGHT_RED)
                                        .size(14.0),
                                );
                            }
                        });
                });
            });
    }

    fn render_settings_window(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        let mut open = self.show_settings;
        egui::Window::new("⚙️ Interrupt Settings")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                if !self.settings_unlocked {
                    ui.heading("Authentication Required");
                    ui.label("Enter current password or master password to access settings:");
                    ui.add_space(8.0);

                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.settings_password_input)
                            .password(true)
                            .hint_text("Password..."),
                    );

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if self.settings.verify_password(&self.settings_password_input) {
                            self.settings_unlocked = true;
                            self.settings_password_input.clear();
                            self.settings_message = None;
                        } else {
                            self.settings_message =
                                Some("Invalid password authentication.".to_string());
                        }
                    }

                    if ui.button("Unlock Settings").clicked() {
                        if self.settings.verify_password(&self.settings_password_input) {
                            self.settings_unlocked = true;
                            self.settings_password_input.clear();
                            self.settings_message = None;
                        } else {
                            self.settings_message =
                                Some("Invalid password authentication.".to_string());
                        }
                    }

                    if let Some(ref msg) = self.settings_message {
                        ui.label(egui::RichText::new(msg).color(egui::Color32::LIGHT_RED));
                    }
                } else {
                    ui.heading("Configure Break Cycles");
                    ui.add_space(8.0);

                    egui::Grid::new("settings_grid")
                        .num_columns(2)
                        .spacing([12.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Play Time (minutes):");
                            ui.add(egui::DragValue::new(&mut self.new_play_time).range(1..=300));
                            ui.end_row();

                            ui.label("Pause Time (minutes):");
                            ui.add(egui::DragValue::new(&mut self.new_pause_time).range(1..=60));
                            ui.end_row();

                            ui.label("New Password (optional):");
                            ui.add(egui::TextEdit::singleline(&mut self.new_password_input).password(true));
                            ui.end_row();
                        });

                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Settings").clicked() {
                            self.settings.play_time_minutes = self.new_play_time;
                            self.settings.pause_time_minutes = self.new_pause_time;
                            if !self.new_password_input.trim().is_empty() {
                                self.settings.set_password(&self.new_password_input);
                                self.new_password_input.clear();
                            }
                            if let Err(e) = self.settings.save() {
                                self.settings_message = Some(format!("Failed to save: {}", e));
                            } else {
                                self.settings_message = Some("Settings saved successfully!".to_string());
                            }
                        }

                        if ui.button("🔒 Lock Screen Now").clicked() {
                            self.transition_to_pause(ctx);
                            self.show_settings = false;
                        }
                    });

                    if let Some(ref msg) = self.settings_message {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(msg).color(egui::Color32::GREEN));
                    }
                }
            });
        self.show_settings = open;
    }
}

impl eframe::App for InterruptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_cycle_state(ctx);

        if self.state == AppState::Play || self.state == AppState::Warning {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("⏱️ Interrupt");
                    ui.separator();

                    let play_dur = self.play_duration();
                    let elapsed = self.state_start.elapsed();
                    let remaining_sec = if play_dur > elapsed {
                        (play_dur - elapsed).as_secs()
                    } else {
                        0
                    };

                    ui.label(format!(
                        "Play Mode | Next break in {:02}:{:02}",
                        remaining_sec / 60,
                        remaining_sec % 60
                    ));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙️ Settings").clicked() {
                            self.show_settings = true;
                        }
                    });
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading("Interrupt Screen Time Manager");
                    ui.add_space(12.0);
                    ui.label("Active cycle running. Keep this window minimized or running in background.");
                    ui.add_space(16.0);
                    ui.label(format!("• Play Time: {} mins", self.settings.play_time_minutes));
                    ui.label(format!("• Pause Time: {} mins", self.settings.pause_time_minutes));
                    ui.label("• Master Password: enabled ('mindfulness')");
                });
            });
        }

        if self.state == AppState::Warning {
            self.render_warning_banner(ctx);
        }

        if self.state == AppState::Pause {
            self.render_pause_screen(ctx);
        }

        self.render_settings_window(ctx);

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Interrupt - Healthy Screen Breaks")
            .with_inner_size([540.0, 360.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Interrupt",
        options,
        Box::new(|cc| Ok(Box::new(InterruptApp::new(cc)))),
    )
}
