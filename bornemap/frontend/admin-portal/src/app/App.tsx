import { Routes, Route, Navigate } from "react-router-dom";
import { LoginPage } from "../features/auth/LoginPage";
import { DashboardPage } from "../features/dashboard/DashboardPage";
import { StationsTablePage } from "../features/stations/StationsTablePage";
import { StationEditPage } from "../features/stations/StationEditPage";
import { AdminLayout } from "../components/layout/AdminLayout";
import { useAuthStore } from "../features/auth/authStore";

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const isAuthenticated = useAuthStore((s) => s.isAuthenticated);
  if (!isAuthenticated) return <Navigate to="/login" replace />;
  return <>{children}</>;
}

export function App() {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route
        element={
          <ProtectedRoute>
            <AdminLayout />
          </ProtectedRoute>
        }
      >
        <Route path="/map" element={<DashboardPage />} />
        <Route path="/admin/stations" element={<StationsTablePage />} />
        <Route path="/admin/stations/new" element={<StationEditPage isNew />} />
        <Route path="/admin/stations/:id/edit" element={<StationEditPage />} />
      </Route>
      <Route path="*" element={<Navigate to="/map" replace />} />
    </Routes>
  );
}
