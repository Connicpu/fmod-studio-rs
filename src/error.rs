use std::{self, fmt};
use std::error::Error;

use ffi::FMOD_RESULT;

pub type Result<T> = std::result::Result<T, FmodError>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FmodError {
    BadCommand,
    ChannelAlloc,
    ChannelStolen,
    Dma,
    DspConnection,
    DspDontprocess,
    DspFormat,
    DspInuse,
    DspNotFound,
    DspReserved,
    DspSilence,
    DspType,
    FileBad,
    FileCouldNotSeek,
    FileDiskEjected,
    FileEof,
    FileEndOfData,
    FileNotFound,
    Format,
    HeaderMismatch,
    Http,
    HttpAccess,
    HttpProxyAuth,
    HttpServerError,
    HttpTimeout,
    Initialization,
    Initialized,
    Internal,
    InvalidFloat,
    InvalidHandle,
    InvalidParam,
    InvalidPosition,
    InvalidSpeaker,
    InvalidSyncPoint,
    InvalidThread,
    InvalidVector,
    MaxAudible,
    Memory,
    MemoryCantPoint,
    Needs3d,
    NeedsHardware,
    NetConnect,
    NetSocketError,
    NetUrl,
    NetWouldBlock,
    NotReady,
    OutputAllocated,
    OutputCreateBuffer,
    OutputDriverCall,
    OutputFormat,
    OutputInit,
    OutputNoDrivers,
    Plugin,
    PluginMissing,
    PluginResource,
    PluginVersion,
    Record,
    ReverbChannelGroup,
    ReverbInstance,
    Subsounds,
    SubsoundAllocated,
    SubsoundCantMove,
    TagNotFound,
    TooManyChannels,
    Truncated,
    Unimplemented,
    Uninitialized,
    Unsupported,
    Version,
    EventAlreadyLoaded,
    EventLiveUpdateBusy,
    EventLiveUpdateMismatch,
    EventLiveUpdateTimeout,
    EventNotFound,
    StudioUninitialized,
    StudioNotLoaded,
    InvalidString,
    AlreadyLocked,
    NotLocked,
    RecordDisconnected,
    TooManySamples,
}

