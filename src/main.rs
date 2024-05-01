use newsletter::configuration::get_configuration;
use newsletter::startup::Application;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init the tracing subsystem.
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load the configuration settings from a YAML file.
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;

    Ok(())
}
