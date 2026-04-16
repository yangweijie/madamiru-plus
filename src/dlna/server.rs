use std::path::Path;

use crate::dlna::error::DlnaError;

pub struct MediaServer {
    _server: crab_dlna::MediaStreamingServer,
    url: String,
}

impl MediaServer {
    pub async fn new(
        media_path: &Path,
        subtitle_path: Option<&Path>,
    ) -> Result<Self, DlnaError> {
        let host_ip = crab_dlna::get_local_ip()
            .await
            .map_err(|e| DlnaError::Server(e.to_string()))?;

        let subtitle = subtitle_path
            .map(|p| crab_dlna::infer_subtitle_from_video(&p.to_path_buf()));

        let server = crab_dlna::MediaStreamingServer::new(
            media_path,
            subtitle.as_deref(),
            &host_ip,
            &crab_dlna::STREAMING_PORT_DEFAULT,
        )
        .map_err(|e| DlnaError::Server(e.to_string()))?;

        let url = format!(
            "http://{}:{}",
            host_ip, crab_dlna::STREAMING_PORT_DEFAULT
        );

        Ok(Self { _server: server, url })
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}