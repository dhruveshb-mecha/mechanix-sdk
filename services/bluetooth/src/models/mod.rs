pub mod adapter_info;
pub mod device_info;

use std::collections::HashMap;
use zbus::zvariant::OwnedValue;

// ---------- Private Extraction Helpers ----------

fn extract_i16(props: &HashMap<String, OwnedValue>, key: &str) -> Option<i16> {
    props.get(key).and_then(|v| i16::try_from(v).ok())
}

fn extract_u32(props: &HashMap<String, OwnedValue>, key: &str) -> Option<u32> {
    props.get(key).and_then(|v| u32::try_from(v).ok())
}

fn extract_bool(props: &HashMap<String, OwnedValue>, key: &str) -> Option<bool> {
    props.get(key).and_then(|v| bool::try_from(v).ok())
}

fn extract_string(props: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    props
        .get(key)
        .and_then(|v| <&str>::try_from(v).ok().map(String::from))
}