impl FmodError {
    pub fn from_result(result: FMOD_RESULT) -> Result<()> {
        use ffi::FMOD_RESULT::*;
        use self::FmodError::*;

        Err(match result {
            FMOD_OK => return Ok(()),
            FMOD_ERR_BADCOMMAND => BadCommand,
            FMOD_ERR_CHANNEL_ALLOC => ChannelAlloc,
            FMOD_ERR_CHANNEL_STOLEN => ChannelStolen,
            FMOD_ERR_DMA => Dma,
            FMOD_ERR_DSP_CONNECTION => DspConnection,
            FMOD_ERR_DSP_DONTPROCESS => DspDontprocess,
            FMOD_ERR_DSP_FORMAT => DspFormat,
            FMOD_ERR_DSP_INUSE => DspInuse,
            FMOD_ERR_DSP_NOTFOUND => DspNotFound,
            FMOD_ERR_DSP_RESERVED => DspReserved,
            FMOD_ERR_DSP_SILENCE => DspSilence,
            FMOD_ERR_DSP_TYPE => DspType,
            FMOD_ERR_FILE_BAD => FileBad,
            FMOD_ERR_FILE_COULDNOTSEEK => FileCouldNotSeek,
            FMOD_ERR_FILE_DISKEJECTED => FileDiskEjected,
            FMOD_ERR_FILE_EOF => FileEof,
            FMOD_ERR_FILE_ENDOFDATA => FileEndOfData,
            FMOD_ERR_FILE_NOTFOUND => FileNotFound,
            FMOD_ERR_FORMAT => Format,
            FMOD_ERR_HEADER_MISMATCH => HeaderMismatch,
            FMOD_ERR_HTTP => Http,
            FMOD_ERR_HTTP_ACCESS => HttpAccess,
            FMOD_ERR_HTTP_PROXY_AUTH => HttpProxyAuth,
            FMOD_ERR_HTTP_SERVER_ERROR => HttpServerError,
            FMOD_ERR_HTTP_TIMEOUT => HttpTimeout,
            FMOD_ERR_INITIALIZATION => Initialization,
            FMOD_ERR_INITIALIZED => Initialized,
            FMOD_ERR_INTERNAL => Internal,
            FMOD_ERR_INVALID_FLOAT => InvalidFloat,
            FMOD_ERR_INVALID_HANDLE => InvalidHandle,
            FMOD_ERR_INVALID_PARAM => InvalidParam,
            FMOD_ERR_INVALID_POSITION => InvalidPosition,
            FMOD_ERR_INVALID_SPEAKER => InvalidSpeaker,
            FMOD_ERR_INVALID_SYNCPOINT => InvalidSyncPoint,
            FMOD_ERR_INVALID_THREAD => InvalidThread,
            FMOD_ERR_INVALID_VECTOR => InvalidVector,
            FMOD_ERR_MAXAUDIBLE => MaxAudible,
            FMOD_ERR_MEMORY => Memory,
            FMOD_ERR_MEMORY_CANTPOINT => MemoryCantPoint,
            FMOD_ERR_NEEDS3D => Needs3d,
            FMOD_ERR_NEEDSHARDWARE => NeedsHardware,
            FMOD_ERR_NET_CONNECT => NetConnect,
            FMOD_ERR_NET_SOCKET_ERROR => NetSocketError,
            FMOD_ERR_NET_URL => NetUrl,
            FMOD_ERR_NET_WOULD_BLOCK => NetWouldBlock,
            FMOD_ERR_NOTREADY => NotReady,
            FMOD_ERR_OUTPUT_ALLOCATED => OutputAllocated,
            FMOD_ERR_OUTPUT_CREATEBUFFER => OutputCreateBuffer,
            FMOD_ERR_OUTPUT_DRIVERCALL => OutputDriverCall,
            FMOD_ERR_OUTPUT_FORMAT => OutputFormat,
            FMOD_ERR_OUTPUT_INIT => OutputInit,
            FMOD_ERR_OUTPUT_NODRIVERS => OutputNoDrivers,
            FMOD_ERR_PLUGIN => Plugin,
            FMOD_ERR_PLUGIN_MISSING => PluginMissing,
            FMOD_ERR_PLUGIN_RESOURCE => PluginResource,
            FMOD_ERR_PLUGIN_VERSION => PluginVersion,
            FMOD_ERR_RECORD => Record,
            FMOD_ERR_REVERB_CHANNELGROUP => ReverbChannelGroup,
            FMOD_ERR_REVERB_INSTANCE => ReverbInstance,
            FMOD_ERR_SUBSOUNDS => Subsounds,
            FMOD_ERR_SUBSOUND_ALLOCATED => SubsoundAllocated,
            FMOD_ERR_SUBSOUND_CANTMOVE => SubsoundCantMove,
            FMOD_ERR_TAGNOTFOUND => TagNotFound,
            FMOD_ERR_TOOMANYCHANNELS => TooManyChannels,
            FMOD_ERR_TRUNCATED => Truncated,
            FMOD_ERR_UNIMPLEMENTED => Unimplemented,
            FMOD_ERR_UNINITIALIZED => Uninitialized,
            FMOD_ERR_UNSUPPORTED => Unsupported,
            FMOD_ERR_VERSION => Version,
            FMOD_ERR_EVENT_ALREADY_LOADED => EventAlreadyLoaded,
            FMOD_ERR_EVENT_LIVEUPDATE_BUSY => EventLiveUpdateBusy,
            FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH => EventLiveUpdateMismatch,
            FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT => EventLiveUpdateTimeout,
            FMOD_ERR_EVENT_NOTFOUND => EventNotFound,
            FMOD_ERR_STUDIO_UNINITIALIZED => StudioUninitialized,
            FMOD_ERR_STUDIO_NOT_LOADED => StudioNotLoaded,
            FMOD_ERR_INVALID_STRING => InvalidString,
            FMOD_ERR_ALREADY_LOCKED => AlreadyLocked,
            FMOD_ERR_NOT_LOCKED => NotLocked,
            FMOD_ERR_RECORD_DISCONNECTED => RecordDisconnected,
            FMOD_ERR_TOOMANYSAMPLES => TooManySamples,
            FMOD_RESULT_FORCEINT => panic!(),
        })
    }

