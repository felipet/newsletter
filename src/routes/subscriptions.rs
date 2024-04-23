/// Module that includes a subscription endpoint.
///
/// # Description
///
/// This module adds an endpoint that allows new clients to subscribe to the
/// newsletter.
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use log::{error, info};
use serde::Deserialize;
use sqlx::{self, PgPool};
use uuid::Uuid;

/// Data that is included in the form that comes along the POST for the endpoint.
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

/// Post endpoint to subscribe clients to the newsletter.
///
/// # Description
///
/// This endpoint allows new clients to subscribe to the newsletter. Existing
/// emails are rejected, so only clients having an email that was not previously
/// registered will be accepted. As of today, there's no endpoint to remove
/// a subscribed client.
///
/// ## Arguments
///
/// - An instance of the `struct` [FormData] that includes the data from the POST.
/// - An instance of the DB's driver to issue the INSERT operation of the new subscription.
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name,
    );
    info!("request_id {request_id} - Saving new subscriber details in the database.");
    // Insert the data from the form into the DB.
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    // Pass an immutable reference to the DB's driver.
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            info!("request_id {request_id} - New subscriber details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("request_id {request_id} - Failed to execute query: {:?}.", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
