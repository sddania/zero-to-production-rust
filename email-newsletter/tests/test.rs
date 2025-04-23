// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("{}/healthz", addr);
    // Act
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn greets_works() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("{}", addr);
    // Act
    let response = client.get(url).send().await?;
    // Assert
    assert!(response.status().is_success());
    let text_content = response.text().await?;
    assert_eq!("Hello World!", text_content);

    Ok(())
}

#[tokio::test]
async fn greets_with_name_works() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let url = format!("{}/pippo", addr);
    // Act
    let response = client.get(url).send().await?;
    // Assert
    assert!(response.status().is_success());
    let text_content = response.text().await?;
    assert_eq!("Hello pippo!", text_content);

    Ok(())
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run_server(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
