import React from 'react';
import { Header } from '@/layout/Header';

export function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-screen flex-col bg-background">
      <Header />
      <main className="flex-1 overflow-y-auto">
        {children}
      </main>
    </div>
  );
}
