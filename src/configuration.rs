use secrecy::{ExposeSecret, Secret};
/// Configurations module.
///
/// This module includes two `struct`s: [Settings] and [DatabaseSettings] that
/// describe all the configuration attributes that are needed to setup
/// the execution and test environments of the **newsletter** application.
use serde;

/// Top level `struct` for the configuration.
///
/// # Description
///
/// This `struct` includes two attributes:
/// - [Settings::database]: an instance of the [DatabaseSettings] struct.
/// - [Settings::application_port]: the port in which the application will listen to.
///
/// The `struct` derives the [serde::Deserialize] trait.
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

/// Data Base related configuration.
///
/// # Description
///
/// This `struct` includes all the attributes that are needed to handle a connection
/// to the data base server:
/// - [DatabaseSettings::username] to keep the DB's username.
/// - [DatabaseSettings::password] to keep the DB's password.
/// - [DatabaseSettings::port] to keep the port in which the DB server is listening.
/// - [DatabaseSettings::host] to keep the host in which the DB server is running.
/// - [DatabaseSettings::database_name] to keep the name of the DB schema.
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub enum Environment {
    Local,
    Production,
}

/// Function that parses a file with the definition for the configuration `struct`s.
///
/// # Description
///
/// This function expects a file named _configuration_ at the root folder of the
/// project. This file shall include all the values for the `struct`s [Settings] and
/// [DatabaseSettings].
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    // Configuration files are stored within '{app root dir}/configuration'
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Load the "default" configuration file.
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Detect what environment is the app running in.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Add the layer of environment-specific values.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.try_into()
}

impl DatabaseSettings {
    /// Method to build the connection (full) string for a Postgres database server.
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    /// Modified version of [DatabaseSettings::connection_string] that doesn't include the DB's name.
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
