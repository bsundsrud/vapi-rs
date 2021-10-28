use super::transform::LogTransform;
use super::{
    CallbackResult, LogCallback, LogGrouping, LogLine, LogRecord, LogTransaction, Reason,
    RecordType, TxType,
};
use crate::error::{Result, VarnishError};
use crate::vsm::{vsm_status, OpenVSM};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{error, warn};
use vapi_sys;

// from vapi/vsl_int.h
const VSL_IDENTMASK: u32 = !(3u32 << 30);
const VSL_LENMASK: u32 = 0xffff;
const VSL_CLIENTMARKER: u32 = 1u32 << 30;
const VSL_BACKENDMARKER: u32 = 1u32 << 31;

#[derive(Debug)]
pub(crate) struct Vsl {
    pub(crate) vsl: *mut vapi_sys::VSL_data,
}

impl Drop for Vsl {
    fn drop(&mut self) {
        unsafe {
            vapi_sys::VSL_Delete(self.vsl);
        }
    }
}

impl Vsl {
    fn new() -> Result<Self> {
        unsafe {
            let vsl = vapi_sys::VSL_New();
            if vsl.is_null() {
                Err(VarnishError::VSLError("Could not allocate VSL".into()))
            } else {
                Ok(Vsl { vsl })
            }
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
            return Err(VarnishError::from_vsl_error(&vsl));
        }
        Ok(VslQ { vslq })
    }

