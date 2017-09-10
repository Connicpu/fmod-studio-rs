use std::cmp::Ordering;
use std::ffi::CString;
use std::mem::{size_of_val, uninitialized, zeroed};
use std::slice;

use ffi::*;
use error::*;

#[derive(Copy, Clone)]
#[repr(C)]
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

impl PartialEq for Guid {
    fn eq(&self, rhs: &Guid) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}

impl Eq for Guid {}

impl PartialOrd for Guid {
    fn partial_cmp(&self, rhs: &Guid) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Guid {
    fn cmp(&self, rhs: &Guid) -> Ordering {
        unsafe {
            let left = slice::from_raw_parts(self as *const _ as *const u8, size_of_val(self));
            let right = slice::from_raw_parts(rhs as *const _ as *const u8, size_of_val(rhs));

            left.cmp(right)
        }
    }
}
