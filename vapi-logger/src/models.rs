use std::collections::HashMap;

use serde::Serialize;
use vapi::{Reason, TxType};

use crate::parsers::{RequestAccounting, Timestamp, VarnishLink, VarnishTtl};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(remote = "TxType")]
pub enum TxTypeDef {
    Unknown,
    Session,
    Request,
    BackendRequest,
    Raw,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(remote = "Reason")]
pub enum ReasonDef {
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
    #[serde(with = "TxTypeDef")]
    pub tx_type: TxType,
    #[serde(with = "ReasonDef")]
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
