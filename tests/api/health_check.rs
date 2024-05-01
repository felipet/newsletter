use crate::helpers::spawn_app;

/// Server health check test case.
///
/// # Description
///
/// This test case checks that the _health check_ end point works as expected.
/// The test applies a black box philosophy: it performs a request to the server
/// via the public API, as any external client would do.
#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app().await.address;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
