use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, JsonSchema)]
pub struct EnhanceParams {
    /// 亮度调整: -100 到 100
    #[serde(rename = "brightness")]
    pub brightness: f32,
    /// 对比度调整: -100 到 100
    #[serde(rename = "contrast")]
    pub contrast: f32,
    /// 饱和度调整: -100 到 100
    #[serde(rename = "saturation")]
    pub saturation: f32,
    /// 色相调整: -180 到 180
    #[serde(rename = "hue")]
    pub hue: f32,
    /// 锐化强度: 0 到 100
    #[serde(rename = "sharpen")]
    pub sharpen: f32,
    /// 降噪强度: 0 到 100
    #[serde(rename = "denoise")]
    pub denoise: f32,
}

impl EnhanceParams {
    /// 默认参数（全部为0）
    pub const DEFAULT: Self = Self {
        brightness: 0.0,
        contrast: 0.0,
        saturation: 0.0,
        hue: 0.0,
        sharpen: 0.0,
        denoise: 0.0,
    };

    /// 将参数应用到 RGB 颜色值
    pub fn apply(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        // 转换为浮点数并进行归一化
        let mut r = r as f32 / 255.0;
        let mut g = g as f32 / 255.0;
        let mut b = b as f32 / 255.0;

        // 1. 应用亮度 (-100 ~ 100 -> -1.0 ~ 1.0)
        let brightness = self.brightness / 100.0;
        r += brightness;
        g += brightness;
        b += brightness;

        // 2. 应用对比度 (-100 ~ 100 -> -1.0 ~ 1.0)
        let contrast = (self.contrast / 100.0) + 1.0;
        r = (r - 0.5) * contrast + 0.5;
        g = (g - 0.5) * contrast + 0.5;
        b = (b - 0.5) * contrast + 0.5;

        // 3. 应用饱和度 (-100 ~ 100 -> -1.0 ~ 1.0)
        let saturation = (self.saturation / 100.0) + 1.0;
        let gray = 0.2989 * r + 0.5870 * g + 0.1140 * b;
        r = gray + saturation * (r - gray);
        g = gray + saturation * (g - gray);
        b = gray + saturation * (b - gray);

        // 4. 应用色相旋转 (-180 ~ 180 -> -PI ~ PI)
        if self.hue != 0.0 {
            let hue_rad = self.hue.to_radians();
            let cos_h = hue_rad.cos();
            let sin_h = hue_rad.sin();

            let new_r = r * (0.213 + cos_h * 0.787 - sin_h * 0.213)
                + g * (0.715 - cos_h * 0.715 - sin_h * 0.715)
                + b * (0.072 - cos_h * 0.072 + sin_h * 0.928);
            let new_g = r * (0.213 - cos_h * 0.213 + sin_h * 0.787)
                + g * (0.715 + cos_h * 0.285 + sin_h * 0.140)
                + b * (0.072 - cos_h * 0.072 - sin_h * 0.283);
            let new_b = r * (0.213 - cos_h * 0.213 - sin_h * 0.786)
                + g * (0.715 - cos_h * 0.715 + sin_h * 0.283)
                + b * (0.072 + cos_h * 0.928 + sin_h * 0.072);

            r = new_r;
            g = new_g;
            b = new_b;
        }

        // 5. 简化锐化 (使用简单的边缘增强)
        // 注意：完整的锐化需要卷积核，这里仅做简单模拟
        let sharpen_factor = self.sharpen / 100.0 * 0.3;
        if self.sharpen > 0.0 {
            // 简单的对比度增强模拟锐化效果
            let avg = (r + g + b) / 3.0;
            r = r + (r - avg) * sharpen_factor;
            g = g + (g - avg) * sharpen_factor;
            b = b + (b - avg) * sharpen_factor;
        }

        // 6. 简化降噪
        // 注意：完整的降噪需要帧间处理，这里仅做简单模拟
        let denoise_factor = self.denoise / 100.0 * 0.2;
        if self.denoise > 0.0 {
            // 简单的平滑处理
            let avg = (r + g + b) / 3.0;
            r = r * (1.0 - denoise_factor) + avg * denoise_factor;
            g = g * (1.0 - denoise_factor) + avg * denoise_factor;
            b = b * (1.0 - denoise_factor) + avg * denoise_factor;
        }

        // 裁剪到有效范围 [0, 1]
        r = r.clamp(0.0, 1.0);
        g = g.clamp(0.0, 1.0);
        b = b.clamp(0.0, 1.0);

        // 转换回 u8
        (
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let params = EnhanceParams::default();
        assert_eq!(params.brightness, 0.0);
        assert_eq!(params.contrast, 0.0);
        assert_eq!(params.saturation, 0.0);
        assert_eq!(params.hue, 0.0);
        assert_eq!(params.sharpen, 0.0);
        assert_eq!(params.denoise, 0.0);
    }

    #[test]
    fn test_apply_default() {
        let params = EnhanceParams::default();
        let (r, g, b) = params.apply(128, 128, 128);
        assert_eq!((r, g, b), (128, 128, 128));
    }

    #[test]
    fn test_apply_brightness() {
        let params = EnhanceParams {
            brightness: 50.0,
            ..Default::default()
        };
        let (r, g, b) = params.apply(128, 128, 128);
        // 128/255 + 0.5 = 0.502 + 0.5 = 1.0 -> 255
        assert_eq!((r, g, b), (255, 255, 255));
    }

    #[test]
    fn test_apply_clipping() {
        let params = EnhanceParams {
            brightness: 100.0,
            ..Default::default()
        };
        let (r, g, b) = params.apply(250, 250, 250);
        // 超过1.0的应该被裁剪
        assert_eq!((r, g, b), (255, 255, 255));
    }
}
