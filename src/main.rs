mod shortener_service;
mod web_ui;
mod models;
mod schema;
mod db_module;
mod utils;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::{info};
use shortener_service::route_shortener_service;
use web_ui::route_static_files;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Service started");

    HttpServer::new(|| {
        App::new()
            .configure(route_shortener_service)
            .configure(route_static_files)
            .service(hello)
    })
        // .workers(4)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}