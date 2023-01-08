use crate::models::user::*;
use diesel::prelude::*;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all(conn: &MysqlConnection) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;

    let res = users
        .load::<User>(conn)?;

    Ok(res)
}

pub fn find_by_id(_id: u64, conn: &MysqlConnection) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let res = users
        .find(_id)
        .first(conn)
        .optional()?;

    Ok(res)
}

pub fn add_user(user: UserPayload, conn: &MysqlConnection) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        name: user.name,
        email: user.email,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(users)
        .values(new_user)
        .execute(conn)?;

    Ok(res)
}

pub fn update_user(_id: u64, user: UserPayload, conn: &MysqlConnection) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let res = diesel::update(users.find(_id))
        .set(user)
        .execute(conn)?;

    Ok(res)
}

pub fn delete_user(_id: u64, conn: &MysqlConnection) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let res = diesel::delete(users.find(_id))
        .execute(conn)?;

    Ok(res)
}
