use std::path::Path;

use rusqlite::{params, types::Value, Connection, OptionalExtension};
use uuid::Uuid;

use crate::errors::{CoreError, Result};
use crate::models::{
    Bookmark, BookmarkFilter, BookmarkInput, Tag, TagCloudEntry, TagJob, TagJobStatus,
    TagSuggestion, TagSource, Workspace,
};

#[derive(Clone)]
pub struct SqliteStore {
    path: String,
}

impl SqliteStore {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn migrate(&self) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute_batch("PRAGMA foreign_keys = ON;")?;
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS workspaces (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    created_at INTEGER NOT NULL
                );
                CREATE TABLE IF NOT EXISTS bookmarks (
                    id TEXT PRIMARY KEY,
                    workspace_id TEXT NOT NULL,
                    url TEXT NOT NULL,
                    title TEXT NOT NULL,
                    notes TEXT,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );
                CREATE TABLE IF NOT EXISTS tags (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL UNIQUE,
                    created_at INTEGER NOT NULL
                );
                CREATE TABLE IF NOT EXISTS bookmark_tags (
                    bookmark_id TEXT NOT NULL,
                    tag_id TEXT NOT NULL,
                    confidence REAL NOT NULL,
                    source TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    PRIMARY KEY (bookmark_id, tag_id)
                );
                CREATE TABLE IF NOT EXISTS tag_jobs (
                    id TEXT PRIMARY KEY,
                    bookmark_id TEXT NOT NULL,
                    status TEXT NOT NULL,
                    attempts INTEGER NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );",
            )?;
            Ok(())
        })
    }

    pub fn create_workspace(&self, name: &str) -> Result<Workspace> {
        if name.trim().is_empty() {
            return Err(CoreError::InvalidInput("workspace name is empty".to_string()));
        }
        let workspace = Workspace {
            id: Uuid::new_v4(),
            name: name.trim().to_string(),
            created_at: now_ts(),
        };
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO workspaces (id, name, created_at) VALUES (?1, ?2, ?3)",
                params![workspace.id.to_string(), workspace.name, workspace.created_at],
            )?;
            Ok(workspace)
        })
    }

    pub fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, created_at FROM workspaces ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Workspace {
                    id: parse_uuid(row.get::<_, String>(0)?),
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?;
            let mut workspaces = Vec::new();
            for workspace in rows {
                workspaces.push(workspace?);
            }
            Ok(workspaces)
        })
    }

    pub fn create_bookmark(&self, input: BookmarkInput) -> Result<Bookmark> {
        if input.url.trim().is_empty() || input.title.trim().is_empty() {
            return Err(CoreError::InvalidInput("bookmark url or title is empty".to_string()));
        }
        let now = now_ts();
        let bookmark = Bookmark {
            id: Uuid::new_v4(),
            workspace_id: input.workspace_id,
            url: input.url.trim().to_string(),
            title: input.title.trim().to_string(),
            notes: input.notes,
            created_at: now,
            updated_at: now,
        };
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO bookmarks (id, workspace_id, url, title, notes, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    bookmark.id.to_string(),
                    bookmark.workspace_id.to_string(),
                    bookmark.url,
                    bookmark.title,
                    bookmark.notes,
                    bookmark.created_at,
                    bookmark.updated_at
                ],
            )?;
            Ok(bookmark)
        })
    }

    pub fn list_bookmarks(&self, filter: BookmarkFilter) -> Result<Vec<Bookmark>> {
        self.with_conn(|conn| {
            let mut sql =
                "SELECT DISTINCT b.id, b.workspace_id, b.url, b.title, b.notes, b.created_at, b.updated_at FROM bookmarks b"
                    .to_string();

            let BookmarkFilter {
                workspace_id,
                tag,
                query,
            } = filter;

            if tag.is_some() {
                sql.push_str(
                    " INNER JOIN bookmark_tags bt ON b.id = bt.bookmark_id INNER JOIN tags t ON bt.tag_id = t.id",
                );
            }

            let mut conditions = Vec::new();
            let mut params: Vec<Value> = Vec::new();

            if let Some(tag) = tag {
                conditions.push("t.name = ?");
                params.push(Value::from(tag));
            }
            if let Some(workspace_id) = workspace_id {
                conditions.push("b.workspace_id = ?");
                params.push(Value::from(workspace_id.to_string()));
            }
            if let Some(query) = query {
                let needle = format!("%{}%", query);
                conditions.push("(b.title LIKE ? OR b.url LIKE ?)");
                params.push(Value::from(needle.clone()));
                params.push(Value::from(needle));
            }

            if !conditions.is_empty() {
                sql.push_str(" WHERE ");
                sql.push_str(&conditions.join(" AND "));
            }

            sql.push_str(" ORDER BY b.updated_at DESC");
            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
                Ok(Bookmark {
                    id: parse_uuid(row.get::<_, String>(0)?),
                    workspace_id: parse_uuid(row.get::<_, String>(1)?),
                    url: row.get(2)?,
                    title: row.get(3)?,
                    notes: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?;

            let mut bookmarks = Vec::new();
            for bookmark in rows {
                bookmarks.push(bookmark?);
            }
            Ok(bookmarks)
        })
    }

    pub fn get_bookmark(&self, id: Uuid) -> Result<Option<Bookmark>> {
        self.with_conn(|conn| {
            conn.query_row(
                "SELECT id, workspace_id, url, title, notes, created_at, updated_at FROM bookmarks WHERE id = ?1",
                params![id.to_string()],
                |row| {
                    Ok(Bookmark {
                        id: parse_uuid(row.get::<_, String>(0)?),
                        workspace_id: parse_uuid(row.get::<_, String>(1)?),
                        url: row.get(2)?,
                        title: row.get(3)?,
                        notes: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            )
            .optional()
            .map_err(CoreError::from)
        })
    }

    pub fn list_tags(&self) -> Result<Vec<Tag>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, created_at FROM tags ORDER BY name")?;
            let rows = stmt.query_map([], |row| {
                Ok(Tag {
                    id: parse_uuid(row.get::<_, String>(0)?),
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?;
            let mut tags = Vec::new();
            for tag in rows {
                tags.push(tag?);
            }
            Ok(tags)
        })
    }

    pub fn get_tag_cloud(&self, limit: usize) -> Result<Vec<TagCloudEntry>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT t.name, COUNT(*) as count, AVG(bt.confidence) as avg_conf
                 FROM tags t
                 INNER JOIN bookmark_tags bt ON t.id = bt.tag_id
                 GROUP BY t.name
                 ORDER BY count DESC
                 LIMIT ?1",
            )?;
            let rows = stmt.query_map([limit as i64], |row| {
                let name: String = row.get(0)?;
                let count: i64 = row.get(1)?;
                let avg_conf: f64 = row.get(2)?;
                Ok(TagCloudEntry {
                    name,
                    weight: (count as f32) * (avg_conf as f32),
                })
            })?;
            let mut entries = Vec::new();
            for entry in rows {
                entries.push(entry?);
            }
            Ok(entries)
        })
    }

    pub fn upsert_tags_for_bookmark(
        &self,
        bookmark_id: Uuid,
        suggestions: &[TagSuggestion],
    ) -> Result<Vec<Tag>> {
        self.with_conn(|conn| {
            let tx = conn.transaction()?;
            let mut tags = Vec::new();
            for suggestion in suggestions {
                let tag_id: Option<String> = tx
                    .query_row(
                        "SELECT id FROM tags WHERE name = ?1",
                        params![suggestion.name],
                        |row| row.get(0),
                    )
                    .optional()?;

                let tag_id = match tag_id {
                    Some(id) => id,
                    None => {
                        let id = Uuid::new_v4().to_string();
                        tx.execute(
                            "INSERT INTO tags (id, name, created_at) VALUES (?1, ?2, ?3)",
                            params![id, suggestion.name, now_ts()],
                        )?;
                        id
                    }
                };

                tx.execute(
                    "INSERT OR REPLACE INTO bookmark_tags (bookmark_id, tag_id, confidence, source, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        bookmark_id.to_string(),
                        tag_id,
                        suggestion.confidence,
                        suggestion.source.as_str(),
                        now_ts()
                    ],
                )?;

                tags.push(Tag {
                    id: parse_uuid(tag_id),
                    name: suggestion.name.clone(),
                    created_at: now_ts(),
                });
            }
            tx.commit()?;
            Ok(tags)
        })
    }

    pub fn enqueue_tag_job(&self, bookmark_id: Uuid) -> Result<TagJob> {
        let now = now_ts();
        let job = TagJob {
            id: Uuid::new_v4(),
            bookmark_id,
            status: TagJobStatus::Pending,
            attempts: 0,
            created_at: now,
            updated_at: now,
        };
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO tag_jobs (id, bookmark_id, status, attempts, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    job.id.to_string(),
                    job.bookmark_id.to_string(),
                    job.status.as_str(),
                    job.attempts,
                    job.created_at,
                    job.updated_at
                ],
            )?;
            Ok(job)
        })
    }

    pub fn fetch_pending_tag_jobs(&self, limit: usize) -> Result<Vec<TagJob>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, bookmark_id, status, attempts, created_at, updated_at
                 FROM tag_jobs
                 WHERE status = 'pending'
                 ORDER BY created_at ASC
                 LIMIT ?1",
            )?;
            let rows = stmt.query_map([limit as i64], |row| {
                Ok(TagJob {
                    id: parse_uuid(row.get::<_, String>(0)?),
                    bookmark_id: parse_uuid(row.get::<_, String>(1)?),
                    status: TagJobStatus::from_str(&row.get::<_, String>(2)?),
                    attempts: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?;
            let mut jobs = Vec::new();
            for job in rows {
                jobs.push(job?);
            }
            Ok(jobs)
        })
    }

    pub fn update_tag_job_status(&self, id: Uuid, status: TagJobStatus) -> Result<()> {
        let now = now_ts();
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE tag_jobs SET status = ?1, updated_at = ?2, attempts = attempts + 1 WHERE id = ?3",
                params![status.as_str(), now, id.to_string()],
            )?;
            Ok(())
        })
    }

    fn with_conn<T>(&self, f: impl FnOnce(&mut Connection) -> Result<T>) -> Result<T> {
        let mut conn = Connection::open(Path::new(&self.path))?;
        f(&mut conn)
    }
}

impl TagSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagSource::Rules => "rules",
            TagSource::Llm => "llm",
        }
    }
}

fn parse_uuid(value: String) -> Uuid {
    Uuid::parse_str(&value).unwrap_or_else(|_| Uuid::nil())
}

fn now_ts() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_lists_workspaces() {
        let path = std::env::temp_dir().join(format!("lester-test-{}.db", Uuid::new_v4()));
        let store = SqliteStore::new(path.to_string_lossy().to_string());
        store.migrate().unwrap();

        let created = store.create_workspace("Research").unwrap();
        let list = store.list_workspaces().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, created.name);

        let _ = std::fs::remove_file(path);
    }
}
