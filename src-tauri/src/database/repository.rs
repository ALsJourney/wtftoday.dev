use rusqlite::{params, Connection};
use crate::error::Result;
use crate::models::*;

// Settings operations
pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?")?;
    let result = stmt.query_row(params![key], |row| row.get(0));

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, strftime('%s', 'now'))",
        params![key, value],
    )?;
    Ok(())
}

// GitHub PR operations
pub fn save_github_prs(conn: &Connection, prs: &[GitHubPullRequest], pr_type: &str) -> Result<()> {
    // Clear old PRs of this type
    conn.execute("DELETE FROM github_pull_requests WHERE pr_type = ?", params![pr_type])?;

    let mut stmt = conn.prepare(
        r#"INSERT OR REPLACE INTO github_pull_requests
           (id, repo_full_name, number, title, state, draft, user_login, user_avatar_url,
            html_url, created_at, updated_at, requested_reviewers, labels, review_status, pr_type, cached_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'))"#
    )?;

    for pr in prs {
        stmt.execute(params![
            pr.id,
            pr.repo_full_name,
            pr.number,
            pr.title,
            pr.state,
            pr.draft,
            pr.user_login,
            pr.user_avatar_url,
            pr.html_url,
            pr.created_at,
            pr.updated_at,
            serde_json::to_string(&pr.requested_reviewers).unwrap_or_default(),
            serde_json::to_string(&pr.labels).unwrap_or_default(),
            pr.review_status,
            pr_type,
        ])?;
    }

    Ok(())
}

pub fn get_github_prs(conn: &Connection, pr_type: &str) -> Result<Vec<GitHubPullRequest>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, repo_full_name, number, title, state, draft, user_login, user_avatar_url,
                  html_url, created_at, updated_at, requested_reviewers, labels, review_status
           FROM github_pull_requests WHERE pr_type = ? ORDER BY updated_at DESC"#
    )?;

    let prs = stmt.query_map(params![pr_type], |row| {
        let reviewers_json: String = row.get(11)?;
        let labels_json: String = row.get(12)?;

        Ok(GitHubPullRequest {
            id: row.get(0)?,
            repo_full_name: row.get(1)?,
            number: row.get(2)?,
            title: row.get(3)?,
            state: row.get(4)?,
            draft: row.get(5)?,
            user_login: row.get(6)?,
            user_avatar_url: row.get(7)?,
            html_url: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
            requested_reviewers: serde_json::from_str(&reviewers_json).unwrap_or_default(),
            labels: serde_json::from_str(&labels_json).unwrap_or_default(),
            review_status: row.get(13)?,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(prs)
}

// GitHub Issues operations
pub fn save_github_issues(conn: &Connection, issues: &[GitHubIssue]) -> Result<()> {
    conn.execute("DELETE FROM github_issues", [])?;

    let mut stmt = conn.prepare(
        r#"INSERT OR REPLACE INTO github_issues
           (id, repo_full_name, number, title, state, user_login, user_avatar_url,
            html_url, body_preview, labels, cached_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'))"#
    )?;

    for issue in issues {
        stmt.execute(params![
            issue.id,
            issue.repo_full_name,
            issue.number,
            issue.title,
            issue.state,
            issue.user_login,
            None::<String>, // user_avatar_url not in our model
            issue.html_url,
            issue.body_preview,
            serde_json::to_string(&issue.labels).unwrap_or_default(),
        ])?;
    }

    Ok(())
}

pub fn get_github_issues(conn: &Connection) -> Result<Vec<GitHubIssue>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, repo_full_name, number, title, state, user_login, html_url, body_preview, labels
           FROM github_issues ORDER BY id DESC"#
    )?;

    let issues = stmt.query_map([], |row| {
        let labels_json: String = row.get(8)?;

        Ok(GitHubIssue {
            id: row.get(0)?,
            repo_full_name: row.get(1)?,
            number: row.get(2)?,
            title: row.get(3)?,
            state: row.get(4)?,
            user_login: row.get(5)?,
            html_url: row.get(6)?,
            body_preview: row.get(7)?,
            labels: serde_json::from_str(&labels_json).unwrap_or_default(),
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(issues)
}

// GitHub Notifications operations
pub fn save_github_notifications(conn: &Connection, notifications: &[GitHubNotification]) -> Result<()> {
    conn.execute("DELETE FROM github_notifications", [])?;

    let mut stmt = conn.prepare(
        r#"INSERT OR REPLACE INTO github_notifications
           (id, repo_full_name, subject_title, subject_type, subject_url, reason, unread, updated_at, cached_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'))"#
    )?;

    for notif in notifications {
        stmt.execute(params![
            notif.id,
            notif.repo_full_name,
            notif.subject_title,
            notif.subject_type,
            notif.url,
            notif.reason,
            notif.unread,
            notif.updated_at,
        ])?;
    }

    Ok(())
}

pub fn get_github_notifications(conn: &Connection) -> Result<Vec<GitHubNotification>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, repo_full_name, subject_title, subject_type, subject_url, reason, unread, updated_at
           FROM github_notifications ORDER BY updated_at DESC"#
    )?;

    let notifications = stmt.query_map([], |row| {
        Ok(GitHubNotification {
            id: row.get(0)?,
            repo_full_name: row.get(1)?,
            subject_title: row.get(2)?,
            subject_type: row.get(3)?,
            url: row.get(4)?,
            reason: row.get(5)?,
            unread: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(notifications)
}

// Calendar operations
pub fn save_calendar_events(conn: &Connection, events: &[CalendarEvent], source: &str) -> Result<()> {
    conn.execute("DELETE FROM calendar_events WHERE source = ?", params![source])?;

    let mut stmt = conn.prepare(
        r#"INSERT OR REPLACE INTO calendar_events
           (id, source, summary, description, location, start_time, end_time, all_day, html_link, cached_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'))"#
    )?;

    for event in events {
        stmt.execute(params![
            event.id,
            source,
            event.summary,
            event.description,
            event.location,
            event.start_time,
            event.end_time,
            event.all_day,
            event.html_link,
        ])?;
    }

    Ok(())
}

pub fn get_calendar_events_for_today(conn: &Connection) -> Result<Vec<CalendarEvent>> {
    let now = chrono::Utc::now();
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    let today_end = now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp();
    let now_ts = now.timestamp();
    let soon_ts = now_ts + 1800; // 30 minutes from now

    let mut stmt = conn.prepare(
        r#"SELECT id, summary, description, location, start_time, end_time, all_day, html_link
           FROM calendar_events
           WHERE (start_time >= ? AND start_time <= ?) OR (start_time <= ? AND end_time >= ?)
           ORDER BY all_day DESC, start_time ASC"#
    )?;

    let events = stmt.query_map(params![today_start, today_end, now_ts, today_start], |row| {
        let start_time: i64 = row.get(4)?;
        let end_time: i64 = row.get(5)?;
        let is_now = start_time <= now_ts && end_time > now_ts;
        let is_soon = !is_now && start_time > now_ts && start_time <= soon_ts;

        Ok(CalendarEvent {
            id: row.get(0)?,
            summary: row.get(1)?,
            description: row.get(2)?,
            location: row.get(3)?,
            start_time,
            end_time,
            all_day: row.get(6)?,
            html_link: row.get(7)?,
            is_now,
            is_soon,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(events)
}

// Cache metadata operations
pub fn set_cache_metadata(conn: &Connection, source: &str, etag: Option<&str>) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO cache_metadata (source, last_fetch, etag) VALUES (?, strftime('%s', 'now'), ?)",
        params![source, etag],
    )?;
    Ok(())
}

pub fn get_cache_metadata(conn: &Connection, source: &str) -> Result<Option<(i64, Option<String>)>> {
    let mut stmt = conn.prepare("SELECT last_fetch, etag FROM cache_metadata WHERE source = ?")?;
    let result = stmt.query_row(params![source], |row| {
        Ok((row.get(0)?, row.get(1)?))
    });

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

// Clear all cache
pub fn clear_all_cache(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        DELETE FROM github_pull_requests;
        DELETE FROM github_issues;
        DELETE FROM github_notifications;
        DELETE FROM calendar_events;
        DELETE FROM cache_metadata;
        "#
    )?;
    Ok(())
}
