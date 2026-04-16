use crate::dlna::error::DlnaError;

#[allow(dead_code)]
pub struct DlnaRenderer {
    render: crab_dlna::Render,
    location: String,
}

#[allow(dead_code)]
impl DlnaRenderer {
    pub fn new(render: crab_dlna::Render, location: String) -> Self {
        Self { render, location }
    }

    /// Play media on the DLNA device
    /// Note: crab-dlna 0.2 does not provide pause/stop/volume functions
    pub async fn play(&self, streaming_server: crab_dlna::MediaStreamingServer) -> Result<(), DlnaError> {
        log::warn!("DLNA: Starting playback on device: {}", self.location);
        let result = crab_dlna::play(self.render.clone(), streaming_server).await;
        match &result {
            Ok(_) => log::warn!("DLNA: Playback started successfully"),
            Err(e) => log::error!("DLNA: Playback failed: {}", e),
        }
        result.map_err(|e| DlnaError::Control(e.to_string()))
    }
}

/// Create a new renderer from a device location
pub async fn create_renderer(
    location: &str,
    _timeout_secs: u64,
) -> Result<DlnaRenderer, DlnaError> {
    log::warn!("DLNA: Creating renderer for device: {}", location);
    let render = crab_dlna::Render::new(crab_dlna::RenderSpec::Location(
        location.to_string(),
    ))
    .await
    .map_err(|e| DlnaError::Discovery(e.to_string()))?;

    log::warn!("DLNA: Renderer created successfully");
    Ok(DlnaRenderer::new(render, location.to_string()))
}