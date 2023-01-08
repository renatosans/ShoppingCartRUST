use chrono;
use crate::schema::produto;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Insertable)]
#[table_name = "produto"]
pub struct NewProduct {
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "produto"]
pub struct ProductPayload {
    pub name: String,
    pub email: String
}
