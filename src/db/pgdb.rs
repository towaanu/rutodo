use crate::config::AppConfig;
use deadpool_postgres::{config::ConfigError, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub fn get_pg_pool(app_config: &AppConfig) -> Result<Pool, ConfigError> {
    let mut config = deadpool_postgres::Config::new();

    config.host = app_config.pg.host.clone();
    config.user = app_config.pg.user.clone();
    config.dbname = app_config.pg.dbname.clone();
    config.password = app_config.pg.password.clone();
    config.port = app_config.pg.port;
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    config.create_pool(NoTls)
}
