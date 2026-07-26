import { useAuth } from "../hooks/useAuth";

export default function DashboardPage() {
  const { user, logout } = useAuth();

  return (
    <div>
      <h1>Welcome, {user?.username}</h1>
      <button onClick={logout}>Log out</button>
    </div>
  );
}
