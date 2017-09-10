use std::ffi::CString;
use std::mem::{uninitialized, zeroed};

use ffi::*;
use error::*;

#[derive(Copy, Clone)]
pub struct Guid {
    pub inner: FMOD_GUID,
}

impl Guid {
    /// # Remarks
    /// This function expects a string representation of a GUID in the following exact format: "{9d348364-8145-4724-b337-5bc9b2afe60f}".
    pub fn parse(id: &str) -> Result<Guid> {
        unsafe {
            let mut guid = uninitialized();
            let cstr = CString::new(id).unwrap();

            FMOD_Studio_ParseID(cstr.as_ptr(), &mut guid).to_err()?;

            Ok(Guid { inner: guid })
        }
    }
}

impl Default for Guid {
    fn default() -> Guid {
        unsafe { zeroed() }
    }
}
