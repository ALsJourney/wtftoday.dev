import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button, Input } from "../../components/ui";
import type { OnboardingData } from "./index";
import type { GitHubUser } from "../../types";

interface GitHubStepProps {
  data: OnboardingData["github"];
  onUpdate: (data: OnboardingData["github"]) => void;
  onNext: () => void;
  onBack: () => void;
}

export function GitHubStep({
  data,
  onUpdate,
  onNext,
  onBack,
}: GitHubStepProps) {
  const [token, setToken] = useState(data.token);
  const [validating, setValidating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const validateToken = async () => {
    if (!token.trim()) {
      setError("Please enter a GitHub token");
      return;
    }

    setValidating(true);
    setError(null);

    try {
      const user = await invoke<GitHubUser>("validate_github_token", { token });
      await invoke("save_github_token", { token });

      onUpdate({ token, validated: true, user });
      onNext();
    } catch (err) {
      setError(
        "Invalid token. Make sure it has the required scopes (repo, notifications, read:user)",
      );
    } finally {
      setValidating(false);
    }
  };

  const skip = () => {
    onUpdate({ token: "", validated: false, user: null });
    onNext();
  };

  return (
    <div>
      <h2 className="text-xl font-semibold text-white mb-2">Connect GitHub</h2>
      <p className="text-sm text-gray-400 mb-6">
        Add your GitHub Personal Access Token to see PRs, issues, and
        notifications.
      </p>

      <div className="space-y-4 mb-6">
        <Input
          label="GitHub Personal Access Token"
          type="password"
          placeholder="ghp_xxx or github_pat_xxx"
          value={token}
          onChange={(e) => setToken(e.target.value)}
          error={error ?? undefined}
        />

        <div className="bg-gray-900 border border-gray-800 rounded-lg p-3">
          <p className="text-xs text-gray-400 mb-2">
            Supports both classic and fine-grained tokens:
          </p>
          <ul className="text-xs text-gray-500 space-y-1">
            <li>
              • <code className="text-gray-400">ghp_xxx</code> - Classic token
            </li>
            <li>
              • <code className="text-gray-400">github_pat_xxx</code> -
              Fine-grained token
            </li>
          </ul>
          <p className="text-xs text-gray-400 mt-3 mb-1">
            Required permissions:
          </p>
          <ul className="text-xs text-gray-500 space-y-1">
            <li>
              • <code className="text-gray-400">repo</code> - Access
              repositories
            </li>
            <li>
              • <code className="text-gray-400">notifications</code> - Read
              notifications
            </li>
            <li>
              • <code className="text-gray-400">read:user</code> - Read user
              profile
            </li>
          </ul>
          <div className="flex gap-3 mt-3">
            <a
              href="https://github.com/settings/tokens/new?description=WTF%20Today&scopes=repo,notifications,read:user"
              target="_blank"
              rel="noopener noreferrer"
              className="text-xs text-brand-400 hover:text-brand-300"
            >
              Create classic token →
            </a>
            <a
              href="https://github.com/settings/personal-access-tokens/new"
              target="_blank"
              rel="noopener noreferrer"
              className="text-xs text-brand-400 hover:text-brand-300"
            >
              Create fine-grained token →
            </a>
          </div>
        </div>
      </div>

      <div className="flex gap-3">
        <Button variant="secondary" onClick={onBack} className="flex-1">
          Back
        </Button>
        <Button onClick={validateToken} loading={validating} className="flex-1">
          {validating ? "Validating..." : "Connect"}
        </Button>
      </div>

      <button
        onClick={skip}
        className="w-full mt-3 text-sm text-gray-500 hover:text-gray-400 transition-colors"
      >
        Skip for now
      </button>
    </div>
  );
}
