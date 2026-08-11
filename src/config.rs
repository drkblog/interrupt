use crate::i18n::Language;
use crate::screensaver::ScreensaverStyle;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub const MASTER_PASSWORD: &str = "mindfulness";

fn default_warning_time_seconds() -> u32 {
    30
}

fn default_math_questions_needed() -> u32 {
    3
}

fn default_math_min_pause_percent() -> u32 {
    50
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathDifficulty {
    Low,
    Medium,
    High,
}

impl Default for MathDifficulty {
    fn default() -> Self {
        MathDifficulty::Medium
    }
}

impl MathDifficulty {
    pub fn all() -> &'static [MathDifficulty] {
        &[MathDifficulty::Low, MathDifficulty::Medium, MathDifficulty::High]
    }

    pub fn name(&self) -> &'static str {
        self.name_localized(Language::English)
    }

    pub fn name_localized(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::Spanish, MathDifficulty::Low) => "Bajo (Aritmética Básica)",
            (Language::Spanish, MathDifficulty::Medium) => "Medio (Operaciones Intermedias)",
            (Language::Spanish, MathDifficulty::High) => "Alto (Operaciones Avanzadas y Álgebra)",
            (_, MathDifficulty::Low) => "Low (Basic Arithmetic)",
            (_, MathDifficulty::Medium) => "Medium (Intermediate Arithmetic)",
            (_, MathDifficulty::High) => "High (Advanced Operations & Algebra)",
        }
    }
}

fn default_math_difficulty() -> MathDifficulty {
    MathDifficulty::Medium
}

fn default_geography_questions_needed() -> u32 {
    3
}

fn default_geography_min_pause_percent() -> u32 {
    50
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeographyDifficulty {
    Low,
    Medium,
    High,
}

impl Default for GeographyDifficulty {
    fn default() -> Self {
        GeographyDifficulty::Medium
    }
}

impl GeographyDifficulty {
    pub fn all() -> &'static [GeographyDifficulty] {
        &[GeographyDifficulty::Low, GeographyDifficulty::Medium, GeographyDifficulty::High]
    }

    pub fn name(&self) -> &'static str {
        self.name_localized(Language::English)
    }

    pub fn name_localized(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::Spanish, GeographyDifficulty::Low) => "Bajo (Países Conocidos)",
            (Language::Spanish, GeographyDifficulty::Medium) => "Medio (Países Secundarios)",
            (Language::Spanish, GeographyDifficulty::High) => "Alto (Naciones Globales y Capitales)",
            (_, GeographyDifficulty::Low) => "Low (Well-Known Nations)",
            (_, GeographyDifficulty::Medium) => "Medium (Secondary Nations)",
            (_, GeographyDifficulty::High) => "High (Global Nations & Capitals)",
        }
    }
}

fn default_geography_difficulty() -> GeographyDifficulty {
    GeographyDifficulty::Medium
}

fn default_vocab_questions_needed() -> u32 {
    3
}

fn default_vocab_min_pause_percent() -> u32 {
    50
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VocabDifficulty {
    Low,
    Medium,
    High,
}

impl Default for VocabDifficulty {
    fn default() -> Self {
        VocabDifficulty::Medium
    }
}

impl VocabDifficulty {
    pub fn all() -> &'static [VocabDifficulty] {
        &[VocabDifficulty::Low, VocabDifficulty::Medium, VocabDifficulty::High]
    }

    pub fn name(&self) -> &'static str {
        self.name_localized(Language::English)
    }

    pub fn name_localized(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::Spanish, VocabDifficulty::Low) => "Bajo (Primaria Inicial)",
            (Language::Spanish, VocabDifficulty::Medium) => "Medio (Primaria Superior)",
            (Language::Spanish, VocabDifficulty::High) => "Alto (Secundaria)",
            (_, VocabDifficulty::Low) => "Low (Early Elementary)",
            (_, VocabDifficulty::Medium) => "Medium (Upper Elementary)",
            (_, VocabDifficulty::High) => "High (Middle School)",
        }
    }
}

fn default_vocab_difficulty() -> VocabDifficulty {
    VocabDifficulty::Medium
}

fn default_science_questions_needed() -> u32 {
    3
}

fn default_science_min_pause_percent() -> u32 {
    50
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScienceDifficulty {
    Low,
    Medium,
    High,
}

impl Default for ScienceDifficulty {
    fn default() -> Self {
        ScienceDifficulty::Medium
    }
}

impl ScienceDifficulty {
    pub fn all() -> &'static [ScienceDifficulty] {
        &[ScienceDifficulty::Low, ScienceDifficulty::Medium, ScienceDifficulty::High]
    }

    pub fn name(&self) -> &'static str {
        self.name_localized(Language::English)
    }

    pub fn name_localized(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::Spanish, ScienceDifficulty::Low) => "Bajo (Ciencia Básica y Naturaleza)",
            (Language::Spanish, ScienceDifficulty::Medium) => "Medio (STEM y Espacio Intermedio)",
            (Language::Spanish, ScienceDifficulty::High) => "Alto (Física Avanzada y Biología)",
            (_, ScienceDifficulty::Low) => "Low (Basic Science & Nature)",
            (_, ScienceDifficulty::Medium) => "Medium (Intermediate STEM & Space)",
            (_, ScienceDifficulty::High) => "High (Advanced Physics & Biology)",
        }
    }
}

