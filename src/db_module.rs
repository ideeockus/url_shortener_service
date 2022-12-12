use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::*;
use crate::schema::url_tokens::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// TODO r2d2 for connection pool

pub fn create_url_token(new_url: &str, new_short_token: &str) -> Option<UrlToken> {
    let conn: &mut PgConnection = &mut establish_connection();

    let new_url_token = NewPost { url: new_url, short_token: new_short_token };

    diesel::insert_into(url_tokens)
        .values(&new_url_token)
        .get_result(conn)
        .ok()
}

pub fn get_urltoken_by_token(short_token_param: &str) -> Option<UrlToken> {
    let conn: &mut PgConnection = &mut establish_connection();

    url_tokens
        .filter(short_token.eq(short_token_param))
        .first::<UrlToken>(conn)
        .optional()
        .expect("Error loading url")
}



fn print_url_records() {
    let connection = &mut establish_connection();
    let results: Vec<UrlToken> = url_tokens
        // .filter(published.eq(true))
        .limit(5)
        .load::<UrlToken>(connection)
        .expect("Error loading url records");

    println!("Displaying {} records", results.len());
    for record in results {
        println!("{}", record.url);
        println!("-----------\n");
        println!("{}", record.short_token);
    }
}