use diesel::prelude::*;
use crate::schema::url_tokens;

#[derive(Queryable)]
pub struct UrlToken {
    pub id: i32,
    pub url: String,
    pub short_token: String,
}

#[derive(Insertable)]
#[diesel(table_name = url_tokens)]
pub struct NewPost<'a> {
    pub url: &'a str,
    pub short_token: &'a str,
}