use crate::{Reason, TxType};

use crate::{
    vsl::models::{CacheHandling, LogRecord, LogRequest, LogResponse},
    vsl::parsers,
};
use anyhow::{bail, Result};

use std::collections::HashMap;
use std::fmt::Debug;

use super::internal::{CursorResult, VslTransaction};
use super::IpSource;

fn try_parse_parent_vxid(s: &str) -> Result<u32> {
    let mut parts = s.split_whitespace();
    if let (Some(_), Some(parent_vxid)) = (parts.next(), parts.next()) {
        Ok(parent_vxid.parse::<u32>()?)
    } else {
        bail!("Couldn't parse vxid out of {}", s)
    }
}

#[derive(Debug)]
pub struct LogTransform {
    req_header_list: Vec<String>,
    resp_header_list: Vec<String>,
    track_headers: bool,
    meta: HashMap<String, String>,
    ip_source: IpSource,
    type_filter: Vec<TxType>,
    reason_filter: Vec<Reason>,
}

impl LogTransform {
    pub fn new() -> Self {
        Self {
            req_header_list: Vec::new(),
            resp_header_list: Vec::new(),
            track_headers: false,
            meta: HashMap::new(),
            ip_source: IpSource::Request,
            type_filter: Vec::new(),
            reason_filter: Vec::new(),
        }
    }

    pub fn track_headers(mut self, d: bool) -> Self {
        self.track_headers = d;
        self
    }

    pub fn req_headers<T: AsRef<str>>(mut self, headers: &[T]) -> Self {
        self.req_header_list = headers
            .into_iter()
            .map(|s| s.as_ref().to_lowercase())
            .collect();
        self
    }

    pub fn resp_headers<T: AsRef<str>>(mut self, headers: &[T]) -> Self {
        self.resp_header_list = headers
            .into_iter()
            .map(|s| s.as_ref().to_lowercase())
            .collect();
        self
    }

    pub fn meta(mut self, meta: HashMap<String, String>) -> Self {
        self.meta = meta;
        self
    }

    pub fn ip_source(mut self, source: &IpSource) -> Self {
        self.ip_source = match source {
            IpSource::Request => IpSource::Request,
            IpSource::Header { name } => IpSource::Header {
                name: name.to_lowercase(),
            },
        };
        self
    }

    pub fn type_filter<V: Into<Vec<TxType>>>(&mut self, types: V) -> &mut Self {
        self.type_filter = types.into();
        self
    }

    pub fn reason_filter<V: Into<Vec<Reason>>>(&mut self, reasons: V) -> &mut Self {
        self.reason_filter = reasons.into();
        self
    }

    fn allow_type(&self, ty: TxType) -> bool {
        self.type_filter.is_empty() || self.type_filter.contains(&ty)
    }

    fn allow_reason(&self, reason: Reason) -> bool {
        self.reason_filter.is_empty() || self.reason_filter.contains(&reason)
    }

