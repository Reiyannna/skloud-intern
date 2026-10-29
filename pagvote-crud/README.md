# Pagvote CRUD — Onboarding Demo

Simple full-stack CRUD: Rust + Axum backend, Vue + Nuxt frontend.
One entity, Contestants. No database, no auth.

## What it does

- Create, list, read, update, and delete contestants
- Contestant fields: `id`, `name`, `contestantNumber`, `bio`, `status`
- Status is only `In-Review` or `Approved` — anything else is rejected
- Backend validates: name must not be blank and at most 100 characters,
  `contestantNumber` must be at least 1, bio at most 500 characters
- `contestantNumber` must be unique: creating or updating to a used number
  returns 409 `{"error": "contestantNumber already in use"}`
- Missing id returns 404 JSON: `{"error": "Contestant not found"}`
- Updating a missing id is always 404, even if the requested number is used
- Data is in-memory: restarting the backend restores the 3 seed contestants
  (Maya Reyes #1 In-Review, Liam Torres #2 Approved, Sofia Cruz #3 In-Review)
- Frontend: contestants in a scrollable card (no page scroll), an
  Add-contestant button that shows the form while the list stays visible,
  and All / In-Review / Approved filters

## Naming decision

Rust fields are snake_case (Rust convention). The JSON wire format and
all TypeScript code are camelCase (JS/TS convention). The backend maps
between them with `#[serde(rename_all = "camelCase")]` on the models,
so Rust `contestant_number` becomes JSON `contestantNumber`.

## Prerequisites — install these before you run anything

- Rust + Cargo (install via rustup: https://rustup.rs)
- Node.js 22.21+ or 24.11+ — required by Nuxt 4.6.3. Node 20 is not enough.
  Install the current LTS from https://nodejs.org
- On Windows only: Visual Studio Build Tools with the
  "Desktop development with C++" workload, installed before Rust.
  Without it, `cargo run` fails at linking with a `link.exe` error.
  You do not need the full Visual Studio IDE — VS Code is enough to code in.
- Git, and a terminal in VS Code (Ctrl + `)

## Run it (two terminals, starting in this folder)

This project lives in a subfolder when cloned as part of a larger repo.
From the repo root, first go into this project folder:
