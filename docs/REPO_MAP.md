# Repo Map: lester-2026

## Purpose and scope
Local-first research browser with tagging, workspaces, and privacy-focused sync. Rust core services with a TypeScript UI.

## Quickstart commands
- API server: `cargo run -p browserd`
- Tagging worker: `cargo run -p llm-worker -- --once`
- UI: `cd apps/ui && npm install && npm run dev`

## Top-level map
- `crates/` - Rust services and shared core.
  - `crates/core/` - storage, tagging rules, sync types.
  - `crates/browserd/` - local JSON API server.
  - `crates/llm-worker/` - background tagging worker.
- `apps/` - frontend applications.
  - `apps/ui/` - Vite-based TypeScript UI.
- `api/` - API schema (`openapi.yaml`).
- `docs/` - architecture, build, and protocol docs.
- `packaging/` - packaging notes (macOS).
- `scripts/` - helper scripts.
- `Cargo.toml`, `Cargo.lock` - Rust workspace.
- `README.md` - quickstart and layout.

## Key entry points
- `crates/browserd/` - API service entry point.
- `crates/llm-worker/` - tagging worker entry point.
- `apps/ui/` - frontend app.
- `api/openapi.yaml` - API contract.

## Core flows and data movement
- UI -> `browserd` JSON API -> core storage and tagging rules.
- Tagging jobs -> `llm-worker` -> updates stored tags -> UI refresh.
- Local-first sync uses append-only op log (see docs).

## External integrations
- LLM provider integration for tagging (details in docs).
- SQLite storage under Rust core.

## Configuration and deployment
- Runtime endpoint configurable via `LESTER_ADDR`.
- Packaging notes under `packaging/macos`.

## Common workflows (build/test/release)
- `cargo run -p browserd`
- `cargo run -p llm-worker -- --once`
- `cd apps/ui && npm run dev`

## Read-next list
- `README.md`
- `docs/ARCHITECTURE.md`
- `docs/BUILD.md`
- `docs/UI.md`
- `docs/RUST_CORE.md`
- `api/openapi.yaml`

## Unknowns and follow-ups
- Test commands and CI workflow are not described in the README.
