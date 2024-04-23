use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set the logger.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Create a TcpListener to bind the address in which the service aims to listen for requests.
    let listener = TcpListener::bind(address).expect("Failed to bind address");

    run(listener, connection_pool)?.await
}
