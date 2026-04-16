# 视频实时增强功能实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为 Madamiru 播放器添加实时视频增强功能（预设、滤镜、对比模式）

**Architecture:** 使用参数化的 CPU 滤镜实现视频后处理，通过 Iced UI 展示控制面板

**Tech Stack:** Rust, image crate (图像处理), Iced UI

---

## 阶段 1: 基础架构

### Task 1: 创建视频增强模块结构

**Files:**
- Create: `src/video_enhance/mod.rs`
- Create: `src/video_enhance/params.rs`
- Create: `src/video_enhance/preset.rs`
- Create: `src/video_enhance/error.rs`

**Step 1: 创建模块入口**

```rust
// src/video_enhance/mod.rs
pub mod error;
pub mod params;
pub mod preset;

pub use error::EnhanceError;
pub use params::EnhanceParams;
pub use preset::{Preset, PRESETS};
```

**Step 2: 创建参数结构**

```rust
// src/video_enhance/params.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct EnhanceParams {
    #[serde(rename = "brightness")]
    pub brightness: f32,  // -100 到 100
    #[serde(rename = "contrast")]
    pub contrast: f32,    // -100 到 100
    #[serde(rename = "saturation")]
    pub saturation: f32,  // -100 到 100
    #[serde(rename = "hue")]
    pub hue: f32,        // -180 到 180
    #[serde(rename = "sharpen")]
    pub sharpen: f32,     // 0 到 100
    #[serde(rename = "denoise")]
    pub denoise: f32,    // 0 到 100
}

impl EnhanceParams {
    pub fn apply(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        // 实现图像处理逻辑
    }
}
```

**Step 3: 创建预设**

```rust
// src/video_enhance/preset.rs
use super::params::EnhanceParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub params: EnhanceParams,
}

pub const PRESETS: &[Preset] = &[
    Preset { id: "anime".into(), name: "动漫优化".into(), params: EnhanceParams { brightness: 0.0, contrast: 10.0, saturation: 15.0, hue: 0.0, sharpen: 30.0, denoise: 5.0 }},
    Preset { id: "movie".into(), name: "电影增强".into(), params: EnhanceParams { brightness: 5.0, contrast: 15.0, saturation: 10.0, hue: 0.0, sharpen: 20.0, denoise: 10.0 }},
    Preset { id: "sdhd".into(), name: "标清转高清".into(), params: EnhanceParams { brightness: 0.0, contrast: 20.0, saturation: 5.0, hue: 0.0, sharpen: 40.0, denoise: 15.0 }},
    Preset { id: "vivid".into(), name: "鲜艳模式".into(), params: EnhanceParams { brightness: 10.0, contrast: 20.0, saturation: 30.0, hue: 0.0, sharpen: 15.0, denoise: 0.0 }},
    Preset { id: "retro".into(), name: "复古风格".into(), params: EnhanceParams { brightness: -10.0, contrast: 15.0, saturation: -20.0, hue: -15.0, sharpen: 10.0, denoise: 20.0 }},
    Preset { id: "off".into(), name: "关闭".into(), params: EnhanceParams::default() },
];
```

**Step 4: 创建错误类型**

```rust
// src/video_enhance/error.rs
use std::fmt;

#[derive(Debug)]
pub enum EnhanceError {
    UnsupportedFormat,
    ProcessingFailed(String),
}

impl fmt::Display for EnhanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat => write!(f, "不支持的格式"),
            Self::ProcessingFailed(e) => write!(f, "处理失败: {}", e),
        }
    }
}
```

**Step 5: 验证编译**

Run: `cargo check`
Expected: 无错误

---

### Task 2: 添加 video_enhance 模块到 main.rs

**Files:**
- Modify: `src/main.rs`

**Step 1: 添加模块声明**

```rust
// src/main.rs
mod cli;
mod dlna;
mod gui;
mod lang;
mod media;
mod metadata;
mod path;
mod prelude;
mod resource;
mod testing;
mod video_enhance;  // 添加这行
```

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

---

## 阶段 2: 配置集成

### Task 3: 扩展配置结构

**Files:**
- Modify: `src/resource/config.rs`

**Step 1: 添加增强配置**

