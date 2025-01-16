import { auth } from "@/hooks/session";

export const PUBLIC_REST_API_BASE_URL = process.env.NEXT_PUBLIC_REST_API_BASE_URL ?? "";

export async function fetchData<T>(url: string, options?: RequestInit): Promise<T> {
  const token = auth.session.token?.token;

  try {
    const response = await fetch(url, {
      ...options,
      headers: {
        ...options?.headers,
        ...(token && { authorization: `Bearer ${token}` })
      }
    });

    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }

    const data: T = await response.json();
    return data;
  } catch (error) {
    console.error("Error fetching data:", error);
    throw error;
  }
}
