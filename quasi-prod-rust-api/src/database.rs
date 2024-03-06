use crate::configs::Configurations;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Debug)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn get_connection_pool(config: &Configurations) -> AppState {
    let url = get_database_url(config);
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    AppState { pool }
}

fn get_database_url(config: &Configurations) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    )
}

#[cfg(test)]
mod test {
    use crate::configs::{Database, Logger, Server, Service, Tracing};

    use super::*;

    #[test]
    fn test_get_database_url() {
        let config = Configurations {
            server: Server {
                host: "".to_string(),
                port: 0,
            },
            logger: Logger {
                level: "".to_string(),
            },
            database: Database {
                host: "localhost".to_string(),
                port: 1234,
                name: "db".to_string(),
                user: "user".to_string(),
                password: "password".to_string(),
            },
            service: Service {
                name: "rust_service".to_string(),
                version: "1.0".to_string(),
            },
            tracing: Tracing {
                host: "localhost".to_string(),
            },
            environment: "development".to_string(),
        };
        let url = get_database_url(&config);
        assert_eq!(
            url,
            format!(
                "postgres://{}:{}@{}:{}/{}",
                config.database.user,
                config.database.password,
                config.database.host,
                config.database.port,
                config.database.name
            )
        );
    }
}
