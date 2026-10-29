<template>
  <div>
    <main class="wrap">
      <header class="header">
        <h1>Contestants</h1>
        <p class="sub">Simple CRUD demo — Rust + Axum backend, Nuxt frontend. Data is in-memory, so restarting the backend restores the 3 seed contestants.</p>
      </header>

      <div class="controls">
        <button @click="toggleForm">{{ showForm ? 'Hide form' : editing ? 'Edit contestant' : 'Add contestant' }}</button>
        <div class="filters" role="group" aria-label="Filter by status">
          <button :class="{ active: filter === 'All' }" @click="filter = 'All'">All</button>
          <button :class="{ active: filter === 'In-Review' }" @click="filter = 'In-Review'">In-Review</button>
          <button :class="{ active: filter === 'Approved' }" @click="filter = 'Approved'">Approved</button>
        </div>
        <span class="count">{{ filteredContestants.length }} of {{ contestants.length }} shown</span>
      </div>

      <section class="card list-card">
        <h2>Contestants</h2>
        <p v-if="loading">Loading…</p>
        <ul class="list">
          <li v-for="c in filteredContestants" :key="c.id" class="item">
            <div class="item-head">
              <strong>#{{ c.contestantNumber }} — {{ c.name }}</strong>
              <span class="badge" :class="c.status === 'Approved' ? 'approved' : 'review'">{{ c.status }}</span>
            </div>
            <p>{{ c.bio || 'No bio yet.' }}</p>
            <small>ID {{ c.id }}</small>
            <div class="row">
              <button @click="startEdit(c)">Edit</button>
              <button class="danger" @click="onDelete(c)">Delete</button>
            </div>
          </li>
        </ul>
        <p v-if="!loading && filteredContestants.length === 0">No contestants match this filter.</p>
      </section>
    </main>

    <!-- Create / Edit prompt card: a centered modal over the list, so the
         contestants stay visible behind it. It does not close on backdrop
         click, so a stray click cannot discard what was typed — only
         Cancel, Hide form, or a successful save closes it. -->
    <section v-if="showForm" class="modal-overlay" role="dialog" aria-modal="true" aria-label="Contestant form">
      <div class="card modal-card">
        <h2>{{ editing ? 'Edit contestant' : 'Add contestant' }}</h2>
        <form @submit.prevent="onSubmit">
          <label>
            Name
            <input v-model="form.name" required placeholder="e.g. Maya Reyes" maxlength="100" />
          </label>
          <label>
            Contestant number
            <input v-model.number="form.contestantNumber" type="number" min="1" required />
          </label>
          <label>
            Status
            <select v-model="form.status">
              <option value="In-Review">In-Review</option>
              <option value="Approved">Approved</option>
            </select>
          </label>
          <label>
            Bio
            <textarea v-model="form.bio" rows="2" placeholder="Optional short bio" maxlength="500" />
          </label>
          <div class="row">
            <button type="submit" :disabled="saving">{{ saving ? 'Saving…' : editing ? 'Save changes' : 'Create' }}</button>
            <button type="button" class="secondary" @click="cancelForm">Cancel</button>
          </div>
        </form>
      </div>
    </section>

    <!-- Error prompt card: errors are no longer an inline banner. They open
         as a centered prompt over everything else, so they cannot be missed
         or scrolled past. OK (or clicking the dimmed background) dismisses
         it and returns to whatever was open — a failed save keeps the form
         open underneath, so the number can be fixed and retried. -->
    <section v-if="error" class="modal-overlay error-overlay" role="alertdialog" aria-modal="true" aria-label="Error" @click.self="error = ''">
      <div class="card modal-card error-card">
        <h2>Something went wrong</h2>
        <p>{{ error }}</p>
        <div class="row">
          <button @click="error = ''">OK</button>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { Contestant, ContestantInput, ContestantStatus } from '~/composables/useContestants'

const { list, create, update, remove } = useContestants()

const contestants = ref<Contestant[]>([])
const loading = ref(true)
const saving = ref(false)
const error = ref('')
const editing = ref<Contestant | null>(null)
const showForm = ref(false)
const filter = ref<'All' | ContestantStatus>('All')
const form = reactive<ContestantInput>({ name: '', contestantNumber: 1, bio: '', status: 'In-Review' })

const filteredContestants = computed(() => {
  if (filter.value === 'All') return contestants.value
  return contestants.value.filter((c) => c.status === filter.value)
})

