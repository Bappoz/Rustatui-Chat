pub struct ServerConfig {
    pub address: String,
    pub buffer_size: usize,
    pub max_clients: usize,
}

impl ServerConfig {
    pub fn default() -> Self {
        Self {
            address: "localhost:4556".to_string(),
            buffer_size: 1024,
            max_clients: 32,
        }
    }
}