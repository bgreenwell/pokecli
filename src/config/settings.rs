use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub api: ApiSettings,
    pub cache: CacheSettings,
    pub output: OutputSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSettings {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub rate_limit_per_second: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheSettings {
    pub enabled: bool,
    pub ttl_hours: u64,
    pub max_size_mb: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputSettings {
    pub default_format: String,
    pub colored: bool,
    pub show_sprites: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api: ApiSettings {
                base_url: "https://pokeapi.co/api/v2".to_string(),
                timeout_seconds: 30,
                rate_limit_per_second: 10,
            },
            cache: CacheSettings {
                enabled: true,
                ttl_hours: 24,
                max_size_mb: 100,
            },
            output: OutputSettings {
                default_format: "table".to_string(),
                colored: true,
                show_sprites: false,
            },
        }
    }
}