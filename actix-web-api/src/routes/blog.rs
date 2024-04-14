use crate::db;
use crate::models::structs::{Blog, Id};
use crate::utils::handle_sql_error;
use actix_web::{
    delete, get, post, put,
    web::{Json, Query},
    Error as ActixError, HttpResponse,
};
use sqlx::Error;

#[post("/blog")]
async fn create_blog(data: Json<Blog>) -> Result<HttpResponse, ActixError> {
    match db::connect().await {
        Ok(pg) => {
            let returned_blog: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                INSERT INTO blog (title, slug, content, image_link, thumbnail_link, featured)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, title, slug, content, image_link, thumbnail_link, featured,
                    to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                    to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                "#,
                data.title,
                data.slug,
                data.content,
                data.image_link,
                data.thumbnail_link,
                data.featured
            )
            .fetch_one(&pg)
            .await;
            match returned_blog {
                Ok(blog) => Ok(HttpResponse::Ok().json(blog)),
                Err(e) => Ok(handle_sql_error(e)),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}

#[get("/blog")]
async fn get_blog_by_id_or_all(Query(id): Query<Id>) -> Result<HttpResponse, ActixError> {
    println!("id: {:?}", id);
    Ok(HttpResponse::Ok().json(id))
}

#[get("/blog/featured")]
async fn get_featured_blogs() -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::Ok().json("featured blogs"))
}

#[put("/blog")]
async fn update_blog(data: Json<Blog>) -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::Ok().json(data.into_inner()))
}

#[delete("/blog")]
async fn delete_blog(id: Json<Id>) -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::Ok().json(id.into_inner()))
}
