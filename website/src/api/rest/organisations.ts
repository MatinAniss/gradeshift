import { fetchData, PUBLIC_REST_API_BASE_URL } from "../rest";

interface Organisation {
  id: string;
  name: string;
}

interface OrganisationsResponsePayload {
  organisations: Organisation[];
}

export async function organisations(): Promise<OrganisationsResponsePayload> {
  return await fetchData<OrganisationsResponsePayload>(`${PUBLIC_REST_API_BASE_URL}/organisations`);
}

interface Subject {
  id: string;
  name: string;
}

interface OrganisationResponsePayload {
  id: string;
  name: string;
  subjects: Subject[];
}

export async function organisation(id: string): Promise<OrganisationResponsePayload> {
  return await fetchData<OrganisationResponsePayload>(`${PUBLIC_REST_API_BASE_URL}/organisations/${id}`);
}
