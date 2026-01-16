export interface CalendarEvent {
  id: string;
  summary: string;
  description: string | null;
  location: string | null;
  start_time: number;
  end_time: number;
  all_day: boolean;
  html_link: string | null;
  is_now: boolean;
  is_soon: boolean;
}

export interface CalendarConfig {
  source_type: "ics_file" | "ics_url" | "none";
  ics_path: string | null;
  ics_url: string | null;
}