    pub fn process_txn(&self, mut tx: VslTransaction) -> Result<Option<LogRecord>> {
        let level = tx.level();
        let vxid = tx.vxid();
        let mut parent_vxid = 0;
        let ty = tx.ty();
        let reason = tx.reason();
        if !(self.allow_type(ty) && self.allow_reason(reason)) {
            return Ok(None);
        }

        // init data structures for resulting LogRecord
        let mut remoteip = None;
        let mut call_chain = Vec::new();
        let mut timings = HashMap::new();
        let mut req_headers = HashMap::new();
        let mut resp_headers = HashMap::new();
        let mut link = None;
        let mut req_method = String::from("");
        let mut req_protocol = String::from("");
        let mut req_url = String::from("");
        let mut req_unset = Vec::new();
        let mut accounting = None;

        let mut resp_status = 0;
        let mut resp_protocol = String::from("");
        let mut resp_unset = Vec::new();
        let mut length = 0;
        let mut ttl = None;
        let mut handling = None;

        // Process tags
        loop {
            match tx.read_next_record() {
                CursorResult::NoData => break,
                CursorResult::NoMatch => continue,
                CursorResult::Error(e) => bail!("read_next_record returned {}", e),
                CursorResult::Match(td) => {
                    let tag = td.tag.to_str().unwrap();
                    let data = &td.data.to_string_lossy();

                    match tag {
                        "HttpGarbage" => {
                            return Ok(None);
                        }
                        "Begin" => {
                            parent_vxid = try_parse_parent_vxid(&data)?;
                        }
                        "End" => {}
                        "VCL_call" => {
                            handling = match data.to_lowercase().as_str() {
                                "hit" => Some(CacheHandling::Hit),
                                "miss" => Some(CacheHandling::Miss),
                                "pass" => Some(CacheHandling::Pass),
                                "synth" => Some(CacheHandling::Synth),
                                "backend_error" => Some(CacheHandling::Error),
                                _ => handling,
                            };
                            call_chain.push(format!("Call {}", &data));
                        }
                        "VCL_return" => {
                            if data.to_lowercase() == "pipe" {
                                handling = Some(CacheHandling::Pipe);
                            }
                            if data.to_lowercase() == "restart" {
                                return Ok(None);
                            }
                            call_chain.push(format!("Return {}", &data));
                        }
                        "Timestamp" => {
                            let (_tag, data) = parsers::timestamp(tag, &data)?;
                            let timing_name = data.event.clone();
                            timings.insert(timing_name, data);
                        }
                        "ReqMethod" | "BereqMethod" => req_method = data.to_string(),
                        "ReqProtocol" | "BereqProtocol" => req_protocol = data.to_string(),
                        "ReqURL" | "BereqURL" => req_url = data.to_string(),
                        "ReqUnset" => {
                            if self.track_headers {
                                req_unset.push(data.to_string())
                            }
                        }
                        "ReqStart" => {
                            let rs = parsers::reqstart(&data)?;
                            remoteip = Some(rs.address);
                        }
                        "ReqHeader" | "BereqHeader" => {
                            let (k, v) = parsers::header(&data)?;
                            let lower_header = k.to_lowercase();
                            if let IpSource::Header { name } = &self.ip_source {
                                if lower_header == *name {
                                    remoteip = Some(parsers::remote_ip(&v)?);
                                }
                            }
                            if self.req_header_list.contains(&lower_header) {
                                req_headers.insert(lower_header, v);
                            }
                        }
                        "ReqAcct" | "BereqAcct" => {
                            let (_tag, acct) = parsers::req_accounting(tag, &data)?;
                            accounting = Some(acct);
                        }
                        "Link" => {
                            link = Some(parsers::link(tag, &data)?.1);
                        }
                        "RespStatus" | "BerespStatus" => {
                            resp_status = parsers::status(&data)?;
                        }
                        "RespProtocol" | "BerespProtocol" => {
                            resp_protocol = data.to_string();
                        }
                        "TTL" => ttl = Some(parsers::ttl(tag, &data)?.1),
                        "RespUnset" => {
                            if self.track_headers {
                                resp_unset.push(data.to_string());
                            }
                        }
                        "RespHeader" | "BerespHeader" => {
                            let (k, v) = parsers::header(&data)?;
                            let lower_header = k.to_lowercase();
                            if lower_header == "content-length" {
                                length = parsers::parse(&v)?;
                            }
                            if self.resp_header_list.contains(&lower_header) {
                                resp_headers.insert(lower_header, v);
                            }
                        }
                        "Length" => {
                            length = parsers::parse(&data)?;
                        }
                        _ => {}
                    }
                }
            }
        }

        let unset = if req_unset.is_empty() {
            None
        } else {
            Some(req_unset)
        };
        let req = LogRequest {
            remoteip,
            url: req_url,
            method: req_method,
            protocol: req_protocol,
            headers: req_headers,
            unset,
        };

        let unset = if resp_unset.is_empty() {
            None
        } else {
            Some(resp_unset)
        };

        let resp = LogResponse {
            status: resp_status,
            protocol: resp_protocol,
            headers: resp_headers,
            unset,
            length,
            ttl,
        };

        let duration_msec = if timings.contains_key("Resp") {
            Some(timings.get("Resp").unwrap().since_start * 1000.0)
        } else if timings.contains_key("BerespBody") {
            Some(timings.get("BerespBody").unwrap().since_start * 1000.0)
        } else if timings.contains_key("PipeSess") {
            Some(timings.get("PipeSess").unwrap().since_start * 1000.0)
        } else if timings.contains_key("Error") {
            Some(timings.get("Error").unwrap().since_start * 1000.0)
        } else {
            None
        };

        let ttfb_msec = if timings.contains_key("Process") {
            Some(timings.get("Process").unwrap().since_start * 1000.0)
        } else if timings.contains_key("Pipe") {
            Some(timings.get("Pipe").unwrap().since_start * 1000.0)
        } else if timings.contains_key("Beresp") {
            Some(timings.get("Beresp").unwrap().since_start * 1000.0)
        } else {
            None
        };

        let rec = LogRecord {
            level,
            vxid,
            parent_vxid,
            tx_type: ty,
            reason,
            handling,
            call_chain,
            timings,
            request: req,
            response: resp,
            link,
            accounting,
            duration_msec,
            ttfb_msec,
            meta: self.meta.clone(),
        };

        Ok(Some(rec))
    }
}
