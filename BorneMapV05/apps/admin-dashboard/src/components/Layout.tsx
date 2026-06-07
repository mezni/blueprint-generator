import { useState, type ReactNode } from 'react'
import { Header } from '@/components/Header'
import { Sidebar } from '@/components/Sidebar'

interface LayoutProps {
  children: ReactNode
}

export function Layout({ children }: LayoutProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  return (
    <div className="flex h-svh flex-col">
      <Header onToggleSidebar={() => setSidebarOpen((v) => !v)} />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar open={sidebarOpen} onClose={() => setSidebarOpen(false)} />
        <main className="flex-1 overflow-y-auto bg-[var(--color-surface-muted)] p-6">
          {children}
        </main>
      </div>
    </div>
  )
}
