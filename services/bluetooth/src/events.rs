use crate::{AdapterInfo, DeviceInfo};

/// Represents all possible hardware and connection events emitted by the Bluetooth system.
#[derive(Debug, Clone)]
pub enum BluetoothEvent {
    // --- Adapter Events ---
    /// Fired when a new Bluetooth adapter is plugged in (e.g., a USB dongle).
    AdapterAdded(AdapterInfo),
    /// Fired when a Bluetooth adapter is physically removed.
    AdapterRemoved { name: String },
    /// Fired when an adapter (e.g., "hci0") is turned on or off.
    AdapterPowerChanged { adapter_name: String, powered: bool },
    /// Fired when an adapter starts or stops scanning.
    DiscoveryStateChanged {
        adapter_name: String,
        discovering: bool,
    },

    // --- Device Lifecycle Events ---
    /// Fired when the radio discovers a new device that wasn't previously cached.
    DeviceDiscovered(DeviceInfo),
    /// Fired when a device goes out of range or is forgotten.
    DeviceLost { path: String },

    // --- Device State Events ---
    /// Fired when a device successfully establishes a data connection.
    DeviceConnected { path: String, address: String },
    /// Fired when a device drops its connection.
    DeviceDisconnected { path: String, address: String },
    /// Fired when a device's battery level changes.
    BatteryChanged { path: String, percentage: u8 },

    /// Fired when a device's signal strength (RSSI) changes.
    /// Values typically range from -100 (weak/far) to 0 (strong/close).
    DeviceRssiChanged { path: String, rssi: i16 },
}
