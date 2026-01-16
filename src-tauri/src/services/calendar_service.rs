use std::fs;
use std::io::BufReader;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc, Local};
use ical::parser::ical::component::IcalEvent;
use ical::IcalParser;
use crate::error::{AppError, Result};
use crate::models::CalendarEvent;

pub struct CalendarService;

impl CalendarService {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_from_url(&self, url: &str) -> Result<Vec<CalendarEvent>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Other(format!(
                "Failed to fetch calendar: {}",
                response.status()
            )));
        }

        let content = response.text().await?;
        self.parse_ics_content(&content)
    }

    pub fn parse_from_file(&self, path: &str) -> Result<Vec<CalendarEvent>> {
        let content = fs::read_to_string(path)?;
        self.parse_ics_content(&content)
    }

    fn parse_ics_content(&self, content: &str) -> Result<Vec<CalendarEvent>> {
        let buf = BufReader::new(content.as_bytes());
        let parser = IcalParser::new(buf);

        let mut events = Vec::new();
        let today = Utc::now().date_naive();
        let tomorrow = today.succ_opt().unwrap_or(today);

        for calendar in parser {
            let calendar = calendar.map_err(|e| AppError::Parse(format!("iCal parse error: {}", e)))?;

            for event in calendar.events {
                if let Some(parsed_event) = self.parse_event(&event, today, tomorrow) {
                    events.push(parsed_event);
                }
            }
        }

        // Sort: all-day events first, then by start time
        events.sort_by(|a, b| {
            match (a.all_day, b.all_day) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.start_time.cmp(&b.start_time),
            }
        });

        Ok(events)
    }

    fn parse_event(&self, event: &IcalEvent, today: NaiveDate, tomorrow: NaiveDate) -> Option<CalendarEvent> {
        let mut uid = None;
        let mut summary = None;
        let mut description = None;
        let mut location = None;
        let mut dtstart = None;
        let mut dtend = None;
        let mut all_day = false;

        for prop in &event.properties {
            match prop.name.as_str() {
                "UID" => uid = prop.value.clone(),
                "SUMMARY" => summary = prop.value.clone(),
                "DESCRIPTION" => description = prop.value.clone(),
                "LOCATION" => location = prop.value.clone(),
                "DTSTART" => {
                    if let Some(ref value) = prop.value {
                        // Check if it's an all-day event (DATE vs DATETIME)
                        let is_date_only = prop.params.as_ref()
                            .and_then(|p| p.iter().find(|(k, _)| k == "VALUE"))
                            .map(|(_, v)| v.iter().any(|v| v == "DATE"))
                            .unwrap_or(value.len() == 8);

                        all_day = is_date_only;
                        dtstart = self.parse_datetime(value, is_date_only);
                    }
                }
                "DTEND" => {
                    if let Some(ref value) = prop.value {
                        let is_date_only = prop.params.as_ref()
                            .and_then(|p| p.iter().find(|(k, _)| k == "VALUE"))
                            .map(|(_, v)| v.iter().any(|v| v == "DATE"))
                            .unwrap_or(value.len() == 8);

                        dtend = self.parse_datetime(value, is_date_only);
                    }
                }
                _ => {}
            }
        }

        let uid = uid?;
        let summary = summary.unwrap_or_else(|| "(No title)".to_string());
        let start_time = dtstart?;
        let end_time = dtend.unwrap_or(start_time + 3600); // Default to 1 hour

        // Filter to today's events only
        let start_date = DateTime::from_timestamp(start_time, 0)?.date_naive();
        let end_date = DateTime::from_timestamp(end_time, 0)?.date_naive();

        // Include if event overlaps with today
        if start_date > tomorrow || end_date < today {
            return None;
        }

        let now = Utc::now().timestamp();
        let is_now = start_time <= now && end_time > now;
        let is_soon = !is_now && start_time > now && start_time <= now + 1800;

        Some(CalendarEvent {
            id: uid,
            summary,
            description,
            location,
            start_time,
            end_time,
            all_day,
            html_link: None,
            is_now,
            is_soon,
        })
    }

    fn parse_datetime(&self, value: &str, is_date_only: bool) -> Option<i64> {
        if is_date_only {
            // Format: YYYYMMDD
            let date = NaiveDate::parse_from_str(value, "%Y%m%d").ok()?;
            let datetime = date.and_hms_opt(0, 0, 0)?;
            Some(datetime.and_utc().timestamp())
        } else {
            // Format: YYYYMMDDTHHMMSS or YYYYMMDDTHHMMSSZ
            let value = value.trim_end_matches('Z');
            let datetime = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%S").ok()?;

            // Assume UTC if ends with Z, otherwise local time
            if value.ends_with('Z') {
                Some(datetime.and_utc().timestamp())
            } else {
                // Convert local time to UTC
                let local = Local::now().timezone();
                local.from_local_datetime(&datetime)
                    .earliest()
                    .map(|dt| dt.timestamp())
            }
        }
    }
}