    pub fn to_fmod_result(self) -> FMOD_RESULT {
        use self::FmodError::*;
        use ffi::FMOD_RESULT::*;

        match self {
            BadCommand => FMOD_ERR_BADCOMMAND,
            ChannelAlloc => FMOD_ERR_CHANNEL_ALLOC,
            ChannelStolen => FMOD_ERR_CHANNEL_STOLEN,
            Dma => FMOD_ERR_DMA,
            DspConnection => FMOD_ERR_DSP_CONNECTION,
            DspDontprocess => FMOD_ERR_DSP_DONTPROCESS,
            DspFormat => FMOD_ERR_DSP_FORMAT,
            DspInuse => FMOD_ERR_DSP_INUSE,
            DspNotFound => FMOD_ERR_DSP_NOTFOUND,
            DspReserved => FMOD_ERR_DSP_RESERVED,
            DspSilence => FMOD_ERR_DSP_SILENCE,
            DspType => FMOD_ERR_DSP_TYPE,
            FileBad => FMOD_ERR_FILE_BAD,
            FileCouldNotSeek => FMOD_ERR_FILE_COULDNOTSEEK,
            FileDiskEjected => FMOD_ERR_FILE_DISKEJECTED,
            FileEof => FMOD_ERR_FILE_EOF,
            FileEndOfData => FMOD_ERR_FILE_ENDOFDATA,
            FileNotFound => FMOD_ERR_FILE_NOTFOUND,
            Format => FMOD_ERR_FORMAT,
            HeaderMismatch => FMOD_ERR_HEADER_MISMATCH,
            Http => FMOD_ERR_HTTP,
            HttpAccess => FMOD_ERR_HTTP_ACCESS,
            HttpProxyAuth => FMOD_ERR_HTTP_PROXY_AUTH,
            HttpServerError => FMOD_ERR_HTTP_SERVER_ERROR,
            HttpTimeout => FMOD_ERR_HTTP_TIMEOUT,
            Initialization => FMOD_ERR_INITIALIZATION,
            Initialized => FMOD_ERR_INITIALIZED,
            Internal => FMOD_ERR_INTERNAL,
            InvalidFloat => FMOD_ERR_INVALID_FLOAT,
            InvalidHandle => FMOD_ERR_INVALID_HANDLE,
            InvalidParam => FMOD_ERR_INVALID_PARAM,
            InvalidPosition => FMOD_ERR_INVALID_POSITION,
            InvalidSpeaker => FMOD_ERR_INVALID_SPEAKER,
            InvalidSyncPoint => FMOD_ERR_INVALID_SYNCPOINT,
            InvalidThread => FMOD_ERR_INVALID_THREAD,
            InvalidVector => FMOD_ERR_INVALID_VECTOR,
            MaxAudible => FMOD_ERR_MAXAUDIBLE,
            Memory => FMOD_ERR_MEMORY,
            MemoryCantPoint => FMOD_ERR_MEMORY_CANTPOINT,
            Needs3d => FMOD_ERR_NEEDS3D,
            NeedsHardware => FMOD_ERR_NEEDSHARDWARE,
            NetConnect => FMOD_ERR_NET_CONNECT,
            NetSocketError => FMOD_ERR_NET_SOCKET_ERROR,
            NetUrl => FMOD_ERR_NET_URL,
            NetWouldBlock => FMOD_ERR_NET_WOULD_BLOCK,
            NotReady => FMOD_ERR_NOTREADY,
            OutputAllocated => FMOD_ERR_OUTPUT_ALLOCATED,
            OutputCreateBuffer => FMOD_ERR_OUTPUT_CREATEBUFFER,
            OutputDriverCall => FMOD_ERR_OUTPUT_DRIVERCALL,
            OutputFormat => FMOD_ERR_OUTPUT_FORMAT,
            OutputInit => FMOD_ERR_OUTPUT_INIT,
            OutputNoDrivers => FMOD_ERR_OUTPUT_NODRIVERS,
            Plugin => FMOD_ERR_PLUGIN,
            PluginMissing => FMOD_ERR_PLUGIN_MISSING,
            PluginResource => FMOD_ERR_PLUGIN_RESOURCE,
            PluginVersion => FMOD_ERR_PLUGIN_VERSION,
            Record => FMOD_ERR_RECORD,
            ReverbChannelGroup => FMOD_ERR_REVERB_CHANNELGROUP,
            ReverbInstance => FMOD_ERR_REVERB_INSTANCE,
            Subsounds => FMOD_ERR_SUBSOUNDS,
            SubsoundAllocated => FMOD_ERR_SUBSOUND_ALLOCATED,
            SubsoundCantMove => FMOD_ERR_SUBSOUND_CANTMOVE,
            TagNotFound => FMOD_ERR_TAGNOTFOUND,
            TooManyChannels => FMOD_ERR_TOOMANYCHANNELS,
            Truncated => FMOD_ERR_TRUNCATED,
            Unimplemented => FMOD_ERR_UNIMPLEMENTED,
            Uninitialized => FMOD_ERR_UNINITIALIZED,
            Unsupported => FMOD_ERR_UNSUPPORTED,
            Version => FMOD_ERR_VERSION,
            EventAlreadyLoaded => FMOD_ERR_EVENT_ALREADY_LOADED,
            EventLiveUpdateBusy => FMOD_ERR_EVENT_LIVEUPDATE_BUSY,
            EventLiveUpdateMismatch => FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH,
            EventLiveUpdateTimeout => FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT,
            EventNotFound => FMOD_ERR_EVENT_NOTFOUND,
            StudioUninitialized => FMOD_ERR_STUDIO_UNINITIALIZED,
            StudioNotLoaded => FMOD_ERR_STUDIO_NOT_LOADED,
            InvalidString => FMOD_ERR_INVALID_STRING,
            AlreadyLocked => FMOD_ERR_ALREADY_LOCKED,
            NotLocked => FMOD_ERR_NOT_LOCKED,
            RecordDisconnected => FMOD_ERR_RECORD_DISCONNECTED,
            TooManySamples => FMOD_ERR_TOOMANYSAMPLES,
        }
    }

