/// Module that includes a subscription endpoint.
///
/// # Description
///
/// This module adds an endpoint that allows new clients to subscribe to the
/// newsletter.
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{self, PgPool};
use tracing::{error, info_span, Instrument};
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
    let request_span = info_span!(
        "Adding as a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _request_span_guard = request_span.enter();

    let query_span = info_span!("Saving new subscriber details in the database.");
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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            info_span!("New subscriber details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("Failed to execute query: {:?}.", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
