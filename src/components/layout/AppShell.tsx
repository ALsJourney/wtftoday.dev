import { ReactNode } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";

interface AppShellProps {
  children: ReactNode;
  onRefresh?: () => void;
  refreshing?: boolean;
  lastUpdated?: Date | null;
  showHeader?: boolean;
  showFooter?: boolean;
}

export function AppShell({
  children,
  onRefresh,
  refreshing,
  lastUpdated,
  showHeader = true,
  showFooter = true,
}: AppShellProps) {
  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 flex flex-col">
      {showHeader && (
        <Header
          onRefresh={onRefresh}
          refreshing={refreshing}
          lastUpdated={lastUpdated}
        />
      )}

      <main className="flex-1 overflow-auto">
        {children}
      </main>

      {showFooter && <Footer />}
    </div>
  );
}
