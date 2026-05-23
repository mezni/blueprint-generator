import { Routes, Route, Navigate } from "react-router-dom";
import { LoginPage } from "../features/auth/LoginPage";
import { MapView } from "../features/map/MapView";
import { StationsTablePage } from "../features/stations/StationsTablePage";
import { StationEditPage } from "../features/stations/StationEditPage";
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
        path="/map"
        element={
          <ProtectedRoute>
            <MapView />
          </ProtectedRoute>
        }
      />
      <Route
        path="/admin/stations"
        element={
          <ProtectedRoute>
            <StationsTablePage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/admin/stations/new"
        element={
          <ProtectedRoute>
            <StationEditPage isNew />
          </ProtectedRoute>
        }
      />
      <Route
        path="/admin/stations/:id/edit"
        element={
          <ProtectedRoute>
            <StationEditPage />
          </ProtectedRoute>
        }
      />
      <Route path="*" element={<Navigate to="/map" replace />} />
    </Routes>
  );
}
