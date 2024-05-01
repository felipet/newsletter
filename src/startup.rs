use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
/// Module that includes helper functions to start the **newsletter** application.
///
use crate::routes;
use crate::EmailClient;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// A type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Create a connection pool to handle connections to the DB.
        let connection_pool = get_connection_pool(&configuration.database);

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

        // Address for the service that will run the newsletter application.
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        // Create a TcpListener to bind the address in which the service aims to listen for requests.
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Create a new HttpServer instance.
///
/// # Description
///
/// This function takes the following arguments:
/// - A [TcpListener] bind to an address and a port.
/// - A [PgPool] that connects to a valid Postgres DB server.
///
/// To constructs a new [HttpServer] and returns it.
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wrap the DB's driver with a web::Data pointer. This way, the driver will
    // be safely shared between threads.
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    // Connect all the services that are featured by the newsletter app.
    let server = HttpServer::new(move || {
        App::new()
            // Add the Logger middleware.
            .wrap(TracingLogger::default())
            // Get health_check endpoint.
            .service(routes::health_check)
            // Post subscribe endpoint.
            .service(routes::subscribe)
            // State of the app: the DB's driver
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    // Attach the listener to the app.
    .listen(listener)?
    // And run the server.
    .run();

    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
