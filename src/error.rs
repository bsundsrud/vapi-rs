use std::ffi::CStr;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum VarnishError {
    #[error("VSM Error: {0}")]
    VSMError(String),
    #[error("VSL Error: {0}")]
    VSLError(String),
}

impl VarnishError {
    pub fn from_vsm_error(v: *const vapi_sys::vsm) -> VarnishError {
        unsafe {
            let c_err = vapi_sys::VSM_Error(v);
            VarnishError::VSMError(CStr::from_ptr(c_err).to_string_lossy().into_owned())
        }
    }

    pub fn from_vsl_error(v: *const vapi_sys::VSL_data) -> VarnishError {
        unsafe {
            let c_err = vapi_sys::VSL_Error(v);
            VarnishError::VSLError(CStr::from_ptr(c_err).to_string_lossy().into_owned())
        }
    }
}

pub type Result<T> = core::result::Result<T, VarnishError>;
