use crate::vsl::internal::Vsl;
use crate::vsm::SharedMem;
use std::ffi::CStr;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum VarnishError {
    #[error("VSM Error: {0}")]
    VSMError(String),
    #[error("VSL Error: {0}")]
    VSLError(String),
    #[error("Log overrun")]
    LogOverrun,
    #[error("I/O read error")]
    IOError,
    #[error("User Status code {0}")]
    UserStatus(i32),
    #[error("Callback Error: {0}")]
    CallbackError(String),
}

impl VarnishError {
    pub(crate) unsafe fn from_raw_vsm_error(v: *const vapi_sys::vsm) -> VarnishError {
        assert!(!v.is_null());
        let c_err = vapi_sys::VSM_Error(v);
        VarnishError::VSMError(CStr::from_ptr(c_err).to_string_lossy().into_owned())
    }

    pub(crate) fn from_vsm_error(v: &SharedMem) -> VarnishError {
        unsafe {
            let c_err = vapi_sys::VSM_Error(v.vsm);
            VarnishError::VSMError(CStr::from_ptr(c_err).to_string_lossy().into_owned())
        }
    }

    pub(crate) fn from_vsl_error(v: &Vsl) -> VarnishError {
        unsafe {
            let c_err = vapi_sys::VSL_Error(v.vsl);
            VarnishError::VSLError(CStr::from_ptr(c_err).to_string_lossy().into_owned())
        }
    }
}

pub type Result<T, E = VarnishError> = core::result::Result<T, E>;