    pub fn description(self) -> &'static str {
        use self::FmodError::*;

        match self {
            BadCommand => {
                "Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound::lock on a streaming sound)."
            }
            ChannelAlloc => "Error trying to allocate a channel.",
            ChannelStolen => "The specified channel has been reused to play another sound.",
            Dma => "DMA Failure.  See debug output for more information.",
            DspConnection => {
                "DSP connection error.  Connection possibly caused a cyclic dependency or connected dsps with incompatible buffer counts."
            }
            DspDontprocess => {
                "DSP return code from a DSP process query callback.  Tells mixer not to call the process callback and therefore not consume CPU.  Use this to optimize the DSP graph."
            }
            DspFormat => {
                "DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format, or a matrix may have been set with the wrong size if the target unit has a specified channel map."
            }
            DspInuse => {
                "DSP is already in the mixer's DSP network. It must be removed before being reinserted or released."
            }
            DspNotFound => "DSP connection error.  Couldn't find the DSP unit specified.",
            DspReserved => {
                "DSP operation error.  Cannot perform operation on this DSP as it is reserved by the system."
            }
            DspSilence => {
                "DSP return code from a DSP process query callback.  Tells mixer silence would be produced from read, so go idle and not consume CPU.  Use this to optimize the DSP graph."
            }
            DspType => "DSP operation cannot be performed on a DSP of this type.",
            FileBad => "Error loading file.",
            FileCouldNotSeek => {
                "Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format."
            }
            FileDiskEjected => "Media was ejected while reading.",
            FileEof => {
                "End of file unexpectedly reached while trying to read essential data (truncated?)."
            }
            FileEndOfData => "End of current chunk reached while trying to read data.",
            FileNotFound => "File not found.",
            Format => "Unsupported file or audio format.",
            HeaderMismatch => {
                "There is a version mismatch between the FMOD header and either the FMOD Studio library or the FMOD Low Level library."
            }
            Http => {
                "A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere."
            }
            HttpAccess => "The specified resource requires authentication or is forbidden.",
            HttpProxyAuth => "Proxy authentication is required to access the specified resource.",
            HttpServerError => "A HTTP server error occurred.",
            HttpTimeout => "The HTTP request timed out.",
            Initialization => "FMOD was not initialized correctly to support this function.",
            Initialized => "Cannot call this command after System::init.",
            Internal => "An error occurred that wasn't supposed to.  Contact support.",
            InvalidFloat => "Value passed in was a NaN, Inf or denormalized float.",
            InvalidHandle => "An invalid object handle was used.",
            InvalidParam => "An invalid parameter was passed to this function.",
            InvalidPosition => "An invalid seek position was passed to this function.",
            InvalidSpeaker => {
                "An invalid speaker was passed to this function based on the current speaker mode."
            }
            InvalidSyncPoint => "The syncpoint did not come from this sound handle.",
            InvalidThread => "Tried to call a function on a thread that is not supported.",
            InvalidVector => "The vectors passed in are not unit length, or perpendicular.",
            MaxAudible => "Reached maximum audible playback count for this sound's soundgroup.",
            Memory => "Not enough memory or resources.",
            MemoryCantPoint => {
                "Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used."
            }
            Needs3d => {
                "Tried to call a command on a 2d sound when the command was meant for 3d sound."
            }
            NeedsHardware => "Tried to use a feature that requires hardware support.",
            NetConnect => "Couldn't connect to the specified host.",
            NetSocketError => {
                "A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere."
            }
            NetUrl => "The specified URL couldn't be resolved.",
            NetWouldBlock => "Operation on a non-blocking socket could not complete immediately.",
            NotReady => {
                "Operation could not be performed because specified sound/DSP connection is not ready."
            }
            OutputAllocated => {
                "Error initializing output device, but more specifically, the output device is already in use and cannot be reused."
            }
            OutputCreateBuffer => "Error creating hardware sound buffer.",
            OutputDriverCall => {
                "A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted."
            }
            OutputFormat => "Soundcard does not support the specified format.",
            OutputInit => "Error initializing output device.",
            OutputNoDrivers => {
                "The output device has no drivers installed.  If pre-init, FMOD_OUTPUT_NOSOUND is selected as the output mode.  If post-init, the function just fails."
            }
            Plugin => "An unspecified error has been returned from a plugin.",
            PluginMissing => "A requested output, dsp unit type or codec was not available.",
            PluginResource => {
                "A resource that the plugin requires cannot be found. (ie the DLS file for MIDI playback)"
            }
            PluginVersion => "A plugin was built with an unsupported SDK version.",
            Record => "An error occurred trying to initialize the recording device.",
            ReverbChannelGroup => {
                "Reverb properties cannot be set on this channel because a parent channelgroup owns the reverb connection."
            }
            ReverbInstance => {
                "Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesn't exist."
            }
            Subsounds => {
                "The error occurred because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound."
            }
            SubsoundAllocated => {
                "This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first."
            }
            SubsoundCantMove => {
                "Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file."
            }
            TagNotFound => "The specified tag could not be found or there are no tags.",
            TooManyChannels => {
                "The sound created exceeds the allowable input channel count.  This can be increased using the 'maxinputchannels' parameter in System::setSoftwareFormat."
            }
            Truncated => {
                "The retrieved string is too long to fit in the supplied buffer and has been truncated."
            }
            Unimplemented => {
                "Something in FMOD hasn't been implemented when it should be! contact support!"
            }
            Uninitialized => {
                "This command failed because System::init or System::setDriver was not called."
            }
            Unsupported => {
                "A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified."
            }
            Version => "The version number of this file format is not supported.",
            EventAlreadyLoaded => "The specified bank has already been loaded.",
            EventLiveUpdateBusy => {
                "The live update connection failed due to the game already being connected."
            }
            EventLiveUpdateMismatch => {
                "The live update connection failed due to the game data being out of sync with the tool."
            }
            EventLiveUpdateTimeout => "The live update connection timed out.",
            EventNotFound => "The requested event, bus or vca could not be found.",
            StudioUninitialized => "The Studio::System object is not yet initialized.",
            StudioNotLoaded => "The specified resource is not loaded, so it can't be unloaded.",
            InvalidString => "An invalid string was passed to this function.",
            AlreadyLocked => "The specified resource is already locked.",
            NotLocked => "The specified resource is not locked, so it can't be unlocked.",
            RecordDisconnected => "The specified recording driver has been disconnected.",
            TooManySamples => "The length provided exceeds the allowable limit.",
        }
    }
}

pub trait ToError {
    fn to_err(self) -> Result<()>;
}

impl ToError for FMOD_RESULT {
    fn to_err(self) -> Result<()> {
        FmodError::from_result(self)
    }
}

impl fmt::Display for FmodError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for FmodError {
    fn description(&self) -> &str {
        FmodError::description(*self)
    }
}
