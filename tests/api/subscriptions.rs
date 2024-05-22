use crate::helpers::spawn_app;
use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;

    let body = "name=jane%20doe&email=jane_doe%40mail.com";

    // Mock server.
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    let response = test_app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscribe_persist_the_new_subscribe() {
    // Prepare
    let test_app = spawn_app().await;

    let body = "name=jane%20doe&email=jane_doe%40mail.com";

    // Mock server.
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;

    let saved = sqlx::query!("SELECT email,name,status FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "jane_doe@mail.com");
    assert_eq!(saved.name, "jane doe");
    assert_eq!(saved.status, "pending_confirmation");
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;

    let test_cases = vec![
        ("name=jane%20doe", "missing the email"),
        ("email=jane_doe%40mail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let test_app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=jane_doe%40mail.com", "empty name"),
        ("name=Jane&email=", "empty email"),
        ("name=Jane&email=not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = test_app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}",
            description
        );
    }
}

#[actix_web::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Prepare
    let test_app = spawn_app().await;
    let body = "name=Jane%20Doe&email=janedoe%40mail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    // Launch the post
    test_app.post_subscriptions(body.into()).await;

    // When the mock servers gets dropped, it performs the checks.
}

#[actix_web::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Prepare
    let test_app = spawn_app().await;
    let body = "name=Jane%20Doe&email=janedoe%40mail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    // Launch the post
    test_app.post_subscriptions(body.into()).await;

    // Check
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_configuration_links(&email_request);

    // Both links must be identical.
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[actix_web::test]
async fn subscribe_fails_if_there_is_a_fatal_database_error() {
    // Prepare
    let test_app = spawn_app().await;
    let body = "name=Jane%20Doe&email=janedoe%40mail.com";

    // Sabotage the database
    sqlx::query!("ALTER TABLE subscription_tokens DROP COLUMN subscription_token;",)
        .execute(&test_app.db_pool)
        .await
        .unwrap();

    // Act
    let response = test_app.post_subscriptions(body.into()).await;
    // Assert
    assert_eq!(response.status().as_u16(), 500);
}

#[actix_web::test]
async fn subscribe_fails_if_second_subscription_attempt_returns_the_same_token() {
    // Prepare
    let test_app = spawn_app().await;
    let body = "name=Jane%20Doe&email=janedoe%40mail.com";
    let email = "janedoe@mail.com";

    // Launch the post
    test_app.post_subscriptions(body.into()).await;

    // Get the associated subscriber_id
    let result = sqlx::query!("SELECT id FROM subscriptions WHERE email = $1;", email,)
        .fetch_optional(&test_app.db_pool)
        .await;

    let subscriber_id: Uuid = match result {
        Ok(record) => record.unwrap().id,
        Err(e) => panic!("Failed to execute query: {:?}", e),
    };

    let result = sqlx::query!(
        "SELECT subscription_token FROM subscription_tokens WHERE subscriber_id = $1",
        subscriber_id,
    )
    .fetch_optional(&test_app.db_pool)
    .await;

    let first_subscriber_token = match result {
        Ok(record) => record.unwrap().subscription_token,
        Err(e) => panic!("Failed to execute query: {:?}", e),
    };

    // Attempt a second subscription
    test_app.post_subscriptions(body.into()).await;

    let result = sqlx::query!(
        "SELECT subscription_token FROM subscription_tokens WHERE subscriber_id = $1",
        subscriber_id,
    )
    .fetch_optional(&test_app.db_pool)
    .await;

    let sec_subscriber_token = match result {
        Ok(record) => record.unwrap().subscription_token,
        Err(e) => panic!("Failed to execute query: {:?}", e),
    };

    assert_ne!(first_subscriber_token, sec_subscriber_token);
}
