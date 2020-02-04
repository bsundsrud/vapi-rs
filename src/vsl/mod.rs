mod internal;
mod vsl_tags;

use crate::error::Result;
use crate::vsm::OpenVSM;
pub use vsl_tags::VslTag;

use internal::query_loop;

use vapi_sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    Client,
    Backend,
    Other,
}

#[derive(Debug)]
pub struct LogLine {
    pub vxid: u32,
    pub tag: VslTag,
    pub data: String,
    pub ty: RecordType,
}

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

const TAIL: u32 = 1 << 0;
const BATCH: u32 = 1 << 1;
const TAILSTOP: u32 = 1 << 2;

#[derive(Debug)]
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
    grouping: LogGrouping,
    query: Option<String>,
    cursor_opts: CursorOpts,
}

impl VarnishLogBuilder {
    pub fn new() -> VarnishLogBuilder {
        VarnishLogBuilder {
            grouping: LogGrouping::Vxid,
            query: None,
            cursor_opts: CursorOpts::new(),
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

    pub fn execute<'vsm>(self, vsm: &'vsm OpenVSM, callback: LogCallback) -> Result<()> {
        query_loop(
            vsm,
            self.grouping,
            self.query,
            self.cursor_opts.into(),
            callback,
        )
    }
}

pub type LogCallback = Box<dyn Fn(LogTransaction) -> CallbackResult>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CallbackResult {
    Continue,
    Stop(i32),
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
}

impl From<u32> for Reason {
    fn from(c: u32) -> Reason {
        use Reason::*;
        match c {
            1 => Http1,
            2 => RxReq,
            3 => Esi,
            4 => Restart,
            5 => Pass,
            6 => Fetch,
            7 => BgFetch,
            8 => Pipe,
            _ => Unknown,
        }
    }
}
#[derive(Debug)]
pub struct LogTransaction {
    pub level: u32,
    pub vxid: u32,
    pub parent_vxid: u32,
    pub ty: TxType,
    pub reason: Reason,
    pub data: Vec<LogLine>,
}
