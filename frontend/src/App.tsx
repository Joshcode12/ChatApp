import { Routes, Route } from "react-router";
import { ProtectedRoute } from "./components/ProtectedRoute";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import EmptyState from "./pages/EmptyState";
import AppShell from "./components/layout/AppShell";

export default function App() {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route path="/register" element={<RegisterPage />} />

      <Route element={<ProtectedRoute />}>
        <Route path="/" element={<AppShell />}>
          <Route index element={<EmptyState />} />
          <Route
            path="rooms/:roomId"
            element={<div>Room page placeholder</div>}
          />
        </Route>
      </Route>
    </Routes>
  );
}
