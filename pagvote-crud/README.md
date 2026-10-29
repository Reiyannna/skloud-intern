# Pagvote CRUD — Onboarding Demo (local only)

Simple full-stack CRUD: Rust + Axum backend, Vue + Nuxt frontend.
One entity, Contestants. No database, no auth, no GitHub push — local demo only.

## What it does

- Create, list, read, update, and delete contestants
- Backend validates: name must not be blank, contestant_number must be at least 1
- Missing id returns 404 JSON: `{"error": "Contestant not found"}`
- Data is in-memory: restarting the backend restores the 3 seed contestants
  (Maya Reyes #1, Liam Torres #2, Sofia Cruz #3)

## Run it (two terminals, from this folder)

Terminal 1 — backend (port 3001):
```
cd backend
cargo run
```
Check: `http://127.0.0.1:3001/health` should return `{"status":"ok"}`

Terminal 2 — frontend (port 3000):
```
cd frontend
npm install
npm run dev
```
Open: http://localhost:3000

Stop either server with Ctrl+C.

## API

| Method | Path | Success |
| --- | --- | --- |
| GET | /health | 200 |
| GET | /api/contestants | 200, list sorted by id |
| GET | /api/contestants/{id} | 200, or 404 |
| POST | /api/contestants | 201 |
| PUT | /api/contestants/{id} | 200, or 404 |
| DELETE | /api/contestants/{id} | 204, or 404 |

Example:
```
curl http://127.0.0.1:3001/api/contestants
curl -X POST http://127.0.0.1:3001/api/contestants \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Name","contestant_number":9,"bio":"Demo"}'
```

## Architecture walkthrough (for the presentation)

Backend (`backend/src/`):
- `main.rs` — binds 127.0.0.1:3001, registers routes, applies CORS.
  CORS allows only http://localhost:3000, only GET/POST/PUT/DELETE,
  only the Content-Type header. It is intentionally not permissive.
- `models.rs` — `Contestant`, `ContestantInput`, validation, error shape.
- `store.rs` — in-memory `HashMap` + atomic id counter, behind `RwLock`.
  This is the one file a database would replace later.
- `handlers.rs` — one handler per endpoint, returning the status codes above.

Frontend (`frontend/app/`):
- `pages/index.vue` — list, create form, edit form, delete with confirm.
  No business logic about URLs lives here.
- `composables/useContestants.ts` — all 4 API calls in one place.
  The page calls `list/create/update/remove` and never builds a URL itself.
- `nuxt.config.ts` — `runtimeConfig.public.apiBase = http://localhost:3001`.

Request flow: page -> composable ($fetch) -> Axum route -> handler ->
validation -> store -> JSON response -> page refreshes the list.

## Known limits (say these out loud in the demo)

- In-memory only: data is lost on backend restart.
- No auth, no pagination, no database — all deliberate for a simple demo.
- CORS is local-only; a deployed version would need a real origin list,
  auth, and a database layer in place of `store.rs`.
- Plain CSS, no Tailwind, to keep the Nuxt install minimal and reliable.

## Versions verified for this build

- Nuxt 4.6.3, Axum 0.8.9, tokio 1.54.1, serde 1.0.229,
  serde_json 1.0.154, tower-http 0.7.1, Rust 1.99.1, Node 24.22.1
