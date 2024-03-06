use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tracing {
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configurations {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub service: Service,
    pub tracing: Tracing,
}

impl Configurations {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "development".into());
        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"));

        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            builder = builder.set_override("logger.level", log_level)?;
        }

        builder.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configurations() {
        let config = Configurations::new().unwrap();
        assert_eq!(config.environment, "development");
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.name, "tododb");
        assert_eq!(config.database.user, "todouser");
        assert_eq!(config.database.password, "todopassword");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.logger.level, "DEBUG");
        assert_eq!(config.tracing.host, "http://localhost:4317");
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.service.name, "todoservice");
        assert_eq!(config.service.version, "1.0.0");
    }

    #[test]
    fn test_load_production() {
        temp_env::with_vars([("ENV", Some("production"))], || {
            let config = Configurations::new().unwrap();
            assert_eq!(config.environment, "production");
            assert_eq!(config.database.host, "localhost");
            assert_eq!(config.database.name, "tododb");
            assert_eq!(config.database.user, "todouser");
            assert_eq!(config.database.password, "todopassword");
            assert_eq!(config.database.port, 5432);
            assert_eq!(config.logger.level, "INFO");
            assert_eq!(config.tracing.host, "http://localhost:4317");
            assert_eq!(config.server.host, "0.0.0.0");
            assert_eq!(config.server.port, 8080);
            assert_eq!(config.service.name, "todoservice");
            assert_eq!(config.service.version, "1.0.0");
        });
    }
    #[test]
    fn test_override_port() {
        temp_env::with_vars([("PORT", Some("8899"))], || {
            let actual = Configurations::new().unwrap();
            assert_eq!(actual.environment, "development");
            assert_eq!(actual.server.port, 8899);
            assert_eq!(actual.logger.level, "DEBUG");
        });
    }
}
