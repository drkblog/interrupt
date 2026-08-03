mod config;
mod screensaver;
mod win32;

use config::AppSettings;
use eframe::egui;
use screensaver::{get_background_color, render_screensaver_style, ScreensaverStyle};
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
    new_warning_time_seconds: u32,
    new_screensaver_style: ScreensaverStyle,
    new_password_input: String,
    settings_message: Option<String>,
    focus_password_field: bool,
    focus_settings_password: bool,
    focus_reset_password: bool,
    show_reset_dialog: bool,
    reset_password_input: String,
    reset_error_message: Option<String>,
    show_pause_unblock_panel: bool,
    last_pause_interaction: Option<Instant>,
}

impl InterruptApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load();
        let new_play_time = settings.play_time_minutes;
        let new_pause_time = settings.pause_time_minutes;
        let new_warning_time_seconds = settings.warning_time_seconds;
        let new_screensaver_style = settings.screensaver_style;

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
            new_warning_time_seconds,
            new_screensaver_style,
            new_password_input: String::new(),
            settings_message: None,
            focus_password_field: false,
            focus_settings_password: false,
            focus_reset_password: false,
            show_reset_dialog: false,
            reset_password_input: String::new(),
            reset_error_message: None,
            show_pause_unblock_panel: false,
            last_pause_interaction: None,
        }
    }

    fn play_duration(&self) -> Duration {
        Duration::from_secs((self.settings.play_time_minutes as u64) * 60)
    }

    fn pause_duration(&self) -> Duration {
        Duration::from_secs((self.settings.pause_time_minutes as u64) * 60)
    }

    fn warning_duration(&self) -> Duration {
        Duration::from_secs(self.settings.warning_time_seconds as u64)
    }

    fn open_settings(&mut self) {
        self.show_settings = true;
        self.focus_settings_password = true;
        self.settings_password_input.clear();
        self.settings_message = None;
    }

    fn close_settings(&mut self) {
        self.show_settings = false;
        self.settings_unlocked = false;
        self.settings_password_input.clear();
        self.settings_message = None;
    }

    fn open_reset_dialog(&mut self) {
        self.show_reset_dialog = true;
        self.focus_reset_password = true;
        self.reset_password_input.clear();
        self.reset_error_message = None;
    }

    fn update_cycle_state(&mut self, ctx: &egui::Context) {
        // While settings are open, global timer is suspended
        if self.show_settings {
            return;
        }

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
        self.show_pause_unblock_panel = false;
        self.focus_password_field = false;
        self.last_pause_interaction = None;

        let rect = win32::get_virtual_screen_rect();
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
            rect.width as f32,
            rect.height as f32,
        )));
        win32::make_app_window_fullscreen_topmost();
    }

    fn transition_to_play(&mut self, ctx: &egui::Context) {
        win32::disable_keyboard_hook();
        self.state = AppState::Play;
        self.state_start = Instant::now();
        self.password_input.clear();
        self.password_error = None;
        self.show_pause_unblock_panel = false;
        self.last_pause_interaction = None;

        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(540.0, 420.0)));
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

    fn try_reset_timer(&mut self) {
        if self.settings.verify_password(&self.reset_password_input) {
            self.state = AppState::Play;
            self.state_start = Instant::now();
            self.show_reset_dialog = false;
            self.reset_password_input.clear();
            self.reset_error_message = None;
        } else {
            self.reset_error_message = Some("Invalid password.".to_string());
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

        let lock_elapsed = self.state_start.elapsed();
        let lock_grace_period = Duration::from_secs(3);

        // Only detect user interaction after the initial 3-second lock grace period
        if lock_elapsed >= lock_grace_period {
            let is_interacting = ctx.input(|i| {
                i.pointer.any_click()
                    || i.pointer.any_down()
                    || i.pointer.delta() != egui::vec2(0.0, 0.0)
                    || !i.events.is_empty()
            });

            if is_interacting {
                self.last_pause_interaction = Some(Instant::now());
                if !self.show_pause_unblock_panel {
                    self.show_pause_unblock_panel = true;
                    self.focus_password_field = true;
                }
            }
        } else {
            // Keep password unblock panel hidden during initial 10 seconds of screen lock
            self.show_pause_unblock_panel = false;
            self.last_pause_interaction = None;
        }

        // Hide unblock panel if no interaction for 20 seconds
        if let Some(last_time) = self.last_pause_interaction {
            if last_time.elapsed() >= Duration::from_secs(20) {
                self.show_pause_unblock_panel = false;
                self.last_pause_interaction = None;
                self.password_input.clear();
                self.password_error = None;
            }
        }

        let bg_color = get_background_color(self.settings.screensaver_style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(bg_color))
            .show(ctx, |ui| {
                let available_height = ui.available_height();

                ui.vertical_centered(|ui| {
                    ui.add_space(available_height * 0.14);

                    // Render modular visual screensaver component
                    render_screensaver_style(
                        self.settings.screensaver_style,
                        ui,
                        remaining_sec,
                    );

                    ui.add_space(30.0);

                    // Password unblock panel (Only visible when user interacts)
                    if self.show_pause_unblock_panel {
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

                                if response.changed() || response.has_focus() {
                                    self.last_pause_interaction = Some(Instant::now());
                                }

                                if self.focus_password_field {
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
                    }
                });
            });
    }

    fn render_reset_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_reset_dialog {
            return;
        }

        let mut open = self.show_reset_dialog;
        egui::Window::new("🔄 Reset Timer")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label("Enter password to reset the play timer:");
                ui.add_space(8.0);

                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.reset_password_input)
                        .password(true)
                        .hint_text("Password..."),
                );

                if self.focus_reset_password {
                    response.request_focus();
                    self.focus_reset_password = false;
                }

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.try_reset_timer();
                }

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Confirm Reset").clicked() {
                        self.try_reset_timer();
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_reset_dialog = false;
                        self.reset_password_input.clear();
                        self.reset_error_message = None;
                    }
                });

                if let Some(ref err) = self.reset_error_message {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(err).color(egui::Color32::LIGHT_RED));
                }
            });

        if !open {
            self.show_reset_dialog = false;
            self.reset_password_input.clear();
            self.reset_error_message = None;
        }
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

                    if self.focus_settings_password {
                        response.request_focus();
                        self.focus_settings_password = false;
                    }

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
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("⏸️ Timer suspended while settings window is open")
                            .color(egui::Color32::YELLOW)
                            .size(13.0),
                    );
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

                            ui.label("Warning Time (seconds):");
                            ui.add(
                                egui::DragValue::new(&mut self.new_warning_time_seconds)
                                    .range(5..=300),
                            );
                            ui.end_row();

                            ui.label("Screensaver Style:");
                            egui::ComboBox::from_id_source("screensaver_style_selector")
                                .selected_text(self.new_screensaver_style.name())
                                .show_ui(ui, |ui| {
                                    for style in ScreensaverStyle::all() {
                                        ui.selectable_value(
                                            &mut self.new_screensaver_style,
                                            *style,
                                            style.name(),
                                        );
                                    }
                                });
                            ui.end_row();

                            ui.label("New Password (optional):");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.new_password_input)
                                    .password(true),
                            );
                            ui.end_row();
                        });

                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Settings").clicked() {
                            self.settings.play_time_minutes = self.new_play_time;
                            self.settings.pause_time_minutes = self.new_pause_time;
                            self.settings.warning_time_seconds = self.new_warning_time_seconds;
                            self.settings.screensaver_style = self.new_screensaver_style;
                            if !self.new_password_input.trim().is_empty() {
                                self.settings.set_password(&self.new_password_input);
                                self.new_password_input.clear();
                            }
                            if let Err(e) = self.settings.save() {
                                self.settings_message = Some(format!("Failed to save: {}", e));
                            } else {
                                self.settings_message =
                                    Some("Settings saved successfully!".to_string());
                                self.close_settings();
                            }
                        }

                        if ui.button("🔒 Lock Screen Now").clicked() {
                            self.close_settings();
                            self.transition_to_pause(ctx);
                        }
                    });

                    if let Some(ref msg) = self.settings_message {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(msg).color(egui::Color32::GREEN));
                    }
                }
            });

        if !open {
            self.close_settings();
        }
    }
}

