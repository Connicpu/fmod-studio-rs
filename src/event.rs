use std::{fmt, mem, ptr};

use ffi::*;
use error::*;
use guid::Guid;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventDescription {
    pub(crate) ptr: *mut FMOD_STUDIO_EVENTDESCRIPTION,
}

unsafe impl Send for EventDescription {}
unsafe impl Sync for EventDescription {}

impl EventDescription {
    pub fn create_instance(&self) -> Result<EventInstance> {
        unsafe {
            let mut ptr = ptr::null_mut();

            FMOD_Studio_EventDescription_CreateInstance(self.ptr, &mut ptr).to_err()?;

            Ok(EventInstance { ptr })
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut FMOD_STUDIO_EVENTDESCRIPTION {
        self.ptr
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventInstance {
    pub(crate) ptr: *mut FMOD_STUDIO_EVENTINSTANCE,
}

unsafe impl Send for EventInstance {}
unsafe impl Sync for EventInstance {}

impl EventInstance {
    pub fn start(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_Start(self.ptr).to_err() }
    }

    pub fn stop(&self, immediately: bool) -> Result<()> {
        let mode = if immediately {
            FMOD_STUDIO_STOP_MODE::FMOD_STUDIO_STOP_IMMEDIATE
        } else {
            FMOD_STUDIO_STOP_MODE::FMOD_STUDIO_STOP_ALLOWFADEOUT
        };

        unsafe { FMOD_Studio_EventInstance_Stop(self.ptr, mode).to_err() }
    }

    pub unsafe fn as_ptr(&self) -> *mut FMOD_STUDIO_EVENTINSTANCE {
        self.ptr
    }
}
