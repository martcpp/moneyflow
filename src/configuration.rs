#[derive(serde::Deserialize, Debug)]
pub struct Settings{
    pub database: DatabaseConfig,
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub server_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_config() -> Result<Settings,config::ConfigError> {
    let mut settings = config::Config::default();
    
    // Add configuration from environment variables
    // settings.merge(config::Environment::with_prefix("APP"))?;
    
    // Add configuration from a file (optional)
    settings.merge(config::File::with_name("configuration"))?;
    
    // Deserialize into the Settings struct
    settings.try_into::<Settings>()
}

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
        &self.jwt_secret
    }
    
    pub fn jwt_expiration(&self) -> u64 {
        self.jwt_expiration
    }
    
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    
}