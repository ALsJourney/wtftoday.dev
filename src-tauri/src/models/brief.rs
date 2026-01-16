use serde::{Deserialize, Serialize};
use super::{GitHubBriefData, CalendarEvent, EmailHeader};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BriefData {
    pub github: Option<GitHubBriefData>,
    pub calendar: Vec<CalendarEvent>,
    pub email: Vec<EmailHeader>,
    pub generated_at: i64,
}

impl Default for BriefData {
    fn default() -> Self {
        Self {
            github: None,
            calendar: Vec::new(),
            email: Vec::new(),
            generated_at: chrono::Utc::now().timestamp(),
        }
    }
}
