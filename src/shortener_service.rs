use actix_web::{web, App, HttpResponse, HttpServer};

pub fn route_shortener_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/short_url", web::post().to(|| async { HttpResponse::Ok().body("test") }))
            .route("/get_url", web::get().to(|| async { HttpResponse::Ok().body("test 2") }))
            .route("/", web::head().to(HttpResponse::MethodNotAllowed)),
    );
}