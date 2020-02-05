use crate::error::{Result, VarnishError};
use std::ffi::CString;
use std::time::Duration;
use vapi_sys;

#[derive(Debug)]
pub(crate) struct SharedMem {
    pub(crate) vsm: *mut vapi_sys::vsm,
}

pub struct VSMBuilder {
    vsm: SharedMem,
    path: Option<String>,
    timeout: Option<Duration>,
}

impl VSMBuilder {
    pub fn new() -> Result<VSMBuilder> {
        Ok(VSMBuilder {
            vsm: SharedMem::new()?,
            path: None,
            timeout: None,
        })
    }

    pub fn path<S: Into<String>>(&mut self, path: S) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn attach(mut self) -> Result<OpenVSM> {
        if let Some(p) = self.path {
            self.vsm.set_path(p)?;
        }
        self.vsm.set_timeout(self.timeout)?;
        self.vsm.attach()
    }
}

impl SharedMem {
    fn new() -> Result<SharedMem> {
        let vsm = unsafe {
            let vsm = vapi_sys::VSM_New();
            if vsm.is_null() {
                return Err(VarnishError::from_raw_vsm_error(vsm));
            }
            vsm
        };

        Ok(SharedMem { vsm })
    }

    fn set_path<S: Into<String>>(&mut self, path: S) -> Result<()> {
        let val = CString::new(path.into()).unwrap();
        let arg = CString::new(String::from("n")).unwrap();
        unsafe {
            if vapi_sys::VSM_Arg(self.vsm, *arg.as_ptr(), val.as_ptr()) != 1 {
                return Err(VarnishError::from_vsm_error(&self));
            }
        }
        Ok(())
    }

    fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
        let val = if let Some(t) = timeout {
            CString::new(t.as_secs().to_string()).unwrap()
        } else {
            CString::new(String::from("off")).unwrap()
        };
        let arg = CString::new(String::from("t")).unwrap();
        unsafe {
            if vapi_sys::VSM_Arg(self.vsm, *arg.as_ptr(), val.as_ptr()) != 1 {
                return Err(VarnishError::from_vsm_error(&self));
            }
        }

        Ok(())
    }

    fn attach(self) -> Result<OpenVSM> {
        unsafe {
            if vapi_sys::VSM_Attach(self.vsm, -1) != 0 {
                return Err(VarnishError::from_vsm_error(&self));
            }
        }
        Ok(OpenVSM(self))
    }
}

impl Drop for SharedMem {
    fn drop(&mut self) {
        unsafe {
            if !self.vsm.is_null() {
                vapi_sys::VSM_Destroy(&mut self.vsm);
            }
        }
    }
}

#[derive(Debug)]
pub struct OpenVSM(pub(crate) SharedMem);

#[allow(dead_code)]
pub mod vsm_status {
    pub const VSM_MGT_RUNNING: u32 = 1 << 1;
    pub const VSM_MGT_CHANGED: u32 = 1 << 2;
    pub const VSM_MGT_RESTARTED: u32 = 1 << 3;
    pub const VSM_WRK_RUNNING: u32 = 1 << 9;
    pub const VSM_WRK_CHANGED: u32 = 1 << 10;
    pub const VSM_WRK_RESTARTED: u32 = 1 << 11;
}

impl OpenVSM {
    pub fn status(&self) -> u32 {
        unsafe { vapi_sys::VSM_Status(self.0.vsm) }
    }
}
