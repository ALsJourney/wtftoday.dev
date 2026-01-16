import { Section, Card, Badge, EmptyState } from "../ui";
import type { GitHubBriefData, GitHubPullRequest, GitHubIssue, GitHubNotification } from "../../types";

interface GitHubSectionProps {
  data: GitHubBriefData | null;
  loading: boolean;
  configured: boolean;
}

export function GitHubSection({ data, loading, configured }: GitHubSectionProps) {
  if (!configured) {
    return (
      <Section title="GitHub" icon={<GitHubIcon />}>
        <EmptyState
          title="GitHub not configured"
          description="Add your GitHub token in settings to see PRs and issues."
        />
      </Section>
    );
  }

  const totalItems =
    (data?.prs_to_review.length ?? 0) +
    (data?.my_open_prs.length ?? 0) +
    (data?.mentioned_issues.length ?? 0) +
    (data?.notifications.filter(n => n.unread).length ?? 0);

  return (
    <Section
      title="GitHub"
      icon={<GitHubIcon />}
      badge={totalItems}
      loading={loading}
    >
      {!data && !loading ? (
        <EmptyState title="No data yet" description="Click refresh to fetch your GitHub data." />
      ) : (
        <div className="space-y-4">
          {/* PRs to Review */}
          {data?.prs_to_review && data.prs_to_review.length > 0 && (
            <div>
              <h3 className="text-xs font-medium text-gray-500 mb-2">PRs to Review</h3>
              <div className="space-y-2">
                {data.prs_to_review.map((pr) => (
                  <PRCard key={pr.id} pr={pr} type="review" />
                ))}
              </div>
            </div>
          )}

          {/* My Open PRs */}
          {data?.my_open_prs && data.my_open_prs.length > 0 && (
            <div>
              <h3 className="text-xs font-medium text-gray-500 mb-2">Your Open PRs</h3>
              <div className="space-y-2">
                {data.my_open_prs.map((pr) => (
                  <PRCard key={pr.id} pr={pr} type="mine" />
                ))}
              </div>
            </div>
          )}

          {/* Mentioned Issues */}
          {data?.mentioned_issues && data.mentioned_issues.length > 0 && (
            <div>
              <h3 className="text-xs font-medium text-gray-500 mb-2">Mentioned In</h3>
              <div className="space-y-2">
                {data.mentioned_issues.map((issue) => (
                  <IssueCard key={issue.id} issue={issue} />
                ))}
              </div>
            </div>
          )}

          {/* Notifications */}
          {data?.notifications && data.notifications.filter(n => n.unread).length > 0 && (
            <div>
              <h3 className="text-xs font-medium text-gray-500 mb-2">Notifications</h3>
              <div className="space-y-2">
                {data.notifications.filter(n => n.unread).slice(0, 5).map((notif) => (
                  <NotificationCard key={notif.id} notification={notif} />
                ))}
              </div>
            </div>
          )}

          {/* Empty state when all arrays are empty */}
          {data &&
           data.prs_to_review.length === 0 &&
           data.my_open_prs.length === 0 &&
           data.mentioned_issues.length === 0 &&
           data.notifications.filter(n => n.unread).length === 0 && (
            <EmptyState
              title="All clear!"
              description="No PRs to review, no open PRs, and no new notifications."
            />
          )}
        </div>
      )}
    </Section>
  );
}

function PRCard({ pr, type }: { pr: GitHubPullRequest; type: "review" | "mine" }) {
  const openUrl = () => {
    window.open(pr.html_url, "_blank");
  };

  return (
    <Card hover onClick={openUrl} className="p-3">
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <p className="text-sm font-medium text-gray-200 truncate">{pr.title}</p>
          <p className="text-xs text-gray-500 mt-0.5">
            {pr.repo_full_name} #{pr.number}
          </p>
        </div>
        <div className="flex items-center gap-1.5 flex-shrink-0">
          {pr.draft && <Badge variant="default">Draft</Badge>}
          {type === "mine" && pr.review_status === "approved" && (
            <Badge variant="success">Approved</Badge>
          )}
          {type === "mine" && pr.review_status === "changes_requested" && (
            <Badge variant="warning">Changes</Badge>
          )}
        </div>
      </div>
    </Card>
  );
}

function IssueCard({ issue }: { issue: GitHubIssue }) {
  const openUrl = () => {
    window.open(issue.html_url, "_blank");
  };

  return (
    <Card hover onClick={openUrl} className="p-3">
      <p className="text-sm font-medium text-gray-200 truncate">{issue.title}</p>
      <p className="text-xs text-gray-500 mt-0.5">
        {issue.repo_full_name} #{issue.number}
      </p>
    </Card>
  );
}

function NotificationCard({ notification }: { notification: GitHubNotification }) {
  const openUrl = () => {
    if (notification.url) {
      window.open(notification.url, "_blank");
    }
  };

  const reasonLabels: Record<string, string> = {
    review_requested: "Review requested",
    mention: "Mentioned",
    author: "Author",
    comment: "Comment",
    assign: "Assigned",
    team_mention: "Team mention",
  };

  return (
    <Card hover onClick={openUrl} className="p-3">
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <p className="text-sm font-medium text-gray-200 truncate">{notification.subject_title}</p>
          <p className="text-xs text-gray-500 mt-0.5">{notification.repo_full_name}</p>
        </div>
        <Badge variant="info">{reasonLabels[notification.reason] ?? notification.reason}</Badge>
      </div>
    </Card>
  );
}

function GitHubIcon() {
  return (
    <svg className="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
    </svg>
  );
}
