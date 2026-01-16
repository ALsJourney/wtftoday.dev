import { useEffect, useState } from "react";
import { Routes, Route, Navigate, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { DailyBrief } from "./pages/DailyBrief";
import { Settings } from "./pages/Settings";
import { Onboarding } from "./pages/Onboarding";
import { useSettingsStore } from "./store/settingsStore";
import { LoadingSpinner } from "./components/ui";

function App() {
  const navigate = useNavigate();
  const { loadSettings } = useSettingsStore();
  const [checking, setChecking] = useState(true);

  useEffect(() => {
    const init = async () => {
      try {
        await loadSettings();

        // Check if onboarding is complete
        const onboardingComplete = await invoke<boolean>(
          "is_onboarding_complete",
        );

        if (!onboardingComplete) {
          navigate("/onboarding");
        }
      } catch (err) {
        // If backend isn't ready yet, show onboarding
        console.error("Init error:", err);
        navigate("/onboarding");
      } finally {
        setChecking(false);
      }
    };

    init();
  }, []);

  if (checking) {
    return (
      <div className="min-h-screen bg-gray-950 flex items-center justify-center">
        <div className="text-center">
          <LoadingSpinner size="lg" />
          <p className="text-gray-500 text-sm mt-4">Loading...</p>
        </div>
      </div>
    );
  }

  return (
    <Routes>
      <Route path="/" element={<DailyBrief />} />
      <Route path="/settings" element={<Settings />} />
      <Route path="/onboarding" element={<Onboarding />} />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}

export default App;
