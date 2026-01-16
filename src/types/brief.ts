import type { GitHubBriefData } from "./github";
import type { CalendarEvent } from "./calendar";
import type { EmailHeader } from "./email";

export interface BriefData {
  github: GitHubBriefData | null;
  calendar: CalendarEvent[];
  email: EmailHeader[];
  generated_at: number;
}
