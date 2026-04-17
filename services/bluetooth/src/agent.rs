use tokio::sync::{mpsc, oneshot};
use zbus::{fdo, interface, zvariant::ObjectPath};

/// BlueZ pairing I/O capability for agent registration.
#[derive(Debug, Clone, Copy)]
pub enum AgentCapability {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
}

impl AgentCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayOnly => "DisplayOnly",
            Self::DisplayYesNo => "DisplayYesNo",
            Self::KeyboardOnly => "KeyboardOnly",
            Self::NoInputNoOutput => "NoInputNoOutput",
            Self::KeyboardDisplay => "KeyboardDisplay",
        }
    }
}

/// Requests sent from the BlueZ daemon to your UI SDK.
#[derive(Debug)]
pub enum PairingRequest {
    /// Legacy devices. UI must prompt user to type a PIN.
    RequestPinCode {
        device_address: String,
        reply: oneshot::Sender<String>,
    },
    /// Numeric passkey flow. UI must prompt user to enter a passkey.
    RequestPasskey {
        device_address: String,
        reply: oneshot::Sender<u32>,
    },
    /// Modern devices. UI must ask: "Does the screen show 123456?"
    RequestConfirmation {
        device_address: String,
        passkey: u32,
        reply: oneshot::Sender<bool>,
    },
    /// UI must display a pincode for legacy keyboards.
    DisplayPinCode {
        device_address: String,
        pin_code: String,
    },
    /// UI must display a 6-digit passkey for the user to type on the external device.
    DisplayPasskey {
        device_address: String,
        passkey: u32,
        entered: u16,
    },
    /// The pairing process was canceled (e.g., timeout or device disconnected).
    Cancel,
}

// ==========================================
// Internal D-Bus Server
// ==========================================

pub(crate) struct BluezAgent {
    pub ui_tx: mpsc::Sender<PairingRequest>,
}

impl BluezAgent {
    /// Helper to cleanly extract MAC address from BlueZ object paths
    fn extract_mac(path: &ObjectPath<'_>) -> String {
        path.as_str()
            .split("dev_")
            .last()
            .unwrap_or("Unknown")
            .replace('_', ":")
    }

    async fn send_to_ui(&self, request: PairingRequest) -> fdo::Result<()> {
        self.ui_tx
            .send(request)
            .await
            .map_err(|_| fdo::Error::Failed("Pairing UI channel closed".into()))
    }
}

#[interface(name = "org.bluez.Agent1")]
impl BluezAgent {
    async fn request_pin_code(&self, device: ObjectPath<'_>) -> fdo::Result<String> {
        let (tx, rx) = oneshot::channel();
        let address = Self::extract_mac(&device);

        self.send_to_ui(PairingRequest::RequestPinCode {
            device_address: address,
            reply: tx,
        })
        .await?;

        rx.await
            .map_err(|_| fdo::Error::AccessDenied("Rejected by user".into()))
    }

    async fn request_passkey(&self, device: ObjectPath<'_>) -> fdo::Result<u32> {
        let (tx, rx) = oneshot::channel();
        let address = Self::extract_mac(&device);

        self.send_to_ui(PairingRequest::RequestPasskey {
            device_address: address,
            reply: tx,
        })
        .await?;

        rx.await
            .map_err(|_| fdo::Error::AccessDenied("Rejected by user".into()))
    }

    async fn request_confirmation(&self, device: ObjectPath<'_>, passkey: u32) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        let address = Self::extract_mac(&device);

        self.send_to_ui(PairingRequest::RequestConfirmation {
            device_address: address,
            passkey,
            reply: tx,
        })
        .await?;

        match rx.await {
            Ok(true) => Ok(()),
            Ok(false) => Err(fdo::Error::AccessDenied("Rejected by user".into())),
            Err(_) => Err(fdo::Error::Failed("UI did not respond".into())),
        }
    }

    async fn display_pin_code(&self, device: ObjectPath<'_>, pincode: String) {
        let address = Self::extract_mac(&device);
        let _ = self
            .send_to_ui(PairingRequest::DisplayPinCode {
                device_address: address,
                pin_code: pincode,
            })
            .await;
    }

    async fn display_passkey(&self, device: ObjectPath<'_>, passkey: u32, entered: u16) {
        let address = Self::extract_mac(&device);
        let _ = self
            .send_to_ui(PairingRequest::DisplayPasskey {
                device_address: address,
                passkey,
                entered,
            })
            .await;
    }

    async fn cancel(&self) {
        let _ = self.ui_tx.send(PairingRequest::Cancel).await;
    }

    // --- Required Stubs for BlueZ ---
    async fn release(&self) {}

    async fn authorize_service(&self, _device: ObjectPath<'_>, _uuid: String) -> fdo::Result<()> {
        Ok(())
    }

    async fn request_authorization(&self, _device: ObjectPath<'_>) -> fdo::Result<()> {
        Ok(())
    }
}
