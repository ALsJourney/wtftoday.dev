import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { WelcomeStep } from "./WelcomeStep";
import { GitHubStep } from "./GitHubStep";
import { CalendarStep } from "./CalendarStep";
import { CompleteStep } from "./CompleteStep";
import { useSettingsStore } from "../../store/settingsStore";
import type { CalendarConfig, GitHubUser } from "../../types";

export interface OnboardingData {
  github: {
    token: string;
    validated: boolean;
    user: GitHubUser | null;
  };
  calendar: CalendarConfig;
}

const TOTAL_STEPS = 4;

export function Onboarding() {
  const navigate = useNavigate();
  const { markOnboardingComplete } = useSettingsStore();
  const [currentStep, setCurrentStep] = useState(0);
  const [data, setData] = useState<OnboardingData>({
    github: { token: "", validated: false, user: null },
    calendar: { source_type: "none", ics_path: null, ics_url: null },
  });

  const nextStep = () => {
    if (currentStep < TOTAL_STEPS - 1) {
      setCurrentStep(currentStep + 1);
    }
  };

  const prevStep = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  const updateGitHub = (github: OnboardingData["github"]) => {
    setData((prev) => ({ ...prev, github }));
  };

  const updateCalendar = (calendar: CalendarConfig) => {
    setData((prev) => ({ ...prev, calendar }));
  };

  const completeOnboarding = async () => {
    await markOnboardingComplete();
    navigate("/");
  };

  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 flex flex-col">
      {/* Progress bar */}
      <div className="px-6 pt-6">
        <div className="max-w-md mx-auto">
          <div className="flex gap-2">
            {Array.from({ length: TOTAL_STEPS }).map((_, i) => (
              <div
                key={i}
                className={`h-1 flex-1 rounded-full transition-colors ${
                  i <= currentStep ? "bg-brand-500" : "bg-gray-800"
                }`}
              />
            ))}
          </div>
        </div>
      </div>

      {/* Step content */}
      <div className="flex-1 flex items-center justify-center p-6">
        <div className="w-full max-w-md">
          {currentStep === 0 && <WelcomeStep onNext={nextStep} />}
          {currentStep === 1 && (
            <GitHubStep
              data={data.github}
              onUpdate={updateGitHub}
              onNext={nextStep}
              onBack={prevStep}
            />
          )}
          {currentStep === 2 && (
            <CalendarStep
              data={data.calendar}
              onUpdate={updateCalendar}
              onNext={nextStep}
              onBack={prevStep}
            />
          )}
          {currentStep === 3 && (
            <CompleteStep
              data={data}
              onComplete={completeOnboarding}
              onBack={prevStep}
            />
          )}
        </div>
      </div>

      {/* Footer */}
      <footer className="px-6 py-4 text-center">
        <p className="text-xs text-gray-600">Made by nock.ing</p>
      </footer>
    </div>
  );
}
