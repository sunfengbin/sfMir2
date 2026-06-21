use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub max_connections: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            listen_host: "127.0.0.1".to_string(),
            listen_port: 7000,
            max_connections: 5000,
        }
    }
}
