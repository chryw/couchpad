use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
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

impl Profile {
    pub fn load(name: Option<&str>, path: Option<PathBuf>) -> Result<Self, String> {
        let file_path = match path {
            Some(p) => p,
            None => profile_path(name.unwrap_or("default")),
        };

        if !file_path.exists() {
            return Err(format!(
                "Profile not found: {}\nRun with --init to create a default profile, or --profile <name> --init to create a named one.",
                file_path.display()
            ));
        }

        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read profile: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse profile: {}", e))
    }

    pub fn create_default(name: Option<&str>) -> Result<PathBuf, String> {
        let file_path = profile_path(name.unwrap_or("default"));

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create profiles directory: {}", e))?;
        }

        let default = r#"{
  "layer_button": "Home",
  "mappings": {
    "A": "return",
    "B": "escape",
    "X": "space",
    "Y": "backspace",
    "DPadUp": "up",
    "DPadDown": "down",
    "DPadLeft": "left",
    "DPadRight": "right",
    "RB": "super+shift+]",
    "LB": "super+shift+[",
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
    "X": "ctrl+c",
    "Y": "super+a",
    "DPadUp": "super+shift+]",
    "DPadDown": "super+shift+[",
    "DPadLeft": "super+[",
    "DPadRight": "super+]",
    "LB": "super+z",
    "RB": "super+shift+z",
    "LT": "super+minus",
    "RT": "super+equal",
    "LS": "super+f",
    "RS": "super+w"
  }
}"#;

        fs::write(&file_path, default)
            .map_err(|e| format!("Failed to write profile: {}", e))?;

        Ok(file_path)
    }

    pub fn list_all() -> Result<Vec<String>, String> {
        let dir = profiles_dir();
        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut profiles = Vec::new();
        let entries = fs::read_dir(&dir)
            .map_err(|e| format!("Failed to read profiles directory: {}", e))?;

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

fn profiles_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("gamepad-mapper")
}

pub fn profile_path(name: &str) -> PathBuf {
    profiles_dir().join(format!("{}.json", name))
}
