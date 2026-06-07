import { Routes, Route, Navigate } from 'react-router'
import { ErrorBoundary } from '@/components/ErrorBoundary'
import { Header } from '@/components/Header'
import { AuthGate } from '@/components/AuthGate'
import { StationsPage } from '@/pages/StationsPage'
import { ChargersPage } from '@/pages/ChargersPage'
import { ProfilePage } from '@/pages/ProfilePage'

function App() {
  return (
    <ErrorBoundary>
      <AuthGate>
        <div className="flex h-svh w-full flex-col overflow-hidden">
          <Header />
          <main className="flex flex-1 overflow-hidden">
            <div className="flex-1 overflow-y-auto">
              <Routes>
                <Route path="/" element={<Navigate to="/stations" replace />} />
                <Route path="/stations" element={<StationsPage />} />
                <Route path="/chargers" element={<ChargersPage />} />
                <Route path="/profile" element={<ProfilePage />} />
              </Routes>
            </div>
          </main>
        </div>
      </AuthGate>
    </ErrorBoundary>
  )
}

export default App
