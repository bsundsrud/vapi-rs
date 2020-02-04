use crate::error::{Result, VarnishError};
use crate::vsl_tags::VslTag;
use crate::vsm::{vsm_status, OpenVSM};
use std::ffi::{CStr, CString};
use std::ptr;
use std::time::Duration;

use vapi_sys;

// from vapi/vsl_int.h
const VSL_IDENTMASK: u32 = !(3u32 << 30);
const VSL_LENMASK: u32 = 0xffff;
const VSL_CLIENTMARKER: u32 = 1u32 << 30;
const VSL_BACKENDMARKER: u32 = 1u32 << 31;

#[derive(Debug)]
struct Vsl {
    vsl: *mut vapi_sys::VSL_data,
}

impl Drop for Vsl {
    fn drop(&mut self) {
        unsafe {
            vapi_sys::VSL_Delete(self.vsl);
        }
    }
}

impl Vsl {
    fn new() -> Self {
        unsafe {
            let vsl = vapi_sys::VSL_New();
            assert!(!vsl.is_null());
            Vsl { vsl }
        }
    }
}

#[derive(Debug)]
struct VslQ {
    vslq: *mut vapi_sys::VSLQ,
}

impl Drop for VslQ {
    fn drop(&mut self) {
        unsafe {
            vapi_sys::VSLQ_Delete(&mut self.vslq);
        }
    }
}

impl VslQ {
    fn new(vsl: &Vsl, grouping: LogGrouping) -> Result<Self> {
        let vslq =
            unsafe { vapi_sys::VSLQ_New(vsl.vsl, ptr::null_mut(), grouping.as_raw(), ptr::null()) };
        if vslq.is_null() {
            return Err(VarnishError::from_vsl_error(vsl.vsl));
        }

        Ok(VslQ { vslq })
    }

    fn new_with_query<Q: Into<String>>(vsl: &Vsl, grouping: LogGrouping, query: Q) -> Result<Self> {
        let query = CString::new(query.into()).unwrap();
        let vslq = unsafe {
            vapi_sys::VSLQ_New(vsl.vsl, ptr::null_mut(), grouping.as_raw(), query.as_ptr())
        };
        if vslq.is_null() {
            return Err(VarnishError::from_vsl_error(vsl.vsl));
        }

        Ok(VslQ { vslq })
    }

    fn clear_cursor(&mut self) {
        unsafe {
            vapi_sys::VSLQ_SetCursor(self.vslq, ptr::null_mut());
        }
    }

    fn set_cursor(&mut self, cursor: &mut VslCursor) {
        unsafe {
            vapi_sys::VSLQ_SetCursor(self.vslq, &mut cursor.cursor);
        }
    }
}

#[derive(Debug)]
struct VslCursor {
    cursor: *mut vapi_sys::VSL_cursor,
}

impl Drop for VslCursor {
    fn drop(&mut self) {
        if self.cursor.is_null() {
            return;
        }
        unsafe {
            vapi_sys::VSL_DeleteCursor(self.cursor);
        }
    }
}

impl VslCursor {
    fn new(vsl: &mut Vsl, vsm: &OpenVSM, opts: u32) -> Option<VslCursor> {
        let cursor = unsafe {
            let c = vapi_sys::VSL_CursorVSM(vsl.vsl, vsm.0.vsm, opts);
            if c.is_null() {
                vapi_sys::VSL_ResetError(vsl.vsl);
                return None;
            }
            c
        };
        Some(VslCursor { cursor })
    }
}

#[derive(Debug)]
pub struct VslTransaction {
    tx: *mut vapi_sys::VSL_transaction,
    vsl: *mut vapi_sys::VSL_data,
}

#[derive(Debug)]
enum CursorStatus {
    Match(LogLine),
    NoMatch,
}

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

impl VslTransaction {
    unsafe fn new(
        tx: *mut vapi_sys::VSL_transaction,
        vsl: *mut vapi_sys::VSL_data,
    ) -> VslTransaction {
        VslTransaction { tx, vsl }
    }
    fn next_record(&self) -> Option<std::result::Result<CursorStatus, i32>> {
        let status = unsafe {
            let c = (*self.tx).c;
            vapi_sys::VSL_Next(c)
        };
        if status < 0 {
            return Some(Err(status));
        }
        if status == 0 {
            None
        } else {
            let matches = unsafe { vapi_sys::VSL_Match(self.vsl, (*self.tx).c) };
            if matches == 1 {
                Some(Ok(CursorStatus::Match(unsafe {
                    self.get_current_record()
                })))
            } else {
                Some(Ok(CursorStatus::NoMatch))
            }
        }
    }

    fn level(&self) -> u32 {
        assert!(!self.tx.is_null());
        unsafe { (*self.tx).level }
    }

