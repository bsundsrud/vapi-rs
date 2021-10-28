use serde::Deserialize;
use std::collections::HashMap;
use vapi::vsl::transform::LogTransform;
use vapi::vsl::IpSource;
use vapi::{LogGrouping, Reason, TxType};

fn default_connect_timeout() -> u64 {
    5
}

fn default_retry_interval() -> u64 {
    5
}

fn default_shm_connect_timeout() -> u64 {
    5
}

fn default_tcp_sender_threads() -> u64 {
    2
}

#[derive(Debug, Deserialize, Default)]
pub struct InputConfig {
    #[serde(default = "default_shm_connect_timeout")]
    pub connect_timeout_secs: u64,
    pub path: Option<String>,
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
        #[serde(default = "default_tcp_sender_threads")]
        sender_threads: u64,
    },
    Null,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self::Stdout
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(remote = "LogGrouping")]
pub enum Grouping {
    Vxid,
    Request,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum LogType {
    Session,
    Request,
    BackendRequest,
    Raw,
}

impl From<LogType> for TxType {
    fn from(t: LogType) -> Self {
        match t {
            LogType::Session => TxType::Session,
            LogType::Request => TxType::Request,
            LogType::BackendRequest => TxType::BackendRequest,
            LogType::Raw => TxType::Raw,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum ReasonType {
    Unknown,
    Http1,
    RxReq,
    Esi,
    Restart,
    Pass,
    Fetch,
    BgFetch,
    Pipe,
}

impl From<ReasonType> for Reason {
    fn from(r: ReasonType) -> Self {
        match r {
            ReasonType::Unknown => Reason::Unknown,
            ReasonType::Http1 => Reason::Http1,
            ReasonType::RxReq => Reason::RxReq,
            ReasonType::Esi => Reason::Esi,
            ReasonType::Restart => Reason::Restart,
            ReasonType::Pass => Reason::Pass,
            ReasonType::Fetch => Reason::Fetch,
            ReasonType::BgFetch => Reason::BgFetch,
            ReasonType::Pipe => Reason::Pipe,
        }
    }
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
    #[serde(with = "Grouping")]
    pub grouping: LogGrouping,
    pub type_filter: Vec<LogType>,
    pub reason_filter: Vec<ReasonType>,
    pub tail: bool,
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
            grouping: LogGrouping::Vxid,
            type_filter: Vec::new(),
            reason_filter: Vec::new(),
            tail: true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub input: InputConfig,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

pub fn transform_from_config(config: &LoggingConfig) -> LogTransform {
    let t = LogTransform::new();

    t.req_headers(&config.request_headers)
        .resp_headers(&config.response_headers)
        .track_headers(config.track_headers)
        .meta(config.tags.clone())
        .ip_source(&config.ip_source)
}
