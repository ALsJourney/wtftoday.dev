import type { CalendarConfig } from "./calendar";
import type { EmailConfig } from "./email";

export interface AllSettings {
  github_configured: boolean;
  github_username: string | null;
  calendar_config: CalendarConfig;
  email_config: EmailConfig;
  onboarding_complete: boolean;
}

export interface CacheStatus {
  github_last_fetch: number | null;
  calendar_last_fetch: number | null;
  email_last_fetch: number | null;
}
