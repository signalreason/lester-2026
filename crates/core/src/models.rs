use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub url: String,
    pub title: String,
    pub notes: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkTag {
    pub bookmark_id: Uuid,
    pub tag_id: Uuid,
    pub confidence: f32,
    pub source: TagSource,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkInput {
    pub workspace_id: Uuid,
    pub url: String,
    pub title: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSuggestion {
    pub name: String,
    pub confidence: f32,
    pub source: TagSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagSource {
    Rules,
    Llm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCloudEntry {
    pub name: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagJob {
    pub id: Uuid,
    pub bookmark_id: Uuid,
    pub status: TagJobStatus,
    pub attempts: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TagJobStatus {
    Pending,
    Running,
    Done,
    Failed,
}

impl TagJobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagJobStatus::Pending => "pending",
            TagJobStatus::Running => "running",
            TagJobStatus::Done => "done",
            TagJobStatus::Failed => "failed",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "running" => TagJobStatus::Running,
            "done" => TagJobStatus::Done,
            "failed" => TagJobStatus::Failed,
            _ => TagJobStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookmarkFilter {
    pub workspace_id: Option<Uuid>,
    pub tag: Option<String>,
    pub query: Option<String>,
}
