use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailHeader {
    pub id: String,
    pub from_address: String,
    pub from_name: Option<String>,
    pub subject: String,
    pub received_at: i64,
    pub is_unread: bool,
    pub is_important: bool,
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub enabled: bool,
    pub imap_server: Option<String>,
    pub imap_port: Option<u16>,
    pub username: Option<String>,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            imap_server: None,
            imap_port: None,
            username: None,
        }
    }
}
