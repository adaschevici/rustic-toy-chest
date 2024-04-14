use actix_web::{web, App, HttpServer};

pub mod db;
pub mod models;
pub mod routes;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(web::scope("/api/v1").service(routes::blog())))
        .bind(("127.0.0.1", 4242))?
        .run()
        .await
        .expect("ERROR: src/main.rs: server initialization fail");

    Ok(())
}
