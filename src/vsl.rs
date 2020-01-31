use crate::error::{Result, VarnishError};
use crate::vsm::{vsm_status, OpenVSM};
use std::convert::TryInto;
use std::ffi::CString;
use std::ptr;

use vapi_sys;

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
            Vsl {
                vsl: vapi_sys::VSL_New(),
            }
        }
    }
}

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
            unsafe { vapi_sys::VSLQ_New(vsl.vsl, *ptr::null(), grouping.as_raw(), ptr::null()) };

        if vslq.is_null() {
            return Err(VarnishError::from_vsl_error(vsl.vsl));
        }

        Ok(VslQ { vslq })
    }

    fn new_with_query<Q: Into<String>>(vsl: &Vsl, grouping: LogGrouping, query: Q) -> Result<Self> {
        let query = CString::new(query.into()).unwrap();
        let vslq =
            unsafe { vapi_sys::VSLQ_New(vsl.vsl, *ptr::null(), grouping.as_raw(), query.as_ptr()) };

        if vslq.is_null() {
            return Err(VarnishError::from_vsl_error(vsl.vsl));
        }

        Ok(VslQ { vslq })
    }

    fn clear_cursor(&mut self) {
        unsafe {
            vapi_sys::VSLQ_SetCursor(self.vslq, *ptr::null());
        }
    }

    fn set_cursor(&mut self, cursor: &mut VslCursor) {
        unsafe {
            vapi_sys::VSLQ_SetCursor(self.vslq, &mut cursor.cursor);
        }
    }
}

struct VslCursor {
    cursor: *mut vapi_sys::VSL_cursor,
}

impl Drop for VslCursor {
    fn drop(&mut self) {
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

pub enum LogGrouping {
    Raw,
    Vxid,
    Request,
    Session,
    Max,
}

impl LogGrouping {
    fn as_raw(&self) -> vapi_sys::VSL_grouping_e {
        use LogGrouping::*;
        match self {
            Raw => vapi_sys::VSL_grouping_e_VSL_g_raw,
            Vxid => vapi_sys::VSL_grouping_e_VSL_g_vxid,
            Request => vapi_sys::VSL_grouping_e_VSL_g_request,
            Session => vapi_sys::VSL_grouping_e_VSL_g_session,
            Max => vapi_sys::VSL_grouping_e_VSL_g__MAX,
        }
    }
}

pub const TAIL: usize = 1 << 0;
pub const BATCH: usize = 1 << 1;
pub const TAILSTOP: usize = 1 << 2;

struct VarnishLogBuilder {
    grouping: LogGrouping,
    query: Option<String>,
    cursor_opts: u32,
}

impl VarnishLogBuilder {
    pub fn new() -> VarnishLogBuilder {
        VarnishLogBuilder {
            grouping: LogGrouping::Vxid,
            query: None,
            cursor_opts: 0,
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

    pub fn cursor_opts(&mut self, opts: u32) -> &mut Self {
        self.cursor_opts = opts;
        self
    }

    pub fn execute<'vsm>(self, vsm: &'vsm OpenVSM, callback: LogCallback) -> Result<()> {
        query_loop(vsm, self.grouping, self.query, self.cursor_opts, callback)
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
            let i = vapi_sys::VSLQ_Dispatch(vslq.vslq, Some(rust_callback), callback);
        };
    }

    Ok(())
}

type LogCallback = Box<dyn Fn(u32, String, String, String) -> i32>;

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

        tx = tx.offset(
            std::mem::size_of::<vapi_sys::VSL_transaction>()
                .try_into()
                .unwrap(),
        );
    }
    0
}
