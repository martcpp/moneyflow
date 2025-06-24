// use std::convert::TryInto;
use serde::Deserialize;
use config::Config;
use dotenvy::dotenv;

#[derive(Deserialize, Debug)]
pub struct Settings{
    pub database: DatabaseConfig,
    pub jwt: Jwt,
    pub server: Server,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Jwt{
    pub secret: String,
    pub expiration: u64,
}
pub fn get_config() -> Result<Settings,config::ConfigError> {
// Load environment variables from .env file
    dotenv().ok();
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());


   Config::builder()
    .add_source(config::File::with_name("config/default"))
    .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
    
    .add_source(config::Environment::with_prefix("APP").separator("__"))
    .build()?
    .try_deserialize()
}

// pub fn get_config() -> Result<Settings, config::ConfigError> {
//     Config::builder()
//         // Layer 1: Base config file
//         .add_source(config::File::with_name("configuration").required(true))

//         // Layer 2: Optional env override (e.g., APP__DATABASE__URL)
//         .add_source(config::Environment::with_prefix("APP").separator("__"))

//         .build()?
//         .try_deserialize()
// }

impl Settings {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database_name
        )
    }
    
    pub fn jwt_secret(&self) -> &str {
        &self.jwt.secret
    }
    
    pub fn jwt_expiration(&self) -> u64 {
        self.jwt.expiration
    }
    
    pub fn server_port(&self) -> u16 {
        self.server.port
    }
     pub fn server_host(&self) -> &str {
        &self.server.host
    }
}