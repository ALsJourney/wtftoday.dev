import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { AllSettings, CalendarConfig } from "../types";

interface SettingsStore {
  settings: AllSettings | null;
  loading: boolean;
  error: string | null;

  loadSettings: () => Promise<void>;
  updateGitHubToken: (token: string) => Promise<void>;
  updateCalendarConfig: (config: CalendarConfig) => Promise<void>;
  clearCache: () => Promise<void>;
  markOnboardingComplete: () => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  settings: null,
  loading: false,
  error: null,

  loadSettings: async () => {
    set({ loading: true, error: null });
    try {
      const settings = await invoke<AllSettings>("get_all_settings");
      set({ settings, loading: false });
    } catch (err) {
      set({ loading: false, error: String(err) });
    }
  },

  updateGitHubToken: async (token: string) => {
    try {
      const user = await invoke<{ login: string }>("validate_github_token", { token });
      await invoke("save_github_token", { token });

      const currentSettings = get().settings;
      if (currentSettings) {
        set({
          settings: {
            ...currentSettings,
            github_configured: true,
            github_username: user.login,
          },
        });
      }
    } catch (err) {
      throw new Error(`Failed to update GitHub token: ${err}`);
    }
  },

  updateCalendarConfig: async (config: CalendarConfig) => {
    try {
      await invoke("save_calendar_config", { config });

      const currentSettings = get().settings;
      if (currentSettings) {
        set({
          settings: {
            ...currentSettings,
            calendar_config: config,
          },
        });
      }
    } catch (err) {
      throw new Error(`Failed to update calendar config: ${err}`);
    }
  },

  clearCache: async () => {
    try {
      await invoke("clear_cache");
    } catch (err) {
      throw new Error(`Failed to clear cache: ${err}`);
    }
  },

  markOnboardingComplete: async () => {
    try {
      await invoke("set_setting", { key: "onboarding_complete", value: "true" });

      const currentSettings = get().settings;
      if (currentSettings) {
        set({
          settings: {
            ...currentSettings,
            onboarding_complete: true,
          },
        });
      }
    } catch (err) {
      throw new Error(`Failed to mark onboarding complete: ${err}`);
    }
  },
}));
