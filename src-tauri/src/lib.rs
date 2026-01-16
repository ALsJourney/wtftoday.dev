mod error;
mod models;
mod database;
mod services;
mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Settings commands
            is_onboarding_complete,
            get_setting,
            set_setting,
            save_github_token,
            validate_github_token,
            save_calendar_config,
            get_all_settings,
            clear_cache,
            // GitHub commands
            fetch_github_data,
            get_cached_github_data,
            // Calendar commands
            fetch_calendar_events,
            get_cached_calendar_events,
            parse_ics_file,
            // Brief commands
            refresh_brief,
            get_brief,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
