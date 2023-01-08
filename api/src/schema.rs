// @generated automatically by Diesel CLI.

diesel::table! {
    produto (id) {
        id -> Unsigned<Bigint>,
        nome -> Varchar,
        preco -> Decimal,
        descricao -> Varchar,
        foto -> Longtext,
        formatoImagem -> Varchar,
        dataCriacao -> Timestamp,
    }
}
