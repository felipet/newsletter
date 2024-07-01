pub mod authentication;
pub mod configuration;
pub mod email_client;
pub mod startup;
pub mod telemetry;

mod routes {
    mod health_check;
    mod newsletters;
    mod subscriptions;
    mod subscriptions_confirm;
    //mod home;

    pub use health_check::*;
    pub use home::*;
    pub use login::*;
    pub use newsletters::*;
    pub use subscriptions::error_chain_fmt;
    pub use subscriptions::*;
    pub use subscriptions_confirm::*;

    mod home {
        use actix_web::{get, http::header::ContentType, HttpResponse};

        #[get("/")]
        pub async fn home() -> HttpResponse {
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(include_str!("routes/home/home.html"))
        }
    }

    mod login {
        mod get;
        mod post;

        pub use get::login_form;
        pub use post::login;
    }
}

mod domain {
    mod new_subscriber;
    mod subscriber_email;
    mod subscriber_name;

    pub use new_subscriber::NewSubscriber;
    pub use subscriber_email::SubscriberEmail;
    pub use subscriber_name::SubscriberName;
}

pub use domain::NewSubscriber;
pub use email_client::EmailClient;
