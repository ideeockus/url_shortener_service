use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use log::debug;
use crate::{db_module, utils};

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

async fn autoredirect_to_url(req: web::Path<GetUrlByTokenRequest>) -> impl Responder {
    debug!("attempt to redirect by token: {}", req.short_token);

    // TODO consider that diesel is blocking
    let url_token = db_module::get_urltoken_by_token(&req.short_token);

    match url_token {
        None => {
            HttpResponse::ServiceUnavailable().body("")
        }
        Some(ut) => {
            // there can be recursive redirection if ut.url is another token
            HttpResponse::Found().append_header(("Location", ut.url)).finish()
        }
    }
}

pub fn route_shortener_service(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
        web::scope("/api")
            .route("/short_url", web::post().to(short_url))
            .route("/get_url", web::get().to(get_url_by_token))
        )
        .route("/{short_token}", web::get().to(autoredirect_to_url));
}
