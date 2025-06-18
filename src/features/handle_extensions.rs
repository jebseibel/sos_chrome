use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::features::profile_utils::{get_chrome_profile_names, ProfileMap};

pub type ExtensionsByProfile = HashMap<String, Vec<String>>;

pub fn run(profile_map: &ProfileMap) -> ExtensionsByProfile {
    let base_path = PathBuf::from("/home/jeb/.config/google-chrome");

    if !base_path.exists() {
        println!("Chrome user data folder not found.");
        return ExtensionsByProfile::new(); // return empty map or appropriate default
    }

    let data = collect_extensions(profile_map, base_path);
    display_extensions(&data);
    data
}

pub fn collect_extensions(profile_map: &ProfileMap, base_path: PathBuf) -> ExtensionsByProfile {
    let base_path = PathBuf::from("/home/jeb/.config/google-chrome");
    let mut result = HashMap::new();

    if let Ok(entries) = fs::read_dir(&base_path) {
        for entry in entries.flatten() {
            let profile_path = entry.path();
            let dir_name = match profile_path.file_name() {
                Some(name) => name.to_string_lossy(),
                None => continue,
            };

            let extensions_path = profile_path.join("Extensions");
            if extensions_path.exists() && extensions_path.is_dir() {
                let display_name = profile_map
                    .get(dir_name.as_ref())
                    .cloned()
                    .unwrap_or_else(|| dir_name.to_string());

                let mut ext_ids = vec![];

                if let Ok(ext_dirs) = fs::read_dir(&extensions_path) {
                    for ext_entry in ext_dirs.flatten() {
                        let ext_id = ext_entry.file_name().to_string_lossy().to_string();
                        ext_ids.push(ext_id);
                    }
                }

                result.insert(display_name, ext_ids);
            }
        }
    }

    result
}


pub fn display_extensions(ext_map: &ExtensionsByProfile) {
    for (profile_name, ext_ids) in ext_map {
        println!("\nProfile: {}", profile_name);
        for ext_id in ext_ids {
            println!("  Extension ID: {}", ext_id);
        }
    }
}