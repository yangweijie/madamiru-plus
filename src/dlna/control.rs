use crate::dlna::error::DlnaError;

pub struct DlnaRenderer {
    render: crab_dlna::Render,
}

impl DlnaRenderer {
    pub fn new(render: crab_dlna::Render) -> Self {
        Self { render }
    }

    pub async fn play(&self, url: &str) -> Result<(), DlnaError> {
        crab_dlna::play(&self.render, url)
            .await
            .map_err(|e| DlnaError::Control(e.to_string()))
    }

    pub async fn pause(&self) -> Result<(), DlnaError> {
        crab_dlna::pause(&self.render)
            .await
            .map_err(|e| DlnaError::Control(e.to_string()))
    }

    pub async fn stop(&self) -> Result<(), DlnaError> {
        crab_dlna::stop(&self.render)
            .await
            .map_err(|e| DlnaError::Control(e.to_string()))
    }

    pub async fn set_volume(&self, volume: u8) -> Result<(), DlnaError> {
        crab_dlna::set_volume(&self.render, volume)
            .await
            .map_err(|e| DlnaError::Control(e.to_string()))
    }
}

/// Create a new renderer from a device location
pub async fn create_renderer(
    location: &str,
    timeout_secs: u64,
) -> Result<DlnaRenderer, DlnaError> {
    let render = crab_dlna::Render::new(crab_dlna::RenderSpec::Location(
        timeout_secs,
        location.to_string(),
    ))
    .await
    .map_err(|e| DlnaError::Discovery(e.to_string()))?;

    Ok(DlnaRenderer::new(render))
}