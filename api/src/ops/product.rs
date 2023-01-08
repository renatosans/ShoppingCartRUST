use diesel::prelude::*;
use crate::models::product::*;


pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all(conn: &MysqlConnection) -> Result<Vec<Product>, DbError> {
    use crate::schema::produto::dsl::*;

    let res = produto
        .load::<Product>(conn)?;

    Ok(res)
}

pub fn find_by_id(_id: u64, conn: &MysqlConnection) -> Result<Option<Product>, DbError> {
    use crate::schema::produto::dsl::*;

    let res = produto
        .find(_id)
        .first(conn)
        .optional()?;

    Ok(res)
}

pub fn add_product(user: ProductPayload, conn: &MysqlConnection) -> Result<usize, DbError> {
    use crate::schema::produto::dsl::*;

    let new_user = NewProduct {
        name: user.name,
        email: user.email,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(produto)
        .values(new_user)
        .execute(conn)?;

    Ok(res)
}

pub fn update_product(_id: u64, user: ProductPayload, conn: &MysqlConnection) -> Result<usize, DbError> {
    use crate::schema::produto::dsl::*;

    let res = diesel::update(produto.find(_id))
        .set(user)
        .execute(conn)?;

    Ok(res)
}

pub fn delete_product(_id: u64, conn: &MysqlConnection) -> Result<usize, DbError> {
    use crate::schema::produto::dsl::*;

    let res = diesel::delete(produto.find(_id))
        .execute(conn)?;

    Ok(res)
}
