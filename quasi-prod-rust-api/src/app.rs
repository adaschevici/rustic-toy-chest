use crate::database::AppState;
use crate::models::{NewTodo, Todo};
use crate::schema::todos;
// use crate::schema::todos;
// use crate::schema::todos::id;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Json, Router,
};
use diesel::prelude::*;
// use diesel::RunQueryDsl;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo))
        //        .route("/todo/:todo_id", delete(delete_todo))
        .route("/todo/:todo_id", put(complete_todo))
        .route("/todo", post(create_todo))
        //        .route("/todo", get(get_all_todos))
        .route("/todo/random", post(create_random_todo))
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http())
}

#[instrument]
async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get connection from the pool.".to_string(),
        )
    })?;
    info!("Creating new Todo record in the db: {:?}", &new_todo);
    let todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create new Todo record.".to_string(),
            )
        })?;
    Ok(Json(todo))
}

#[instrument]
async fn get_todo(
    state: State<Arc<AppState>>,
    Path(todo_id): Path<i32>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get connection from the pool.".to_string(),
        )
    })?;
    info!("Retrieving Todo record from the db: id: {}", &todo_id);
    let todo = todos::dsl::todos
        .find(todo_id)
        .select(Todo::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                format!("Todo with id: {} not found.", todo_id),
            )
        })?;
    Ok(Json(todo))
}

#[derive(serde::Deserialize, Debug)]
pub struct Activity {
    pub activity: String,
    #[serde(alias = "type")]
    pub activity_type: String,
}

#[instrument]
async fn create_random_todo(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let random_activity: Activity = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get random activity.".to_string(),
            )
        })?
        .json()
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse random activity.".to_string(),
            )
        })?;
    info!("Random activity: {:?}", &random_activity);

    let new_todo = NewTodo {
        title: random_activity.activity,
        body: random_activity.activity_type,
    };

    let mut conn = state.pool.get().map_err(internal_error)?;
    info!("Creating new Todo record in the db: {:?}", &new_todo);

    let res = diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[instrument]
async fn complete_todo(
    state: State<Arc<AppState>>,
    Path(todo_id): Path<i32>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;
    info!("Completing Todo record in the db: id: {}", &todo_id);
    let todo = diesel::update(todos::dsl::todos.find(todo_id))
        .set(todos::completed.eq(true))
        .get_result(&mut conn)
        .map_err(internal_error)?;
    Ok(Json(todo))
}

#[instrument]
async fn delete_todo(
    state: State<Arc<AppState>>,
    Path(todo_id): Path<i32>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;
    info!("Deleting Todo record from the db: id: {}", &todo_id);
    let todo = diesel::delete(todos::dsl::todos.find(todo_id))
        .get_result(&mut conn)
        .map_err(internal_error)?;
    Ok(Json(todo))
}

#[instrument]
async fn get_all_todos(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;
    info!("Retrieving all Todo records from the db");
    let todos = todos::dsl::todos
        .select(Todo::as_select())
        .load(&mut conn)
        .map_err(internal_error)?;
    Ok(Json(todos))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
