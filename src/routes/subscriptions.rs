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
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
