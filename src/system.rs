use std::{mem, ptr};
use std::ffi::CString;

use ffi::*;
use error::*;
use bank::Bank;
use event::EventDescription;
use guid::Guid;

#[repr(C)]
pub struct System {
    pub(crate) ptr: *mut FMOD_STUDIO_SYSTEM,
}

unsafe impl Send for System {}
unsafe impl Sync for System {}

impl System {
    pub fn new(maxchannels: i32, liveupdate: bool) -> Result<Self> {
        unsafe {
            let mut ptr = ptr::null_mut();

            let flags = if liveupdate {
                FMOD_STUDIO_INIT_LIVEUPDATE
            } else {
                FMOD_STUDIO_INIT_NORMAL
            };

            FMOD_Studio_System_Create(&mut ptr, FMOD_VERSION).to_err()?;
            FMOD_Studio_System_Initialize(
                ptr,
                maxchannels,
                flags,
                FMOD_INIT_NORMAL,
                ptr::null_mut(),
            ).to_err()?;

            Ok(System { ptr })
        }
    }

    pub fn update(&mut self) -> Result<()> {
        unsafe { FMOD_Studio_System_Update(self.ptr).to_err() }
    }

    pub fn flush_commands(&mut self) -> Result<()> {
        unsafe { FMOD_Studio_System_FlushCommands(self.ptr).to_err() }
    }

    pub fn flush_sample_loading(&mut self) -> Result<()> {
        unsafe { FMOD_Studio_System_FlushSampleLoading(self.ptr).to_err() }
    }

    pub fn get_bank(&self, id: &str) -> Result<Bank> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let cstr = CString::new(id).unwrap();

            FMOD_Studio_System_GetBank(self.ptr, cstr.as_ptr(), &mut ptr).to_err()?;

            Ok(Bank { ptr })
        }
    }

    pub fn get_bank_by_id(&self, id: &Guid) -> Result<Bank> {
        unsafe {
            let mut ptr = ptr::null_mut();

            FMOD_Studio_System_GetBankByID(self.ptr, &id.inner, &mut ptr).to_err()?;

            Ok(Bank { ptr })
        }
    }

    pub fn get_cpu_usage(&self) -> Result<FMOD_STUDIO_CPU_USAGE> {
        unsafe {
            let mut usage = mem::uninitialized();

            FMOD_Studio_System_GetCPUUsage(self.ptr, &mut usage).to_err()?;

            Ok(usage)
        }
    }

    pub fn get_event(&self, id: &str) -> Result<EventDescription> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let cstr = CString::new(id).unwrap();

            FMOD_Studio_System_GetEvent(self.ptr, cstr.as_ptr(), &mut ptr).to_err()?;

            Ok(EventDescription { ptr })
        }
    }

    pub fn get_event_by_id(&self, id: &Guid) -> Result<EventDescription> {
        unsafe {
            let mut ptr = ptr::null_mut();

            FMOD_Studio_System_GetEventByID(self.ptr, &id.inner, &mut ptr).to_err()?;

            Ok(EventDescription { ptr })
        }
    }

    pub fn load_bank_file(&mut self, file: &str, async: bool) -> Result<Bank> {
        let flags = if async {
            FMOD_STUDIO_LOAD_BANK_NONBLOCKING
        } else {
            FMOD_STUDIO_LOAD_BANK_NORMAL
        };

        unsafe {
            let mut ptr = ptr::null_mut();
            let cstr = CString::new(file).unwrap();

            FMOD_Studio_System_LoadBankFile(self.ptr, cstr.as_ptr(), flags, &mut ptr).to_err()?;

            Ok(Bank { ptr })
        }
    }

    pub fn load_bank_memory(&mut self, buffer: &[u8], async: bool) -> Result<Bank> {
        let flags = if async {
            FMOD_STUDIO_LOAD_BANK_NONBLOCKING
        } else {
            FMOD_STUDIO_LOAD_BANK_NORMAL
        };

        unsafe {
            let mut ptr = ptr::null_mut();

            FMOD_Studio_System_LoadBankMemory(
                self.ptr,
                buffer.as_ptr() as *const _,
                buffer.len() as i32,
                FMOD_STUDIO_LOAD_MEMORY_MODE::FMOD_STUDIO_LOAD_MEMORY,
                flags,
                &mut ptr,
            ).to_err()?;

            Ok(Bank { ptr })
        }
    }

    pub fn lookup_id(&self, id: &str) -> Result<Guid> {
        unsafe {
            let mut guid = mem::uninitialized();
            let cstr = CString::new(id).unwrap();

            FMOD_Studio_System_LookupID(self.ptr, cstr.as_ptr(), &mut guid).to_err()?;

            Ok(Guid { inner: guid })
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut FMOD_STUDIO_SYSTEM {
        self.ptr
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe {
            FMOD_Studio_System_Release(self.ptr)
                .to_err()
                .map_err(|err| {
                    eprintln!(
                        "Error dropping fmod_studio::System: {:?}: {}",
                        err,
                        err.description()
                    );
                })
                .ok();
        }
    }
}
