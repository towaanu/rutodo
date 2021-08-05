use deadpool_postgres::{config::ConfigError, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

// TODO: use app config for database
pub fn get_pg_pool() -> Result<Pool, ConfigError> {
    let mut config = deadpool_postgres::Config::new();

    config.host = Some("pgdb".to_string());
    config.user = Some("rutodo".to_string());
    config.dbname = Some("rutodo_dev".to_string());
    config.password = Some("secret".to_string());
    config.port = Some(5432);
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    config.create_pool(NoTls)
}
