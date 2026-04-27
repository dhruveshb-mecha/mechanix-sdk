use tokio::sync::oneshot;

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

// ---------- Typed Responders ----------

#[derive(Debug)]
pub struct PinResponder(pub(crate) oneshot::Sender<String>);

impl PinResponder {
    pub fn reply(self, pin: impl Into<String>) {
        let _ = self.0.send(pin.into());
    }
    pub fn reject(self) {
        drop(self.0);
    }
}

#[derive(Debug)]
pub struct PasskeyResponder(pub(crate) oneshot::Sender<u32>);

impl PasskeyResponder {
    pub fn reply(self, passkey: u32) {
        let _ = self.0.send(passkey);
    }
    pub fn reject(self) {
        drop(self.0);
    }
}

#[derive(Debug)]
pub struct ConfirmationResponder(pub(crate) oneshot::Sender<bool>);

impl ConfirmationResponder {
    pub fn confirm(self) {
        let _ = self.0.send(true);
    }
    pub fn reject(self) {
        let _ = self.0.send(false);
    }
}

// ---------- Public API ----------

#[derive(Debug)]
pub enum PairingRequest {
    RequestPinCode {
        device_address: String,
        responder: PinResponder,
    },
    RequestPasskey {
        device_address: String,
        responder: PasskeyResponder,
    },
    RequestConfirmation {
        device_address: String,
        passkey: u32,
        responder: ConfirmationResponder,
    },
    DisplayPinCode {
        device_address: String,
        pin_code: String,
    },
    DisplayPasskey {
        device_address: String,
        passkey: u32,
        entered: u16,
    },
    Cancel,
}
