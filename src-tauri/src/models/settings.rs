use serde::{Deserialize, Serialize};
use super::{CalendarConfig, EmailConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllSettings {
    pub github_configured: bool,
    pub github_username: Option<String>,
    pub calendar_config: CalendarConfig,
    pub email_config: EmailConfig,
    pub onboarding_complete: bool,
}

impl Default for AllSettings {
    fn default() -> Self {
        Self {
            github_configured: false,
            github_username: None,
            calendar_config: CalendarConfig::default(),
            email_config: EmailConfig::default(),
            onboarding_complete: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatus {
    pub github_last_fetch: Option<i64>,
    pub calendar_last_fetch: Option<i64>,
    pub email_last_fetch: Option<i64>,
}
