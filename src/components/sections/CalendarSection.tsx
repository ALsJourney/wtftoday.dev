import { Section, Card, Badge, EmptyState } from "../ui";
import type { CalendarEvent } from "../../types";
import { format } from "date-fns";

interface CalendarSectionProps {
  events: CalendarEvent[];
  loading: boolean;
  configured: boolean;
}

export function CalendarSection({
  events,
  loading,
  configured,
}: CalendarSectionProps) {
  if (!configured) {
    return (
      <Section title="Calendar" icon={<CalendarIcon />}>
        <EmptyState
          title="Calendar not configured"
          description="Add a calendar source in settings to see today's events."
        />
      </Section>
    );
  }

  // Sort events: all-day first, then by start time
  const sortedEvents = [...events].sort((a, b) => {
    if (a.all_day && !b.all_day) return -1;
    if (!a.all_day && b.all_day) return 1;
    return a.start_time - b.start_time;
  });

  return (
    <Section
      title="Today's Calendar"
      icon={<CalendarIcon />}
      badge={events.length}
      loading={loading}
    >
      {events.length === 0 ? (
        <EmptyState
          title="No events today"
          description="Your calendar is clear for today."
        />
      ) : (
        <div className="space-y-2">
          {sortedEvents.map((event) => (
            <EventCard key={event.id} event={event} />
          ))}
        </div>
      )}
    </Section>
  );
}

function EventCard({ event }: { event: CalendarEvent }) {
  const startDate = new Date(event.start_time * 1000);
  const endDate = new Date(event.end_time * 1000);

  const openUrl = () => {
    if (event.html_link) {
      window.open(event.html_link, "_blank");
    }
  };

  const formatTime = (date: Date) => format(date, "h:mm a");

  return (
    <Card
      hover={!!event.html_link}
      onClick={event.html_link ? openUrl : undefined}
      className={`p-3 ${event.is_now ? "border-brand-500 bg-brand-950/30" : ""}`}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <p className="text-sm font-medium text-gray-200 truncate">
              {event.summary}
            </p>
            {event.is_now && <Badge variant="success">Now</Badge>}
            {event.is_soon && !event.is_now && (
              <Badge variant="warning">Soon</Badge>
            )}
          </div>

          {event.location && (
            <p className="text-xs text-gray-500 mt-0.5 truncate">
              {event.location}
            </p>
          )}
        </div>

        <div className="text-right flex-shrink-0">
          {event.all_day ? (
            <span className="text-xs text-gray-400">All day</span>
          ) : (
            <div className="text-xs text-gray-400">
              <div>{formatTime(startDate)}</div>
              <div className="text-gray-600">to {formatTime(endDate)}</div>
            </div>
          )}
        </div>
      </div>
    </Card>
  );
}

function CalendarIcon() {
  return (
    <svg
      className="w-4 h-4"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
      />
    </svg>
  );
}
