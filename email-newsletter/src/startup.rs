use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use crate::routes::{health_check, subscriptions};

pub fn run_server(lts: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(health_check::health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
    })
    .listen(lts)?
    .run();

    Ok(server)
}
