use crate::db;
use crate::models::structs::{Blog, Id};
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
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
                Ok(record) => Ok(HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .json(record)),
                Err(e) => Ok(handle_sql_error(e)),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}

#[get("/blog")]
async fn get_blog_by_id_or_all(Query(id): Query<Id>) -> Result<HttpResponse, ActixError> {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned_blog: Result<Blog, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                    SELECT id, title, slug, content, image_link, thumbnail_link, featured, to_char(created, 'DD Month YYYY HH12:MI AM') as created, to_char(edited, 'DD Month YYYY HH12:MI AM') as edited FROM blog WHERE id = $1 LIMIT 1
                    "#,
                    id.id
                    )
                    .fetch_one(&pg)
                    .await;
                match returned_blog {
                    Ok(record) => Ok(HttpResponse::Ok().json(record)),
                    Err(e) => Ok(handle_sql_error(e)),
                }
            },
            Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
        }
    } else {
        match db::connect().await {
            Ok(pg) => {
                let returned_blogs: Result<Vec<Blog>, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                    SELECT id, title, slug, content, image_link, thumbnail_link, featured, to_char(created, 'DD Month YYYY HH12:MI AM') as created, to_char(edited, 'DD Month YYYY HH12:MI AM') as edited FROM blog
                    "#,
                    )
                    .fetch_all(&pg)
                    .await;
                match returned_blogs {
                    Ok(records) => Ok(HttpResponse::Ok().json(records)),
                    Err(e) => Ok(handle_sql_error(e)),
                }
            },
            Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
        }
    }
}

#[get("/blog/featured")]
async fn get_featured_blogs() -> Result<HttpResponse, ActixError> {
    match db::connect().await {
        Ok(pg) => {
            let returned_blogs: Result<Vec<Blog>, Error> = sqlx::query_as!(
                Blog,
                r#"
                SELECT id, title, slug, content, image_link, thumbnail_link, featured, to_char(created, 'DD Month YYYY HH12:MI AM') as created, to_char(edited, 'DD Month YYYY HH12:MI AM') as edited FROM blog WHERE featured = TRUE LIMIT 3
                "#,
            )
            .fetch_all(&pg)
            .await;
            match returned_blogs {
                Ok(records) => Ok(HttpResponse::Ok().json(records)),
                Err(e) => Ok(handle_sql_error(e)),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}

#[put("/blog")]
async fn update_blog(data: Json<Blog>) -> Result<HttpResponse, ActixError> {
    match db::connect().await {
        Ok(pg) => {
            let returned_blog: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                INSERT INTO blog (id, title, slug, content, image_link, thumbnail_link, featured)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id)
                DO UPDATE
                SET
                    title = EXCLUDED.title,
                    slug = EXCLUDED.slug,
                    content = EXCLUDED.content,
                    image_link = EXCLUDED.image_link,
                    thumbnail_link = EXCLUDED.thumbnail_link,
                    featured = EXCLUDED.featured,
                    edited = now()
                RETURNING id, title, slug, content, image_link, thumbnail_link, featured,
                    to_char(created, 'DD Month YYYY HH12:MI AM') as created,
                    to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                "#,
                data.id,
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
                Ok(record) => Ok(HttpResponse::Ok().json(record)),
                Err(e) => Ok(handle_sql_error(e)),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}

#[delete("/blog")]
async fn delete_blog(id: Json<Id>) -> Result<HttpResponse, ActixError> {
    match db::connect().await {
        Ok(pg) => {
            let returned_blog: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                DELETE FROM blog WHERE id = $1 RETURNING id, title, slug, content, image_link, thumbnail_link, featured, to_char(created, 'DD Month YYYY HH12:MI AM') as created, to_char(edited, 'DD Month YYYY HH12:MI AM') as edited
                "#,
                id.id
            )
            .fetch_one(&pg)
            .await;
            match returned_blog {
                Ok(record) => Ok(HttpResponse::Ok().json(record)),
                Err(e) => Ok(handle_sql_error(e)),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e)),
    }
}
