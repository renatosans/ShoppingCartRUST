// @generated automatically by Diesel CLI.

diesel::table! {
    produto (id) {
        id -> Unsigned<Bigint>,
        nome -> Varchar,
        preco -> Float8,
        descricao -> Varchar,
        foto -> Longtext,
        formatoImagem -> Varchar,
        dataCriacao -> Timestamp,
    }
}
