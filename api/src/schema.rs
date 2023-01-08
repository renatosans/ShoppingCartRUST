// @generated automatically by Diesel CLI.

diesel::table! {
    produto (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}