    fn vxid(&self) -> u32 {
        assert!(!self.tx.is_null());
        unsafe { (*self.tx).vxid }
    }

    fn parent_vxid(&self) -> u32 {
        assert!(!self.tx.is_null());
        unsafe { (*self.tx).vxid_parent }
    }

    fn ty(&self) -> TxType {
        assert!(!self.tx.is_null());
        unsafe { (*self.tx).type_.into() }
    }

    fn reason(&self) -> Reason {
        assert!(!self.tx.is_null());
        unsafe { (*self.tx).reason.into() }
    }

    unsafe fn get_current_record(&self) -> LogLine {
        assert!(!self.tx.is_null());
        let rec_ptr = (*(*self.tx).c).rec.ptr;
        let header: &[u32] = std::slice::from_raw_parts(rec_ptr, 2);
        let tag: u32 = header[0] >> 24;
        let tag: vapi_sys::VSL_tag_e = std::mem::transmute(tag);
        let tag: VslTag = tag.into();
        let vxid = header[1] & VSL_IDENTMASK;
        let data_length: u32 = header[0] & VSL_LENMASK;
        let is_client: u32 = header[1] & VSL_CLIENTMARKER;
        let is_backend: u32 = header[1] & VSL_BACKENDMARKER;
        let ty = if is_client > 0 {
            RecordType::Client
        } else if is_backend > 0 {
            RecordType::Backend
        } else {
            RecordType::Other
        };
        let data: &[u32] = std::slice::from_raw_parts(rec_ptr, data_length as usize + 2);
        let data = &data[2..];
        let data: &[u8] = as_u8_slice(&data, data_length);
        let data = CStr::from_bytes_with_nul_unchecked(data);
        let data = data.to_string_lossy().into_owned();
        LogLine {
            vxid,
            tag,
            data,
            ty,
        }
    }
}

fn as_u8_slice(v: &[u32], len: u32) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, len as usize) }
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

fn query_loop(
    vsm: &OpenVSM,
    grouping: LogGrouping,
    query: Option<String>,
    cursor_opts: u32,
    callback: LogCallback,
) -> Result<()> {
    let mut vsl = Vsl::new();
    let mut vslq = if let Some(q) = query {
        VslQ::new_with_query(&vsl, grouping, q)?
    } else {
        VslQ::new(&vsl, grouping)?
    };
    let mut cursor = None;
    loop {
        if vsm.status() & vsm_status::VSM_WRK_RESTARTED != 0 {
            if cursor.is_some() {
                vslq.clear_cursor();
                cursor = None;
            }
        }
        if cursor.is_none() {
            if let Some(mut c) = VslCursor::new(&mut vsl, vsm, cursor_opts) {
                vslq.set_cursor(&mut c);
                cursor = Some(c);
            } else {
                continue;
            }
        }
        let res = unsafe {
            let callback: *mut std::os::raw::c_void = std::mem::transmute(&callback);
            vapi_sys::VSLQ_Dispatch(vslq.vslq, Some(rust_callback), callback)
        };
        match res {
            0 => {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }
            1 => continue,
            -1 => break,
            -2 => {
                vslq.clear_cursor();
                cursor = None
            }
            -3 => return Err(VarnishError::LogOverrun),
            -4 => return Err(VarnishError::IOError),
            e @ _ => return Err(VarnishError::UserStatus(e)),
        }
    }

    Ok(())
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

#[no_mangle]
unsafe extern "C" fn rust_callback(
    vsl: *mut vapi_sys::VSL_data,
    mut tx: *const *mut vapi_sys::VSL_transaction,
    priv_: *mut std::os::raw::c_void,
) -> std::os::raw::c_int {
    let callback: &LogCallback = std::mem::transmute(priv_);
    loop {
        if tx.is_null() {
            break;
        }
        let t = *tx;
        if t.is_null() {
            break;
        }
        let mut data = Vec::new();
        let this_tx = VslTransaction::new(t, vsl);
        let level = this_tx.level();
        let vxid = this_tx.vxid();
        let parent_vxid = this_tx.parent_vxid();
        let ty = this_tx.ty();
        let reason = this_tx.reason();
        while let Some(r) = this_tx.next_record() {
            match r {
                Ok(CursorStatus::NoMatch) => {
                    continue;
                }
                Ok(CursorStatus::Match(line)) => {
                    data.push(line);
                }
                Err(i) => {
                    return i;
                }
            }
        }
        let txn = LogTransaction {
            level,
            vxid,
            parent_vxid,
            ty,
            reason,
            data,
        };
        match callback(txn) {
            CallbackResult::Continue => {}
            CallbackResult::Stop(res) => return res,
        }

        tx = tx.add(1);
    }
    0
}
