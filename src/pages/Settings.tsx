import { useState } from "react";
import { AppShell } from "../components/layout";
import { Button, Card, Input } from "../components/ui";
import { useSettingsStore } from "../store/settingsStore";
import { useNavigate } from "react-router-dom";

export function Settings() {
  const navigate = useNavigate();
  const { settings, updateGitHubToken, updateCalendarConfig, clearCache } =
    useSettingsStore();

  const [githubToken, setGithubToken] = useState("");
  const [calendarUrl, setCalendarUrl] = useState(
    settings?.calendar_config.ics_url ?? "",
  );
  const [calendarPath, setCalendarPath] = useState(
    settings?.calendar_config.ics_path ?? "",
  );
  const [saving, setSaving] = useState(false);

  const handleSaveGitHub = async () => {
    if (!githubToken.trim()) return;
    setSaving(true);
    try {
      await updateGitHubToken(githubToken);
      setGithubToken("");
    } finally {
      setSaving(false);
    }
  };

  const handleSaveCalendar = async (type: "ics_url" | "ics_file") => {
    setSaving(true);
    try {
      await updateCalendarConfig({
        source_type: type,
        ics_url: type === "ics_url" ? calendarUrl : null,
        ics_path: type === "ics_file" ? calendarPath : null,
      });
    } finally {
      setSaving(false);
    }
  };

  const handleClearCache = async () => {
    if (
      confirm(
        "Clear all cached data? You'll need to refresh to fetch new data.",
      )
    ) {
      await clearCache();
    }
  };

  const handleRerunOnboarding = () => {
    navigate("/onboarding");
  };

  return (
    <AppShell>
      <div className="max-w-2xl mx-auto px-6 py-6">
        <h1 className="text-xl font-semibold text-white mb-6">Settings</h1>

        {/* GitHub Section */}
        <Card className="mb-4">
          <h2 className="text-sm font-semibold text-gray-300 uppercase tracking-wide mb-4">
            GitHub
          </h2>

          <div className="space-y-3">
            <div className="flex items-center gap-2 text-sm">
              <span className="text-gray-400">Status:</span>
              {settings?.github_configured ? (
                <span className="text-green-400">
                  Connected as @{settings.github_username}
                </span>
              ) : (
                <span className="text-yellow-400">Not configured</span>
              )}
            </div>

            <Input
              type="password"
              placeholder="Enter new GitHub token to update..."
              value={githubToken}
              onChange={(e) => setGithubToken(e.target.value)}
            />

            <Button
              size="sm"
              onClick={handleSaveGitHub}
              disabled={!githubToken.trim() || saving}
              loading={saving}
            >
              Update Token
            </Button>
          </div>
        </Card>

        {/* Calendar Section */}
        <Card className="mb-4">
          <h2 className="text-sm font-semibold text-gray-300 uppercase tracking-wide mb-4">
            Calendar
          </h2>

          <div className="space-y-4">
            <div className="flex items-center gap-2 text-sm">
              <span className="text-gray-400">Source:</span>
              <span className="text-gray-200">
                {settings?.calendar_config.source_type === "ics_url" &&
                  "iCal URL"}
                {settings?.calendar_config.source_type === "ics_file" &&
                  "Local .ics file"}
                {settings?.calendar_config.source_type === "none" &&
                  "Not configured"}
              </span>
            </div>

            <div>
              <Input
                label="iCal URL"
                type="url"
                placeholder="https://calendar.google.com/calendar/ical/..."
                value={calendarUrl}
                onChange={(e) => setCalendarUrl(e.target.value)}
              />
              <Button
                size="sm"
                variant="secondary"
                className="mt-2"
                onClick={() => handleSaveCalendar("ics_url")}
                disabled={!calendarUrl.trim() || saving}
              >
                Save URL
              </Button>
            </div>

            <div className="border-t border-gray-800 pt-4">
              <Input
                label="Local .ics file path"
                type="text"
                placeholder="/path/to/calendar.ics"
                value={calendarPath}
                onChange={(e) => setCalendarPath(e.target.value)}
              />
              <Button
                size="sm"
                variant="secondary"
                className="mt-2"
                onClick={() => handleSaveCalendar("ics_file")}
                disabled={!calendarPath.trim() || saving}
              >
                Save Path
              </Button>
            </div>
          </div>
        </Card>

        {/* Email Section */}
        <Card className="mb-4">
          <h2 className="text-sm font-semibold text-gray-300 uppercase tracking-wide mb-4">
            Email
          </h2>
          <p className="text-sm text-gray-500">
            Email integration coming soon.
          </p>
        </Card>

        {/* Actions */}
        <Card>
          <h2 className="text-sm font-semibold text-gray-300 uppercase tracking-wide mb-4">
            Actions
          </h2>

          <div className="flex flex-wrap gap-3">
            <Button variant="secondary" size="sm" onClick={handleClearCache}>
              Clear Cache
            </Button>
            <Button
              variant="secondary"
              size="sm"
              onClick={handleRerunOnboarding}
            >
              Re-run Onboarding
            </Button>
          </div>
        </Card>
      </div>
    </AppShell>
  );
}
