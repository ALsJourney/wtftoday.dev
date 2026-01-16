use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub all_day: bool,
    pub html_link: Option<String>,
    pub is_now: bool,
    pub is_soon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarConfig {
    pub source_type: String, // "ics_file", "ics_url", "none"
    pub ics_path: Option<String>,
    pub ics_url: Option<String>,
}

impl Default for CalendarConfig {
    fn default() -> Self {
        Self {
            source_type: "none".to_string(),
            ics_path: None,
            ics_url: None,
        }
    }
}
