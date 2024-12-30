use std::env;

#[allow(dead_code)]
pub struct AppConfig {
    connection_string: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            connection_string: env::var("CONNECTION_STRING")
                .map_or(String::new(), |var| var.to_string()),
        })
    }
}
