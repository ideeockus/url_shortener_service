// fn main() {
//     println!("Hello, world!");
// }

mod shortener_service;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, info, warn, error};
use shortener_service::route_shortener_service;

/*
Generally I need endpoints:
- web ui (static)
- endpoint for <short_url>
- endpoint for <shortened_url> -> redirect

then prepare docker compose: backend + postgresql and then deploy it to server
*/


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    debug!("Test debug logging");

    HttpServer::new(|| {
        App::new()
            .configure(route_shortener_service)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        // .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}