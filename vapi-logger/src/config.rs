use std::collections::HashMap;

use serde::Deserialize;

fn default_connect_timeout() -> u64 {
    5
}

fn default_retry_interval() -> u64 {
    5
}

#[derive(Debug, Deserialize)]
#[serde(tag = "destination", rename_all = "snake_case")]
pub enum OutputConfig {
    Stdout,
    Tcp {
        host: String,
        port: u16,
        #[serde(default = "default_connect_timeout")]
        connect_timeout_secs: u64,
        #[serde(default = "default_retry_interval")]
        retry_interval_secs: u64,
    },
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self::Stdout
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "ip_source", rename_all = "snake_case")]
pub enum IpSource {
    Request,
    Header {
        #[serde(rename = "ip_source_header")]
        name: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    pub track_headers: bool,
    pub request_headers: Vec<String>,
    pub response_headers: Vec<String>,
    pub tags: HashMap<String, String>,
    pub query: String,
    #[serde(flatten)]
    pub ip_source: IpSource,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            track_headers: false,
            request_headers: Vec::new(),
            response_headers: Vec::new(),
            tags: HashMap::new(),
            query: String::new(),
            ip_source: IpSource::Request,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}