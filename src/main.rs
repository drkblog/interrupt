#![windows_subsystem = "windows"]

mod config;
mod i18n;
mod screensaver;
mod win32;

use config::{AppSettings, GeographyDifficulty, MathDifficulty, ScienceDifficulty, VocabDifficulty};
use eframe::egui;
use i18n::{get_science_question_pool, get_vocab_question_pool, tr, Language};
use screensaver::{get_background_color, render_screensaver_style_localized, ScreensaverStyle};
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
    new_enable_logging: bool,
    settings_message: Option<String>,
    focus_password_field: bool,
    focus_settings_password: bool,
    focus_reset_password: bool,
    show_reset_dialog: bool,
    reset_password_input: String,
    reset_error_message: Option<String>,
    show_pause_unblock_panel: bool,
    last_pause_interaction: Option<Instant>,
    
    // Tray icon integration
    tray_registered: bool,
    should_exit: bool,

    // Math exercises screensaver state
    math_problem_text: String,
    math_problem_answer: i32,
    math_user_input: String,
    math_solved_count: u32,
    math_feedback: Option<String>,
    math_feedback_color: egui::Color32,

    // Geography exercises screensaver state
    geography_question_text: String,
    geography_choices: Vec<String>,
    geography_correct_idx: usize,
    geography_solved_count: u32,
    geography_feedback: Option<String>,
    geography_feedback_color: egui::Color32,

    // Vocab & Spelling exercises screensaver state
    vocab_question_text: String,
    vocab_choices: Vec<String>,
    vocab_correct_idx: usize,
    vocab_solved_count: u32,
    vocab_feedback: Option<String>,
    vocab_feedback_color: egui::Color32,

    // Science & Nature exercises screensaver state
    science_question_text: String,
    science_choices: Vec<String>,
    science_correct_idx: usize,
    science_explanation: String,
    science_solved_count: u32,
    science_feedback: Option<String>,
    science_feedback_color: egui::Color32,

    active_settings_tab: usize,
    new_language: Language,
    new_math_questions_needed: u32,
    new_math_min_pause_percent: u32,
    new_math_difficulty: MathDifficulty,
    new_geography_questions_needed: u32,
    new_geography_min_pause_percent: u32,
    new_geography_difficulty: GeographyDifficulty,
    new_vocab_questions_needed: u32,
    new_vocab_min_pause_percent: u32,
    new_vocab_difficulty: VocabDifficulty,
    new_science_questions_needed: u32,
    new_science_min_pause_percent: u32,
    new_science_difficulty: ScienceDifficulty,
}

