use super::{
    CallbackResult, LogCallback, LogGrouping, LogLine, LogTransaction, Reason, RecordType, TxType,
};
use crate::error::{Result, VarnishError};
use crate::vsm::{vsm_status, OpenVSM};
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;
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
        unsafe { (*self.tx).level }
    }

    fn vxid(&self) -> u32 {
        unsafe { (*self.tx).vxid }
    }

    fn parent_vxid(&self) -> u32 {
        unsafe { (*self.tx).vxid_parent }
    }

    fn ty(&self) -> TxType {
        unsafe { (*self.tx).type_.into() }
    }

    fn reason(&self) -> Reason {
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
    callback: LogCallback,
    stop: Option<Receiver<()>>,
) -> Result<()> {
    let mut vsl = Vsl::new()?;
    let mut vslq = if let Some(q) = query {
        VslQ::new_with_query(&vsl, grouping, q)?
    } else {
        VslQ::new(&vsl, grouping)?
    };
    let mut cursor = None;
    loop {
        if vsm.status() & vsm_status::VSM_WRK_RESTARTED != 0 && cursor.is_some() {
            vslq.clear_cursor();
            cursor = None;
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
            let callback = &callback as *const LogCallback as *mut std::ffi::c_void;
            vapi_sys::VSLQ_Dispatch(vslq.vslq, Some(rust_callback), callback)
        };
        let should_stop = stop
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

        if should_stop {
            return Ok(());
        }
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
            e => return Err(VarnishError::UserStatus(e)),
        }
    }

    Ok(())
}

#[no_mangle]
unsafe extern "C" fn rust_callback(
    vsl: *mut vapi_sys::VSL_data,
    mut tx: *const *mut vapi_sys::VSL_transaction,
    priv_: *mut std::os::raw::c_void,
) -> std::os::raw::c_int {
    let callback: &LogCallback = &*(priv_ as *const LogCallback);
    loop {
        if tx.is_null() {
            break;
        }
        let t = *tx;
        if t.is_null() {
            break;
        }
        let mut data = Vec::new();
        let this_tx = match VslTransaction::new(t, vsl) {
            Ok(t) => t,
            Err(e) => panic!("Tried to create Tx from null data: {}", e),
        };
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