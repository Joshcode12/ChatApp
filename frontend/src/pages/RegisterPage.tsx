import { useState } from "react";
import { useNavigate } from "react-router";
import { register } from "../api/user";
import { getErrorMessage, getFieldErrors } from "../types/api";

export default function RegisterPage() {
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [bio, setBio] = useState("");
  const [avatar, setAvatar] = useState("");
  const [generalError, setGeneralError] = useState<string | null>(null);
  const [fieldErrors, setFieldErrors] = useState<Record<string, string>>({});
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.SubmitEvent) => {
    e.preventDefault();
    setGeneralError(null);
    setFieldErrors({});
    setIsSubmitting(true);
    try {
      await register({ username, password, bio, avatar });
      navigate("/", { replace: true });
    } catch (err) {
      const fields = getFieldErrors(err);
      setFieldErrors(fields);
      if (Object.keys(fields).length === 0) {
        setGeneralError(getErrorMessage(err));
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} noValidate>
      <h1>Register</h1>
      {generalError && <p role="alert">{generalError}</p>}

      <div>
        <input
          type="text"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoComplete="username"
          aria-invalid={!!fieldErrors.username}
        />
        {fieldErrors.username && <p role="alert">{fieldErrors.username}</p>}
      </div>

      <div>
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          autoComplete="new-password"
          aria-invalid={!!fieldErrors.password}
        />
        {fieldErrors.password && <p role="alert">{fieldErrors.password}</p>}
      </div>

      <div>
        <input
          type="text"
          placeholder="Biography"
          value={bio}
          onChange={(e) => setBio(e.target.value)}
          aria-invalid={!!fieldErrors.bio}
        />
        {fieldErrors.bio && <p role="alert">{fieldErrors.bio}</p>}
      </div>

      <div>
        <input
          type="text"
          placeholder="Avatar"
          value={avatar}
          onChange={(e) => setAvatar(e.target.value)}
          aria-invalid={!!fieldErrors.avatar}
        />
      </div>

      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? "Creating account..." : "Register"}
      </button>
    </form>
  );
}
