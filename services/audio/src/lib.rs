pub mod audio;
pub mod error;

pub use audio::client::AudioClient;
pub use audio::types::{AudioDevice, DeviceType};
pub use error::AudioError;
