use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;
use crate::routes::{health_check, subscriptions};


pub fn run_server(lts: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/healthz", web::get().to(health_check::health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(lts)?
    .run();

    Ok(server)
}
