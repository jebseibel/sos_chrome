use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;

/// Type alias for Chrome profile mapping: directory name → user-visible profile name
pub type ProfileMap = HashMap<String, String>;

pub fn get_chrome_profile_names(local_state_path: &Path) -> ProfileMap {
    let mut profile_map = HashMap::new();

    if let Ok(contents) = fs::read_to_string(local_state_path) {
        if let Ok(json) = serde_json::from_str::<Value>(&contents) {
            if let Some(info_cache) = json.get("profile").and_then(|p| p.get("info_cache")) {
                if let Some(map) = info_cache.as_object() {
                    for (dir_name, info) in map {
                        if let Some(name) = info.get("name").and_then(|n| n.as_str()) {
                            profile_map.insert(dir_name.clone(), name.to_string());
                        }
                    }
                }
            }
        }
    }

    profile_map
}
