use crate::models::structs::Blog;
use actix_web::{
    post,
    web,
    Error,
    HttpResponse,
};

#[post("/blog")]
async fn create_blog(data: web::Json<Blog>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(data.into_inner()))
}
