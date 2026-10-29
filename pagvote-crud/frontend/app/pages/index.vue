<template>
  <main class="wrap">
    <h1>Contestants</h1>
    <p class="sub">Simple CRUD demo — Rust + Axum backend, Nuxt frontend. Data is in-memory, so restarting the backend restores the 3 seed contestants.</p>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="loading">Loading…</p>

    <section class="card">
      <h2>{{ editing ? 'Edit contestant' : 'Add contestant' }}</h2>
      <form @submit.prevent="onSubmit">
        <label>
          Name
          <input v-model="form.name" required placeholder="e.g. Maya Reyes" />
        </label>
        <label>
          Contestant number
          <input v-model.number="form.contestant_number" type="number" min="1" required />
        </label>
        <label>
          Bio
          <textarea v-model="form.bio" rows="2" placeholder="Optional short bio" />
        </label>
        <div class="row">
          <button type="submit" :disabled="saving">{{ saving ? 'Saving…' : editing ? 'Save changes' : 'Create' }}</button>
          <button v-if="editing" type="button" class="secondary" @click="cancelEdit">Cancel</button>
        </div>
      </form>
    </section>

    <section>
      <ul class="list">
        <li v-for="c in contestants" :key="c.id" class="card">
          <strong>#{{ c.contestant_number }} — {{ c.name }}</strong>
          <p>{{ c.bio || 'No bio yet.' }}</p>
          <small>ID {{ c.id }}</small>
          <div class="row">
            <button @click="startEdit(c)">Edit</button>
            <button class="danger" @click="onDelete(c)">Delete</button>
          </div>
        </li>
      </ul>
      <p v-if="!loading && contestants.length === 0">No contestants yet. Add one above.</p>
    </section>
  </main>
</template>

<script setup lang="ts">
import type { Contestant, ContestantInput } from '~/composables/useContestants'

const { list, create, update, remove } = useContestants()

const contestants = ref<Contestant[]>([])
const loading = ref(true)
const saving = ref(false)
const error = ref('')
const editing = ref<Contestant | null>(null)
const form = reactive<ContestantInput>({ name: '', contestant_number: 1, bio: '' })

function showError(e: unknown, fallback: string) {
  // $fetch errors carry the backend JSON body in e.data when present.
  const data = (e as { data?: { error?: string } })?.data
  error.value = data?.error || fallback
}

async function refresh() {
  loading.value = true
  error.value = ''
  try {
    contestants.value = await list()
  } catch (e) {
    showError(e, 'Could not load contestants. Is the backend running on port 3001?')
  } finally {
    loading.value = false
  }
}

function resetForm() {
  form.name = ''
  form.contestant_number = 1
  form.bio = ''
}

function startEdit(c: Contestant) {
  editing.value = c
  form.name = c.name
  form.contestant_number = c.contestant_number
  form.bio = c.bio
  error.value = ''
}

function cancelEdit() {
  editing.value = null
  resetForm()
}

async function onSubmit() {
  saving.value = true
  error.value = ''
  try {
    if (editing.value) {
      await update(editing.value.id, { ...form })
      editing.value = null
    } else {
      await create({ ...form })
    }
    resetForm()
    await refresh()
  } catch (e) {
    showError(e, 'Save failed.')
  } finally {
    saving.value = false
  }
}

async function onDelete(c: Contestant) {
  if (!confirm(`Delete ${c.name} (#${c.contestant_number})?`)) return
  error.value = ''
  try {
    await remove(c.id)
    await refresh()
  } catch (e) {
    showError(e, 'Delete failed.')
  }
}

onMounted(refresh)
</script>

<style>
.wrap { max-width: 720px; margin: 2rem auto; padding: 0 1rem; font-family: system-ui, sans-serif; color: #1c1c28; }
.sub { color: #555; }
.card { border: 1px solid #ddd; border-radius: 12px; padding: 1rem; margin: 1rem 0; background: #fff; }
label { display: block; margin: 0.6rem 0; font-weight: 600; }
input, textarea { display: block; width: 100%; margin-top: 0.3rem; padding: 0.6rem; border: 1px solid #bbb; border-radius: 8px; font: inherit; }
.row { display: flex; gap: 0.6rem; margin-top: 0.8rem; }
button { padding: 0.55rem 1rem; border: 0; border-radius: 8px; background: #5b21b6; color: #fff; cursor: pointer; }
button.secondary { background: #e5e7eb; color: #111; }
button.danger { background: #dc2626; }
button:disabled { opacity: 0.6; }
.list { list-style: none; padding: 0; }
.error { background: #fee2e2; border: 1px solid #fca5a5; padding: 0.8rem; border-radius: 8px; }
</style>
