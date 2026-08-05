import { Shield } from 'lucide-react';

export function Header() {
  return (
    <header className="sticky top-0 z-50 flex h-16 items-center justify-between border-b border-border bg-surface/50 px-6 backdrop-blur-md">
      <div className="flex items-center gap-3">
        <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/20 text-primary">
          <Shield size={22} strokeWidth={2.5} />
        </div>
        <div>
          <h2 className="text-sm font-semibold tracking-wide text-slate-200">Antigraviiti Extension Protect</h2>
          <p className="text-xs text-slate-500 font-medium">Desktop Agent Core</p>
        </div>
      </div>
    </header>
  );
}
