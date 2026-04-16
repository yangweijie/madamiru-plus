use serde::{Deserialize, Serialize};

use crate::dlna::error::DlnaError;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        .map(|r| DlnaDevice {
            name: r.friendly_name().to_string(),
            location: r.description_url().to_string(),
            udn: r.udn().to_string(),
        })
        .collect())
}
