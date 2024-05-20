//! Module that includes a health check endpoint.

use actix_web::{get, HttpResponse, Responder};

/// Endpoint that tells a client about the server health status.
///
/// # Description
///
/// This endpoint can be used by clients to check whether the server is working as
/// expected, or it's blocked and unresponsive for some reason.
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
