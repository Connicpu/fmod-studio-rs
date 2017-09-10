use ffi::*;
use error::*;

use std::ptr;

pub struct System {
    ptr: *mut FMOD_STUDIO_SYSTEM,
}

impl System {
    fn new() -> Result<Self> {
        let mut ptr = ptr::null_mut();



        Ok(System { ptr })
    }
}

impl Drop for System {
    fn drop(&mut self) {}
}
