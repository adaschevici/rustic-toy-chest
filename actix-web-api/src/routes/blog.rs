use crate::db;
use crate::models::structs::{Blog, Id};
use actix_web::{
    delete, get, post, put,
    web::{Json, Query},
    Error, HttpResponse,
};

#[post("/blog")]
async fn create_blog(data: Json<Blog>) -> Result<HttpResponse, Error> {
    match db::connect().await {
        Ok(_) => Ok(HttpResponse::Ok().json(data.into_inner())),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}

#[get("/blog")]
async fn get_blog_by_id_or_all(Query(id): Query<Id>) -> Result<HttpResponse, Error> {
    println!("id: {:?}", id);
    Ok(HttpResponse::Ok().json(id))
}

#[get("/blog/featured")]
async fn get_featured_blogs() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("featured blogs"))
}

#[put("/blog")]
async fn update_blog(data: Json<Blog>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(data.into_inner()))
}

#[delete("/blog")]
async fn delete_blog(id: Json<Id>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(id.into_inner()))
}
