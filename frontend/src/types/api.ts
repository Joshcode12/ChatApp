export interface FieldError {
  code: string;
  message: string;
  params?: Record<string, unknown>;
}

export interface ApiError {
  error?: string;
  message?: string;
  errors?: Record<string, Array<string | FieldError> | string>;
}

export function getErrorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "response" in err) {
    const res = (err as { response?: { data?: ApiError } }).response;
    const data = res?.data;

    if (!data) return "Something went wrong. Please try again.";

    if (data.errors && typeof data.errors === "object") {
      const firstField = Object.values(data.errors)[0];

      if (Array.isArray(firstField) && firstField.length > 0) {
        const firstErr = firstField[0];
        if (
          typeof firstErr === "object" &&
          firstErr !== null &&
          "message" in firstErr &&
          firstErr.message
        ) {
          return firstErr.message;
        }
        if (typeof firstErr === "string") {
          return firstErr;
        }
      } else if (typeof firstField === "string") {
        return firstField;
      }
    }

    if (typeof data.error === "string" && data.error) return data.error;
    if (typeof data.message === "string" && data.message) return data.message;
  }

  if (err instanceof Error && err.message) {
    return err.message;
  }

  return "Something went wrong. Please try again.";
}

export function getFieldErrors(err: unknown): Record<string, string> {
  if (typeof err === "object" && err !== null && "response" in err) {
    const data = (err as { response?: { data?: ApiError } }).response?.data;

    if (data?.errors && typeof data.errors === "object") {
      const result: Record<string, string> = {};

      for (const [field, errs] of Object.entries(data.errors)) {
        if (Array.isArray(errs) && errs.length > 0) {
          const firstErr = errs[0];
          if (typeof firstErr === "string") {
            result[field] = firstErr;
          } else if (typeof firstErr === "object" && firstErr?.message) {
            result[field] = firstErr.message;
          }
        } else if (typeof errs === "string") {
          result[field] = errs;
        }
      }

      return result;
    }
  }

  return {};
}
