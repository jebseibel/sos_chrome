mod features;
mod load_config;
mod main_ui;

use std::path::Path;
use main_ui::MyApp;
use crate::features::profile_utils::get_chrome_profile_names;

fn main() -> Result<(), eframe::Error> {
    // Load config first
    let config = load_config::Config::load_from_file("config.json").expect("Failed to load config");

    //load the google profiles
    let path = Path::new(&config.chrome.local_state_path);
    let profile_map = get_chrome_profile_names(path);
    
    let options = eframe::NativeOptions::default();

    // Clone config so we can move one into the app and keep one for the window title
    let window_title = config.app_title.clone();

    eframe::run_native(
        &window_title,
        options,
        Box::new(|_cc| Box::new(MyApp {
            logo: None,
            config,
            profile_map,
        })),
    )
}