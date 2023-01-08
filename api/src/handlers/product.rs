use actix_web::{get, post, patch, delete, web, HttpResponse, Error};
use crate::{models::product::ProductPayload, ops::product::*};
use crate::DbPool;

#[get("/products")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let products = web::block(move || {
        let conn = pool.get()?;
        find_all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

#[get("/products/{id}")]
async fn select(pool: web::Data<DbPool>, id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get()?; 
        find_by_id(id.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[post("/products")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<ProductPayload>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get()?;
        add_product(payload.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(product))
}

#[patch("/products/{id}")]
async fn update(pool: web::Data<DbPool>, id: web::Path<u64>, payload: web::Json<ProductPayload>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get()?;
        update_product(id.into_inner(), payload.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[delete("/products/{id}")]
async fn delete(pool: web::Data<DbPool>, id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get()?;
        delete_product(id.into_inner(), &conn)
    })
    .await?
    .map(|_product| HttpResponse::Ok().json(_product))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(product)
}
