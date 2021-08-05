use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct AppConfig {
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { port: 3030 }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
struct FlatConfig {
    pub port: u16,
}

impl Default for FlatConfig {
    fn default() -> Self {
        Self { port: 3030 }
    }
}

impl From<FlatConfig> for AppConfig {
    fn from(flat_config: FlatConfig) -> Self {
        Self {
            port: flat_config.port,
        }
    }
}

pub fn get_config() -> AppConfig {
    let flat_config: FlatConfig = envy::prefixed("RUTODO_").from_env().unwrap();
    flat_config.into()
}
