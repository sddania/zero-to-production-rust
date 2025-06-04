use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::{self, get_configuration};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run_server;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = get_connection_pool(&configuration);
    let host_listener = get_listener(
        &configuration.application.host,
        &configuration.application.port,
    );
    let sender = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender,
        configuration.email_client.authorization_token,
    );

    run_server(connection_pool, host_listener, email_client)?.await
}

fn get_listener(host: &str, port: &u16) -> TcpListener {
    let address = format!("{}:{}", host, port);
    get_listener_from_address(&address)
}

fn get_listener_from_address(address: &str) -> TcpListener {
    let listener = TcpListener::bind(address).expect("cannot bind address");

    listener
}

fn get_connection_pool(configuration: &configuration::Settings) -> PgPool {
    let conection_string = configuration.database.connection_string();
    let conection_string = conection_string.expose_secret();
    let connection_pool =
        PgPool::connect_lazy(&conection_string).expect("Failed to connect to Postgres.");

    connection_pool
}
