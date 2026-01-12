# Sync Protocol

## Goals
- Encrypted, transport-agnostic payloads.
- Append-only op log for conflict-safe merges.
- Field-level conflict reporting.

## Message envelope
```json
{
  "device_id": "uuid",
  "ops": [
    {
      "id": "uuid",
      "entity": "bookmark",
      "entity_id": "uuid",
      "field": "title",
      "value": "New Title",
      "timestamp": 1710000000,
      "device_id": "uuid"
    }
  ]
}
```

## Merge rules
- Sort by entity, entity_id, field, and timestamp.
- Apply last-write-wins for conflicting fields.
- If timestamps collide from different devices, emit a conflict record.

## Encryption
Payloads are encrypted with XChaCha20-Poly1305 using per-device keys. Transport
is left to the sync service (HTTP/WebSocket or file-based export).
