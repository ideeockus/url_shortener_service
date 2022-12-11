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


// /// creating new url short records
// fn add_url_to_db(url: String) -> String {
//     let connection = &mut establish_connection();
//     let results = posts
//         .filter(published.eq(true))
//         .limit(5)
//         .load::<Post>(connection)
//         .expect("Error loading posts");
//
//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.title);
//         println!("-----------\n");
//         println!("{}", post.body);
//     }
// }

pub fn create_url_token(conn: &mut PgConnection, url: &str, url_token: &str) -> UrlToken {
    let new_url_token = NewPost { url, url_token };

    diesel::insert_into(url_tokens::table)
        .values(&new_url_token)
        .get_result(conn)
        .expect("Error saving new post")
}



/// some func
fn get_url_records() {
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
        println!("{}", record.url_token);
    }
}