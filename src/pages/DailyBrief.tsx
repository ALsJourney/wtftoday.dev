import { AppShell } from "../components/layout";
import { GitHubSection } from "../components/sections/GitHubSection";
import { CalendarSection } from "../components/sections/CalendarSection";
import { EmailSection } from "../components/sections/EmailSection";
import { useBriefStore } from "../store/briefStore";
import { useSettingsStore } from "../store/settingsStore";
import { useEffect } from "react";

export function DailyBrief() {
  const { brief, loading, refreshing, lastUpdated, loadBrief, refreshBrief } = useBriefStore();
  const { settings } = useSettingsStore();

  useEffect(() => {
    loadBrief();
  }, [loadBrief]);

  return (
    <AppShell
      onRefresh={refreshBrief}
      refreshing={refreshing}
      lastUpdated={lastUpdated}
    >
      <div className="max-w-3xl mx-auto px-6 py-6">
        <div className="mb-6">
          <p className="text-gray-400 text-sm">
            WTF should I focus on today?
          </p>
        </div>

        <GitHubSection
          data={brief?.github ?? null}
          loading={loading}
          configured={settings?.github_configured ?? false}
        />

        <CalendarSection
          events={brief?.calendar ?? []}
          loading={loading}
          configured={settings?.calendar_config.source_type !== "none"}
        />

        <EmailSection
          emails={brief?.email ?? []}
          loading={loading}
          configured={settings?.email_config.enabled ?? false}
        />
      </div>
    </AppShell>
  );
}
