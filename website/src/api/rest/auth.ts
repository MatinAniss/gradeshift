import { fetchData, PUBLIC_REST_API_BASE_URL } from "../rest";

interface LoginRequestPayload {
  email: string;
  password: string;
}

interface LoginResponsePayload {
  token: { token: string; createdAt: string; expiresAt: string };
  user: {
    id: string;
    email: string;
    firstName: string;
    lastName: string;
  };
}

export async function login(payload: LoginRequestPayload): Promise<LoginResponsePayload> {
  return await fetchData<LoginResponsePayload>(`${PUBLIC_REST_API_BASE_URL}/auth/login`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(payload) });
}

interface LogoutResponsePayload {
  success: boolean;
}

export async function logout(): Promise<LogoutResponsePayload> {
  return await fetchData<LogoutResponsePayload>(`${PUBLIC_REST_API_BASE_URL}/auth/logout`);
}

interface RefreshResponsePayload {
  token: { token: string; createdAt: string; expiresAt: string };
}

export async function refresh(): Promise<RefreshResponsePayload> {
  return await fetchData<RefreshResponsePayload>(`${PUBLIC_REST_API_BASE_URL}/auth/refresh`);
}
