import { fetchData, PUBLIC_REST_API_BASE_URL } from "../rest";

interface Task {
  id: string;
  name: string;
}

interface SubjectResponsePayload {
  id: string;
  name: string;
  tasks: Task[];
}

export async function subject(id: string): Promise<SubjectResponsePayload> {
  return await fetchData<SubjectResponsePayload>(`${PUBLIC_REST_API_BASE_URL}/subjects/${id}`);
}
