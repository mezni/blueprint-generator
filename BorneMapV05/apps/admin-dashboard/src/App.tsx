import { Routes, Route, Navigate } from 'react-router'
import { ErrorBoundary } from '@/components/ErrorBoundary'
import { AuthGate } from '@/components/AuthGate'
import { Layout } from '@/components/Layout'
import DashboardPage from '@/pages/DashboardPage'
import PartnersPage from '@/pages/PartnersPage'
import StationsPage from '@/pages/StationsPage'
import ReviewsPage from '@/pages/ReviewsPage'
import UsersPage from '@/pages/UsersPage'

function App() {
  return (
    <ErrorBoundary>
      <AuthGate>
        <Layout>
          <Routes>
            <Route path="/" element={<DashboardPage />} />
            <Route path="/partners" element={<PartnersPage />} />
            <Route path="/stations" element={<StationsPage />} />
            <Route path="/reviews" element={<ReviewsPage />} />
            <Route path="/users" element={<UsersPage />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </Layout>
      </AuthGate>
    </ErrorBoundary>
  )
}

export default App
