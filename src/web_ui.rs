use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files::{NamedFile, Files};
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
use actix_web::dev::Path;
use serde::Deserialize;
use log::debug;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    debug!("current dir {:?}", env::current_dir());
    // let path: PathBuf = req.match_info().query("index.html").parse().unwrap();
    let path: PathBuf = env::current_dir().unwrap().join("index.html");
    debug!("index file path {:?}", path);
    Ok(NamedFile::open(path)?)
}

pub fn route_static_files(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/web_ui")
            .route("", web::get().to(index))
            .service(Files::new("/static", ".").show_files_listing())
    );
}