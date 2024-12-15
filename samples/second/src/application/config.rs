pub struct AppConfig {
    pub(super) _connection_string: String,
}

impl AppConfig {
    pub(super) fn read_env() -> anyhow::Result<Self> {
        Ok(Self {
            _connection_string: "read_valiable_from_env".to_string()
        })
    }
}