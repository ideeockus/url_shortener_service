use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use log::debug;

#[derive(Deserialize)]
struct ShortUrlRequest {
    url_to_short: String,
}

async fn short_url(req: web::Query<ShortUrlRequest>) -> impl Responder {
    // put to the database and return token
    debug!("short url req: {}", req.url_to_short);
    HttpResponse::Ok().body(format!("Hey there! {}", req.url_to_short))
}

pub fn route_shortener_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // .route("/short_url", web::post().to(|| async { HttpResponse::Ok().body("test") }))
            .route("/short_url", web::post().to(short_url))
            .route("/get_url", web::get().to(|| async { HttpResponse::Ok().body("test 2") }))
            .route("/", web::head().to(HttpResponse::MethodNotAllowed)),
    );
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{
//         http::{self, header::ContentType},
//         test,
//     };
//
//     #[actix_web::test]
//     async fn test_short_url() {
//         let req = test::TestRequest::default()
//             .set_payload(ShortUrlRequest { url_to_short: "http://127.0.0.1/?param=test".to_string() })
//             .to_http_request();
//         let resp = short_url(req).await;
//         assert_eq!(resp.status(), http::StatusCode::OK);
//     }

    // #[actix_web::test]
    // async fn test_index_not_ok() {
    //     let req = test::TestRequest::default().to_http_request();
    //     let resp = index(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }
// }