use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEnvelope {
    pub device_id: Uuid,
    pub ops: Vec<SyncOp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOp {
    pub id: Uuid,
    pub entity: String,
    pub entity_id: Uuid,
    pub field: String,
    pub value: serde_json::Value,
    pub timestamp: i64,
    pub device_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    pub merged_ops: Vec<SyncOp>,
    pub conflicts: Vec<SyncConflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub entity: String,
    pub entity_id: Uuid,
    pub field: String,
    pub left: SyncOp,
    pub right: SyncOp,
}

pub fn merge_logs(left: &[SyncOp], right: &[SyncOp]) -> MergeResult {
    let mut merged = Vec::new();
    let mut conflicts = Vec::new();

    let mut all_ops: Vec<SyncOp> = left.iter().cloned().chain(right.iter().cloned()).collect();
    all_ops.sort_by_key(|op| (op.entity.clone(), op.entity_id, op.field.clone(), op.timestamp));

    let mut last_map: std::collections::HashMap<(String, Uuid, String), SyncOp> =
        std::collections::HashMap::new();
    for op in all_ops {
        let key = (op.entity.clone(), op.entity_id, op.field.clone());
        if let Some(existing) = last_map.get(&key) {
            if existing.timestamp == op.timestamp && existing.device_id != op.device_id {
                conflicts.push(SyncConflict {
                    entity: op.entity.clone(),
                    entity_id: op.entity_id,
                    field: op.field.clone(),
                    left: existing.clone(),
                    right: op.clone(),
                });
            }
        }
        last_map.insert(key, op);
    }

    merged.extend(last_map.values().cloned());
    merged.sort_by_key(|op| op.timestamp);

    MergeResult {
        merged_ops: merged,
        conflicts,
    }
}
