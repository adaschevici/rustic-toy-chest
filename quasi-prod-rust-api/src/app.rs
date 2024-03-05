use crate::database::AppState;
use axum::extract::Path;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo))
        .route("/todo/:todo_id", delete(delete_todo))
        .route("/todo/:todo_id", put(complete_todo))
        .route("/todo", post(create_todo))
        .route("/todo", get(get_all_todos))
        .route("/todo/random", post(create_random_todo))
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http())
}

#[instrument]
async fn get_todo(
    Path(todo_id): Path<String>,
    state: Data<Arc<AppState>>,
) -> Result<Json<Todo>, StatusCode> {
    let todo_id = todo_id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let todo = state
        .get_todo_by_id(todo_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todo))
}
