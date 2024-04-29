pub mod configuration;
pub mod domain;
pub mod startup;
pub mod telemetry;

mod routes {
    mod health_check;
    mod subscriptions;

    pub use health_check::*;
    pub use subscriptions::*;
}
