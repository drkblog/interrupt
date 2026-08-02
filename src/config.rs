use crate::screensaver::ScreensaverStyle;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub const MASTER_PASSWORD: &str = "mindfulness";

fn default_warning_time_seconds() -> u32 {
    30
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
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            play_time_minutes: 30,
            pause_time_minutes: 5,
            warning_time_seconds: 30,
            screensaver_style: ScreensaverStyle::Default,
            password_hash: hash_password("1234"),
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
}
