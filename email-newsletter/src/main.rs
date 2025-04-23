use std::net::TcpListener;
use zero2prod::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind 8000 port");
    let server = run_server(listener).expect("failed to bind address");
    server.await
}
