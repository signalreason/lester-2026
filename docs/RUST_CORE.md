# Rust Core

## Purpose
`lester-core` owns the domain model and storage logic. It is intentionally
standalone so the daemon, worker, and future clients can reuse it.

## Modules
- `models.rs`: workspace, bookmark, tag, job structures.
- `storage.rs`: SQLite persistence and queries.
- `tagging.rs`: deterministic tagging rules.
- `sync.rs`: sync op log and merge rules.

## Storage
`SqliteStore` opens the configured database, applies migrations, and exposes
CRUD for workspaces, bookmarks, and tags. Tag jobs are enqueued for AI tagging.

## Tests
Unit tests cover tagging rules and basic storage behavior.
