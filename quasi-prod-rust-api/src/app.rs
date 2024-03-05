use crate::database::AppState;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Json, Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::instrument;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo))
        //        .route("/todo/:todo_id", delete(delete_todo))
        //        .route("/todo/:todo_id", put(complete_todo))
        //        .route("/todo", post(create_todo))
        //        .route("/todo", get(get_all_todos))
        //        .route("/todo/random", post(create_random_todo))
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http())
}

#[instrument]
async fn get_todo(
    state: State<Arc<AppState>>,
    Path(todo_id): Path<String>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo_id = todo_id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let todo = state
        .get_todo_by_id(todo_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todo))
}
