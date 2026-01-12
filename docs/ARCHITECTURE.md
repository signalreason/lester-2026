# Architecture

## Overview
Lester is a local-first research browser. A Rust core library owns the domain
model, SQLite storage, tagging rules, and sync logic. A Rust daemon (`browserd`)
exposes a local JSON API to a TypeScript UI. A background `llm-worker` enriches
bookmarks with AI-generated tags.

## Components
- `lester-core` (Rust): models, storage, tagging rules, sync types.
- `browserd` (Rust): local HTTP API, tag job queue, workspace routing.
- `llm-worker` (Rust): pulls tag jobs, produces AI tags.
- `ui` (TypeScript): workspaces, tab sets, tag clouds, search.

## Data flow
1. User saves a bookmark in the UI.
2. `browserd` stores the bookmark in SQLite and enqueues a tag job.
3. `llm-worker` reads the job, generates tags, and writes them back.
4. UI refreshes tags and the tag cloud.

## Storage layout
SQLite tables:
- `workspaces`, `bookmarks`, `tags`, `bookmark_tags`, `tag_jobs`.

## Sync
The core sync model uses an append-only op log (see `docs/SYNC_PROTOCOL.md`).
Merge is last-write-wins with field-level conflict reporting.

## Security and privacy
- Local-first by default, no telemetry.
- Sync payloads are encrypted; device keys are scoped per device.
