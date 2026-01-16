use rusqlite::Connection;
use crate::error::Result;

pub fn init_database(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Settings table for non-sensitive configuration
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- GitHub Pull Requests cache
        CREATE TABLE IF NOT EXISTS github_pull_requests (
            id INTEGER PRIMARY KEY,
            repo_full_name TEXT NOT NULL,
            number INTEGER NOT NULL,
            title TEXT NOT NULL,
            state TEXT NOT NULL,
            draft INTEGER NOT NULL DEFAULT 0,
            user_login TEXT NOT NULL,
            user_avatar_url TEXT,
            html_url TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            requested_reviewers TEXT,
            labels TEXT,
            review_status TEXT,
            pr_type TEXT NOT NULL DEFAULT 'other',
            cached_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            UNIQUE(repo_full_name, number)
        );

        -- GitHub Issues cache
        CREATE TABLE IF NOT EXISTS github_issues (
            id INTEGER PRIMARY KEY,
            repo_full_name TEXT NOT NULL,
            number INTEGER NOT NULL,
            title TEXT NOT NULL,
            state TEXT NOT NULL,
            user_login TEXT NOT NULL,
            user_avatar_url TEXT,
            html_url TEXT NOT NULL,
            body_preview TEXT,
            labels TEXT,
            cached_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            UNIQUE(repo_full_name, number)
        );

        -- GitHub Notifications cache
        CREATE TABLE IF NOT EXISTS github_notifications (
            id TEXT PRIMARY KEY,
            repo_full_name TEXT NOT NULL,
            subject_title TEXT NOT NULL,
            subject_type TEXT NOT NULL,
            subject_url TEXT,
            reason TEXT NOT NULL,
            unread INTEGER NOT NULL DEFAULT 1,
            updated_at TEXT NOT NULL,
            cached_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Calendar Events cache
        CREATE TABLE IF NOT EXISTS calendar_events (
            id TEXT PRIMARY KEY,
            source TEXT NOT NULL,
            summary TEXT NOT NULL,
            description TEXT,
            location TEXT,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL,
            all_day INTEGER NOT NULL DEFAULT 0,
            html_link TEXT,
            cached_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Cache metadata for invalidation
        CREATE TABLE IF NOT EXISTS cache_metadata (
            source TEXT PRIMARY KEY,
            last_fetch INTEGER NOT NULL,
            etag TEXT
        );

        -- Create indexes
        CREATE INDEX IF NOT EXISTS idx_github_prs_cached ON github_pull_requests(cached_at);
        CREATE INDEX IF NOT EXISTS idx_github_prs_type ON github_pull_requests(pr_type);
        CREATE INDEX IF NOT EXISTS idx_github_issues_cached ON github_issues(cached_at);
        CREATE INDEX IF NOT EXISTS idx_calendar_events_start ON calendar_events(start_time);
        "#,
    )?;

    Ok(())
}
