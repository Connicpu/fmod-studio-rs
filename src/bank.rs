use std::{fmt, mem, ptr};

use ffi::*;
use error::*;
use event::EventDescription;
use guid::Guid;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bank {
    pub(crate) ptr: *mut FMOD_STUDIO_BANK,
}

unsafe impl Send for Bank {}
unsafe impl Sync for Bank {}

impl Bank {
    // TODO: Buses

    pub fn get_event_count(&self) -> Result<i32> {
        unsafe {
            let mut count = 0;
            FMOD_Studio_Bank_GetEventCount(self.ptr, &mut count).to_err()?;
            Ok(count)
        }
    }

    pub fn get_events(&self) -> Result<Vec<EventDescription>> {
        unsafe {
            let mut count = 0;
            FMOD_Studio_Bank_GetEventCount(self.ptr, &mut count).to_err()?;

            let mut events = Vec::with_capacity(count as usize);
            FMOD_Studio_Bank_GetEventList(
                self.ptr,
                events.as_mut_ptr() as *mut _,
                events.capacity() as i32,
                &mut count,
            ).to_err()?;
            events.set_len(count as usize);

            Ok(events)
        }
    }

    pub fn get_id(&self) -> Result<Guid> {
        unsafe {
            let mut guid = mem::uninitialized();

            FMOD_Studio_Bank_GetID(self.ptr, &mut guid).to_err()?;

            Ok(Guid { inner: guid })
        }
    }

    pub fn get_loading_state(&self) -> Result<LoadingState> {
        unsafe {
            let mut state = mem::uninitialized();

            FMOD_Studio_Bank_GetLoadingState(self.ptr, &mut state).to_err()?;

            let state = match state {
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_UNLOADING => {
                    LoadingState::Unloading
                }
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_UNLOADED => {
                    LoadingState::Unloaded
                }
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_LOADING => {
                    LoadingState::Loading
                }
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_LOADED => LoadingState::Loaded,
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_ERROR => LoadingState::Error,
                _ => panic!(),
            };

            Ok(state)
        }
    }

    pub fn get_sample_loading_state(&self) -> Result<LoadingState> {
        unsafe {
            let mut state = mem::uninitialized();

            FMOD_Studio_Bank_GetSampleLoadingState(self.ptr, &mut state).to_err()?;

            let state = match state {
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_UNLOADING => {
                    LoadingState::Unloading
                }
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_UNLOADED => {
                    LoadingState::Unloaded
                }
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_LOADING => {
                    LoadingState::Loading
                }
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_LOADED => LoadingState::Loaded,
                FMOD_STUDIO_LOADING_STATE::FMOD_STUDIO_LOADING_STATE_ERROR => LoadingState::Error,
                _ => panic!(),
            };

            Ok(state)
        }
    }

    pub fn get_path(&self) -> Result<String> {
        unsafe {
            let mut size = 0;
            FMOD_Studio_Bank_GetPath(self.ptr, ptr::null_mut(), 0, &mut size).to_err()?;

            let mut data = vec![0u8; size as usize];
            FMOD_Studio_Bank_GetPath(self.ptr, data.as_mut_ptr() as *mut _, size, ptr::null_mut())
                .to_err()?;

            Ok(String::from_utf8_lossy(&data).into_owned())
        }
    }

    // TODO: VCAs

    pub fn load_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_LoadSampleData(self.ptr).to_err() }
    }

    pub fn unload(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_Unload(self.ptr).to_err() }
    }

    pub unsafe fn as_ptr(&self) -> *mut FMOD_STUDIO_BANK {
        self.ptr
    }
}

impl fmt::Debug for Bank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Bank")
            .field("path", &self.get_path())
            .field("event_count", &self.get_event_count())
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LoadingState {
    Unloading,
    Unloaded,
    Loading,
    Loaded,
    Error,
}
