export interface GitHubUser {
  login: string;
  avatar_url: string;
  name: string | null;
}

export interface GitHubLabel {
  name: string;
  color: string;
}

export interface GitHubPullRequest {
  id: number;
  repo_full_name: string;
  number: number;
  title: string;
  state: string;
  draft: boolean;
  user_login: string;
  user_avatar_url: string | null;
  html_url: string;
  created_at: string;
  updated_at: string;
  requested_reviewers: string[];
  labels: GitHubLabel[];
  review_status: string | null;
}

export interface GitHubIssue {
  id: number;
  repo_full_name: string;
  number: number;
  title: string;
  state: string;
  user_login: string;
  html_url: string;
  body_preview: string | null;
  labels: GitHubLabel[];
}

export interface GitHubNotification {
  id: string;
  repo_full_name: string;
  subject_title: string;
  subject_type: string;
  reason: string;
  unread: boolean;
  updated_at: string;
  url: string | null;
}

export interface GitHubBriefData {
  prs_to_review: GitHubPullRequest[];
  my_open_prs: GitHubPullRequest[];
  mentioned_issues: GitHubIssue[];
  notifications: GitHubNotification[];
  last_updated: number | null;
}
