use super::params::EnhanceParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub id: &'static str,
    pub name: &'static str,
    pub params: EnhanceParams,
}

/// 所有预设的 ID 列表
pub const PRESET_IDS: &[&str] = &["anime", "movie", "sdhd", "vivid", "retro", "off"];

pub const PRESETS: &[Preset] = &[
    Preset {
        id: "anime",
        name: "动漫优化",
        params: EnhanceParams {
            brightness: 0.0,
            contrast: 10.0,
            saturation: 15.0,
            hue: 0.0,
            sharpen: 30.0,
            denoise: 5.0,
        },
    },
    Preset {
        id: "movie",
        name: "电影增强",
        params: EnhanceParams {
            brightness: 5.0,
            contrast: 15.0,
            saturation: 10.0,
            hue: 0.0,
            sharpen: 20.0,
            denoise: 10.0,
        },
    },
    Preset {
        id: "sdhd",
        name: "标清转高清",
        params: EnhanceParams {
            brightness: 0.0,
            contrast: 20.0,
            saturation: 5.0,
            hue: 0.0,
            sharpen: 40.0,
            denoise: 15.0,
        },
    },
    Preset {
        id: "vivid",
        name: "鲜艳模式",
        params: EnhanceParams {
            brightness: 10.0,
            contrast: 20.0,
            saturation: 30.0,
            hue: 0.0,
            sharpen: 15.0,
            denoise: 0.0,
        },
    },
    Preset {
        id: "retro",
        name: "复古风格",
        params: EnhanceParams {
            brightness: -10.0,
            contrast: 15.0,
            saturation: -20.0,
            hue: -15.0,
            sharpen: 10.0,
            denoise: 20.0,
        },
    },
    Preset {
        id: "off",
        name: "关闭",
        params: EnhanceParams::DEFAULT,
    },
];
