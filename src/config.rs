use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct PgConfig {
    pub host: Option<String>,
    pub user: Option<String>,
    pub dbname: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
}

impl Default for PgConfig {
    fn default() -> Self {
        Self {
            host: None,
            user: None,
            dbname: None,
            password: None,
            port: Some(5432),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct AppConfig {
    pub port: u16,
    pub pg: PgConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 3030,
            pg: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
struct FlatConfig {
    pub port: u16,

    pub pg_host: Option<String>,
    pub pg_user: Option<String>,
    pub pg_dbname: Option<String>,
    pub pg_password: Option<String>,
    pub pg_port: Option<u16>,
}

impl Default for FlatConfig {
    fn default() -> Self {
        Self {
            port: 3030,

            pg_host: None,
            pg_user: None,
            pg_dbname: None,
            pg_password: None,
            pg_port: Some(5432),
        }
    }
}

impl From<FlatConfig> for AppConfig {
    fn from(flat_config: FlatConfig) -> Self {
        Self {
            port: flat_config.port,

            pg: PgConfig {
                host: flat_config.pg_host,
                user: flat_config.pg_user,
                dbname: flat_config.pg_dbname,
                password: flat_config.pg_password,
                port: flat_config.pg_port,
            },
        }
    }
}

pub fn get_config() -> AppConfig {
    let flat_config: FlatConfig = envy::prefixed("RUTODO_").from_env().unwrap();
    flat_config.into()
}
