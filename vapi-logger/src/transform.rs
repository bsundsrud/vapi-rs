use crate::{
    config::{IpSource, LoggingConfig, OutputConfig},
    models::{CacheHandling, LogRecord, LogRequest, LogResponse},
    parsers,
};
use anyhow::{anyhow, bail, Result};
use crossbeam::select;
use crossbeam_channel::Receiver;
use std::io::prelude::*;
use std::net::TcpStream;
use std::{collections::HashMap, time::Duration};
use std::{fmt::Debug, net::SocketAddr, net::ToSocketAddrs};
use tracing::{error, info};
use vapi::LogTransaction;

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
}

impl LogTransform {
    pub fn new() -> Self {
        Self {
            req_header_list: Vec::new(),
            resp_header_list: Vec::new(),
            track_headers: false,
            meta: HashMap::new(),
            ip_source: IpSource::Request,
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

    pub fn from_config(self, config: &LoggingConfig) -> Self {
        self.req_headers(&config.request_headers)
            .resp_headers(&config.response_headers)
            .track_headers(config.track_headers)
            .meta(config.tags.clone())
            .ip_source(&config.ip_source)
    }

    pub fn transform(&self, log: LogTransaction) -> Result<LogRecord> {
        let level = log.level;
        let vxid = log.vxid;
        let mut parent_vxid = 0;
        let tx_type = log.ty;
        let reason = log.reason;
        let mut remoteip = None;
        let mut call_chain = Vec::new();
        let mut timings = HashMap::new();
        let mut req_headers = HashMap::new();
        let mut resp_headers = HashMap::new();
        let mut link = None;
        let mut req_method = "".into();
        let mut req_protocol = "".into();
        let mut req_url = "".into();
        let mut req_unset = Vec::new();
        let mut accounting = None;

        let mut resp_status = 0;
        let mut resp_protocol = "".into();
        let mut resp_unset = Vec::new();
        let mut length = 0;
        let mut ttl = None;
        let mut handling = None;

        for line in log.data {
            let tag = line.tag;
            let data = line.data;

            match tag.as_str() {
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
                    call_chain.push(format!("Return {}", &data));
                }
                "Timestamp" => {
                    let (_tag, data) = parsers::timestamp(tag, &data)?;
                    let timing_name = data.event.clone();
                    timings.insert(timing_name, data);
                }
                "ReqMethod" | "BereqMethod" => req_method = data,
                "ReqProtocol" | "BereqProtocol" => req_protocol = data,
                "ReqURL" | "BereqURL" => req_url = data,
                "ReqUnset" => {
                    if self.track_headers {
                        req_unset.push(data)
                    }
                }
                "ReqStart" => {
                    let rs = parsers::reqstart(&data)?;
                    remoteip = Some(rs.address);
                }
                "ReqHeader" | "BereqHeader" => {
                    let (k, v) = parsers::header(tag, &data)?;
                    let lower_header = k.to_lowercase();
                    if let IpSource::Header { name } = &self.ip_source {
                        if lower_header == *name {
                            let forwarded_for = parsers::forwarded_for(&v)?;
                            remoteip = Some(forwarded_for);
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
                    resp_protocol = data;
                }
                "TTL" => ttl = Some(parsers::ttl(tag, &data)?.1),
                "RespUnset" => {
                    if self.track_headers {
                        resp_unset.push(data);
                    }
                }
                "RespHeader" | "BerespHeader" => {
                    let (k, v) = parsers::header(tag, &data)?;
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
            tx_type,
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
        Ok(rec)
    }
}

fn send_to_stdout(t: LogTransform, rx: Receiver<LogTransaction>) -> ! {
    loop {
        select! {
            recv(rx) -> res => match res {
                Ok(log) => match t.transform(log) {
                    Ok(out) => {
                        println!("-----------------------------------------");
                        println!("{}", serde_json::to_string_pretty(&out).unwrap());
                    },
                    Err(e) => { error!("Error in transform: {}", e)},
                },
                Err(e) => error!("Error receiving log data: {}", e)
            }
        }
    }
}

fn loop_until_connected(
    addr: &SocketAddr,
    timeout: Duration,
    retry_interval: Duration,
) -> TcpStream {
    loop {
        match TcpStream::connect_timeout(addr, timeout) {
            Ok(s) => {
                info!("Connected to {}", addr);
                return s;
            }
            Err(e) => {
                error!("Failed to connect: {}", e);
                std::thread::sleep(retry_interval);
            }
        }
    }
}

fn send_to_tcp(
    t: LogTransform,
    rx: Receiver<LogTransaction>,
    host: &str,
    port: u16,
    timeout: u64,
    retry_interval: u64,
) -> Result<()> {
    let addr: SocketAddr = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow!("Invalid addr: {}:{}", host, port))?;
    let timeout = Duration::from_secs(timeout);
    let retry_interval = Duration::from_secs(retry_interval);
    let mut stream = loop_until_connected(&addr, timeout, retry_interval);

    loop {
        select! {
            recv(rx) -> res => match res {
                Ok(log) => match t.transform(log) {
                    Ok(out) => {
                        let mut json: String = serde_json::to_string(&out)?;
                        json.push('\n');
                        if let Err(e) = stream.write(json.as_bytes()) {
                            error!("Error writing to TCP socket: {}", e);
                            stream = loop_until_connected(&addr, timeout, retry_interval);
                        }
                    },
                    Err(e) => { error!("Error in transform: {}", e)},
                },
                Err(e) => error!("Error receiving log data: {}", e)
            }
        }
    }
}

pub fn consume_logs_forever(t: LogTransform, output: &OutputConfig, rx: Receiver<LogTransaction>) {
    let res = match output {
        OutputConfig::Stdout => send_to_stdout(t, rx),
        OutputConfig::Tcp {
            host,
            port,
            connect_timeout_secs,
            retry_interval_secs,
        } => send_to_tcp(
            t,
            rx,
            host,
            *port,
            *connect_timeout_secs,
            *retry_interval_secs,
        ),
    };
    if let Err(e) = res {
        error!("Output failure: {}", e);
        std::process::exit(1);
    }
}
