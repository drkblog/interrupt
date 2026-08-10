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
    0
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
        match self {
            MathDifficulty::Low => "Low",
            MathDifficulty::Medium => "Medium",
            MathDifficulty::High => "High",
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
    0
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
        match self {
            GeographyDifficulty::Low => "Low (Well-Known Nations)",
            GeographyDifficulty::Medium => "Medium (Secondary Nations)",
            GeographyDifficulty::High => "High (Global Nations & Capitals)",
        }
    }
}

fn default_geography_difficulty() -> GeographyDifficulty {
    GeographyDifficulty::Medium
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
            math_questions_needed: 3,
            math_min_pause_percent: 0,
            math_difficulty: MathDifficulty::Medium,
            geography_questions_needed: 3,
            geography_min_pause_percent: 0,
            geography_difficulty: GeographyDifficulty::Medium,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
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
}
