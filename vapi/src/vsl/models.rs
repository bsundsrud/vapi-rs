use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::parsers::{RequestAccounting, Timestamp, VarnishLink, VarnishTtl};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "ip_source", rename_all = "snake_case")]
pub enum IpSource {
    Request,
    Header {
        #[serde(rename = "ip_source_header")]
        name: String,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum TxType {
    Unknown,
    Session,
    Request,
    BackendRequest,
    Raw,
}

impl From<u32> for TxType {
    fn from(c: u32) -> TxType {
        use TxType::*;
        match c {
            1 => Session,
            2 => Request,
            3 => BackendRequest,
            4 => Raw,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Reason {
    Unknown,
    Http1,
    RxReq,
    Esi,
    Restart,
    Pass,
    Fetch,
    BgFetch,
    Pipe,
    NotHandled(u32),
}

impl From<u32> for Reason {
    fn from(c: u32) -> Reason {
        use Reason::*;
        match c {
            0 => Unknown,
            1 => Http1,
            2 => RxReq,
            3 => Esi,
            4 => Restart,
            5 => Pass,
            6 => Fetch,
            7 => BgFetch,
            8 => Pipe,
            v => NotHandled(v),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum CacheHandling {
    Hit,
    Miss,
    Pass,
    Synth,
    Pipe,
    Error,
}

#[derive(Debug, Serialize)]
pub struct LogRequest {
    pub remoteip: Option<String>,
    pub url: String,
    pub method: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unset: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct LogResponse {
    pub status: u16,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unset: Option<Vec<String>>,
    pub length: u64,
    pub ttl: Option<VarnishTtl>,
}

#[derive(Debug, Serialize)]
pub struct LogRecord {
    pub level: u32,
    pub vxid: u32,
    pub parent_vxid: u32,
    pub tx_type: TxType,
    pub reason: Reason,
    pub call_chain: Vec<String>,
    pub timings: HashMap<String, Timestamp>,
    pub handling: Option<CacheHandling>,
    pub request: LogRequest,
    pub response: LogResponse,
    pub link: Option<VarnishLink>,
    pub accounting: Option<RequestAccounting>,
    pub duration_msec: Option<f64>,
    pub ttfb_msec: Option<f64>,
    pub meta: HashMap<String, String>,
}
