pub mod configuration;
pub mod email_client;
pub mod startup;
pub mod telemetry;

mod routes {
    mod health_check;
    mod subscriptions;

    pub use health_check::*;
    pub use subscriptions::*;
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
