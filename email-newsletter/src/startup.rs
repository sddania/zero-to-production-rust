use crate::{
    configuration::Settings,
    email_client::EmailClient,
    routes::{health_check, subscriptions},
};
use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let listener = get_listener(
            &configuration.application.host,
            &configuration.application.port,
        );

        let port = listener.local_addr().unwrap().port();
        let server = run_server(connection_pool, listener, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run_server(
    db_pool: PgPool,
    host_listener: TcpListener,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/healthz", web::get().to(health_check::health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(host_listener)?
    .run();

    Ok(server)
}

fn get_listener(host: &str, port: &u16) -> TcpListener {
    let address = format!("{}:{}", host, port);
    get_listener_from_address(&address)
}

fn get_listener_from_address(address: &str) -> TcpListener {
    let listener = TcpListener::bind(address).expect("cannot bind address");

    listener
}

pub fn get_connection_pool(configuration: &Settings) -> PgPool {
    let conection_string = configuration.database.connection_string();
    let conection_string = conection_string.expose_secret();
    let connection_pool =
        PgPool::connect_lazy(&conection_string).expect("Failed to connect to Postgres.");

    connection_pool
}
