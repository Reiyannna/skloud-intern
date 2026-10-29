// All backend calls live here, so the page never hard-codes a URL
// and the walkthrough can point at one file for "the API layer".
//
// Naming decision, chosen 2026-10-29 after review:
// Rust fields are snake_case, the JSON wire format and all TypeScript
// code are camelCase. The backend maps between them with
// `#[serde(rename_all = "camelCase")]`, so this file uses
// `contestantNumber`, never `contestant_number`.
export type ContestantStatus = 'In-Review' | 'Approved'

export interface Contestant {
  id: number
  name: string
  contestantNumber: number
  bio: string
  status: ContestantStatus
}

export interface ContestantInput {
  name: string
  contestantNumber: number
  bio: string
  status: ContestantStatus
}

export function useContestants() {
  const config = useRuntimeConfig()
  const base = config.public.apiBase as string

  async function list(): Promise<Contestant[]> {
    return await $fetch<Contestant[]>(`${base}/api/contestants`)
  }

  async function create(input: ContestantInput): Promise<Contestant> {
    return await $fetch<Contestant>(`${base}/api/contestants`, {
      method: 'POST',
      body: input,
    })
  }

  async function update(id: number, input: ContestantInput): Promise<Contestant> {
    return await $fetch<Contestant>(`${base}/api/contestants/${id}`, {
      method: 'PUT',
      body: input,
    })
  }

  async function remove(id: number): Promise<void> {
    await $fetch(`${base}/api/contestants/${id}`, { method: 'DELETE' })
  }

  return { list, create, update, remove }
}
