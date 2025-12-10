pub(crate) mod internal;
pub mod models;
pub(crate) mod parsers;
pub mod transform;

pub use models::*;

use std::time::Instant;

use crate::error::Result;
use crate::vsm::OpenVSM;
use crossbeam_channel::{Receiver, Sender};
use internal::query_loop;

use vapi_sys;

use self::transform::LogTransform;

pub type LogCallback = Box<dyn Fn(LogTransaction) -> CallbackResult>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    Client,
    Backend,
    Other,
}

#[derive(Debug)]
pub struct LogLine {
    pub vxid: u32,
    pub tag: String,
    pub data: String,
    pub ty: RecordType,
}

#[derive(Debug)]
pub enum LogGrouping {
    Raw,
    Vxid,
    Request,
    Session,
}

impl LogGrouping {
    fn as_raw(&self) -> vapi_sys::VSL_grouping_e {
        use LogGrouping::*;
        match self {
            Raw => vapi_sys::VSL_grouping_e_VSL_g_raw,
            Vxid => vapi_sys::VSL_grouping_e_VSL_g_vxid,
            Request => vapi_sys::VSL_grouping_e_VSL_g_request,
            Session => vapi_sys::VSL_grouping_e_VSL_g_session,
        }
    }
}

const TAIL: u32 = 1;
const BATCH: u32 = 1 << 1;
const TAILSTOP: u32 = 1 << 2;

#[derive(Debug, Default, Copy, Clone)]
pub struct CursorOpts(u32);

impl CursorOpts {
    pub fn new() -> CursorOpts {
        CursorOpts(0)
    }

    pub fn tail(mut self) -> Self {
        self.0 |= TAIL;
        self
    }

    pub fn batch(mut self) -> Self {
        self.0 |= BATCH;
        self
    }
    pub fn tail_stop(mut self) -> Self {
        self.0 |= TAILSTOP;
        self
    }

    pub fn is_tail(&self) -> bool {
        self.0 & TAIL > 0
    }
    pub fn is_batch(&self) -> bool {
        self.0 & BATCH > 0
    }
    pub fn is_tail_stop(&self) -> bool {
        self.0 & TAILSTOP > 0
    }
}

impl From<u32> for CursorOpts {
    fn from(o: u32) -> CursorOpts {
        CursorOpts(o)
    }
}

impl From<CursorOpts> for u32 {
    fn from(c: CursorOpts) -> u32 {
        c.0
    }
}

pub(crate) struct VarnishLogBuilder {
    pub(crate) grouping: LogGrouping,
    pub(crate) query: Option<String>,
    pub(crate) cursor_opts: CursorOpts,
    pub(crate) reacquire: bool,
    pub(crate) reacquire_signal: Option<Sender<()>>,
    pub(crate) type_filter: Vec<TxType>,
    pub(crate) reason_filter: Vec<Reason>,
    pub(crate) log_sender: Sender<LogRecord>,
    pub(crate) transform: LogTransform,
}

impl VarnishLogBuilder {
    pub fn new(log_sender: Sender<LogRecord>, transform: LogTransform) -> VarnishLogBuilder {
        VarnishLogBuilder {
            grouping: LogGrouping::Vxid,
            query: None,
            cursor_opts: CursorOpts::new(),
            reacquire: false,
            reacquire_signal: None,
            type_filter: Vec::new(),
            reason_filter: Vec::new(),
            log_sender,
            transform,
        }
    }
    pub fn grouping(&mut self, grouping: LogGrouping) -> &mut Self {
        self.grouping = grouping;
        self
    }

    pub fn query(&mut self, query: String) -> &mut Self {
        self.query = Some(query);
        self
    }

    pub fn cursor_opts(&mut self, opts: CursorOpts) -> &mut Self {
        self.cursor_opts = opts;
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

    pub fn reacquire_if_overrun(&mut self) -> &mut Self {
        self.reacquire = true;
        self
    }

    pub fn reacquire_and_notify_if_overrun(&mut self, tx: Sender<()>) -> &mut Self {
        self.reacquire = true;
        self.reacquire_signal = Some(tx);
        self
    }

    pub fn execute(self, vsm: &OpenVSM, stop_channel: Option<Receiver<()>>) -> Result<()> {
        query_loop(vsm, self, stop_channel)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CallbackResult {
    Continue,
    Stop(i32),
}

#[derive(Debug)]
pub struct LogTransaction {
    pub started: Instant,
    pub level: u32,
    pub vxid: u32,
    pub parent_vxid: u32,
    pub ty: TxType,
    pub reason: Reason,
    pub data: Vec<LogLine>,
}
