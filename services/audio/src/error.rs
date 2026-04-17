use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("PipeWire initialization failed: {0}")]
    InitFailed(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("No default output device available")]
    NoDefaultOutput,

    #[error("No default input device available")]
    NoDefaultInput,

    #[error("Failed to set volume for device '{device}': {reason}")]
    SetVolumeFailed { device: String, reason: String },

    #[error("Failed to set mute for device '{device}': {reason}")]
    SetMuteFailed { device: String, reason: String },

    #[error("Failed to set default {direction} device: {reason}")]
    SetDefaultFailed { direction: String, reason: String },

    #[error("PipeWire operation timed out")]
    Timeout,

    #[error("PipeWire error: {0}")]
    Pipewire(String),
}
