use actix_web::http::StatusCode;
use actix_web::{web::Json, HttpResponse};
use sqlx::Error;

// sqlx errors need to be converted so that they can be sent back to the client

pub fn handle_sql_error(e: Error) -> HttpResponse {
    let status_code = match e {
        Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    let error_message = format!("SQLx error: {:?}", e);
    HttpResponse::build(status_code)
        .content_type("application/json")
        .body(
            serde_json::to_string(&Json(error_message))
                .unwrap_or_else(|err| format!("JSON serialization error: {:?}", err)),
        )
}
