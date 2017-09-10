use ffi::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventDescription {
    pub(crate) ptr: *mut FMOD_STUDIO_EVENTDESCRIPTION,
}

unsafe impl Send for EventDescription {}
unsafe impl Sync for EventDescription {}

impl EventDescription {
    pub unsafe fn as_ptr(&self) -> *mut FMOD_STUDIO_EVENTDESCRIPTION {
        self.ptr
    }
}
