use secrecy::{ExposeSecret, Secret};

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value:String)->Result<Self,Self::Error>{
        match value.as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", value)),
        }
    }
}


#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSetting,
    pub moralis_client: MoralisClientSetting,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSetting {
    pub host: String,
    pub port: u16,
    pub base_url: String,
}

impl ApplicationSetting {
    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(serde::Deserialize)]
pub struct MoralisClientSetting {
    // pub app_id: String,
    pub url: String,
    // pub chain : String,
    pub key: Secret<String>,
    pub timeout_milliseconds: u64,
}

impl MoralisClientSetting {
    pub fn api_key(&self) -> String {
        self.key.expose_secret().to_string()
    }
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path= std::env::current_dir().expect("Failed to get current directory");
    let configuration_directory = base_path.join("configuration");
    let environment:Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Unknown environment");
        let environment_filename = format!("{}.yaml", environment.as_str());
        let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml")
    ))
    .add_source(config::File::from(
        configuration_directory.join(environment_filename)
    ))
    .add_source(
        config::Environment::with_prefix("APP")
            .prefix_separator("_")
            .separator("__"),
    )
    .build()?;
    settings.try_deserialize::<Settings>()

}