impl InterruptApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load();
        let new_play_time = settings.play_time_minutes;
        let new_pause_time = settings.pause_time_minutes;
        let new_warning_time_seconds = settings.warning_time_seconds;
        let new_screensaver_style = settings.screensaver_style;
        let new_enable_logging = settings.enable_logging;
        let new_language = settings.language;
        let new_math_questions_needed = settings.math_questions_needed;
        let new_math_min_pause_percent = settings.math_min_pause_percent;
        let new_math_difficulty = settings.math_difficulty;
        let new_geography_questions_needed = settings.geography_questions_needed;
        let new_geography_min_pause_percent = settings.geography_min_pause_percent;
        let new_geography_difficulty = settings.geography_difficulty;
        let new_vocab_questions_needed = settings.vocab_questions_needed;
        let new_vocab_min_pause_percent = settings.vocab_min_pause_percent;
        let new_vocab_difficulty = settings.vocab_difficulty;
        let new_science_questions_needed = settings.science_questions_needed;
        let new_science_min_pause_percent = settings.science_min_pause_percent;
        let new_science_difficulty = settings.science_difficulty;

        win32::init_logging(settings.enable_logging);
        
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(15, 23, 42); // slate 900
        visuals.window_fill = egui::Color32::from_rgb(30, 41, 59); // slate 800
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(15, 23, 42);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(203, 213, 225); // slate 300
        
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(15, 23, 42); // slate 900
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(241, 245, 249); // slate 100
        visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(79, 70, 229); // indigo 600
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(67, 56, 202); // indigo 700
        visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        
        visuals.widgets.open.bg_fill = egui::Color32::from_rgb(51, 65, 85); // slate 700
        visuals.widgets.open.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.open.rounding = egui::Rounding::same(8.0);
        
        visuals.selection.bg_fill = egui::Color32::from_rgb(99, 102, 241); // indigo 500
        visuals.selection.stroke.color = egui::Color32::WHITE;
        cc.egui_ctx.set_visuals(visuals);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.item_spacing = egui::vec2(8.0, 12.0);
        cc.egui_ctx.set_style(style);

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
            new_enable_logging,
            settings_message: None,
            focus_password_field: false,
            focus_settings_password: false,
            focus_reset_password: false,
            show_reset_dialog: false,
            reset_password_input: String::new(),
            reset_error_message: None,
            show_pause_unblock_panel: false,
            last_pause_interaction: None,
            tray_registered: false,
            should_exit: false,
            math_problem_text: String::new(),
            math_problem_answer: 0,
            math_user_input: String::new(),
            math_solved_count: 0,
            math_feedback: None,
            math_feedback_color: egui::Color32::LIGHT_GRAY,
            geography_question_text: String::new(),
            geography_choices: Vec::new(),
            geography_correct_idx: 0,
            geography_solved_count: 0,
            geography_feedback: None,
            geography_feedback_color: egui::Color32::LIGHT_GRAY,
            vocab_question_text: String::new(),
            vocab_choices: Vec::new(),
            vocab_correct_idx: 0,
            vocab_solved_count: 0,
            vocab_feedback: None,
            vocab_feedback_color: egui::Color32::LIGHT_GRAY,
            science_question_text: String::new(),
            science_choices: Vec::new(),
            science_correct_idx: 0,
            science_explanation: String::new(),
            science_solved_count: 0,
            science_feedback: None,
            science_feedback_color: egui::Color32::LIGHT_GRAY,
            active_settings_tab: 0,
            new_language,
            new_math_questions_needed,
            new_math_min_pause_percent,
            new_math_difficulty,
            new_geography_questions_needed,
            new_geography_min_pause_percent,
            new_geography_difficulty,
            new_vocab_questions_needed,
            new_vocab_min_pause_percent,
            new_vocab_difficulty,
            new_science_questions_needed,
            new_science_min_pause_percent,
            new_science_difficulty,
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

    fn generate_math_problem(&mut self) {
        let ticks = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        let difficulty = self.settings.math_difficulty;
        let (problem, answer) = match difficulty {
            MathDifficulty::Low => {
                let op = (ticks % 2) as u32;
                match op {
                    0 => {
                        // Addition with total <= 40, single/double digits
                        let total = (ticks % 41) as i32; // 0..=40
                        let a = ((ticks >> 4) % (total as u128 + 1)) as i32;
                        let b = total - a;
                        (format!("{} + {}", a, b), total)
                    }
                    _ => {
                        // Subtraction with total <= 40, single/double digits
                        let total = (ticks % 41) as i32; // 0..=40
                        let b = ((ticks >> 4) % 60) as i32; // 0..=59
                        let a = total + b;
                        (format!("{} - {}", a, b), total)
                    }
                }
            }
            MathDifficulty::Medium => {
                let op = (ticks % 4) as u32;
                match op {
                    0 => {
                        // 3-digit addition
                        let a = ((ticks % 900) + 100) as i32;
                        let b = (((ticks >> 4) % 900) + 100) as i32;
                        (format!("{} + {}", a, b), a + b)
                    }
                    1 => {
                        // 3-digit subtraction
                        let a = ((ticks % 800) + 200) as i32; // 200..999
                        let b = (((ticks >> 4) % (a as u128 - 100)) + 100) as i32; // 100..(a-1)
                        (format!("{} - {}", a, b), a - b)
                    }
                    2 => {
                        // Multiplication / division tables
                        let sub_op = (ticks >> 8) % 2;
                        if sub_op == 0 {
                            let a = ((ticks % 11) + 2) as i32; // 2..12
                            let b = (((ticks >> 4) % 11) + 2) as i32; // 2..12
                            (format!("{} × {}", a, b), a * b)
                        } else {
                            let a = ((ticks % 11) + 2) as i32; // 2..12
                            let b = (((ticks >> 4) % 11) + 2) as i32; // 2..12
                            let c = a * b;
                            (format!("{} ÷ {}", c, a), b)
                        }
                    }
                    _ => {
                        // Two-step operations
                        let sub_op = (ticks >> 8) % 2;
                        if sub_op == 0 {
                            let a = ((ticks % 41) + 10) as i32; // 10..50
                            let b = (((ticks >> 4) % 41) + 10) as i32; // 10..50
                            let c = (((ticks >> 12) % ((a + b) as u128 - 2)) + 2) as i32;
                            (format!("{} + {} - {}", a, b, c), a + b - c)
                        } else {
                            let a = ((ticks % 41) + 20) as i32; // 20..60
                            let b = (((ticks >> 4) % (a as u128 - 10)) + 10) as i32; // 10..(a-1)
                            let c = (((ticks >> 12) % 41) + 10) as i32; // 10..50
                            (format!("{} - {} + {}", a, b, c), a - b + c)
                        }
                    }
                }
            }
            MathDifficulty::High => {
                let op = (ticks % 3) as u32;
                match op {
                    0 => {
                        // Parentheses, multiplication, division, addition
                        let c = ((ticks % 9) + 2) as i32; // 2..10
                        let factor = (((ticks >> 4) % 11) + 2) as i32; // 2..12
                        let a = c * factor;
                        let b = (((ticks >> 8) % 9) + 2) as i32; // 2..10
                        let d = (((ticks >> 12) % 46) + 5) as i32; // 5..50
                        (format!("({} × {}) ÷ {} + {}", a, b, c, d), (a * b) / c + d)
                    }
                    1 => {
                        // Algebra: ax + b = c or ax - b = c, solve for x
                        let a = ((ticks % 9) + 2) as i32; // 2..10
                        let x = (((ticks >> 4) % 9) + 2) as i32; // 2..10
                        let sub_op = (ticks >> 12) % 2;
                        if sub_op == 0 {
                            let b = (((ticks >> 8) % 20) + 1) as i32; // 1..20
                            let c = a * x + b;
                            (format!("Solve: {}x + {} = {}", a, b, c), x)
                        } else {
                            let limit = (a * x - 1).max(1) as u128;
                            let b = (((ticks >> 8) % limit) + 1) as i32;
                            let c = a * x - b;
                            (format!("Solve: {}x - {} = {}", a, b, c), x)
                        }
                    }
                    _ => {
                        // Complex multiplication, subtraction, addition
                        let a = ((ticks % 11) + 10) as i32; // 10..20
                        let b = (((ticks >> 4) % 6) + 3) as i32; // 3..8
                        let c = (((ticks >> 8) % 41) + 10) as i32; // 10..50
                        let d = (((ticks >> 12) % 26) + 5) as i32; // 5..30
                        (format!("{} × {} - {} + {}", a, b, c, d), a * b - c + d)
                    }
                }
            }
        };
        
        self.math_problem_text = problem;
        self.math_problem_answer = answer;
        self.math_user_input.clear();
    }

    fn generate_geography_problem(&mut self) {
        let ticks = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let difficulty = self.settings.geography_difficulty;
        let pool: &[(&str, &str, [&str; 3])] = match difficulty {
            GeographyDifficulty::Low => &[
                ("Which country has this flag: 🇫🇷?", "France 🇫🇷", ["Italy 🇮🇹", "Spain 🇪🇸", "Germany 🇩🇪"]),
                ("Which country has this flag: 🇯🇵?", "Japan 🇯🇵", ["China 🇨🇳", "South Korea 🇰🇷", "Thailand 🇹🇭"]),
                ("Which country has this flag: 🇺🇸?", "United States 🇺🇸", ["Canada 🇨🇦", "United Kingdom 🇬🇧", "Australia 🇦🇺"]),
                ("Which country has this flag: 🇧🇷?", "Brazil 🇧🇷", ["Argentina 🇦🇷", "Colombia 🇨🇴", "Peru 🇵🇪"]),
                ("Which country has this flag: 🇩🇪?", "Germany 🇩🇪", ["Austria 🇦🇹", "Belgium 🇧🇪", "Netherlands 🇳🇱"]),
                ("Which country has this flag: 🇮🇹?", "Italy 🇮🇹", ["France 🇫🇷", "Spain 🇪🇸", "Greece 🇬🇷"]),
                ("Which country has this flag: 🇬🇧?", "United Kingdom 🇬🇧", ["Ireland 🇮🇪", "Australia 🇦🇺", "New Zealand 🇳🇿"]),
                ("Which country has this flag: 🇲🇽?", "Mexico 🇲🇽", ["Spain 🇪🇸", "Colombia 🇨🇴", "Argentina 🇦🇷"]),
                ("What is the capital of France 🇫🇷?", "Paris", ["Lyon", "Marseille", "Nice"]),
                ("Which continent is Brazil 🇧🇷 in?", "South America", ["North America", "Europe", "Africa"]),
                ("What is the capital of Japan 🇯🇵?", "Tokyo", ["Kyoto", "Osaka", "Yokohama"]),
                ("What is the capital of the United States 🇺🇸?", "Washington, D.C.", ["New York", "Los Angeles", "Chicago"]),
                ("Which continent is Egypt 🇪🇬 in?", "Africa", ["Asia", "Europe", "South America"]),
                ("What is the capital of Italy 🇮🇹?", "Rome", ["Milan", "Venice", "Naples"]),
                ("Which continent is Australia 🇦🇺 in?", "Oceania", ["Europe", "Asia", "Africa"]),
                ("What is the capital of Germany 🇩🇪?", "Berlin", ["Munich", "Frankfurt", "Hamburg"]),
                ("What is the capital of Spain 🇪🇸?", "Madrid", ["Barcelona", "Seville", "Valencia"]),
                ("What is the capital of the United Kingdom 🇬🇧?", "London", ["Edinburgh", "Dublin", "Manchester"]),
            ],
            GeographyDifficulty::Medium => &[
                ("Which country has this flag: 🇨🇦?", "Canada 🇨🇦", ["United States 🇺🇸", "Australia 🇦🇺", "New Zealand 🇳🇿"]),
                ("Which country has this flag: 🇦🇷?", "Argentina 🇦🇷", ["Uruguay 🇺🇾", "Chile 🇨🇱", "Brazil 🇧🇷"]),
                ("Which country has this flag: 🇰🇷?", "South Korea 🇰🇷", ["Japan 🇯🇵", "China 🇨🇳", "Vietnam 🇻🇳"]),
                ("Which country has this flag: 🇪🇬?", "Egypt 🇪🇬", ["Morocco 🇲🇦", "Saudi Arabia 🇸🇦", "Greece 🇬🇷"]),
                ("Which country has this flag: 🇸🇪?", "Sweden 🇸🇪", ["Norway 🇳🇴", "Finland 🇫🇮", "Denmark 🇩🇰"]),
                ("Which country has this flag: 🇬🇷?", "Greece 🇬🇷", ["Italy 🇮🇹", "Turkey 🇹🇷", "Cyprus 🇨🇾"]),
                ("Which country has this flag: 🇮🇳?", "India 🇮🇳", ["Pakistan 🇵🇰", "Bangladesh 🇧🇩", "Sri Lanka 🇱🇰"]),
                ("Which country has this flag: 🇨🇴?", "Colombia 🇨🇴", ["Ecuador 🇪🇨", "Venezuela 🇻🇪", "Bolivia 🇧🇴"]),
                ("What is the capital of Argentina 🇦🇷?", "Buenos Aires", ["Cordoba", "Rosario", "Mendoza"]),
                ("What is the capital of Canada 🇨🇦?", "Ottawa", ["Toronto", "Montreal", "Vancouver"]),
                ("Which continent is India 🇮🇳 in?", "Asia", ["Europe", "Africa", "Oceania"]),
                ("What is the capital of South Korea 🇰🇷?", "Seoul", ["Busan", "Incheon", "Daegu"]),
                ("What is the capital of Mexico 🇲🇽?", "Mexico City", ["Guadalajara", "Monterrey", "Cancun"]),
                ("What is the capital of Greece 🇬🇷?", "Athens", ["Thessaloniki", "Heraklion", "Patras"]),
                ("What is the capital of Sweden 🇸🇪?", "Stockholm", ["Gothenburg", "Malmo", "Uppsala"]),
                ("What is the capital of Thailand 🇹🇭?", "Bangkok", ["Chiang Mai", "Phuket", "Pattaya"]),
                ("What is the capital of Egypt 🇪🇬?", "Cairo", ["Alexandria", "Giza", "Luxor"]),
                ("What is the capital of Norway 🇳🇴?", "Oslo", ["Bergen", "Trondheim", "Stavanger"]),
            ],
            GeographyDifficulty::High => &[
                ("Which country has this flag: 🇦🇺?", "Australia 🇦🇺", ["New Zealand 🇳🇿", "United Kingdom 🇬🇧", "Fiji 🇫🇯"]),
                ("Which country has this flag: 🇨🇭?", "Switzerland 🇨🇭", ["Austria 🇦🇹", "Denmark 🇩🇰", "Sweden 🇸🇪"]),
                ("Which country has this flag: 🇹🇷?", "Turkey 🇹🇷", ["Greece 🇬🇷", "Egypt 🇪🇬", "Tunisia 🇹🇳"]),
                ("Which country has this flag: 🇿🇦?", "South Africa 🇿🇦", ["Kenya 🇰🇪", "Nigeria 🇳🇬", "Zimbabwe 🇿🇼"]),
                ("Which country has this flag: 🇳🇿?", "New Zealand 🇳🇿", ["Australia 🇦🇺", "United Kingdom 🇬🇧", "Iceland 🇮🇸"]),
                ("Which country has this flag: 🇰🇪?", "Kenya 🇰🇪", ["Ethiopia 🇪🇹", "Tanzania 🇹🇿", "Uganda 🇺🇬"]),
                ("Which country has this flag: 🇳🇵?", "Nepal 🇳🇵", ["Bhutan 🇧🇹", "India 🇮🇳", "Myanmar 🇲🇲"]),
                ("Which country has this flag: 🇲🇦?", "Morocco 🇲🇦", ["Algeria 🇩🇿", "Tunisia 🇹🇳", "Egypt 🇪🇬"]),
                ("What is the capital of Australia 🇦🇺?", "Canberra", ["Sydney", "Melbourne", "Brisbane"]),
                ("What is the capital of Brazil 🇧🇷?", "Brasilia", ["Rio de Janeiro", "Sao Paulo", "Salvador"]),
                ("What is the capital of Kazakhstan 🇰🇿?", "Astana", ["Almaty", "Shymkent", "Karaganda"]),
                ("What is the capital of Kenya 🇰🇪?", "Nairobi", ["Mombasa", "Kisumu", "Nakuru"]),
                ("What is the capital of Uruguay 🇺🇾?", "Montevideo", ["Salto", "Ciudad de la Costa", "Paysandu"]),
                ("What is the capital of Madagascar 🇲🇬?", "Antananarivo", ["Toamasina", "Antsirabe", "Mahajanga"]),
                ("What is the capital of Nepal 🇳🇵?", "Kathmandu", ["Pokhara", "Lalitpur", "Bharatpur"]),
                ("What is the capital of Estonia 🇪🇪?", "Tallinn", ["Tartu", "Narva", "Parnu"]),
                ("What is the capital of Morocco 🇲🇦?", "Rabat", ["Casablanca", "Marrakesh", "Fes"]),
                ("What is the capital of Switzerland 🇨🇭?", "Bern", ["Zurich", "Geneva", "Basel"]),
                ("What is the capital of Turkey 🇹🇷?", "Ankara", ["Istanbul", "Izmir", "Bursa"]),
                ("What is the capital of New Zealand 🇳🇿?", "Wellington", ["Auckland", "Christchurch", "Hamilton"]),
            ],
        };

        let idx = (ticks as usize) % pool.len();
        let (prompt, correct, wrong) = pool[idx];

        let mut choices = vec![wrong[0].to_string(), wrong[1].to_string(), wrong[2].to_string()];
        let correct_idx = ((ticks >> 4) % 4) as usize;
        choices.insert(correct_idx, correct.to_string());

        self.geography_question_text = prompt.to_string();
        self.geography_choices = choices;
        self.geography_correct_idx = correct_idx;
        self.geography_feedback = None;
    }

    fn generate_vocab_problem(&mut self) {
        let ticks = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let pool = get_vocab_question_pool(self.settings.language, self.settings.vocab_difficulty);
        if pool.is_empty() {
            return;
        }

        let idx = (ticks as usize) % pool.len();
        let item = &pool[idx];

        let mut choices = vec![
            item.wrong[0].to_string(),
            item.wrong[1].to_string(),
            item.wrong[2].to_string(),
        ];
        let correct_idx = ((ticks >> 4) % 4) as usize;
        choices.insert(correct_idx, item.correct.to_string());

        self.vocab_question_text = item.prompt.to_string();
        self.vocab_choices = choices;
        self.vocab_correct_idx = correct_idx;
        self.vocab_feedback = None;
    }

    fn generate_science_problem(&mut self) {
        let ticks = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let pool = get_science_question_pool(self.settings.language, self.settings.science_difficulty);
        if pool.is_empty() {
            return;
        }

        let idx = (ticks as usize) % pool.len();
        let item = &pool[idx];

        let mut choices = vec![
            item.wrong[0].to_string(),
            item.wrong[1].to_string(),
            item.wrong[2].to_string(),
        ];
        let correct_idx = ((ticks >> 4) % 4) as usize;
        choices.insert(correct_idx, item.correct.to_string());

        self.science_question_text = item.prompt.to_string();
        self.science_choices = choices;
        self.science_correct_idx = correct_idx;
        self.science_explanation = item.explanation.to_string();
        self.science_feedback = None;
    }

    fn draw_circular_timer(
        &self,
        ui: &mut egui::Ui,
        elapsed_sec: f32,
        total_pause_sec: f32,
        min_pause_sec: f32,
    ) {
        let size = 120.0;
        let (rect, _response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        let center = rect.center();
        let painter = ui.painter();

        let outer_radius = size / 2.0 - 6.0;
        let inner_radius = outer_radius - 12.0;

        // Draw background rings
        painter.circle_stroke(
            center,
            outer_radius,
            egui::Stroke::new(3.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15)),
        );
        if min_pause_sec > 0.0 {
            painter.circle_stroke(
                center,
                inner_radius,
                egui::Stroke::new(3.0, egui::Color32::from_rgba_unmultiplied(168, 85, 247, 15)),
            );
        }

        let outer_fraction = (1.0 - (elapsed_sec / total_pause_sec)).clamp(0.0, 1.0);
        let inner_fraction = if min_pause_sec > 0.0 {
            (1.0 - (elapsed_sec / min_pause_sec)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let start_angle = -std::f32::consts::FRAC_PI_2;

        // Draw outer ring (light blue countdown sweep)
        if outer_fraction > 0.0 {
            let outer_end_angle = start_angle + outer_fraction * 2.0 * std::f32::consts::PI;
            let mut points = vec![];
            let steps = 40;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let angle = start_angle + t * (outer_end_angle - start_angle);
                points.push(center + egui::vec2(angle.cos() * outer_radius, angle.sin() * outer_radius));
            }
            for w in points.windows(2) {
                painter.line_segment([w[0], w[1]], egui::Stroke::new(4.0, egui::Color32::from_rgb(56, 189, 248)));
            }
        }

        // Draw inner ring (purple countdown sweep)
        if min_pause_sec > 0.0 && inner_fraction > 0.0 {
            let inner_end_angle = start_angle + inner_fraction * 2.0 * std::f32::consts::PI;
            let mut points = vec![];
            let steps = 40;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let angle = start_angle + t * (inner_end_angle - start_angle);
                points.push(center + egui::vec2(angle.cos() * inner_radius, angle.sin() * inner_radius));
            }
            for w in points.windows(2) {
                painter.line_segment([w[0], w[1]], egui::Stroke::new(4.0, egui::Color32::from_rgb(168, 85, 247)));
            }
        }

        let total_remaining = (total_pause_sec - elapsed_sec).max(0.0) as u32;
        let min_remaining = (min_pause_sec - elapsed_sec).max(0.0) as u32;

        let total_min = total_remaining / 60;
        let total_sec = total_remaining % 60;
        let min_min = min_remaining / 60;
        let min_sec = min_remaining % 60;

        painter.text(
            center - egui::vec2(0.0, 8.0),
            egui::Align2::CENTER_CENTER,
            format!("{:02}:{:02}", total_min, total_sec),
            egui::FontId::proportional(22.0),
            egui::Color32::WHITE,
        );

        let subtext = if min_remaining > 0 {
            format!("Min: {:02}:{:02}", min_min, min_sec)
        } else if min_pause_sec > 0.0 {
            "Min met!".to_string()
        } else {
            "Break".to_string()
        };

        painter.text(
            center + egui::vec2(0.0, 14.0),
            egui::Align2::CENTER_CENTER,
            subtext,
            egui::FontId::proportional(11.0),
            if min_remaining > 0 { egui::Color32::from_rgb(168, 85, 247) } else { egui::Color32::from_rgb(52, 211, 153) },
        );
    }

    fn open_settings(&mut self) {
        self.show_settings = true;
        self.focus_settings_password = true;
        self.settings_password_input.clear();
        self.settings_message = None;
        self.new_play_time = self.settings.play_time_minutes;
        self.new_pause_time = self.settings.pause_time_minutes;
        self.new_warning_time_seconds = self.settings.warning_time_seconds;
        self.new_screensaver_style = self.settings.screensaver_style;
        self.new_enable_logging = self.settings.enable_logging;
        self.new_language = self.settings.language;
        self.new_math_questions_needed = self.settings.math_questions_needed;
        self.new_math_min_pause_percent = self.settings.math_min_pause_percent;
        self.new_math_difficulty = self.settings.math_difficulty;
        self.new_geography_questions_needed = self.settings.geography_questions_needed;
        self.new_geography_min_pause_percent = self.settings.geography_min_pause_percent;
        self.new_geography_difficulty = self.settings.geography_difficulty;
        self.new_vocab_questions_needed = self.settings.vocab_questions_needed;
        self.new_vocab_min_pause_percent = self.settings.vocab_min_pause_percent;
        self.new_vocab_difficulty = self.settings.vocab_difficulty;
        self.new_science_questions_needed = self.settings.science_questions_needed;
        self.new_science_min_pause_percent = self.settings.science_min_pause_percent;
        self.new_science_difficulty = self.settings.science_difficulty;
        self.active_settings_tab = 0;
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
                    win32::play_sound_warning();
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
                    self.transition_to_play(ctx, "timer expired");
                } else if self.settings.screensaver_style == ScreensaverStyle::Math
                    && self.math_solved_count >= self.settings.math_questions_needed
                {
                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.math_min_pause_percent) / 100;
                    if elapsed.as_secs() >= min_dur_sec as u64 {
                        self.transition_to_play(ctx, "math exercises complete & min duration met");
                    }
                } else if self.settings.screensaver_style == ScreensaverStyle::Geography
                    && self.geography_solved_count >= self.settings.geography_questions_needed
                {
                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.geography_min_pause_percent) / 100;
                    if elapsed.as_secs() >= min_dur_sec as u64 {
                        self.transition_to_play(ctx, "geography exercises complete & min duration met");
                    }
                } else if self.settings.screensaver_style == ScreensaverStyle::Vocab
                    && self.vocab_solved_count >= self.settings.vocab_questions_needed
                {
                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.vocab_min_pause_percent) / 100;
                    if elapsed.as_secs() >= min_dur_sec as u64 {
                        self.transition_to_play(ctx, "vocab exercises complete & min duration met");
                    }
                } else if self.settings.screensaver_style == ScreensaverStyle::Science
                    && self.science_solved_count >= self.settings.science_questions_needed
                {
                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.science_min_pause_percent) / 100;
                    if elapsed.as_secs() >= min_dur_sec as u64 {
                        self.transition_to_play(ctx, "science exercises complete & min duration met");
                    }
                }
            }
        }
    }

    fn transition_to_pause(&mut self, ctx: &egui::Context) {
        win32::record_visibility_before_lock();
        win32::capture_foreground_window();
        win32::enable_keyboard_hook();
        self.state = AppState::Pause;
        self.state_start = Instant::now();
        self.password_input.clear();
        self.password_error = None;
        self.show_pause_unblock_panel = false;
        self.focus_password_field = false;
        self.last_pause_interaction = None;

        self.math_solved_count = 0;
        self.math_feedback = None;
        self.generate_math_problem();

        self.geography_solved_count = 0;
        self.geography_feedback = None;
        self.generate_geography_problem();

        self.vocab_solved_count = 0;
        self.vocab_feedback = None;
        self.generate_vocab_problem();

        self.science_solved_count = 0;
        self.science_feedback = None;
        self.generate_science_problem();

        let rect = win32::get_virtual_screen_rect();
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
            rect.width as f32,
            rect.height as f32,
        )));
        win32::make_app_window_fullscreen_topmost();
    }

    fn transition_to_play(&mut self, ctx: &egui::Context, reason: &str) {
        win32::log_to_file(&format!("[HOOK] transition_to_play called. Reason: {}", reason));
        win32::disable_keyboard_hook();
        self.state = AppState::Play;
        self.state_start = Instant::now();
        self.password_input.clear();
        self.password_error = None;
        self.show_pause_unblock_panel = false;
        self.last_pause_interaction = None;

        win32::log_to_file("[DEBUG] transition_to_play: sending Fullscreen(false), Normal level, size 640x560");
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(640.0, 560.0)));

        win32::restore_app_window_normal();
        win32::restore_foreground_window();
        win32::play_sound_info();
    }

    fn try_unblock(&mut self, ctx: &egui::Context) {
        win32::log_to_file(&format!("[DEBUG] try_unblock: input = {:?}", self.password_input));
        if self.settings.verify_password(&self.password_input) {
            self.transition_to_play(ctx, "unlocked via password");
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
                                egui::RichText::new(tr(self.settings.language, "⚠️ SCREEN LOCK WARNING"))
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            );
                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} {:02}:{:02} {}",
                                    tr(self.settings.language, "Screen will lock in"),
                                    remaining_sec / 60,
                                    remaining_sec % 60,
                                    tr(self.settings.language, "minutes")
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
                if self.settings.screensaver_style != ScreensaverStyle::Math
                    && self.settings.screensaver_style != ScreensaverStyle::Geography
                    && self.settings.screensaver_style != ScreensaverStyle::Vocab
                    && self.settings.screensaver_style != ScreensaverStyle::Science
                    && !self.show_pause_unblock_panel
                {
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
                    render_screensaver_style_localized(
                        self.settings.screensaver_style,
                        ui,
                        remaining_sec,
                        self.settings.language,
                    );

                    if self.settings.screensaver_style == ScreensaverStyle::Math {
                        ui.add_space(20.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_unmultiplied(15, 23, 42, 220))
                            .rounding(16.0)
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(168, 85, 247, 100)))
                            .inner_margin(egui::Margin::same(24.0))
                            .show(ui, |ui| {
                                ui.set_max_width(400.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("🧠 Mind Exercises")
                                            .size(24.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(168, 85, 247)),
                                    );
                                    ui.add_space(4.0);
                                    
                                    let total_needed = self.settings.math_questions_needed;
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Solve {} math problems to unlock early ({}/{} solved)",
                                            total_needed,
                                            self.math_solved_count,
                                            total_needed
                                        ))
                                        .size(14.0)
                                        .color(egui::Color32::LIGHT_GRAY),
                                    );
                                    ui.add_space(16.0);

                                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.math_min_pause_percent) / 100;
                                    let elapsed_sec = self.state_start.elapsed().as_secs();

                                    self.draw_circular_timer(
                                        ui,
                                        elapsed_sec as f32,
                                        (self.settings.pause_time_minutes * 60) as f32,
                                        min_dur_sec as f32,
                                    );
                                    ui.add_space(20.0);

                                    if self.math_solved_count >= total_needed {
                                        ui.label(
                                            egui::RichText::new("🎉 All questions solved!")
                                                .size(20.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(52, 211, 153)),
                                        );
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new("Break must continue to meet the minimum required off-game duration.")
                                                .size(14.0)
                                                .color(egui::Color32::YELLOW),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(&self.math_problem_text)
                                                .size(40.0)
                                                .strong()
                                                .color(egui::Color32::WHITE),
                                        );
                                        ui.add_space(16.0);

                                        let mut submit_answer = false;
                                        ui.horizontal(|ui| {
                                            ui.add_space(80.0);
                                            let res = ui.add(
                                                egui::TextEdit::singleline(&mut self.math_user_input)
                                                    .hint_text("Answer...")
                                                    .font(egui::FontId::proportional(20.0))
                                                    .desired_width(120.0),
                                            );
                                            
                                            if self.math_solved_count < total_needed && self.state == AppState::Pause && !self.show_pause_unblock_panel {
                                                res.request_focus();
                                            }

                                            if ui.button(egui::RichText::new("Submit").size(16.0)).clicked() {
                                                submit_answer = true;
                                            }
                                        });

                                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !self.math_user_input.is_empty() {
                                            if !self.show_pause_unblock_panel || !self.focus_password_field {
                                                submit_answer = true;
                                            }
                                        }

                                        if submit_answer {
                                            if let Ok(val) = self.math_user_input.trim().parse::<i32>() {
                                                if val == self.math_problem_answer {
                                                    self.math_solved_count += 1;
                                                    self.math_user_input.clear();
                                                    if self.math_solved_count >= total_needed {
                                                        if elapsed_sec >= min_dur_sec as u64 {
                                                            self.transition_to_play(ctx, "solved math exercises");
                                                        } else {
                                                            self.math_feedback = Some("All questions solved! Waiting for break duration...".to_string());
                                                            self.math_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                        }
                                                    } else {
                                                        self.math_feedback = Some("Correct! Next problem...".to_string());
                                                        self.math_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                        self.generate_math_problem();
                                                    }
                                                } else {
                                                    self.math_feedback = Some("Incorrect answer, try again!".to_string());
                                                    self.math_feedback_color = egui::Color32::from_rgb(248, 113, 113);
                                                    self.math_user_input.clear();
                                                }
                                            } else {
                                                self.math_feedback = Some("Please enter a valid number.".to_string());
                                                self.math_feedback_color = egui::Color32::from_rgb(248, 113, 113);
                                                self.math_user_input.clear();
                                            }
                                        }

                                        if let Some(ref feedback) = self.math_feedback {
                                            ui.add_space(10.0);
                                            ui.label(
                                                egui::RichText::new(feedback)
                                                    .size(14.0)
                                                    .color(self.math_feedback_color),
                                            );
                                        }
                                    }

                                    if !self.show_pause_unblock_panel {
                                        ui.add_space(12.0);
                                        if ui.link("🔑 Use Administrator Password").clicked() {
                                            self.show_pause_unblock_panel = true;
                                            self.focus_password_field = true;
                                        }
                                    }
                                });
                            });
                    }

                    if self.settings.screensaver_style == ScreensaverStyle::Geography {
                        ui.add_space(20.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_unmultiplied(15, 23, 42, 220))
                            .rounding(16.0)
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(56, 189, 248, 100)))
                            .inner_margin(egui::Margin::same(24.0))
                            .show(ui, |ui| {
                                ui.set_max_width(460.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("🌍 World Geography Quiz")
                                            .size(24.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(56, 189, 248)),
                                    );
                                    ui.add_space(4.0);
                                    
                                    let total_needed = self.settings.geography_questions_needed;
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Answer {} geography questions to unlock early ({}/{} solved)",
                                            total_needed,
                                            self.geography_solved_count,
                                            total_needed
                                        ))
                                        .size(14.0)
                                        .color(egui::Color32::LIGHT_GRAY),
                                    );
                                    ui.add_space(16.0);

                                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.geography_min_pause_percent) / 100;
                                    let elapsed_sec = self.state_start.elapsed().as_secs();

                                    self.draw_circular_timer(
                                        ui,
                                        elapsed_sec as f32,
                                        (self.settings.pause_time_minutes * 60) as f32,
                                        min_dur_sec as f32,
                                    );
                                    ui.add_space(20.0);

                                    if self.geography_solved_count >= total_needed {
                                        ui.label(
                                            egui::RichText::new("🎉 All questions solved!")
                                                .size(20.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(52, 211, 153)),
                                        );
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new("Break must continue to meet the minimum required off-game duration.")
                                                .size(14.0)
                                                .color(egui::Color32::YELLOW),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(&self.geography_question_text)
                                                .size(18.0)
                                                .strong()
                                                .color(egui::Color32::WHITE),
                                        );
                                        ui.add_space(16.0);

                                        let mut selected_choice: Option<usize> = None;
                                        egui::Grid::new("geo_choices_grid")
                                            .num_columns(2)
                                            .spacing([12.0, 10.0])
                                            .show(ui, |ui| {
                                                for (idx, choice) in self.geography_choices.iter().enumerate() {
                                                    let btn = ui.add_sized(
                                                        [200.0, 36.0],
                                                        egui::Button::new(
                                                            egui::RichText::new(format!("{}. {}", idx + 1, choice))
                                                                .size(14.0)
                                                                .strong()
                                                        )
                                                    );
                                                    if btn.clicked() {
                                                        selected_choice = Some(idx);
                                                    }
                                                    if idx % 2 == 1 {
                                                        ui.end_row();
                                                    }
                                                }
                                            });

                                        if selected_choice.is_none() {
                                            for idx in 0..self.geography_choices.len() {
                                                let key = match idx {
                                                    0 => egui::Key::Num1,
                                                    1 => egui::Key::Num2,
                                                    2 => egui::Key::Num3,
                                                    3 => egui::Key::Num4,
                                                    _ => egui::Key::Num0,
                                                };
                                                if ui.input(|i| i.key_pressed(key)) && (!self.show_pause_unblock_panel || !self.focus_password_field) {
                                                    selected_choice = Some(idx);
                                                    break;
                                                }
                                            }
                                        }

                                        if let Some(idx) = selected_choice {
                                            if idx == self.geography_correct_idx {
                                                self.geography_solved_count += 1;
                                                if self.geography_solved_count >= total_needed {
                                                    if elapsed_sec >= min_dur_sec as u64 {
                                                        self.transition_to_play(ctx, "solved geography exercises");
                                                    } else {
                                                        self.geography_feedback = Some("Correct! All questions solved! Waiting for break duration...".to_string());
                                                        self.geography_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                    }
                                                } else {
                                                    self.geography_feedback = Some("Correct! Excellent job! Next question...".to_string());
                                                    self.geography_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                    self.generate_geography_problem();
                                                }
                                            } else {
                                                self.geography_feedback = Some("Incorrect choice, try again!".to_string());
                                                self.geography_feedback_color = egui::Color32::from_rgb(248, 113, 113);
                                            }
                                        }

                                        if let Some(ref feedback) = self.geography_feedback {
                                            ui.add_space(10.0);
                                            ui.label(
                                                egui::RichText::new(feedback)
                                                    .size(14.0)
                                                    .color(self.geography_feedback_color),
                                            );
                                        }
                                    }

                                    if !self.show_pause_unblock_panel {
                                        ui.add_space(12.0);
                                        if ui.link("🔑 Use Administrator Password").clicked() {
                                            self.show_pause_unblock_panel = true;
                                            self.focus_password_field = true;
                                        }
                                    }
                                });
                            });
                    }

                    if self.settings.screensaver_style == ScreensaverStyle::Vocab {
                        ui.add_space(20.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_unmultiplied(15, 23, 42, 220))
                            .rounding(16.0)
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(251, 146, 60, 100)))
                            .inner_margin(egui::Margin::same(24.0))
                            .show(ui, |ui| {
                                ui.set_max_width(460.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new(tr(self.settings.language, "vocab_title"))
                                            .size(24.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(251, 146, 60)),
                                    );
                                    ui.add_space(4.0);
                                    
                                    let total_needed = self.settings.vocab_questions_needed;
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Answer {} vocabulary & spelling questions to unlock early ({}/{} solved)",
                                            total_needed,
                                            self.vocab_solved_count,
                                            total_needed
                                        ))
                                        .size(14.0)
                                        .color(egui::Color32::LIGHT_GRAY),
                                    );
                                    ui.add_space(16.0);

                                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.vocab_min_pause_percent) / 100;
                                    let elapsed_sec = self.state_start.elapsed().as_secs();

                                    self.draw_circular_timer(
                                        ui,
                                        elapsed_sec as f32,
                                        (self.settings.pause_time_minutes * 60) as f32,
                                        min_dur_sec as f32,
                                    );
                                    ui.add_space(20.0);

                                    if self.vocab_solved_count >= total_needed {
                                        ui.label(
                                            egui::RichText::new("🎉 All questions solved!")
                                                .size(20.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(52, 211, 153)),
                                        );
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new("Break must continue to meet the minimum required off-game duration.")
                                                .size(14.0)
                                                .color(egui::Color32::YELLOW),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(&self.vocab_question_text)
                                                .size(18.0)
                                                .strong()
                                                .color(egui::Color32::WHITE),
                                        );
                                        ui.add_space(16.0);

                                        let mut selected_choice: Option<usize> = None;
                                        egui::Grid::new("vocab_choices_grid")
                                            .num_columns(2)
                                            .spacing([12.0, 10.0])
                                            .show(ui, |ui| {
                                                for (idx, choice) in self.vocab_choices.iter().enumerate() {
                                                    let btn = ui.add_sized(
                                                        [200.0, 36.0],
                                                        egui::Button::new(
                                                            egui::RichText::new(format!("{}. {}", idx + 1, choice))
                                                                .size(14.0)
                                                                .strong()
                                                        )
                                                    );
                                                    if btn.clicked() {
                                                        selected_choice = Some(idx);
                                                    }
                                                    if idx % 2 == 1 {
                                                        ui.end_row();
                                                    }
                                                }
                                            });

                                        if selected_choice.is_none() {
                                            for idx in 0..self.vocab_choices.len() {
                                                let key = match idx {
                                                    0 => egui::Key::Num1,
                                                    1 => egui::Key::Num2,
                                                    2 => egui::Key::Num3,
                                                    3 => egui::Key::Num4,
                                                    _ => egui::Key::Num0,
                                                };
                                                if ui.input(|i| i.key_pressed(key)) && (!self.show_pause_unblock_panel || !self.focus_password_field) {
                                                    selected_choice = Some(idx);
                                                    break;
                                                }
                                            }
                                        }

                                        if let Some(idx) = selected_choice {
                                            if idx == self.vocab_correct_idx {
                                                self.vocab_solved_count += 1;
                                                if self.vocab_solved_count >= total_needed {
                                                    if elapsed_sec >= min_dur_sec as u64 {
                                                        self.transition_to_play(ctx, "solved vocab exercises");
                                                    } else {
                                                        self.vocab_feedback = Some("Correct! All questions solved! Waiting for break duration...".to_string());
                                                        self.vocab_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                    }
                                                } else {
                                                    self.vocab_feedback = Some(tr(self.settings.language, "correct_feedback").to_string());
                                                    self.vocab_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                    self.generate_vocab_problem();
                                                }
                                            } else {
                                                self.vocab_feedback = Some(tr(self.settings.language, "incorrect_feedback").to_string());
                                                self.vocab_feedback_color = egui::Color32::from_rgb(248, 113, 113);
                                            }
                                        }

                                        if let Some(ref feedback) = self.vocab_feedback {
                                            ui.add_space(10.0);
                                            ui.label(
                                                egui::RichText::new(feedback)
                                                    .size(14.0)
                                                    .color(self.vocab_feedback_color),
                                            );
                                        }
                                    }

                                    if !self.show_pause_unblock_panel {
                                        ui.add_space(12.0);
                                        if ui.link("🔑 Use Administrator Password").clicked() {
                                            self.show_pause_unblock_panel = true;
                                            self.focus_password_field = true;
                                        }
                                    }
                                });
                            });
                    }

                    if self.settings.screensaver_style == ScreensaverStyle::Science {
                        ui.add_space(20.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_unmultiplied(10, 22, 40, 230))
                            .rounding(16.0)
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgba_unmultiplied(34, 211, 238, 120)))
                            .inner_margin(egui::Margin::same(24.0))
                            .show(ui, |ui| {
                                ui.set_max_width(480.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new(tr(self.settings.language, "science_title"))
                                            .size(24.0)
                                            .strong()
                                            .color(egui::Color32::from_rgb(34, 211, 238)),
                                    );
                                    ui.add_space(4.0);
                                    
                                    let total_needed = self.settings.science_questions_needed;
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Answer {} STEM trivia questions to unlock early ({}/{} solved)",
                                            total_needed,
                                            self.science_solved_count,
                                            total_needed
                                        ))
                                        .size(14.0)
                                        .color(egui::Color32::LIGHT_GRAY),
                                    );
                                    ui.add_space(16.0);

                                    let min_dur_sec = ((self.settings.pause_time_minutes * 60) * self.settings.science_min_pause_percent) / 100;
                                    let elapsed_sec = self.state_start.elapsed().as_secs();

                                    self.draw_circular_timer(
                                        ui,
                                        elapsed_sec as f32,
                                        (self.settings.pause_time_minutes * 60) as f32,
                                        min_dur_sec as f32,
                                    );
                                    ui.add_space(20.0);

                                    if self.science_solved_count >= total_needed {
                                        ui.label(
                                            egui::RichText::new("🎉 All STEM questions solved!")
                                                .size(20.0)
                                                .strong()
                                                .color(egui::Color32::from_rgb(52, 211, 153)),
                                        );
                                        ui.add_space(8.0);
                                        ui.label(
                                            egui::RichText::new("Break must continue to meet the minimum required off-game duration.")
                                                .size(14.0)
                                                .color(egui::Color32::YELLOW),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(&self.science_question_text)
                                                .size(18.0)
                                                .strong()
                                                .color(egui::Color32::WHITE),
                                        );
                                        ui.add_space(16.0);

                                        let mut selected_choice: Option<usize> = None;
                                        egui::Grid::new("science_choices_grid")
                                            .num_columns(2)
                                            .spacing([12.0, 10.0])
                                            .show(ui, |ui| {
                                                for (idx, choice) in self.science_choices.iter().enumerate() {
                                                    let btn = ui.add_sized(
                                                        [210.0, 36.0],
                                                        egui::Button::new(
                                                            egui::RichText::new(format!("{}. {}", idx + 1, choice))
                                                                .size(14.0)
                                                                .strong()
                                                        )
                                                    );
                                                    if btn.clicked() {
                                                        selected_choice = Some(idx);
                                                    }
                                                    if idx % 2 == 1 {
                                                        ui.end_row();
                                                    }
                                                }
                                            });

                                        if selected_choice.is_none() {
                                            for idx in 0..self.science_choices.len() {
                                                let key = match idx {
                                                    0 => egui::Key::Num1,
                                                    1 => egui::Key::Num2,
                                                    2 => egui::Key::Num3,
                                                    3 => egui::Key::Num4,
                                                    _ => egui::Key::Num0,
                                                };
                                                if ui.input(|i| i.key_pressed(key)) && (!self.show_pause_unblock_panel || !self.focus_password_field) {
                                                    selected_choice = Some(idx);
                                                    break;
                                                }
                                            }
                                        }

                                        if let Some(idx) = selected_choice {
                                            if idx == self.science_correct_idx {
                                                self.science_solved_count += 1;
                                                if self.science_solved_count >= total_needed {
                                                    if elapsed_sec >= min_dur_sec as u64 {
                                                        self.transition_to_play(ctx, "solved science exercises");
                                                    } else {
                                                        self.science_feedback = Some("Correct! All questions solved! Waiting for break duration...".to_string());
                                                        self.science_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                    }
                                                } else {
                                                    self.science_feedback = Some(tr(self.settings.language, "correct_feedback").to_string());
                                                    self.science_feedback_color = egui::Color32::from_rgb(52, 211, 153);
                                                    self.generate_science_problem();
                                                }
                                            } else {
                                                self.science_feedback = Some(tr(self.settings.language, "incorrect_feedback").to_string());
                                                self.science_feedback_color = egui::Color32::from_rgb(248, 113, 113);
                                            }
                                        }

                                        if let Some(ref feedback) = self.science_feedback {
                                            ui.add_space(10.0);
                                            ui.label(
                                                egui::RichText::new(feedback)
                                                    .size(14.0)
                                                    .color(self.science_feedback_color),
                                            );
                                        }

                                        if !self.science_explanation.is_empty() {
                                            ui.add_space(6.0);
                                            ui.label(
                                                egui::RichText::new(&self.science_explanation)
                                                    .size(13.0)
                                                    .italics()
                                                    .color(egui::Color32::from_rgb(34, 211, 238)),
                                            );
                                        }
                                    }

                                    if !self.show_pause_unblock_panel {
                                        ui.add_space(12.0);
                                        if ui.link(tr(self.settings.language, "🔑 Use Administrator Password")).clicked() {
                                            self.show_pause_unblock_panel = true;
                                            self.focus_password_field = true;
                                        }
                                    }
                                });
                            });
                    }

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
                                    egui::RichText::new(tr(self.settings.language, "Enter administrator password to unblock immediately:"))
                                        .size(16.0)
                                        .color(egui::Color32::LIGHT_GRAY),
                                );
                                ui.add_space(8.0);

                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.password_input)
                                        .password(true)
                                        .hint_text(tr(self.settings.language, "Password..."))
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

                                if ui.input(|i| i.key_pressed(egui::Key::Enter)) || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                    self.try_unblock(ctx);
                                }

                                ui.add_space(12.0);

                                if ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new(format!("🔓 {}", tr(self.settings.language, "Unlock Computer")))
                                                .size(16.0)
                                                .strong(),
                                        )
                                        .min_size(egui::vec2(200.0, 40.0)),
                                    )
                                    .clicked()
                                {
                                    self.try_unblock(ctx);
                                }

                                if let Some(ref _err) = self.password_error {
                                    ui.add_space(8.0);
                                    ui.label(
                                        egui::RichText::new(tr(self.settings.language, "Incorrect password. Please try again."))
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
        egui::Window::new(tr(self.settings.language, "Reset Timer Confirmation"))
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .fixed_size(egui::vec2(340.0, 180.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label(tr(self.settings.language, "Enter password to reset the play timer back to zero:"));
                ui.add_space(8.0);

                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.reset_password_input)
                        .password(true)
                        .hint_text(tr(self.settings.language, "Password...")),
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
                    if ui.button(tr(self.settings.language, "Confirm")).clicked() {
                        self.try_reset_timer();
                    }
                    if ui.button(tr(self.settings.language, "Cancel")).clicked() {
                        self.show_reset_dialog = false;
                        self.reset_password_input.clear();
                        self.reset_error_message = None;
                    }
                });

                if let Some(ref _err) = self.reset_error_message {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(tr(self.settings.language, "Invalid password.")).color(egui::Color32::LIGHT_RED));
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
        egui::Window::new(tr(self.settings.language, "⚙ Interrupt Settings"))
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .fixed_size(egui::vec2(480.0, 460.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(420.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if !self.settings_unlocked {
                            ui.heading(tr(self.settings.language, "Authentication Required"));
                            ui.label(tr(self.settings.language, "Enter current password or master password to access settings:"));
                            ui.add_space(8.0);

                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.settings_password_input)
                                    .password(true)
                                    .hint_text(tr(self.settings.language, "Password...")),
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
                                        Some(tr(self.settings.language, "Invalid password authentication.").to_string());
                                }
                            }

                            if ui.button(tr(self.settings.language, "Unlock Settings")).clicked() {
                                if self.settings.verify_password(&self.settings_password_input) {
                                    self.settings_unlocked = true;
                                    self.settings_password_input.clear();
                                    self.settings_message = None;
                                } else {
                                    self.settings_message =
                                        Some(tr(self.settings.language, "Invalid password authentication.").to_string());
                                }
                            }

                            if let Some(ref msg) = self.settings_message {
                                ui.label(egui::RichText::new(msg).color(egui::Color32::LIGHT_RED));
                            }
                        } else {
                            ui.heading(tr(self.settings.language, "Configure Settings"));
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(tr(self.settings.language, "⏸ Timer suspended while settings window is open"))
                                    .color(egui::Color32::YELLOW)
                                    .size(13.0),
                            );
                            ui.add_space(8.0);

                            // Tab Selector
                            ui.horizontal(|ui| {
                                ui.selectable_value(&mut self.active_settings_tab, 0, tr(self.settings.language, "📅 Break Cycles"));
                                ui.selectable_value(&mut self.active_settings_tab, 1, tr(self.settings.language, "🔒 Lock Screen Settings"));
                            });
                            ui.add_space(10.0);

                            if self.active_settings_tab == 0 {
                                egui::Grid::new("settings_grid")
                                    .num_columns(2)
                                    .spacing([16.0, 10.0])
                                    .show(ui, |ui| {
                                        ui.label(tr(self.settings.language, "Interface Language:"));
                                        egui::ComboBox::from_id_source("language_selector")
                                            .selected_text(self.new_language.name())
                                            .width(180.0)
                                            .show_ui(ui, |ui| {
                                                for lang in Language::all() {
                                                    ui.selectable_value(
                                                        &mut self.new_language,
                                                        *lang,
                                                        lang.name(),
                                                    );
                                                }
                                            });
                                        ui.end_row();

                                        ui.label(tr(self.settings.language, "Play Time (minutes):"));
                                        ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_play_time).range(1..=300));
                                        ui.end_row();

                                        ui.label(tr(self.settings.language, "Pause Time (minutes):"));
                                        ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_pause_time).range(1..=60));
                                        ui.end_row();

                                        ui.label(tr(self.settings.language, "Warning Time (seconds):"));
                                        ui.add_sized(
                                            [180.0, 22.0],
                                            egui::DragValue::new(&mut self.new_warning_time_seconds).range(5..=300),
                                        );
                                        ui.end_row();

                                        ui.label(tr(self.settings.language, "Screensaver Style:"));
                                        egui::ComboBox::from_id_source("screensaver_style_selector")
                                            .selected_text(self.new_screensaver_style.name_localized(self.new_language))
                                            .width(180.0)
                                            .show_ui(ui, |ui| {
                                                for style in ScreensaverStyle::all() {
                                                    ui.selectable_value(
                                                        &mut self.new_screensaver_style,
                                                        *style,
                                                        style.name_localized(self.new_language),
                                                    );
                                                }
                                            });
                                        ui.end_row();

                                        ui.label(tr(self.settings.language, "New Password (optional):"));
                                        ui.add_sized(
                                            [180.0, 22.0],
                                            egui::TextEdit::singleline(&mut self.new_password_input)
                                                .password(true),
                                        );
                                        ui.end_row();

                                        ui.label(tr(self.settings.language, "Enable Debug Logging:"));
                                        ui.checkbox(&mut self.new_enable_logging, "");
                                        ui.end_row();
                                    });
                            } else {
                                ui.heading(format!("{}: {}", tr(self.settings.language, "Style:"), self.new_screensaver_style.name_localized(self.new_language)));
                                ui.add_space(8.0);

                                match self.new_screensaver_style {
                                    ScreensaverStyle::Math => {
                                        egui::Grid::new("math_settings_grid")
                                            .num_columns(2)
                                            .spacing([16.0, 10.0])
                                            .show(ui, |ui| {
                                                ui.label(tr(self.settings.language, "Questions to Solve:"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_math_questions_needed).range(1..=20));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Min Break Duration (%):"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_math_min_pause_percent).range(30..=100));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Difficulty:"));
                                                egui::ComboBox::from_id_source("math_difficulty_selector")
                                                    .selected_text(self.new_math_difficulty.name_localized(self.new_language))
                                                    .width(180.0)
                                                    .show_ui(ui, |ui| {
                                                        for diff in MathDifficulty::all() {
                                                            ui.selectable_value(
                                                                &mut self.new_math_difficulty,
                                                                *diff,
                                                                diff.name_localized(self.new_language),
                                                            );
                                                        }
                                                    });
                                                ui.end_row();
                                            });
                                    }
                                    ScreensaverStyle::Geography => {
                                        egui::Grid::new("geography_settings_grid")
                                            .num_columns(2)
                                            .spacing([16.0, 10.0])
                                            .show(ui, |ui| {
                                                ui.label(tr(self.settings.language, "Questions to Solve:"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_geography_questions_needed).range(1..=20));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Min Break Duration (%):"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_geography_min_pause_percent).range(30..=100));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Difficulty:"));
                                                egui::ComboBox::from_id_source("geography_difficulty_selector")
                                                    .selected_text(self.new_geography_difficulty.name_localized(self.new_language))
                                                    .width(180.0)
                                                    .show_ui(ui, |ui| {
                                                        for diff in GeographyDifficulty::all() {
                                                            ui.selectable_value(
                                                                &mut self.new_geography_difficulty,
                                                                *diff,
                                                                diff.name_localized(self.new_language),
                                                            );
                                                        }
                                                    });
                                                ui.end_row();
                                            });
                                    }
                                    ScreensaverStyle::Vocab => {
                                        egui::Grid::new("vocab_settings_grid")
                                            .num_columns(2)
                                            .spacing([16.0, 10.0])
                                            .show(ui, |ui| {
                                                ui.label(tr(self.settings.language, "Questions to Solve:"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_vocab_questions_needed).range(1..=20));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Min Break Duration (%):"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_vocab_min_pause_percent).range(30..=100));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Difficulty:"));
                                                egui::ComboBox::from_id_source("vocab_difficulty_selector")
                                                    .selected_text(self.new_vocab_difficulty.name_localized(self.new_language))
                                                    .width(180.0)
                                                    .show_ui(ui, |ui| {
                                                        for diff in VocabDifficulty::all() {
                                                            ui.selectable_value(
                                                                &mut self.new_vocab_difficulty,
                                                                *diff,
                                                                diff.name_localized(self.new_language),
                                                            );
                                                        }
                                                    });
                                                ui.end_row();
                                            });
                                    }
                                    ScreensaverStyle::Science => {
                                        egui::Grid::new("science_settings_grid")
                                            .num_columns(2)
                                            .spacing([16.0, 10.0])
                                            .show(ui, |ui| {
                                                ui.label(tr(self.settings.language, "Questions to Solve:"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_science_questions_needed).range(1..=20));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Min Break Duration (%):"));
                                                ui.add_sized([180.0, 22.0], egui::DragValue::new(&mut self.new_science_min_pause_percent).range(30..=100));
                                                ui.end_row();

                                                ui.label(tr(self.settings.language, "Difficulty:"));
                                                egui::ComboBox::from_id_source("science_difficulty_selector")
                                                    .selected_text(self.new_science_difficulty.name_localized(self.new_language))
                                                    .width(180.0)
                                                    .show_ui(ui, |ui| {
                                                        for diff in ScienceDifficulty::all() {
                                                            ui.selectable_value(
                                                                &mut self.new_science_difficulty,
                                                                *diff,
                                                                diff.name_localized(self.new_language),
                                                            );
                                                        }
                                                    });
                                                ui.end_row();
                                            });
                                    }
                                    _ => {
                                        ui.label(egui::RichText::new(tr(self.settings.language, "This screensaver style has no custom configuration parameters."))
                                            .color(egui::Color32::LIGHT_GRAY));
                                    }
                                }
                            }

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                if ui.button(tr(self.settings.language, "save_settings")).clicked() {
                                    self.settings.play_time_minutes = self.new_play_time;
                                    self.settings.pause_time_minutes = self.new_pause_time;
                                    self.settings.warning_time_seconds = self.new_warning_time_seconds;
                                    self.settings.screensaver_style = self.new_screensaver_style;
                                    self.settings.enable_logging = self.new_enable_logging;
                                    self.settings.language = self.new_language;
                                    self.settings.math_questions_needed = self.new_math_questions_needed;
                                    self.settings.math_min_pause_percent = self.new_math_min_pause_percent;
                                    self.settings.math_difficulty = self.new_math_difficulty;
                                    self.settings.geography_questions_needed = self.new_geography_questions_needed;
                                    self.settings.geography_min_pause_percent = self.new_geography_min_pause_percent;
                                    self.settings.geography_difficulty = self.new_geography_difficulty;
                                    self.settings.vocab_questions_needed = self.new_vocab_questions_needed;
                                    self.settings.vocab_min_pause_percent = self.new_vocab_min_pause_percent;
                                    self.settings.vocab_difficulty = self.new_vocab_difficulty;
                                    self.settings.science_questions_needed = self.new_science_questions_needed;
                                    self.settings.science_min_pause_percent = self.new_science_min_pause_percent;
                                    self.settings.science_difficulty = self.new_science_difficulty;
                                    win32::init_logging(self.new_enable_logging);
                                    if !self.new_password_input.trim().is_empty() {
                                        self.settings.set_password(&self.new_password_input);
                                        self.new_password_input.clear();
                                    }
                                    if let Err(e) = self.settings.save() {
                                        self.settings_message = Some(format!("Failed to save: {}", e));
                                    } else {
                                        self.settings_message =
                                            Some(tr(self.settings.language, "settings_saved").to_string());
                                        self.close_settings();
                                    }
                                }
                            });

                            if let Some(ref msg) = self.settings_message {
                                ui.add_space(8.0);
                                ui.label(egui::RichText::new(msg).color(egui::Color32::GREEN));
                            }
                        }
                    });
            });

        if !open {
            self.close_settings();
        }
    }
}

