use chrono;
use crate::schema::produto;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: u64,
    pub nome: String,
    pub preco: f64,
    pub descricao: String,
    pub foto: String,
    pub formatoImagem: String,
    pub dataCriacao: chrono::NaiveDateTime
}

#[derive(Debug, Insertable)]
#[table_name = "produto"]
pub struct NewProduct {
    pub nome: String,
    pub preco: f64,
    pub descricao: String,
    pub foto: String,
    pub formatoImagem: String,
    pub dataCriacao: chrono::NaiveDateTime
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "produto"]
pub struct ProductPayload {
    pub nome: String,
    pub preco: f64,
    pub descricao: String,
    pub foto: String,
    pub formatoImagem: String
}
