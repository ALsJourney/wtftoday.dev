import { Section, EmptyState } from "../ui";

interface EmailSectionProps {
  emails: unknown[];
  loading: boolean;
  configured: boolean;
}

export function EmailSection({}: EmailSectionProps) {
  // For MVP, email is not implemented - show coming soon
  return (
    <Section title="Email" icon={<EmailIcon />}>
      <EmptyState
        title="Coming Soon"
        description="Email integration will be available in a future update."
      />
    </Section>
  );
}

function EmailIcon() {
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
        d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
      />
    </svg>
  );
}
