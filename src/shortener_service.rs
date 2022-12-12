use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use log::debug;
use crate::{db_module, utils};
use crate::models::UrlToken;

#[derive(Deserialize)]
struct ShortUrlRequest {
    url_to_short: String,
}

#[derive(Deserialize)]
struct GetUrlByTokenRequest {
    short_token: String,
}

async fn short_url(req: web::Query<ShortUrlRequest>) -> impl Responder {
    debug!("short url req: {}", req.url_to_short);

    // TODO consider that diesel is blocking
    let url_token = db_module::create_url_token(
        &req.url_to_short,
        &utils::gen_token()
    );

    match url_token {
        None => {
            HttpResponse::ServiceUnavailable().body("")
        }
        Some(ut) => {
            HttpResponse::Created().body(format!("{}", ut.short_token))
        }
    }
}

async fn get_url_by_token(req: web::Query<GetUrlByTokenRequest>) -> impl Responder {
    debug!("get url by token: {}", req.short_token);

    // TODO consider that diesel is blocking
    let url_token = db_module::get_urltoken_by_token(&req.short_token);

    match url_token {
        None => {
            HttpResponse::NotFound().body(format!("url for token {} not found", req.short_token))
        }
        Some(ut) => {
            HttpResponse::Ok().body(format!("{}", ut.url))
        }
    }


}

// REMINDER it possible to use actix_web_lab::Redirect::new("/old", "/new") for redirect

pub fn route_shortener_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // .route("/short_url", web::post().to(|| async { HttpResponse::Ok().body("test") }))
            .route("/short_url", web::post().to(short_url))
            .route("/get_url", web::get().to(get_url_by_token))

            // .route("/", web::head().to(HttpResponse::MethodNotAllowed)),
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