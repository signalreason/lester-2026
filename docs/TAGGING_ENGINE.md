# Bookmark and Tagging Engine

## Rule-based tagging
The deterministic rules extract tags from:
- URL domain
- Title keywords (with stopword filtering)

## LLM enrichment
The `llm-worker` can be swapped to use a local model or a remote API. In the
current scaffold it reuses rule suggestions and tags them as `llm` source.

## Tag cloud
Tag weights are derived from frequency and average confidence. The UI uses the
weights to scale tag sizes.
