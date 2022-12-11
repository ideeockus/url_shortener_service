// @generated automatically by Diesel CLI.

diesel::table! {
    url_tokens (id) {
        id -> Int4,
        url -> Varchar,
        short_token -> Varchar,
    }
}
