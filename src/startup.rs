use crate::routes;
/// Module that includes helper functions to start the **newsletter** application.
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

/// Create a new HttpServer instance.
///
/// # Description
///
/// This function takes the following arguments:
/// - A [TcpListener] bind to an address and a port.
/// - A [PgPool] that connects to a valid Postgres DB server.
///
/// To constructs a new [HttpServer] and returns it.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the DB's driver with a web::Data pointer. This way, the driver will
    // be safely shared between threads.
    let db_pool = web::Data::new(db_pool);
    // Connect all the services that are featured by the newsletter app.
    let server = HttpServer::new(move || {
        App::new()
            // Get health_check endpoint.
            .service(routes::health_check)
            // Post subscribe endpoint.
            .service(routes::subscribe)
            // State of the app: the DB's driver
            .app_data(db_pool.clone())
    })
    // Attach the listener to the app.
    .listen(listener)?
    // And run the server.
    .run();

    Ok(server)
}
