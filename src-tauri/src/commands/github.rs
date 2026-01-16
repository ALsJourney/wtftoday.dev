use tauri::{AppHandle, Manager};
use rusqlite::Connection;
use crate::error::{AppError, Result};
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
pub async fn fetch_github_data(app: AppHandle) -> Result<GitHubBriefData> {
    // Get the GitHub token
    let token = services::get_credential("github_token")?
        .ok_or_else(|| AppError::NotFound("GitHub token not configured".to_string()))?;

    let service = services::GitHubService::new(token)?;

    // Get current user
    let user = service.get_current_user().await?;
    let username = &user.login;

    // Store username for settings display
    let conn = get_connection(&app)?;
    database::set_setting(&conn, "github_username", username)?;

    // Fetch all data in parallel
    let (prs_to_review, my_open_prs, mentioned_issues, notifications) = tokio::join!(
        service.get_prs_to_review(username),
        service.get_my_open_prs(username),
        service.get_mentioned_issues(username),
        service.get_notifications()
    );

    let prs_to_review = prs_to_review.unwrap_or_default();
    let my_open_prs = my_open_prs.unwrap_or_default();
    let mentioned_issues = mentioned_issues.unwrap_or_default();
    let notifications = notifications.unwrap_or_default();

    // Save to cache
    database::save_github_prs(&conn, &prs_to_review, "review")?;
    database::save_github_prs(&conn, &my_open_prs, "mine")?;
    database::save_github_issues(&conn, &mentioned_issues)?;
    database::save_github_notifications(&conn, &notifications)?;
    database::set_cache_metadata(&conn, "github", None)?;

    let now = chrono::Utc::now().timestamp();

    Ok(GitHubBriefData {
        prs_to_review,
        my_open_prs,
        mentioned_issues,
        notifications,
        last_updated: Some(now),
    })
}

#[tauri::command]
pub async fn get_cached_github_data(app: AppHandle) -> Result<GitHubBriefData> {
    let conn = get_connection(&app)?;

    let prs_to_review = database::get_github_prs(&conn, "review")?;
    let my_open_prs = database::get_github_prs(&conn, "mine")?;
    let mentioned_issues = database::get_github_issues(&conn)?;
    let notifications = database::get_github_notifications(&conn)?;

    let last_updated = database::get_cache_metadata(&conn, "github")?
        .map(|(ts, _)| ts);

    Ok(GitHubBriefData {
        prs_to_review,
        my_open_prs,
        mentioned_issues,
        notifications,
        last_updated,
    })
}