```rust
// 在 Config 结构体中添加
pub struct Config {
    // ... 现有字段 ...
    pub enhance: EnhanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhanceConfig {
    pub enabled: bool,
    pub preset_id: String,
}

impl Default for EnhanceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            preset_id: "anime".into(),
        }
    }
}
```

**Step 2: 添加 view 字段**

在 View 结构体中添加: `pub enhance: EnhanceConfig`

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

---

## 阶段 3: GUI 集成

### Task 4: 添加增强消息类型

**Files:**
- Modify: `src/gui/common.rs`

**Step 1: 添加消息枚举**

```rust
#[derive(Debug, Clone)]
pub enum EnhanceMessage {
    ToggleEnabled,
    SelectPreset(String),
    SetBrightness(f32),
    SetContrast(f32),
    SetSaturation(f32),
    SetHue(f32),
    SetSharpen(f32),
    SetDenoise(f32),
    ToggleCompareMode,
    SetComparePosition(f32),
    Close,
}
```

**Step 2: 添加主消息包装**

在 Message 枚举中添加: `Enhance(EnhanceMessage)`

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

---

### Task 5: 添加增强图标

**Files:**
- Modify: `src/gui/icon.rs`

**Step 1: 添加图标枚举**

```rust
pub enum Icon {
    // ... 现有 ...
    Sparkles,  // 增强图标
}
```

**Step 2: 实现 as_char 方法**

```rust
impl Icon {
    pub const fn as_char(&self) -> char {
        match self {
            // ... 现有 ...
            Self::Sparkles => '\u{2728}',  // ✨
        }
    }
}
```

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

---

### Task 6: 添加增强模态框

**Files:**
- Modify: `src/gui/modal.rs`

**Step 1: 添加 Modal 变体**

```rust
pub enum Modal {
    // ... 现有 ...
    VideoEnhance {
        enabled: bool,
        preset_id: String,
        params: crate::video_enhance::EnhanceParams,
        compare_mode: bool,
        compare_position: f32,
    },
}
```

**Step 2: 实现 Modal 方法**

为新模态框实现 `title()`, `body()`, `controls()` 方法

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

---

### Task 7: 在 App 中添加增强状态

**Files:**
- Modify: `src/gui/app.rs`

**Step 1: 添加状态字段**

```rust
pub struct App {
    // ... 现有字段 ...
    enhance_enabled: bool,
    enhance_preset_id: String,
    enhance_params: crate::video_enhance::EnhanceParams,
    enhance_compare_mode: bool,
    enhance_compare_position: f32,
}
```

**Step 2: 初始化状态**

在 App::new 中初始化增强状态

**Step 3: 添加 update 方法**

处理 EnhanceMessage 消息

**Step 4: 验证编译**

Run: `cargo check`
Expected: 无错误

---

### Task 8: 添加工具栏增强按钮

**Files:**
- Modify: `src/gui/app.rs` (view 方法)

**Step 1: 在工具栏添加按钮**

在播放控制栏添加增强按钮，点击打开 VideoEnhance 模态框

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

---

## 阶段 4: 对比模式实现

### Task 9: 实现对比滑块组件

**Files:**
- Modify: `src/gui/player.rs`

**Step 1: 添加对比模式渲染**

当 enhance_compare_mode 为 true 时，在视频上方叠加对比滑块

**Step 2: 实现分割渲染**

- 左侧：原画
- 右侧：增强后
- 滑块位置动态调整

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

---

## 阶段 5: 测试与完善

### Task 10: 功能测试

**Step 1: 运行 cargo clippy**

Run: `cargo clippy -- -D warnings`
Expected: 无警告

**Step 2: 运行 cargo test**

Run: `cargo test`
Expected: 所有测试通过

**Step 3: 构建调试版本**

Run: `cargo build`
Expected: 构建成功

---

## 实现顺序

1. Task 1: 创建视频增强模块结构
2. Task 2: 添加模块到 main.rs
3. Task 3: 扩展配置结构
4. Task 4: 添加增强消息类型
5. Task 5: 添加增强图标
6. Task 6: 添加增强模态框
7. Task 7: 在 App 中添加增强状态
8. Task 8: 添加工具栏增强按钮
9. Task 9: 实现对比模式
10. Task 10: 功能测试
