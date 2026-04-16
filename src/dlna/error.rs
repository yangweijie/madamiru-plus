use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DlnaError {
    Discovery(String),
    Server(String),
    Control(String),
    NoDevicesFound,
    DeviceNotFound,
    NetworkError(String),
}

impl fmt::Display for DlnaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discovery(e) => write!(f, "设备发现失败: {}", e),
            Self::Server(e) => write!(f, "媒体服务器错误: {}", e),
            Self::Control(e) => write!(f, "播放控制失败: {}", e),
            Self::NoDevicesFound => write!(f, "未发现可用的 DLNA 设备"),
            Self::DeviceNotFound => write!(f, "未找到指定的设备"),
            Self::NetworkError(e) => write!(f, "网络错误: {}", e),
        }
    }
}

impl std::error::Error for DlnaError {}
