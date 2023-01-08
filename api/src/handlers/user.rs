use actix_web::{get, post, patch, delete, web, HttpResponse, Error};
use crate::{models::user::UserPayload, ops::user::*};
use crate::DbPool;

#[get("/users")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = pool.get()?;
        find_all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn select(pool: web::Data<DbPool>, id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?; 
        find_by_id(id.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<UserPayload>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        add_user(payload.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

#[patch("/users/{id}")]
async fn update(pool: web::Data<DbPool>, id: web::Path<u64>, payload: web::Json<UserPayload>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        update_user(id.into_inner(), payload.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(pool: web::Data<DbPool>, id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        delete_user(id.into_inner(), &conn)
    })
    .await?
    .map(|_user| HttpResponse::Ok().json(_user))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(user)
}
