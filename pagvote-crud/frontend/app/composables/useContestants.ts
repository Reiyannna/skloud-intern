// All backend calls live here, so the page never hard-codes a URL
// and the walkthrough can point at one file for "the API layer".
export interface Contestant {
  id: number
  name: string
  contestant_number: number
  bio: string
}

export interface ContestantInput {
  name: string
  contestant_number: number
  bio: string
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
