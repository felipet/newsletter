use actix_web::{http::header::LOCATION, post, HttpResponse};

#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