    fn new_with_query<Q: Into<String>>(vsl: &Vsl, grouping: LogGrouping, query: Q) -> Result<Self> {
        let query = CString::new(query.into()).unwrap();
        let vslq = unsafe {
            vapi_sys::VSLQ_New(vsl.vsl, ptr::null_mut(), grouping.as_raw(), query.as_ptr())
        };
        if vslq.is_null() {
            return Err(VarnishError::from_vsl_error(&vsl));
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
    fn new(vsl: &mut Vsl, vsm: &OpenVSM, opts: u32) -> Result<VslCursor> {
        let cursor = unsafe {
            let c = vapi_sys::VSL_CursorVSM(vsl.vsl, vsm.0.vsm, opts);
            if c.is_null() {
                let e = vapi_sys::VSL_Error(vsl.vsl);
                let error_msg = CStr::from_ptr(e).to_string_lossy().to_string();
                vapi_sys::VSL_ResetError(vsl.vsl);
                return Err(VarnishError::VSLError(error_msg));
            }
            c
        };
        Ok(VslCursor { cursor })
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

pub(crate) enum CursorResult<'rec> {
    NoData,
    NoMatch,
    Error(i32),
    Match(TagData<'rec>),
}

pub(crate) struct TagData<'rec> {
    pub vxid: u32,
    pub tag: &'rec CStr,
    pub data: &'rec CStr,
    pub ty: RecordType,
}

impl VslTransaction {
    unsafe fn new(
        tx: *mut vapi_sys::VSL_transaction,
        vsl: *mut vapi_sys::VSL_data,
    ) -> Result<VslTransaction> {
        if tx.is_null() {
            Err(VarnishError::VSLError("Null transaction".into()))
        } else if vsl.is_null() {
            Err(VarnishError::VSLError("Null VSL".into()))
        } else {
            Ok(VslTransaction { tx, vsl })
        }
    }

    fn advance_next(&mut self) -> Result<bool, i32> {
        let status = unsafe {
            assert!(!self.tx.is_null());
            let c = (*self.tx).c;
            vapi_sys::VSL_Next(c)
        };
        if status < 0 {
            Err(status)
        } else if status == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn vsl_match(&self) -> bool {
        let matches = unsafe {
            assert!(!self.tx.is_null());
            vapi_sys::VSL_Match(self.vsl, (*self.tx).c)
        };
        matches == 1
    }

    fn read_tag_data(&self) -> TagData {
        let rec_ptr = unsafe {
            assert!(!self.tx.is_null());
            assert!(!(*self.tx).c.is_null());
            (*(*self.tx).c).rec.ptr
        };
        let header: &[u32] = unsafe { std::slice::from_raw_parts(rec_ptr, 2) };
        let tag: u32 = header[0] >> 24;
        let tag = unsafe { CStr::from_ptr(vapi_sys::VSL_tags[tag as usize]) };

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
        let data: &[u32] = unsafe { std::slice::from_raw_parts(rec_ptr, data_length as usize + 2) };
        let data = &data[2..];
        let data: &[u8] = as_u8_slice(&data, data_length);
        let data = unsafe { CStr::from_bytes_with_nul_unchecked(data) };

        TagData {
            vxid,
            tag,
            data,
            ty,
        }
    }

    pub(crate) fn read_next_record(&mut self) -> CursorResult {
        match self.advance_next() {
            Ok(false) => return CursorResult::NoData,
            Err(status) => return CursorResult::Error(status),
            Ok(true) => {}
        }
        if self.vsl_match() {
            CursorResult::Match(self.read_tag_data())
        } else {
            CursorResult::NoMatch
        }
    }

    fn next_record(&self) -> Option<Result<CursorStatus, i32>> {
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

    pub(crate) fn level(&self) -> u32 {
        unsafe { (*self.tx).level }
    }

    pub(crate) fn vxid(&self) -> u32 {
        unsafe { (*self.tx).vxid }
    }

    pub(crate) fn parent_vxid(&self) -> u32 {
        unsafe { (*self.tx).vxid_parent }
    }

    pub(crate) fn ty(&self) -> TxType {
        unsafe { (*self.tx).type_.into() }
    }

    pub(crate) fn reason(&self) -> Reason {
        unsafe { (*self.tx).reason.into() }
    }

    unsafe fn get_current_record(&self) -> LogLine {
        let rec_ptr = (*(*self.tx).c).rec.ptr;
        let header: &[u32] = std::slice::from_raw_parts(rec_ptr, 2);
        let tag: u32 = header[0] >> 24;
        let tag = CStr::from_ptr(vapi_sys::VSL_tags[tag as usize])
            .to_string_lossy()
            .to_string();
        //let tag: vapi_sys::VSL_tag_e = std::mem::transmute(tag);
        //let tag: VslTag = tag.into();
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

pub(crate) fn query_loop(
    vsm: &OpenVSM,
    grouping: LogGrouping,
    query: Option<String>,
    cursor_opts: u32,
    reacquire_on_overrun: bool,
    reacquire_signal: Option<Sender<()>>,
    log_sender: Sender<LogRecord>,
    type_filter: Vec<TxType>,
    reason_filter: Vec<Reason>,
    transform: LogTransform,
    stop: Option<Receiver<()>>,
) -> Result<()> {
    let mut vsl = Vsl::new()?;
    let mut vslq = if let Some(q) = query {
        VslQ::new_with_query(&vsl, grouping, q)?
    } else {
        VslQ::new(&vsl, grouping)?
    };
    let mut cursor = None;
    let callback_data = CallbackData {
        log_sender,
        type_filter,
        reason_filter,
        transform,
    };
    loop {
        if vsm.status() & vsm_status::VSM_WRK_RESTARTED != 0 && cursor.is_some() {
            vslq.clear_cursor();
            cursor = None;
        }
        if cursor.is_none() {
            match VslCursor::new(&mut vsl, vsm, cursor_opts) {
                Ok(mut c) => {
                    vslq.set_cursor(&mut c);
                    cursor = Some(c);
                }
                Err(e) => {
                    warn!("Error creating cursor: {}", e);
                    continue;
                }
            }
        }
        let callback = &callback_data as *const CallbackData as *mut std::ffi::c_void;
        let mut res = vapi_sys::vsl_status_vsl_more;
        let mut should_stop = false;
        while res == vapi_sys::vsl_status_vsl_more && !should_stop {
            res = unsafe { vapi_sys::VSLQ_Dispatch(vslq.vslq, Some(rust_dispatch), callback) };

            should_stop = stop
                .as_ref()
                .map(|r| match r.try_recv() {
                    // received signal, stop
                    Ok(_) => true,
                    // didn't receive anything, keep going
                    Err(TryRecvError::Empty) => false,
                    // channel is closed or errored, stop
                    _ => true,
                })
                // if there's no channel, don't stop ever
                .unwrap_or(false);
        }
        if should_stop {
            return Ok(());
        }
        if res == vapi_sys::vsl_status_vsl_more {
            continue;
        } else if res == vapi_sys::vsl_status_vsl_end {
            std::thread::sleep(Duration::from_millis(10));
            continue;
        } else if res == vapi_sys::vsl_status_vsl_e_eof {
            break;
        }

        unsafe { vapi_sys::VSLQ_Flush(vslq.vslq, Some(rust_dispatch), callback) };

        if res == vapi_sys::vsl_status_vsl_e_abandon {
            vslq.clear_cursor();
            cursor = None
        } else if res == vapi_sys::vsl_status_vsl_e_overrun {
            if reacquire_on_overrun {
                vslq.clear_cursor();
                cursor = None;
                if let Some(tx) = reacquire_signal.as_ref() {
                    tx.send(()).map_err(|_| VarnishError::LogOverrun)?;
                }
            } else {
                return Err(VarnishError::LogOverrun);
            }
        } else if res == vapi_sys::vsl_status_vsl_e_io {
            return Err(VarnishError::IOError);
        } else if res == -6 {
            return Err(VarnishError::CallbackError("Log channel error".into()));
        } else if res == -7 {
            return Err(VarnishError::CallbackError(
                "Log transformation error".into(),
            ));
        } else {
            return Err(VarnishError::UserStatus(res));
        }
    }
    Ok(())
}

#[repr(C)]
struct CallbackData {
    log_sender: Sender<LogRecord>,
    type_filter: Vec<TxType>,
    reason_filter: Vec<Reason>,
    transform: LogTransform,
}

impl CallbackData {
    fn allow_type(&self, ty: TxType) -> bool {
        self.type_filter.is_empty() || self.type_filter.contains(&ty)
    }

    fn allow_reason(&self, reason: Reason) -> bool {
        self.reason_filter.is_empty() || self.reason_filter.contains(&reason)
    }
}

#[no_mangle]
unsafe extern "C" fn rust_dispatch(
    vsl: *mut vapi_sys::VSL_data,
    mut tx: *const *mut vapi_sys::VSL_transaction,
    priv_: *mut std::os::raw::c_void,
) -> std::os::raw::c_int {
    let callback_data: &CallbackData = &*(priv_ as *const CallbackData);
    loop {
        if tx.is_null() {
            break;
        }
        let t = *tx;
        if t.is_null() {
            break;
        }

        let this_tx = match VslTransaction::new(t, vsl) {
            Ok(t) => t,
            Err(e) => panic!("Tried to create Tx from null data: {}", e),
        };
        match callback_data.transform.process_txn(this_tx) {
            Ok(Some(log)) => match callback_data.log_sender.send(log) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send log data: {}", e);
                    return -6;
                }
            },
            Ok(None) => {}
            Err(e) => {
                error!("Failed to process log message: {}", e);
                return -7;
            }
        }

        tx = tx.add(1);
    }
    0
}
