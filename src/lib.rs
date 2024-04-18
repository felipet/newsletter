use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

/// Endpoint that tells a client about the server health status.
///
/// # Description
///
/// This endpoint can be used by clients to check whether the server is working as
/// expected, or it's blocked and unresponsive for some reason.
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Create a new HttpServer instance.
///
/// # Description
///
/// This function takes as argument a [TcpListener] bind to an address and a port,
/// constructs a new [HttpServer] and returns it.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}