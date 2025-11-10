use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub wireframe_color: u32,
    pub flat_color: u32,
    pub background_color: u32,
    pub camera_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub default_fov: f32,
    pub recent_files: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wireframe_color: 0x00FF00,
            flat_color: 0xFFBF00,
            background_color: 0x000000,
            camera_sensitivity: 0.01,
            zoom_sensitivity: 0.1,
            pan_sensitivity: 0.001,
            default_fov: 45.0,
            recent_files: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("bookish-3d-viewer");
        path.push("config.toml");
        path
    }

    pub fn add_recent_file(&mut self, path: String) {
        self.recent_files.retain(|p| p != &path);
        self.recent_files.insert(0, path);
        if self.recent_files.len() > 10 {
            self.recent_files.truncate(10);
        }
    }
}

