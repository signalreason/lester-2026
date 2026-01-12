# LLM Tagging Worker

## Role
The worker consumes tag jobs created by `browserd`, enriches bookmarks with AI
suggestions, and writes tags back to SQLite.

## Operation
- Polls for `pending` jobs.
- Marks a job `running`.
- Loads the bookmark and generates tags.
- Writes tags and marks the job `done` or `failed`.

## Configuration
- `--db-path`: SQLite path (default `lester.db`).
- `--poll-interval-ms`: polling delay.
- `--batch-size`: jobs per poll.
- `--once`: process a single batch and exit.
