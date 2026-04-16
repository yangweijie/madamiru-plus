use crate::dlna::error::DlnaError;

#[allow(dead_code)]
pub struct DlnaRenderer {
    render: crab_dlna::Render,
}

#[allow(dead_code)]
impl DlnaRenderer {
    pub fn new(render: crab_dlna::Render) -> Self {
        Self { render }
    }

    /// Play media on the DLNA device
    /// Note: crab-dlna 0.2 does not provide pause/stop/volume functions
    pub async fn play(&self, streaming_server: crab_dlna::MediaStreamingServer) -> Result<(), DlnaError> {
        crab_dlna::play(self.render.clone(), streaming_server)
            .await
            .map_err(|e| DlnaError::Control(e.to_string()))
    }
}

/// Create a new renderer from a device location
pub async fn create_renderer(
    location: &str,
    _timeout_secs: u64,
) -> Result<DlnaRenderer, DlnaError> {
    let render = crab_dlna::Render::new(crab_dlna::RenderSpec::Location(
        location.to_string(),
    ))
    .await
    .map_err(|e| DlnaError::Discovery(e.to_string()))?;

    Ok(DlnaRenderer::new(render))
}