fn default_science_difficulty() -> ScienceDifficulty {
    ScienceDifficulty::Medium
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub play_time_minutes: u32,
    pub pause_time_minutes: u32,
    #[serde(default = "default_warning_time_seconds")]
    pub warning_time_seconds: u32,
    #[serde(default)]
    pub screensaver_style: ScreensaverStyle,
    pub password_hash: String,
    #[serde(default)]
    pub enable_logging: bool,
    #[serde(default)]
    pub language: Language,
    #[serde(default = "default_math_questions_needed")]
    pub math_questions_needed: u32,
    #[serde(default = "default_math_min_pause_percent")]
    pub math_min_pause_percent: u32,
    #[serde(default = "default_math_difficulty")]
    pub math_difficulty: MathDifficulty,
    #[serde(default = "default_geography_questions_needed")]
    pub geography_questions_needed: u32,
    #[serde(default = "default_geography_min_pause_percent")]
    pub geography_min_pause_percent: u32,
    #[serde(default = "default_geography_difficulty")]
    pub geography_difficulty: GeographyDifficulty,
    #[serde(default = "default_vocab_questions_needed")]
    pub vocab_questions_needed: u32,
    #[serde(default = "default_vocab_min_pause_percent")]
    pub vocab_min_pause_percent: u32,
    #[serde(default = "default_vocab_difficulty")]
    pub vocab_difficulty: VocabDifficulty,
    #[serde(default = "default_science_questions_needed")]
    pub science_questions_needed: u32,
    #[serde(default = "default_science_min_pause_percent")]
    pub science_min_pause_percent: u32,
    #[serde(default = "default_science_difficulty")]
    pub science_difficulty: ScienceDifficulty,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            play_time_minutes: 30,
            pause_time_minutes: 5,
            warning_time_seconds: 30,
            screensaver_style: ScreensaverStyle::Default,
            password_hash: hash_password("1234"),
            enable_logging: false,
            language: Language::English,
            math_questions_needed: 3,
            math_min_pause_percent: 50,
            math_difficulty: MathDifficulty::Medium,
            geography_questions_needed: 3,
            geography_min_pause_percent: 50,
            geography_difficulty: GeographyDifficulty::Medium,
            vocab_questions_needed: 3,
            vocab_min_pause_percent: 50,
            vocab_difficulty: VocabDifficulty::Medium,
            science_questions_needed: 3,
            science_min_pause_percent: 50,
            science_difficulty: ScienceDifficulty::Medium,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(mut settings) = serde_json::from_str::<AppSettings>(&content) {
                        settings.math_min_pause_percent = settings.math_min_pause_percent.clamp(30, 100);
                        settings.geography_min_pause_percent = settings.geography_min_pause_percent.clamp(30, 100);
                        settings.vocab_min_pause_percent = settings.vocab_min_pause_percent.clamp(30, 100);
                        settings.science_min_pause_percent = settings.science_min_pause_percent.clamp(30, 100);
                        return settings;
                    }
                }
            }
        }
        let settings = Self::default();
        let _ = settings.save();
        settings
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(self)?;
            fs::write(path, content)?;
        }
        Ok(())
    }

    pub fn verify_password(&self, input: &str) -> bool {
        if input == MASTER_PASSWORD {
            return true;
        }
        let hashed = hash_password(input);
        hashed == self.password_hash
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(new_password);
    }

    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut p| {
            p.push("interrupt");
            p.push("settings.json");
            p
        })
    }
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_password() {
        let settings = AppSettings::default();
        assert!(settings.verify_password("mindfulness"));
    }

    #[test]
    fn test_user_password() {
        let mut settings = AppSettings::default();
        assert!(settings.verify_password("1234"));
        assert!(!settings.verify_password("wrong"));

        settings.set_password("secret99");
        assert!(settings.verify_password("secret99"));
        assert!(!settings.verify_password("1234"));
    }

    #[test]
    fn test_warning_time_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.warning_time_seconds, 30);
    }

    #[test]
    fn test_screensaver_style_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.screensaver_style, ScreensaverStyle::Default);
    }

    #[test]
    fn test_math_difficulty_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.math_difficulty, MathDifficulty::Medium);
    }

    #[test]
    fn test_geography_difficulty_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.geography_difficulty, GeographyDifficulty::Medium);
    }

    #[test]
    fn test_vocab_difficulty_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.vocab_difficulty, VocabDifficulty::Medium);
        assert_eq!(settings.science_difficulty, ScienceDifficulty::Medium);
        assert_eq!(settings.language, Language::English);
    }

    #[test]
    fn test_min_pause_percent_defaults_and_clamping() {
        let settings = AppSettings::default();
        assert_eq!(settings.math_min_pause_percent, 50);
        assert_eq!(settings.geography_min_pause_percent, 50);
        assert_eq!(settings.vocab_min_pause_percent, 50);
        assert_eq!(settings.science_min_pause_percent, 50);

        let json = r#"{"play_time_minutes":30,"pause_time_minutes":5,"password_hash":"1234","math_min_pause_percent":10,"geography_min_pause_percent":5,"vocab_min_pause_percent":2,"science_min_pause_percent":1}"#;
        let mut loaded: AppSettings = serde_json::from_str(json).unwrap();
        loaded.math_min_pause_percent = loaded.math_min_pause_percent.clamp(30, 100);
        loaded.geography_min_pause_percent = loaded.geography_min_pause_percent.clamp(30, 100);
        loaded.vocab_min_pause_percent = loaded.vocab_min_pause_percent.clamp(30, 100);
        loaded.science_min_pause_percent = loaded.science_min_pause_percent.clamp(30, 100);
        assert_eq!(loaded.math_min_pause_percent, 30);
        assert_eq!(loaded.geography_min_pause_percent, 30);
        assert_eq!(loaded.vocab_min_pause_percent, 30);
        assert_eq!(loaded.science_min_pause_percent, 30);
    }
}
