use tauri::AppHandle;
use crate::error::Result;
use crate::models::*;
use super::{fetch_github_data, get_cached_github_data, fetch_calendar_events, get_cached_calendar_events};
use crate::services;

#[tauri::command]
pub async fn refresh_brief(app: AppHandle) -> Result<BriefData> {
    // Check if GitHub is configured
    let github_configured = services::get_credential("github_token")?.is_some();

    // Fetch GitHub data if configured
    let github = if github_configured {
        match fetch_github_data(app.clone()).await {
            Ok(data) => Some(data),
            Err(e) => {
                log::warn!("Failed to fetch GitHub data: {}", e);
                // Try to use cached data on error
                get_cached_github_data(app.clone()).await.ok()
            }
        }
    } else {
        None
    };

    // Fetch calendar events
    let calendar = match fetch_calendar_events(app.clone()).await {
        Ok(events) => events,
        Err(e) => {
            log::warn!("Failed to fetch calendar events: {}", e);
            get_cached_calendar_events(app.clone()).await.unwrap_or_default()
        }
    };

    // Email is not implemented yet
    let email = Vec::new();

    let now = chrono::Utc::now().timestamp();

    Ok(BriefData {
        github,
        calendar,
        email,
        generated_at: now,
    })
}

#[tauri::command]
pub async fn get_brief(app: AppHandle) -> Result<BriefData> {
    // Check if GitHub is configured
    let github_configured = services::get_credential("github_token")?.is_some();

    // Get cached GitHub data if configured
    let github = if github_configured {
        get_cached_github_data(app.clone()).await.ok()
    } else {
        None
    };

    // Get cached calendar events
    let calendar = get_cached_calendar_events(app.clone()).await.unwrap_or_default();

    // Email is not implemented yet
    let email = Vec::new();

    let now = chrono::Utc::now().timestamp();

    Ok(BriefData {
        github,
        calendar,
        email,
        generated_at: now,
    })
}