impl eframe::App for InterruptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Freeze timer accumulation if settings window is active
        if self.show_settings {
            let dt = Duration::from_secs_f32(ctx.input(|i| i.stable_dt));
            self.state_start += dt;
        } else {
            self.update_cycle_state(ctx);
        }

        if self.state == AppState::Play || self.state == AppState::Warning {
            let play_dur = self.play_duration();
            let elapsed = self.state_start.elapsed();
            let remaining_sec = if play_dur > elapsed {
                (play_dur - elapsed).as_secs()
            } else {
                0
            };

            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("⏱️ Interrupt");
                    ui.separator();

                    let status_text = if self.show_settings {
                        format!(
                            "Play Mode [PAUSED] | Next break in {:02}:{:02}",
                            remaining_sec / 60,
                            remaining_sec % 60
                        )
                    } else {
                        format!(
                            "Play Mode | Next break in {:02}:{:02}",
                            remaining_sec / 60,
                            remaining_sec % 60
                        )
                    };

                    ui.label(status_text);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙️ Settings").clicked() {
                            self.open_settings();
                        }
                        if ui.button("🔒 Lock Now").clicked() {
                            self.transition_to_pause(ctx);
                        }
                        if ui.button("🔄 Reset Timer").clicked() {
                            self.open_reset_dialog();
                        }
                    });
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading("Interrupt Screen Time Manager");
                    ui.add_space(8.0);
                    let sub_label = if self.show_settings {
                        "Settings open — timer suspended until settings window is closed."
                    } else {
                        "Active cycle running. Time remaining until next screen lock:"
                    };
                    ui.label(sub_label);
                    ui.add_space(16.0);

                    // Prominent Live Timer Display
                    ui.label(
                        egui::RichText::new(format!(
                            "{:02}:{:02}",
                            remaining_sec / 60,
                            remaining_sec % 60
                        ))
                        .size(64.0)
                        .color(if self.show_settings {
                            egui::Color32::YELLOW
                        } else if self.state == AppState::Warning {
                            egui::Color32::LIGHT_RED
                        } else {
                            egui::Color32::from_rgb(56, 189, 248)
                        })
                        .monospace()
                        .strong(),
                    );

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        ui.add_space(100.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("🔒 Lock Now").size(16.0).strong(),
                                )
                                .min_size(egui::vec2(140.0, 36.0)),
                            )
                            .clicked()
                        {
                            self.transition_to_pause(ctx);
                        }
                        ui.add_space(12.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("🔄 Reset Timer")
                                        .size(16.0)
                                        .strong(),
                                )
                                .min_size(egui::vec2(140.0, 36.0)),
                            )
                            .clicked()
                        {
                            self.open_reset_dialog();
                        }
                    });

                    ui.add_space(20.0);
                    ui.label(format!("• Play Time: {} mins", self.settings.play_time_minutes));
                    ui.label(format!("• Pause Time: {} mins", self.settings.pause_time_minutes));
                    ui.label(format!("• Warning Time: {} secs", self.settings.warning_time_seconds));
                    ui.label(format!("• Screensaver: {}", self.settings.screensaver_style.name()));
                    ui.label("• Master Password: enabled ('mindfulness')");
                });
            });
        }

        if self.state == AppState::Warning && !self.show_settings {
            self.render_warning_banner(ctx);
        }

        if self.state == AppState::Pause {
            self.render_pause_screen(ctx);
            ctx.request_repaint();
        } else {
            ctx.request_repaint_after(Duration::from_millis(500));
        }

        self.render_reset_dialog(ctx);
        self.render_settings_window(ctx);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Interrupt - Healthy Screen Breaks")
            .with_inner_size([540.0, 420.0])
            .with_min_inner_size([400.0, 350.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Interrupt",
        options,
        Box::new(|cc| Ok(Box::new(InterruptApp::new(cc)))),
    )
}
