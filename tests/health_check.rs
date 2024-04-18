use actix_web::rt::spawn;
use reqwest;
use std::net::TcpListener;

/// Helper function that sets up a server and binds it to an address that is
/// returned. This way, individual tests know where to send their requests.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");
    let _ = spawn(server);

    format!("http://127.0.0.1:{port}")
}

/// Server health check test case.
///
/// # Description
///
/// This test case checks that the _health check_ end point works as expected.
/// The test applies a black box philosophy: it performs a request to the server
/// via the public API, as any external client would do.
#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}