use std::net::TcpListener;
use serde::Deserialize;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// https://www.joshmcguigan.com/blog/understanding-serde/
#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<Info>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run_server(lts: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(lts)?
    .run();

    Ok(server)
}