function showError(e: unknown, fallback: string) {
  // $fetch errors carry the backend body in e.data when present.
  // Most errors are JSON { error: "..." } — a duplicate number arrives
  // as a 409 with "Contestant number already in use". A malformed body
  // (for example an invalid status) arrives as plain text with a 422,
  // so handle a string body too instead of falling back blindly.
  // Setting error opens the error prompt card; it does not close the form.
  const data = (e as { data?: unknown })?.data
  if (typeof data === 'string' && data.trim()) {
    error.value = data.length > 300 ? data.slice(0, 300) : data
  } else if (typeof data === 'object' && data !== null && 'error' in data) {
    error.value = (data as { error: string }).error
  } else {
    error.value = fallback
  }
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
  form.contestantNumber = 1
  form.bio = ''
  form.status = 'In-Review'
}

function toggleForm() {
  if (showForm.value) {
    cancelForm()
  } else {
    editing.value = null
    resetForm()
    showForm.value = true
    error.value = ''
  }
}

function startEdit(c: Contestant) {
  editing.value = c
  form.name = c.name
  form.contestantNumber = c.contestantNumber
  form.bio = c.bio
  form.status = c.status
  showForm.value = true
  error.value = ''
}

function cancelForm() {
  editing.value = null
  showForm.value = false
  resetForm()
}

async function onSubmit() {
  // v-model.number can leave an empty or invalid number as null/NaN.
  // Catch it here with a useful message instead of sending it and
  // getting a generic backend rejection back.
  if (!Number.isInteger(form.contestantNumber) || form.contestantNumber < 1) {
    error.value = 'Contestant number must be at least 1'
    return
  }
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
    showForm.value = false
    await refresh()
  } catch (e) {
    showError(e, 'Save failed.')
  } finally {
    saving.value = false
  }
}

async function onDelete(c: Contestant) {
  if (!confirm(`Delete ${c.name} (#${c.contestantNumber})?`)) return
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
/* No page scrolling: the page itself is fixed to the viewport height,
   and only the contestants card scrolls internally. */
.wrap { max-width: 720px; height: 100dvh; margin: 0 auto; padding: 1rem; box-sizing: border-box; display: flex; flex-direction: column; gap: 0.8rem; overflow: hidden; font-family: system-ui, sans-serif; color: #1c1c28; }
.header { flex-shrink: 0; }
.header h1 { margin: 0 0 0.3rem; }
.sub { color: #555; margin: 0; }
.controls { flex-shrink: 0; display: flex; flex-wrap: wrap; gap: 0.6rem; align-items: center; }
.filters { display: flex; gap: 0.4rem; }
.filters button { background: #e5e7eb; color: #111; }
.filters button.active { background: #5b21b6; color: #fff; }
.count { color: #555; font-size: 0.9rem; }
.card { border: 1px solid #ddd; border-radius: 12px; padding: 1rem; background: #fff; }
.list-card { flex: 1; min-height: 0; overflow-y: auto; }
.list-card h2 { margin-top: 0; }
/* Prompt cards: centered modal over the page, list visible behind the dim. */
.modal-overlay { position: fixed; inset: 0; background: rgba(0, 0, 0, 0.45); display: flex; align-items: center; justify-content: center; padding: 1rem; box-sizing: border-box; z-index: 10; font-family: system-ui, sans-serif; color: #1c1c28; }
.modal-overlay.error-overlay { z-index: 20; }
.modal-card { width: 100%; max-width: 480px; max-height: 90dvh; overflow-y: auto; }
.modal-card h2 { margin-top: 0; }
.error-card { max-width: 420px; }
label { display: block; margin: 0.6rem 0; font-weight: 600; }
input, textarea, select { display: block; width: 100%; margin-top: 0.3rem; padding: 0.6rem; border: 1px solid #bbb; border-radius: 8px; font: inherit; box-sizing: border-box; }
.row { display: flex; gap: 0.6rem; margin-top: 0.8rem; }
button { padding: 0.55rem 1rem; border: 0; border-radius: 8px; background: #5b21b6; color: #fff; cursor: pointer; }
button.secondary { background: #e5e7eb; color: #111; }
button.danger { background: #dc2626; }
button:disabled { opacity: 0.6; }
.list { list-style: none; padding: 0; margin: 0; }
.item { border: 1px solid #eee; border-radius: 10px; padding: 0.8rem; margin: 0.7rem 0; }
.item-head { display: flex; justify-content: space-between; gap: 0.6rem; align-items: center; }
.badge { font-size: 0.8rem; font-weight: 700; padding: 0.25rem 0.6rem; border-radius: 999px; white-space: nowrap; }
.badge.review { background: #fef3c7; color: #92400e; }
.badge.approved { background: #dcfce7; color: #166534; }
</style>
