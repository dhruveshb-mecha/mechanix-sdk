use super::types::{ConfirmationResponder, PairingRequest, PasskeyResponder, PinResponder};
use tokio::sync::{mpsc, oneshot};
use zbus::{fdo, interface, zvariant::ObjectPath};

pub(crate) struct BluezAgent {
    pub ui_tx: mpsc::Sender<PairingRequest>,
}

impl BluezAgent {
    fn extract_mac(path: &ObjectPath<'_>) -> fdo::Result<String> {
        path.as_str()
            .split("dev_")
            .last()
            .map(|s| s.replace('_', ":"))
            .filter(|s| !s.is_empty())
            .ok_or_else(|| fdo::Error::Failed(format!("malformed device path: {path}")))
    }

    async fn forward(&self, request: PairingRequest) -> fdo::Result<()> {
        self.ui_tx
            .send(request)
            .await
            .map_err(|_| fdo::Error::Failed("Pairing channel closed".into()))
    }
}

#[interface(name = "org.bluez.Agent1")]
impl BluezAgent {
    async fn request_pin_code(&self, device: ObjectPath<'_>) -> fdo::Result<String> {
        let (tx, rx) = oneshot::channel();
        self.forward(PairingRequest::RequestPinCode {
            device_address: Self::extract_mac(&device)?,
            responder: PinResponder(tx),
        })
        .await?;

        rx.await
            .map_err(|_| fdo::Error::AccessDenied("Rejected by user".into()))
    }

    async fn request_passkey(&self, device: ObjectPath<'_>) -> fdo::Result<u32> {
        let (tx, rx) = oneshot::channel();
        self.forward(PairingRequest::RequestPasskey {
            device_address: Self::extract_mac(&device)?,
            responder: PasskeyResponder(tx),
        })
        .await?;

        rx.await
            .map_err(|_| fdo::Error::AccessDenied("Rejected by user".into()))
    }

    async fn request_confirmation(&self, device: ObjectPath<'_>, passkey: u32) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.forward(PairingRequest::RequestConfirmation {
            device_address: Self::extract_mac(&device)?,
            passkey,
            responder: ConfirmationResponder(tx),
        })
        .await?;

        match rx.await {
            Ok(true) => Ok(()),
            Ok(false) | Err(_) => Err(fdo::Error::AccessDenied("Rejected by user".into())),
        }
    }

    async fn display_pin_code(&self, device: ObjectPath<'_>, pincode: String) {
        if let Ok(addr) = Self::extract_mac(&device) {
            let _ = self
                .forward(PairingRequest::DisplayPinCode {
                    device_address: addr,
                    pin_code: pincode,
                })
                .await;
        }
    }

    async fn display_passkey(&self, device: ObjectPath<'_>, passkey: u32, entered: u16) {
        if let Ok(addr) = Self::extract_mac(&device) {
            let _ = self
                .forward(PairingRequest::DisplayPasskey {
                    device_address: addr,
                    passkey,
                    entered,
                })
                .await;
        }
    }

    async fn cancel(&self) {
        let _ = self.ui_tx.send(PairingRequest::Cancel).await;
    }

    async fn release(&self) {}

    async fn authorize_service(&self, _device: ObjectPath<'_>, _uuid: String) -> fdo::Result<()> {
        Ok(())
    }

    async fn request_authorization(&self, _device: ObjectPath<'_>) -> fdo::Result<()> {
        Ok(())
    }
}
