use serde::{Deserialize, Serialize};
use std::fmt;

use crate::dlna::error::DlnaError;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct DlnaDevice {
    pub name: String,
    pub location: String,
    pub udn: String,
}

impl fmt::Display for DlnaDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for DlnaDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DlnaDevice")
            .field("name", &self.name)
            .field("location", &self.location)
            .field("udn", &self.udn)
            .finish()
    }
}

/// Discover DLNA devices on the local network
pub async fn discover_devices(timeout_secs: u64) -> Result<Vec<DlnaDevice>, DlnaError> {
    let renders = crab_dlna::Render::discover(timeout_secs)
        .await
        .map_err(|e| DlnaError::Discovery(e.to_string()))?;

    Ok(renders
        .into_iter()
        .map(|r| {
            let device = &r.device;
            DlnaDevice {
                name: device.friendly_name().to_string(),
                location: device.url().to_string(),
                udn: device.url().to_string(), // Use URL as fallback for UDN
            }
        })
        .collect())
}