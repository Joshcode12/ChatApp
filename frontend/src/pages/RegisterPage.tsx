import { useState } from "react";
import { useNavigate } from "react-router";
import { getErrorMessage } from "../types/api";
import { register } from "../api/user";

export default function RegisterPage() {
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.SubmitEvent) => {
    e.preventDefault();
    setError(null);
    setIsSubmitting(true);
    try {
      await register({ username, password });
      navigate("/", { replace: true });
      window.location.reload();
    } catch (err) {
      setError(getErrorMessage(err));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h1>Register</h1>
      {error && <p role="alert">{error}</p>}
      <input
        type="text"
        placeholder="Username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        autoComplete="username"
        required
      />
      <input
        type="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        autoComplete="new-password"
        required
      />
      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? "Creating account..." : "Register"}
      </button>
    </form>
  );
}