impl eframe::App for InterruptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.tray_registered {
            win32::register_tray_icon();
            self.tray_registered = true;
        }

        // Poll native tray commands from subclassed window procedure
        let tray_cmd = win32::poll_pending_tray_command();
        if tray_cmd > 0 {
            win32::log_to_file(&format!("[LOG] Received native tray command: {}", tray_cmd));
            if tray_cmd == 1001 {
                win32::show_app_window(true);
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            } else if tray_cmd == 1002 {
                if self.state == AppState::Play || self.state == AppState::Warning {
                    self.transition_to_pause(ctx);
                }
            } else if tray_cmd == 1003 {
                self.should_exit = true;
                win32::SHOULD_EXIT.store(true, std::sync::atomic::Ordering::SeqCst);
                win32::unregister_tray_icon();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }

        // Intercept close request from window title bar or Alt+F4 to minimize to tray
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.should_exit {
                // Allow eframe process exit to take place
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                win32::show_app_window(false);
            }
        }

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

            egui::TopBottomPanel::top("top_panel")
                .frame(egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30, 41, 59))
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 65, 85)))
                )
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new("⏱️ Interrupt")
                                .strong()
                                .color(egui::Color32::from_rgb(241, 245, 249))
                        );
                        
                        ui.add_space(8.0);
                        
                        let mode_name = tr(self.settings.language, "Status: Play Mode");
                        let status_text = if self.show_settings {
                            format!(
                                "{} [PAUSED] • {:02}:{:02}",
                                mode_name,
                                remaining_sec / 60,
                                remaining_sec % 60
                            )
                        } else {
                            format!(
                                "{} • {:02}:{:02}",
                                mode_name,
                                remaining_sec / 60,
                                remaining_sec % 60
                            )
                        };

                        ui.label(
                            egui::RichText::new(status_text)
                                .color(egui::Color32::from_rgb(148, 163, 184))
                                .size(13.0)
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let settings_btn = ui.add(
                                egui::Button::new(egui::RichText::new(tr(self.settings.language, "⚙️ Settings")).strong())
                                    .min_size(egui::vec2(90.0, 28.0))
                            );
                            if settings_btn.clicked() {
                                self.open_settings();
                            }
                        });
                    });
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(16.0);
                    
                    ui.label(
                        egui::RichText::new(tr(self.settings.language, "Interrupt Screen Time Manager"))
                            .size(22.0)
                            .strong()
                            .color(egui::Color32::WHITE)
                    );
                    
                    let sub_label = if self.show_settings {
                        tr(self.settings.language, "Settings open — timer suspended until settings window is closed.")
                    } else {
                        tr(self.settings.language, "Active cycle running. Time remaining until next screen lock:")
                    };
                    ui.label(
                        egui::RichText::new(sub_label)
                            .color(egui::Color32::from_rgb(148, 163, 184))
                            .size(13.0)
                    );
                    ui.add_space(12.0);

                    // Large Card for Timer & Progress
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(30, 41, 59))
                        .rounding(16.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(71, 85, 105)))
                        .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                        .show(ui, |ui| {
                            ui.set_max_width(420.0);
                            ui.vertical_centered(|ui| {
                                // Timer
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{:02}:{:02}",
                                        remaining_sec / 60,
                                        remaining_sec % 60
                                    ))
                                    .size(64.0)
                                    .color(if self.show_settings {
                                        egui::Color32::from_rgb(234, 179, 8) // amber-500
                                    } else if self.state == AppState::Warning {
                                        egui::Color32::from_rgb(239, 68, 68) // red-500
                                    } else {
                                        egui::Color32::from_rgb(56, 189, 248) // sky-400
                                    })
                                    .monospace()
                                    .strong(),
                                );
                                
                                ui.add_space(8.0);
                                
                                // Progress bar
                                let total_play_sec = (self.settings.play_time_minutes * 60) as f32;
                                let remaining_f32 = remaining_sec as f32;
                                let progress_fraction = if total_play_sec > 0.0 {
                                    (remaining_f32 / total_play_sec).clamp(0.0, 1.0)
                                } else {
                                    0.0
                                };
                                
                                let progress_color = if self.show_settings {
                                    egui::Color32::from_rgb(234, 179, 8)
                                } else if self.state == AppState::Warning {
                                    egui::Color32::from_rgb(239, 68, 68)
                                } else {
                                    egui::Color32::from_rgb(99, 102, 241)
                                };
                                
                                ui.add(
                                    egui::ProgressBar::new(progress_fraction)
                                        .show_percentage()
                                        .fill(progress_color)
                                );
                            });
                        });
                        
                    ui.add_space(16.0);
                    
                    // Main action buttons
                    ui.horizontal(|ui| {
                        ui.add_space(76.0);
                        
                        // Primary button
                        let lock_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new(tr(self.settings.language, "🔒 Lock Now")).size(16.0).strong().color(egui::Color32::WHITE)
                            )
                            .fill(egui::Color32::from_rgb(79, 70, 229)) // indigo 600
                            .min_size(egui::vec2(160.0, 40.0))
                        );
                        if lock_btn.clicked() {
                            self.transition_to_pause(ctx);
                        }
                        
                        ui.add_space(16.0);
                        
                        // Secondary button
                        let reset_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new(tr(self.settings.language, "🔄 Reset Timer")).size(16.0).strong().color(egui::Color32::WHITE)
                            )
                            .fill(egui::Color32::from_rgb(51, 65, 85)) // slate 700
                            .min_size(egui::vec2(160.0, 40.0))
                        );
                        if reset_btn.clicked() {
                            self.open_reset_dialog();
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    // Info pills / badges
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                        
                        let info_items = vec![
                            (format!("{}: {} {}", tr(self.settings.language, "Play:"), self.settings.play_time_minutes, tr(self.settings.language, "min")), egui::Color32::from_rgb(56, 189, 248)),
                            (format!("{}: {} {}", tr(self.settings.language, "Break:"), self.settings.pause_time_minutes, tr(self.settings.language, "min")), egui::Color32::from_rgb(168, 85, 247)),
                            (format!("Warn: {}s", self.settings.warning_time_seconds), egui::Color32::from_rgb(244, 63, 94)),
                            (format!("{}: {}", tr(self.settings.language, "Style:"), self.settings.screensaver_style.name_localized(self.settings.language).split(' ').next().unwrap_or("")), egui::Color32::from_rgb(52, 211, 153)),
                            ("Master PW: On".to_string(), egui::Color32::from_rgb(148, 163, 184)),
                        ];
                        
                        ui.add_space(24.0); // Center adjustment
                        for (text, color) in info_items {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(30, 41, 59))
                                .rounding(6.0)
                                .stroke(egui::Stroke::new(1.0, color))
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(text)
                                            .size(12.0)
                                            .color(egui::Color32::WHITE)
                                    );
                                });
                        }
                    });
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
 
