use serde::Serialize;
use serde_json::Value;

#[allow(unused)]
#[derive(Debug, Serialize)]
pub struct LogOutput {
    pub vxid: u32,
    pub parent_vxid: u32,
    pub level: u32,
    pub tx_type: String,
    pub reason: String,
    pub data: Value,
}
