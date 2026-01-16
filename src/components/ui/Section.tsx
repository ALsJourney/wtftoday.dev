import { ReactNode } from "react";
import { LoadingSpinner } from "./LoadingSpinner";

interface SectionProps {
  title: string;
  icon?: ReactNode;
  badge?: number;
  loading?: boolean;
  error?: string;
  onRefresh?: () => void;
  children: ReactNode;
}

export function Section({ title, icon, badge, loading, error, children }: SectionProps) {
  return (
    <div className="mb-6">
      <div className="flex items-center gap-2 mb-3">
        {icon && <span className="text-gray-400">{icon}</span>}
        <h2 className="text-sm font-semibold text-gray-300 uppercase tracking-wide">{title}</h2>
        {badge !== undefined && badge > 0 && (
          <span className="inline-flex items-center justify-center px-2 py-0.5 text-xs font-medium bg-brand-600 text-white rounded-full">
            {badge}
          </span>
        )}
        {loading && <LoadingSpinner size="sm" />}
      </div>

      {error ? (
        <div className="bg-red-900/20 border border-red-800 rounded-lg p-3">
          <p className="text-sm text-red-400">{error}</p>
        </div>
      ) : (
        children
      )}
    </div>
  );
}
