use config::Config;

#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    #[serde(default = "default_database_url")]
    pub(crate) database_url: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::new("config.toml", config::FileFormat::Toml))
            .build()?;

        settings.try_deserialize()
    }
}

fn default_database_url() -> String {
    "postgres://postgres:postgres@localhost/app".to_string()
}

fn default_port() -> u16 {
    3000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings: Settings = Config::default().try_deserialize().unwrap();
        assert_eq!(
            settings.database_url,
            "postgres://postgres:postgres@localhost/app"
        );
        assert_eq!(settings.port, 3000);
    }
}
