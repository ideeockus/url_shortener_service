use diesel::dsl::Nullable;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct UrlToken {
    pub id: i32,
    pub url: String,
    pub url_token: String,
}

#[derive(Insertable)]
#[diesel(table_name = url_tokens)]
pub struct NewPost<'a> {
    pub url: &'a str,
    pub url_token: &'a str,
}