impl Drop for InterruptApp {
    fn drop(&mut self) {
        win32::log_to_file("[LOG] InterruptApp::drop() called - cleaning up tray icon");
        win32::unregister_tray_icon();
    }
}

fn main() -> eframe::Result<()> {
    win32::log_to_file("[LOG] main: Starting eframe application");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Interrupt - Healthy Screen Breaks")
            .with_inner_size([640.0, 560.0])
            .with_resizable(false),
        ..Default::default()
    };

    let res = eframe::run_native(
        "Interrupt",
        options,
        Box::new(|cc| Ok(Box::new(InterruptApp::new(cc)))),
    );
    win32::log_to_file(&format!("[LOG] main: eframe::run_native returned: {:?}", res));
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{GeographyDifficulty, VocabDifficulty};
    use i18n::Language;

    fn create_test_app(settings: AppSettings) -> InterruptApp {
        InterruptApp {
            settings: settings.clone(),
            state: AppState::Play,
            state_start: Instant::now(),
            password_input: String::new(),
            settings_password_input: String::new(),
            password_error: None,
            settings_unlocked: false,
            show_settings: false,
            new_play_time: settings.play_time_minutes,
            new_pause_time: settings.pause_time_minutes,
            new_warning_time_seconds: settings.warning_time_seconds,
            new_screensaver_style: settings.screensaver_style,
            new_password_input: String::new(),
            new_enable_logging: settings.enable_logging,
            settings_message: None,
            focus_password_field: false,
            focus_settings_password: false,
            focus_reset_password: false,
            show_reset_dialog: false,
            reset_password_input: String::new(),
            reset_error_message: None,
            show_pause_unblock_panel: false,
            last_pause_interaction: None,
            tray_registered: false,
            should_exit: false,
            math_problem_text: String::new(),
            math_problem_answer: 0,
            math_user_input: String::new(),
            math_solved_count: 0,
            math_feedback: None,
            math_feedback_color: egui::Color32::LIGHT_GRAY,
            geography_question_text: String::new(),
            geography_choices: Vec::new(),
            geography_correct_idx: 0,
            geography_solved_count: 0,
            geography_feedback: None,
            geography_feedback_color: egui::Color32::LIGHT_GRAY,
            vocab_question_text: String::new(),
            vocab_choices: Vec::new(),
            vocab_correct_idx: 0,
            vocab_solved_count: 0,
            vocab_feedback: None,
            vocab_feedback_color: egui::Color32::LIGHT_GRAY,
            science_question_text: String::new(),
            science_choices: Vec::new(),
            science_correct_idx: 0,
            science_explanation: String::new(),
            science_solved_count: 0,
            science_feedback: None,
            science_feedback_color: egui::Color32::LIGHT_GRAY,
            active_settings_tab: 0,
            new_language: settings.language,
            new_math_questions_needed: settings.math_questions_needed,
            new_math_min_pause_percent: settings.math_min_pause_percent,
            new_math_difficulty: settings.math_difficulty,
            new_geography_questions_needed: settings.geography_questions_needed,
            new_geography_min_pause_percent: settings.geography_min_pause_percent,
            new_geography_difficulty: settings.geography_difficulty,
            new_vocab_questions_needed: settings.vocab_questions_needed,
            new_vocab_min_pause_percent: settings.vocab_min_pause_percent,
            new_vocab_difficulty: settings.vocab_difficulty,
            new_science_questions_needed: settings.science_questions_needed,
            new_science_min_pause_percent: settings.science_min_pause_percent,
            new_science_difficulty: settings.science_difficulty,
        }
    }

    #[test]
    fn test_geography_problem_flag_generation() {
        let settings = AppSettings::default();
        let mut app = create_test_app(settings);

        for difficulty in &[GeographyDifficulty::Low, GeographyDifficulty::Medium, GeographyDifficulty::High] {
            app.settings.geography_difficulty = *difficulty;
            app.generate_geography_problem();

            assert!(!app.geography_question_text.is_empty(), "Geography question text should not be empty");
            assert_eq!(app.geography_choices.len(), 4, "Geography problem should have exactly 4 choices");
            assert!(app.geography_correct_idx < 4, "Correct index should be within choices bounds");

            // Verify that the prompt or choices contain flag emojis / unicode flag badges
            let has_flag_in_prompt = app.geography_question_text.chars().any(|c| c >= '\u{1F1E6}' && c <= '\u{1F1FF}');
            let has_flag_in_choices = app.geography_choices.iter().any(|choice| {
                choice.chars().any(|c| c >= '\u{1F1E6}' && c <= '\u{1F1FF}')
            });

            assert!(
                has_flag_in_prompt || has_flag_in_choices,
                "Geography problem for difficulty {:?} should incorporate flag indicators/emojis in prompt or choices",
                difficulty
            );
        }
    }

    #[test]
    fn test_vocab_problem_generation_multilingual() {
        let settings = AppSettings::default();
        let mut app = create_test_app(settings);

        for lang in &[Language::English, Language::Spanish] {
            app.settings.language = *lang;
            for difficulty in &[VocabDifficulty::Low, VocabDifficulty::Medium, VocabDifficulty::High] {
                app.settings.vocab_difficulty = *difficulty;
                app.generate_vocab_problem();

                assert!(!app.vocab_question_text.is_empty(), "Vocab question text should not be empty");
                assert_eq!(app.vocab_choices.len(), 4, "Vocab problem should have exactly 4 choices");
                assert!(app.vocab_correct_idx < 4, "Correct index should be within choices bounds");
            }
        }
    }

    #[test]
    fn test_science_problem_generation_multilingual() {
        let settings = AppSettings::default();
        let mut app = create_test_app(settings);

        for lang in &[Language::English, Language::Spanish] {
            app.settings.language = *lang;
            for difficulty in &[ScienceDifficulty::Low, ScienceDifficulty::Medium, ScienceDifficulty::High] {
                app.settings.science_difficulty = *difficulty;
                app.generate_science_problem();

                assert!(!app.science_question_text.is_empty(), "Science question text should not be empty");
                assert_eq!(app.science_choices.len(), 4, "Science problem should have exactly 4 choices");
                assert!(app.science_correct_idx < 4, "Correct index should be within choices bounds");
                assert!(!app.science_explanation.is_empty(), "Science explanation should not be empty");
            }
        }
    }
}


