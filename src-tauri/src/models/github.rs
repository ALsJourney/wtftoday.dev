use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub avatar_url: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubLabel {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPullRequest {
    pub id: i64,
    pub repo_full_name: String,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub draft: bool,
    pub user_login: String,
    pub user_avatar_url: Option<String>,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub requested_reviewers: Vec<String>,
    pub labels: Vec<GitHubLabel>,
    pub review_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: i64,
    pub repo_full_name: String,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub user_login: String,
    pub html_url: String,
    pub body_preview: Option<String>,
    pub labels: Vec<GitHubLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubNotification {
    pub id: String,
    pub repo_full_name: String,
    pub subject_title: String,
    pub subject_type: String,
    pub reason: String,
    pub unread: bool,
    pub updated_at: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHubBriefData {
    pub prs_to_review: Vec<GitHubPullRequest>,
    pub my_open_prs: Vec<GitHubPullRequest>,
    pub mentioned_issues: Vec<GitHubIssue>,
    pub notifications: Vec<GitHubNotification>,
    pub last_updated: Option<i64>,
}

// GitHub API response types
#[derive(Debug, Deserialize)]
pub struct GitHubApiUser {
    pub login: String,
    pub avatar_url: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiLabel {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiPullRequest {
    pub id: i64,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub draft: Option<bool>,
    pub user: GitHubApiUser,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub requested_reviewers: Vec<GitHubApiUser>,
    pub labels: Vec<GitHubApiLabel>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiIssue {
    pub id: i64,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub user: GitHubApiUser,
    pub html_url: String,
    pub body: Option<String>,
    pub labels: Vec<GitHubApiLabel>,
    pub pull_request: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiSearchResult<T> {
    pub total_count: i32,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiNotification {
    pub id: String,
    pub repository: GitHubApiRepository,
    pub subject: GitHubApiSubject,
    pub reason: String,
    pub unread: bool,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiRepository {
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubApiSubject {
    pub title: String,
    #[serde(rename = "type")]
    pub subject_type: String,
    pub url: Option<String>,
}
