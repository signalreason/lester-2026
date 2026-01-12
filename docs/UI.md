# TypeScript UI

## Goals
- Workspace-driven research flow.
- Fast search and tag navigation.
- Tag clouds that highlight trends.

## Layout
- Left rail: workspaces and tag cloud.
- Main panel: bookmark list and quick actions.
- Focus on readable typography and reduced visual noise.

## Data flow
The UI talks to `browserd` via the local JSON API. The API base URL is
configurable via `VITE_API_URL`.
