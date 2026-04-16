use std::path::Path;

use crate::dlna::error::DlnaError;

#[allow(dead_code)]
pub struct MediaServer {
    _server: crab_dlna::MediaStreamingServer,
    url: String,
}

#[allow(dead_code)]
impl MediaServer {
    pub async fn new(
        media_path: &Path,
        subtitle_path: Option<&Path>,
    ) -> Result<Self, DlnaError> {
        let host_ip = crab_dlna::get_local_ip()
            .await
            .map_err(|e| DlnaError::Server(e.to_string()))?;

        // infer_subtitle_from_video returns Option<Option<PathBuf>>
        // We need to extract the inner Option<PathBuf> if exists
        #[allow(clippy::needless_match, clippy::manual_map)]
        let subtitle: Option<std::path::PathBuf> = subtitle_path.and_then(|p| {
            let inferred = crab_dlna::infer_subtitle_from_video(p);
            // Option<Option<PathBuf>> -> Option<PathBuf>
            match inferred {
                Some(inner) => Some(inner),
                None => None,
            }
        });

        let server = crab_dlna::MediaStreamingServer::new(
            media_path,
            &subtitle,
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
