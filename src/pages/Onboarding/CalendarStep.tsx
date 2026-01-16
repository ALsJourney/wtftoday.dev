import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button, Input } from "../../components/ui";
import type { CalendarConfig } from "../../types";

interface CalendarStepProps {
  data: CalendarConfig;
  onUpdate: (data: CalendarConfig) => void;
  onNext: () => void;
  onBack: () => void;
}

type SourceType = "ics_url" | "ics_file" | "none";

export function CalendarStep({ data, onUpdate, onNext, onBack }: CalendarStepProps) {
  const [sourceType, setSourceType] = useState<SourceType>(data.source_type);
  const [icsUrl, setIcsUrl] = useState(data.ics_url ?? "");
  const [icsPath, setIcsPath] = useState(data.ics_path ?? "");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const saveAndContinue = async () => {
    setError(null);

    if (sourceType === "ics_url" && !icsUrl.trim()) {
      setError("Please enter an iCal URL");
      return;
    }

    if (sourceType === "ics_file" && !icsPath.trim()) {
      setError("Please enter a file path");
      return;
    }

    setSaving(true);

    try {
      const config: CalendarConfig = {
        source_type: sourceType,
        ics_url: sourceType === "ics_url" ? icsUrl : null,
        ics_path: sourceType === "ics_file" ? icsPath : null,
      };

      await invoke("save_calendar_config", { config });
      onUpdate(config);
      onNext();
    } catch (err) {
      setError(String(err));
    } finally {
      setSaving(false);
    }
  };

  const skip = () => {
    onUpdate({ source_type: "none", ics_url: null, ics_path: null });
    onNext();
  };

  return (
    <div>
      <h2 className="text-xl font-semibold text-white mb-2">Connect Calendar</h2>
      <p className="text-sm text-gray-400 mb-6">
        Add a calendar to see today's events.
      </p>

      <div className="space-y-3 mb-6">
        {/* iCal URL option */}
        <button
          onClick={() => setSourceType("ics_url")}
          className={`w-full text-left p-3 rounded-lg border transition-colors ${
            sourceType === "ics_url"
              ? "border-brand-500 bg-brand-950/30"
              : "border-gray-800 bg-gray-900 hover:border-gray-700"
          }`}
        >
          <div className="flex items-center gap-3">
            <div className={`w-4 h-4 rounded-full border-2 ${
              sourceType === "ics_url" ? "border-brand-500 bg-brand-500" : "border-gray-600"
            }`}>
              {sourceType === "ics_url" && (
                <div className="w-full h-full rounded-full bg-brand-500" />
              )}
            </div>
            <div>
              <p className="text-sm font-medium text-gray-200">iCal URL</p>
              <p className="text-xs text-gray-500">Google Calendar, Outlook, etc.</p>
            </div>
          </div>
        </button>

        {sourceType === "ics_url" && (
          <div className="pl-7">
            <Input
              type="url"
              placeholder="https://calendar.google.com/calendar/ical/..."
              value={icsUrl}
              onChange={(e) => setIcsUrl(e.target.value)}
            />
            <p className="text-xs text-gray-500 mt-2">
              In Google Calendar: Settings → Your calendar → "Secret address in iCal format"
            </p>
          </div>
        )}

        {/* Local file option */}
        <button
          onClick={() => setSourceType("ics_file")}
          className={`w-full text-left p-3 rounded-lg border transition-colors ${
            sourceType === "ics_file"
              ? "border-brand-500 bg-brand-950/30"
              : "border-gray-800 bg-gray-900 hover:border-gray-700"
          }`}
        >
          <div className="flex items-center gap-3">
            <div className={`w-4 h-4 rounded-full border-2 ${
              sourceType === "ics_file" ? "border-brand-500 bg-brand-500" : "border-gray-600"
            }`}>
              {sourceType === "ics_file" && (
                <div className="w-full h-full rounded-full bg-brand-500" />
              )}
            </div>
            <div>
              <p className="text-sm font-medium text-gray-200">Local .ics file</p>
              <p className="text-xs text-gray-500">Exported calendar file</p>
            </div>
          </div>
        </button>

        {sourceType === "ics_file" && (
          <div className="pl-7">
            <Input
              type="text"
              placeholder="/path/to/calendar.ics"
              value={icsPath}
              onChange={(e) => setIcsPath(e.target.value)}
            />
          </div>
        )}
      </div>

      {error && (
        <p className="text-sm text-red-400 mb-4">{error}</p>
      )}

      <div className="flex gap-3">
        <Button variant="secondary" onClick={onBack} className="flex-1">
          Back
        </Button>
        <Button onClick={saveAndContinue} loading={saving} className="flex-1">
          {saving ? "Saving..." : "Continue"}
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
