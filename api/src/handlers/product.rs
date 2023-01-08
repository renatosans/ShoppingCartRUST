use crate::DbPool;
use crate::models::product::*;
use crate::schema::produto::dsl::*;
use crate::models::product::ProductPayload;
use diesel::prelude::*;
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/products")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let products = web::block(move || {
        let conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Vec<Product>, diesel::result::Error> = produto.load::<Product>(&conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}

#[get("/products/{prod_id}")]
async fn select(pool: web::Data<DbPool>, prod_id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Option<Product>, diesel::result::Error> = produto.find(prod_id.into_inner()).first(&conn).optional();
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[post("/products")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<ProductPayload>) -> Result<HttpResponse, Error> {

    let prodPayload = payload.into_inner();
    let new_product = NewProduct {
        nome: prodPayload.nome,
        preco: prodPayload.preco,
        descricao: prodPayload.descricao,
        foto: prodPayload.foto,
        formatoImagem: prodPayload.formatoImagem,
        dataCriacao: chrono::Local::now().naive_local(),
    };

    let product = web::block(move || {
        let conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(produto).values(new_product).execute(&conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(product))
}

#[patch("/products/{prod_id}")]
async fn update(pool: web::Data<DbPool>, prod_id: web::Path<u64>, payload: web::Json<ProductPayload>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::update(produto.find(prod_id.into_inner())).set(payload.into_inner()).execute(&conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}

#[delete("/products/{prod_id}")]
async fn delete(pool: web::Data<DbPool>, prod_id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let product = web::block(move || {
        let conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::delete(produto.find(prod_id.into_inner())).execute(&conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(product))
}
