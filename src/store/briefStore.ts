import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { BriefData } from "../types";

interface BriefStore {
  brief: BriefData | null;
  loading: boolean;
  refreshing: boolean;
  error: string | null;
  lastUpdated: Date | null;

  loadBrief: () => Promise<void>;
  refreshBrief: () => Promise<void>;
  clearError: () => void;
}

export const useBriefStore = create<BriefStore>((set) => ({
  brief: null,
  loading: false,
  refreshing: false,
  error: null,
  lastUpdated: null,

  loadBrief: async () => {
    set({ loading: true, error: null });
    try {
      const brief = await invoke<BriefData>("get_brief");
      set({
        brief,
        loading: false,
        lastUpdated: brief.generated_at
          ? new Date(brief.generated_at * 1000)
          : null,
      });
    } catch (err) {
      set({ loading: false, error: String(err) });
    }
  },

  refreshBrief: async () => {
    set({ refreshing: true, error: null });
    try {
      const brief = await invoke<BriefData>("refresh_brief");
      set({
        brief,
        refreshing: false,
        lastUpdated: new Date(),
      });
    } catch (err) {
      set({ refreshing: false, error: String(err) });
    }
  },

  clearError: () => set({ error: null }),
}));
