use tauri::{AppHandle, Manager};
use rusqlite::Connection;
use crate::error::{AppError, Result};
use crate::models::*;
use crate::database;
use crate::services::CalendarService;

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
pub async fn fetch_calendar_events(app: AppHandle) -> Result<Vec<CalendarEvent>> {
    let conn = get_connection(&app)?;

    // Get calendar config
    let config: CalendarConfig = database::get_setting(&conn, "calendar_config")?
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default();

    let service = CalendarService::new();

    let events = match config.source_type.as_str() {
        "ics_url" => {
            let url = config.ics_url.ok_or_else(|| {
                AppError::Validation("Calendar URL not configured".to_string())
            })?;
            service.fetch_from_url(&url).await?
        }
        "ics_file" => {
            let path = config.ics_path.ok_or_else(|| {
                AppError::Validation("Calendar file path not configured".to_string())
            })?;
            service.parse_from_file(&path)?
        }
        _ => {
            return Ok(Vec::new());
        }
    };

    // Save to cache
    database::save_calendar_events(&conn, &events, &config.source_type)?;
    database::set_cache_metadata(&conn, "calendar", None)?;

    Ok(events)
}

#[tauri::command]
pub async fn get_cached_calendar_events(app: AppHandle) -> Result<Vec<CalendarEvent>> {
    let conn = get_connection(&app)?;
    database::get_calendar_events_for_today(&conn)
}

#[tauri::command]
pub async fn parse_ics_file(path: String) -> Result<Vec<CalendarEvent>> {
    let service = CalendarService::new();
    service.parse_from_file(&path)
}
