use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Map of button names to key combos (e.g., "A" -> "return", "LT" -> "super+shift+p")
    pub mappings: HashMap<String, String>,
    /// Alternate mappings when the layer modifier button is held
    #[serde(default)]
    pub layer_mappings: HashMap<String, String>,
    /// Button that activates the layer (default: "Home")
    #[serde(default = "default_layer_button")]
    pub layer_button: String,
}

fn default_layer_button() -> String {
    "Home".to_string()
}

impl Config {
    pub fn load_profile(profile: Option<&str>, path: Option<PathBuf>) -> Result<Self, String> {
        let config_path = match path {
            Some(p) => p,
            None => profile_path(profile.unwrap_or("default")),
        };

        if !config_path.exists() {
            return Err(format!(
                "Config file not found: {}\nRun with --init to create a default config, or --profile <name> --init to create a named profile.",
                config_path.display()
            ));
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn create_default(profile: Option<&str>) -> Result<PathBuf, String> {
        let config_path = profile_path(profile.unwrap_or("default"));

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let default = r#"{
  "layer_button": "Home",
  "mappings": {
    "A": "return",
    "B": "escape",
    "X": "space",
    "Y": "delete",
    "DPadUp": "up",
    "DPadDown": "down",
    "DPadLeft": "left",
    "DPadRight": "right",
    "RB": "tab",
    "LB": "shift+tab",
    "RT": "pagedown",
    "LT": "pageup",
    "Start": "return",
    "Select": "tab",
    "LS": "home",
    "RS": "end"
  },
  "layer_mappings": {
    "A": "super+v",
    "B": "super+c",
    "X": "super+x",
    "Y": "super+a",
    "DPadUp": "shift+up",
    "DPadDown": "shift+down",
    "DPadLeft": "shift+left",
    "DPadRight": "shift+right",
    "LB": "super+z",
    "RB": "super+shift+z",
    "LT": "super+left",
    "RT": "super+right",
    "LS": "super+f",
    "RS": "super+w"
  }
}"#;

        fs::write(&config_path, default)
            .map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(config_path)
    }

    pub fn list_profiles() -> Result<Vec<String>, String> {
        let profiles_dir = config_dir();
        if !profiles_dir.exists() {
            return Ok(vec![]);
        }

        let mut profiles = Vec::new();
        let entries = fs::read_dir(&profiles_dir)
            .map_err(|e| format!("Failed to read config directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    profiles.push(stem.to_string());
                }
            }
        }

        profiles.sort();
        Ok(profiles)
    }
}

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("gamepad-mapper")
}

pub fn profile_path(name: &str) -> PathBuf {
    config_dir().join(format!("{}.json", name))
}
