# Lester 2026

AI-first research browser optimized for tagging, workspaces, and privacy.

## Purpose
- Provide a local-first research browser with tagging and workspace workflows.

## Goals
- Keep data private by default while supporting optional encrypted sync.
- Expose a local API for UI workflows and automation.
- Automate tag enrichment through background jobs.

## Architecture at a glance
- Rust core for storage, tagging rules, and sync logic.
- `browserd` daemon exposes a local JSON API.
- TypeScript UI renders workspaces, tag clouds, and research flows.
- `llm-worker` runs background tag enrichment jobs.

## Quickstart (local dev)
1. Start the API server:
   - `cargo run -p browserd`
2. In a second terminal, run the tagging worker:
   - `cargo run -p llm-worker -- --once`
3. Run the UI:
   - `cd apps/ui`
   - `npm install`
   - `npm run dev`

The API defaults to `http://127.0.0.1:7316`. Override with `LESTER_ADDR`.

## Docs
- `docs/REPO_MAP.md`
- `docs/ARCHITECTURE.md`
- `docs/RUST_CORE.md`
- `docs/UI.md`
- `docs/TAGGING_ENGINE.md`
- `docs/SYNC_PROTOCOL.md`
- `docs/LLM_WORKER.md`
- `docs/BUILD.md`
- `docs/PACKAGING_MACOS.md`
- `docs/ROADMAP.md`

## Layout
- `crates/core`: domain models, SQLite storage, tagging rules, sync types.
- `crates/browserd`: local API server.
- `crates/llm-worker`: background tagging worker.
- `apps/ui`: TypeScript UI (Vite).
- `api/openapi.yaml`: API schema.
- `packaging/macos`: macOS packaging notes.
