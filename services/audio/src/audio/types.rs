/// device type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// playback device
    Output,
    /// recording device
    Input,
}

#[derive(Debug, Clone)]
/// Audio device.
pub struct AudioDevice {
    /// PipeWire node ID.
    pub id: u32,
    /// device name.
    pub name: String,
    /// Optional device description.
    pub description: Option<String>,
    /// device type.
    pub device_type: DeviceType,
    /// Current volume in range [0.0, 1.0].
    pub volume: f32,
    /// Current mute state.
    pub muted: bool,
    /// True when this is the system default for its direction.
    pub is_default: bool,
}

impl AudioDevice {
    /// returns true if this is an output (playback) device.
    pub fn is_output(&self) -> bool {
        self.device_type == DeviceType::Output
    }

    /// returns true if this is an input (recording) device.
    pub fn is_input(&self) -> bool {
        self.device_type == DeviceType::Input
    }
}
