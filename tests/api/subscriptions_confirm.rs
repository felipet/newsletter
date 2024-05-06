use crate::helpers::spawn_app;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[actix_web::test]
async fn confirmation_without_token_are_rejected_with_400() {
    // Prepare
    let test_app = spawn_app().await;

    // Test
    let response = reqwest::get(&format!("{}/subscriptions/confirm", test_app.address))
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}

#[actix_web::test]
async fn the_link_returned_by_subscribe_resturns_a_200_if_called() {
    // Prepare
    let test_app = spawn_app().await;
    let body = "name=jane%20doe&email=janedoe%40mail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_configuration_links(&email_request);

    // Perform the request to the confirmation link.
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // Check
    assert_eq!(response.status().as_u16(), 200);
}

#[actix_web::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // Prepare
    let test_app = spawn_app().await;
    let body = "name=jane%20doe&email=janedoe%40mail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_configuration_links(&email_request);

    // Test
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Checks
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "janedoe@mail.com");
    assert_eq!(saved.name, "jane doe");
    assert_eq!(saved.status, "confirmed");
}
