//use crate::domain::SubscriberEmail;
use secrecy::ExposeSecret;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    //    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
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

//#[derive(serde::Deserialize)]
//pub struct EmailClientSettings {
//    pub base_url: String,
//    pub sender: String,
//}
//
//impl EmailClientSettings {
//    pub fn sender(&self) -> Result<SubscriberEmail, String> {
//        SubscriberEmail::parse(self.sender.clone())
//    }
//}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialize the configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    // convert the config into our Settings type
    settings.try_deserialize::<Settings>()
}
