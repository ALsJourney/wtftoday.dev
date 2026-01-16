use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use crate::error::{AppError, Result};
use crate::models::*;

const GITHUB_API_BASE: &str = "https://api.github.com";

pub struct GitHubService {
    client: reqwest::Client,
    token: String,
}

impl GitHubService {
    pub fn new(token: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("WTFToday/1.0"));
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );

        let auth_value = format!("Bearer {}", token);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).map_err(|e| AppError::Other(e.to_string()))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, token })
    }

    pub async fn get_current_user(&self) -> Result<GitHubUser> {
        let url = format!("{}/user", GITHUB_API_BASE);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Validation(format!(
                "GitHub API error ({}): {}",
                status, body
            )));
        }

        let user: GitHubApiUser = response.json().await?;
        Ok(GitHubUser {
            login: user.login,
            avatar_url: user.avatar_url,
            name: user.name,
        })
    }

    pub async fn get_prs_to_review(&self, username: &str) -> Result<Vec<GitHubPullRequest>> {
        let query = format!("is:open is:pr review-requested:{} archived:false", username);
        let url = format!(
            "{}/search/issues?q={}&sort=updated&order=desc&per_page=20",
            GITHUB_API_BASE,
            urlencoding::encode(&query)
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let search_result: GitHubApiSearchResult<GitHubApiIssue> = response.json().await?;

        let prs: Vec<GitHubPullRequest> = search_result
            .items
            .into_iter()
            .filter(|item| item.pull_request.is_some())
            .map(|item| self.issue_to_pr(item))
            .collect();

        Ok(prs)
    }

    pub async fn get_my_open_prs(&self, username: &str) -> Result<Vec<GitHubPullRequest>> {
        let query = format!("is:open is:pr author:{} archived:false", username);
        let url = format!(
            "{}/search/issues?q={}&sort=updated&order=desc&per_page=20",
            GITHUB_API_BASE,
            urlencoding::encode(&query)
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let search_result: GitHubApiSearchResult<GitHubApiIssue> = response.json().await?;

        let prs: Vec<GitHubPullRequest> = search_result
            .items
            .into_iter()
            .filter(|item| item.pull_request.is_some())
            .map(|item| self.issue_to_pr(item))
            .collect();

        Ok(prs)
    }

    pub async fn get_mentioned_issues(&self, username: &str) -> Result<Vec<GitHubIssue>> {
        let query = format!("is:open mentions:{} archived:false", username);
        let url = format!(
            "{}/search/issues?q={}&sort=updated&order=desc&per_page=20",
            GITHUB_API_BASE,
            urlencoding::encode(&query)
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let search_result: GitHubApiSearchResult<GitHubApiIssue> = response.json().await?;

        let issues: Vec<GitHubIssue> = search_result
            .items
            .into_iter()
            .filter(|item| item.pull_request.is_none()) // Exclude PRs
            .map(|item| self.api_issue_to_issue(item))
            .collect();

        Ok(issues)
    }

    pub async fn get_notifications(&self) -> Result<Vec<GitHubNotification>> {
        let url = format!("{}/notifications?per_page=30", GITHUB_API_BASE);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let api_notifications: Vec<GitHubApiNotification> = response.json().await?;

        let notifications: Vec<GitHubNotification> = api_notifications
            .into_iter()
            .map(|n| GitHubNotification {
                id: n.id,
                repo_full_name: n.repository.full_name,
                subject_title: n.subject.title,
                subject_type: n.subject.subject_type,
                reason: n.reason,
                unread: n.unread,
                updated_at: n.updated_at,
                url: n.subject.url.map(|u| self.convert_api_url_to_html(&u)),
            })
            .collect();

        Ok(notifications)
    }

    fn issue_to_pr(&self, item: GitHubApiIssue) -> GitHubPullRequest {
        let repo_full_name = self.extract_repo_from_url(&item.html_url);

        GitHubPullRequest {
            id: item.id,
            repo_full_name,
            number: item.number,
            title: item.title,
            state: item.state,
            draft: false, // Draft status not available from search API
            user_login: item.user.login,
            user_avatar_url: Some(item.user.avatar_url),
            html_url: item.html_url,
            created_at: String::new(),
            updated_at: String::new(),
            requested_reviewers: Vec::new(),
            labels: item.labels.into_iter().map(|l| GitHubLabel {
                name: l.name,
                color: l.color,
            }).collect(),
            review_status: None,
        }
    }

    fn api_issue_to_issue(&self, item: GitHubApiIssue) -> GitHubIssue {
        let repo_full_name = self.extract_repo_from_url(&item.html_url);
        let body_preview = item.body.map(|b| {
            if b.len() > 200 {
                format!("{}...", &b[..197])
            } else {
                b
            }
        });

        GitHubIssue {
            id: item.id,
            repo_full_name,
            number: item.number,
            title: item.title,
            state: item.state,
            user_login: item.user.login,
            html_url: item.html_url,
            body_preview,
            labels: item.labels.into_iter().map(|l| GitHubLabel {
                name: l.name,
                color: l.color,
            }).collect(),
        }
    }

    fn extract_repo_from_url(&self, url: &str) -> String {
        // Extract owner/repo from URLs like https://github.com/owner/repo/...
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() >= 5 {
            format!("{}/{}", parts[3], parts[4])
        } else {
            String::new()
        }
    }

    fn convert_api_url_to_html(&self, api_url: &str) -> String {
        // Convert https://api.github.com/repos/owner/repo/issues/123
        // to https://github.com/owner/repo/issues/123
        api_url
            .replace("api.github.com/repos", "github.com")
            .replace("/pulls/", "/pull/")
    }
}
