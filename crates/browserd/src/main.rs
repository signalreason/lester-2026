use std::net::SocketAddr;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use lester_core::{BookmarkFilter, BookmarkInput, SqliteStore, TagCloudEntry, TagJob, WorkspaceInput};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    store: SqliteStore,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db_path = std::env::var("LESTER_DB_PATH").unwrap_or_else(|_| "lester.db".to_string());
    let addr = std::env::var("LESTER_ADDR").unwrap_or_else(|_| "127.0.0.1:7316".to_string());

    let store = SqliteStore::new(db_path);
    store.migrate()?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/workspaces", get(list_workspaces).post(create_workspace))
        .route("/bookmarks", get(list_bookmarks).post(create_bookmark))
        .route("/tags", get(list_tags))
        .route("/tag-cloud", get(tag_cloud))
        .with_state(AppState { store });

    let addr: SocketAddr = addr
        .parse()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid bind address"))?;
    info!("browserd listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn list_workspaces(State(state): State<AppState>) -> Result<Json<Vec<lester_core::Workspace>>, AppError> {
    let workspaces = state.store.list_workspaces()?;
    Ok(Json(workspaces))
}

async fn create_workspace(
    State(state): State<AppState>,
    Json(input): Json<WorkspaceInput>,
) -> Result<Json<lester_core::Workspace>, AppError> {
    let workspace = state.store.create_workspace(&input.name)?;
    Ok(Json(workspace))
}

async fn list_bookmarks(
    State(state): State<AppState>,
    Query(query): Query<BookmarkQuery>,
) -> Result<Json<Vec<lester_core::Bookmark>>, AppError> {
    let filter = BookmarkFilter {
        workspace_id: query.workspace_id,
        tag: query.tag,
        query: query.q,
    };
    let bookmarks = state.store.list_bookmarks(filter)?;
    Ok(Json(bookmarks))
}

async fn create_bookmark(
    State(state): State<AppState>,
    Json(input): Json<BookmarkInput>,
) -> Result<Json<CreateBookmarkResponse>, AppError> {
    let bookmark = state.store.create_bookmark(input)?;
    let job = state.store.enqueue_tag_job(bookmark.id)?;
    Ok(Json(CreateBookmarkResponse { bookmark, job }))
}

async fn list_tags(
    State(state): State<AppState>,
) -> Result<Json<Vec<lester_core::Tag>>, AppError> {
    let tags = state.store.list_tags()?;
    Ok(Json(tags))
}

async fn tag_cloud(
    State(state): State<AppState>,
    Query(query): Query<TagCloudQuery>,
) -> Result<Json<Vec<TagCloudEntry>>, AppError> {
    let limit = query.limit.unwrap_or(40);
    let cloud = state.store.get_tag_cloud(limit)?;
    Ok(Json(cloud))
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Deserialize)]
struct BookmarkQuery {
    workspace_id: Option<Uuid>,
    tag: Option<String>,
    q: Option<String>,
}

#[derive(Deserialize)]
struct TagCloudQuery {
    limit: Option<usize>,
}

#[derive(Serialize)]
struct CreateBookmarkResponse {
    bookmark: lester_core::Bookmark,
    job: TagJob,
}

#[derive(Debug)]
enum AppError {
    Core(lester_core::CoreError),
    Other(String),
}

impl From<lester_core::CoreError> for AppError {
    fn from(err: lester_core::CoreError) -> Self {
        AppError::Core(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Core(lester_core::CoreError::InvalidInput(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::Core(lester_core::CoreError::NotFound) => (StatusCode::NOT_FOUND, "not found".to_string()),
            AppError::Core(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
