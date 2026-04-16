use std::fmt;

#[derive(Debug)]
pub enum EnhanceError {
    UnsupportedFormat,
    ProcessingFailed(String),
    InvalidParameter(String),
}

impl fmt::Display for EnhanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat => write!(f, "不支持的格式"),
            Self::ProcessingFailed(e) => write!(f, "处理失败: {}", e),
            Self::InvalidParameter(e) => write!(f, "无效参数: {}", e),
        }
    }
}

impl std::error::Error for EnhanceError {}
