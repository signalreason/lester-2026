# Build System

## Rust workspace
- `cargo build` builds all Rust crates.
- `cargo run -p browserd` starts the local API.
- `cargo run -p llm-worker -- --once` processes tag jobs.

## TypeScript UI
- `cd apps/ui`
- `npm install`
- `npm run dev` for local UI
- `npm run build` for production assets

## API schema
`api/openapi.yaml` defines the local API. Future work can generate TypeScript
clients from the schema.
