use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use newsletter::EmailClient;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init the tracing subsystem.
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load the configuration settings from a YML file.
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Create a connection pool to handle connections to the DB.
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    // Address for the service that will run the newsletter application.
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    // Build an `EmailClient` to handle all the stuff related to sending mails.
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    // Create a TcpListener to bind the address in which the service aims to listen for requests.
    let listener = TcpListener::bind(address).expect("Failed to bind address");

    run(listener, connection_pool, email_client)?.await
}
