use tauri::{AppHandle, Manager};
use rusqlite::Connection;
use crate::error::Result;
use crate::models::*;
use crate::database;
use crate::services;

fn get_db_path(app: &AppHandle) -> std::path::PathBuf {
    let app_data = app.path().app_data_dir().expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_data).ok();
    app_data.join("wtftoday.db")
}

fn get_connection(app: &AppHandle) -> Result<Connection> {
    let path = get_db_path(app);
    let conn = Connection::open(path)?;
    database::init_database(&conn)?;
    Ok(conn)
}

#[tauri::command]
pub async fn is_onboarding_complete(app: AppHandle) -> Result<bool> {
    let conn = get_connection(&app)?;
    let value = database::get_setting(&conn, "onboarding_complete")?;
    Ok(value.map(|v| v == "true").unwrap_or(false))
}

#[tauri::command]
pub async fn get_setting(app: AppHandle, key: String) -> Result<Option<String>> {
    let conn = get_connection(&app)?;
    database::get_setting(&conn, &key)
}

#[tauri::command]
pub async fn set_setting(app: AppHandle, key: String, value: String) -> Result<()> {
    let conn = get_connection(&app)?;
    database::set_setting(&conn, &key, &value)
}

#[tauri::command]
pub async fn save_github_token(token: String) -> Result<()> {
    services::save_credential("github_token", &token)
}

#[tauri::command]
pub async fn validate_github_token(token: String) -> Result<GitHubUser> {
    let service = services::GitHubService::new(token)?;
    service.get_current_user().await
}

#[tauri::command]
pub async fn save_calendar_config(app: AppHandle, config: CalendarConfig) -> Result<()> {
    let conn = get_connection(&app)?;
    let config_json = serde_json::to_string(&config).unwrap_or_default();
    database::set_setting(&conn, "calendar_config", &config_json)
}

#[tauri::command]
pub async fn get_all_settings(app: AppHandle) -> Result<AllSettings> {
    let conn = get_connection(&app)?;

    // Check GitHub configuration
    let github_token = services::get_credential("github_token")?;
    let github_configured = github_token.is_some();

    // Get GitHub username if configured
    let github_username = if github_configured {
        database::get_setting(&conn, "github_username")?
    } else {
        None
    };

    // Get calendar config
    let calendar_config = database::get_setting(&conn, "calendar_config")?
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default();

    // Get email config (not implemented yet)
    let email_config = EmailConfig::default();

    // Get onboarding status
    let onboarding_complete = database::get_setting(&conn, "onboarding_complete")?
        .map(|v| v == "true")
        .unwrap_or(false);

    Ok(AllSettings {
        github_configured,
        github_username,
        calendar_config,
        email_config,
        onboarding_complete,
    })
}

#[tauri::command]
pub async fn clear_cache(app: AppHandle) -> Result<()> {
    let conn = get_connection(&app)?;
    database::clear_all_cache(&conn)
}
