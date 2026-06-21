use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub file_path: String,
    pub level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            file_path: "./logs/mir2.log".to_string(),
            level: "info".to_string(),
        }
    }
}
