export type ApiError = {
  readonly message: string;
  readonly code?: string;
};

export function getErrorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "response" in err) {
    const res = (err as { response?: { data?: ApiError } }).response;
    if (res?.data?.message) return res.data.message;
  }
  return "Something went wrong. Please try again.";
}
