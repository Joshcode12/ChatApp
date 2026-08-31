import { Routes, Route } from "react-router";
import { ProtectedRoute } from "./components/ProtectedRoute";

export default function App() {
  return (
    <Routes>
      <Route element={<ProtectedRoute />}>
      </Route>
    </Routes>
  );
}
