use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use log::info;

mod config;
mod auth;
mod cli;
mod web;
mod jwt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting FLUX server...");

    HttpServer::new(|| {
        App::new()
            .configure(web::init_routes)
            .wrap(auth::auth_middleware())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
