use std::time::Duration;
use url::Url;

use super::device::DlnaDevice;
use super::DlnaError;

#[derive(Debug, Clone)]
pub enum DlnaState {
    Idle,
    Scanning,
    DevicesReady(Vec<DlnaDevice>),
    Connecting(DlnaDevice),
    Playing {
        device: DlnaDevice,
        media_url: Url,
        position: Duration,
        is_paused: bool,
    },
    Error(DlnaError),
}

impl Default for DlnaState {
    fn default() -> Self {
        Self::Idle
    }
}

impl PartialEq for DlnaState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Idle, Self::Idle) => true,
            (Self::Scanning, Self::Scanning) => true,
            (Self::DevicesReady(a), Self::DevicesReady(b)) => a == b,
            (Self::Connecting(a), Self::Connecting(b)) => a == b,
            (
                Self::Playing {
                    device: ad,
                    media_url: au,
                    position: ap,
                    is_paused: ai,
                },
                Self::Playing {
                    device: bd,
                    media_url: bu,
                    position: bp,
                    is_paused: bi,
                },
            ) => ad == bd && au == bu && ap == bp && ai == bi,
            (Self::Error(a), Self::Error(b)) => a.to_string() == b.to_string(),
            _ => false,
        }
    }
}
