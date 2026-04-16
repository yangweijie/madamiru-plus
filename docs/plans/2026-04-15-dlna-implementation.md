# DLNA 投屏功能实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 实现 DLNA 投屏功能，允许用户将当前播放的视频/音频投屏到电视或其他 DLNA 设备

**Architecture:** 使用 crab-dlna 库实现设备发现和媒体流传输，通过内置 HTTP 服务器提供媒体流，UI 使用 Iced 模态框选择设备和显示控制栏

**Tech Stack:** Rust, crab-dlna 0.2, tokio, Iced

---

## 阶段 1: 基础架构搭建

### Task 1: 添加依赖

**Files:**
- Modify: `Cargo.toml`

**Step 1: 添加 crab-dlna 和 tokio 依赖**

在 `[dependencies]` 段添加:
```toml
[dependencies]
crab-dlna = "0.2"
tokio = { version = "1", features = ["full"] }
```

**Step 2: 验证依赖可用**

Run: `cargo check`
Expected: 无错误（可能需要运行 `cargo update`）

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add crab-dlna and tokio dependencies"
```

---

### Task 2: 创建 DLNA 模块结构

**Files:**
- Create: `src/dlna/mod.rs`
- Create: `src/dlna/error.rs`
- Create: `src/dlna/device.rs`
- Create: `src/dlna/server.rs`
- Create: `src/dlna/control.rs`
- Create: `src/dlna/state.rs`

**Step 1: 创建模块入口**

```rust
// src/dlna/mod.rs
pub mod control;
pub mod device;
pub mod error;
pub mod server;
pub mod state;

pub use error::DlnaError;
pub use device::DlnaDevice;
pub use state::DlnaState;
```

**Step 2: 创建错误类型**

```rust
// src/dlna/error.rs
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DlnaError {
    #[error("设备发现失败: {0}")]
    Discovery(String),
    #[error("媒体服务器错误: {0}")]
    Server(String),
    #[error("播放控制失败: {0}")]
    Control(String),
    #[error("未发现可用的 DLNA 设备")]
    NoDevicesFound,
    #[error("未找到指定的设备")]
    DeviceNotFound,
    #[error("网络错误: {0}")]
    NetworkError(String),
}
```

**Step 3: 创建设备结构**

```rust
// src/dlna/device.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlnaDevice {
    pub name: String,
    pub location: String,
    pub udn: String,
}

impl std::fmt::Display for DlnaDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
```

**Step 4: 创建状态定义**

```rust
// src/dlna/state.rs
use std::time::Duration;
use url::Url;

use super::device::DlnaDevice;

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
    Error(super::DlnaError),
}

impl Default for DlnaState {
    fn default() -> Self {
        Self::Idle
    }
}
```

**Step 5: 创建控制模块占位**

```rust
// src/dlna/control.rs
// 占位文件，后续实现
```

**Step 6: 创建服务器模块占位**

```rust
// src/dlna/server.rs
// 占位文件，后续实现
```

**Step 7: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 8: Commit**

```bash
git add src/dlna/
git commit -m "feat(dlna): create dlna module structure"
```

---

## 阶段 2: 设备发现实现

### Task 3: 实现设备发现功能

**Files:**
- Modify: `src/dlna/device.rs`

**Step 1: 实现设备发现函数**

```rust
// src/dlna/device.rs
use crab_dlna::Render;

use super::error::DlnaError;

pub async fn discover_devices(timeout_secs: u64) -> Result<Vec<DlnaDevice>, DlnaError> {
    let renders = Render::discover(timeout_secs)
        .await
        .map_err(|e| DlnaError::Discovery(e.to_string()))?;
    
    Ok(renders
        .into_iter()
        .map(|r| DlnaDevice {
            name: r.friendly_name().to_string(),
            location: r.description_url().to_string(),
            udn: r.udn().to_string(),
        })
        .collect())
}
```

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 3: Commit**

```bash
git add src/dlna/device.rs
git commit -m "feat(dlna): implement device discovery"
```

---

## 阶段 3: 媒体服务器实现

### Task 4: 实现媒体服务器

**Files:**
- Modify: `src/dlna/server.rs`

**Step 1: 实现媒体服务器**

```rust
// src/dlna/server.rs
use std::path::Path;
use crab_dlna::{MediaStreamingServer, STREAMING_PORT_DEFAULT, get_local_ip, infer_subtitle_from_video};

use super::error::DlnaError;

pub struct MediaServer {
    _server: MediaStreamingServer,
    url: String,
}

impl MediaServer {
    pub async fn new(
        media_path: &Path,
        subtitle_path: Option<&Path>,
    ) -> Result<Self, DlnaError> {
        let host_ip = get_local_ip()
            .await
            .map_err(|e| DlnaError::Server(e.to_string()))?;
        
        let subtitle = subtitle_path.map(|p| infer_subtitle_from_video(&p.into()));
        
        let server = MediaStreamingServer::new(
            media_path,
            subtitle.as_deref(),
            &host_ip,
            &STREAMING_PORT_DEFAULT,
        )
        .map_err(|e| DlnaError::Server(e.to_string()))?;
        
        let url = format!("http://{}:{}", host_ip, STREAMING_PORT_DEFAULT);
        
        Ok(Self { _server: server, url })
    }
    
    pub fn url(&self) -> &str {
        &self.url
    }
}
```

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 3: Commit**

```bash
git add src/dlna/server.rs
git commit -m "feat(dlna): implement media server"
```

---

## 阶段 4: 播放控制实现

### Task 5: 实现播放控制

**Files:**
- Modify: `src/dlna/control.rs`

**Step 1: 实现播放控制函数**

```rust
// src/dlna/control.rs
use crab_dlna::{Render, play, pause, stop as dlna_stop, set_volume};

use super::error::DlnaError;

pub async fn play_media(render: &Render, url: &str) -> Result<(), DlnaError> {
    play(render, url)
        .await
        .map_err(|e| DlnaError::Control(e.to_string()))
}

pub async fn pause_playback(render: &Render) -> Result<(), DlnaError> {
    pause(render)
        .await
        .map_err(|e| DlnaError::Control(e.to_string()))
}

pub async fn stop_playback(render: &Render) -> Result<(), DlnaError> {
    dlna_stop(render)
        .await
        .map_err(|e| DlnaError::Control(e.to_string()))
}

pub async fn set_volume_level(render: &Render, volume: u8) -> Result<(), DlnaError> {
    set_volume(render, volume)
        .await
        .map_err(|e| DlnaError::Control(e.to_string()))
}
```

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 3: Commit**

```bash
git add src/dlna/control.rs
git commit -m "feat(dlna): implement playback control"
```

---

## 阶段 5: GUI 集成 - 消息和状态

### Task 6: 添加 DLNA 消息类型

**Files:**
- Modify: `src/gui/common.rs`

**Step 1: 添加 DLNA 消息枚举**

在 `Message` 枚举中添加:
```rust
#[derive(Debug, Clone)]
pub enum DlnaMessage {
    ScanDevices,
    DevicesFound(Vec<crate::dlna::DlnaDevice>),
    ScanError(String),
    SelectDevice(crate::dlna::DlnaDevice),
    CastMedia {
        path: StrictPath,
        device: crate::dlna::DlnaDevice,
    },
    Play,
    Pause,
    Stop,
    Seek(u64),
    SetVolume(u8),
    StopCast,
}
```

在 `Message` 变体中添加:
```rust
Dlna(DlnaMessage),
```

**Step 2: 更新 Message 的克隆实现**

确保 `DlnaMessage` 和 `Message::Dlna` 实现了 `Clone`

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 4: Commit**

```bash
git add src/gui/common.rs
git commit -m "feat(dlna): add dlna message types"
```

---

### Task 7: 在 App 中添加 DLNA 状态

**Files:**
- Modify: `src/gui/app.rs`

**Step 1: 添加 DLNA 状态字段**

在 `App` 结构体中添加:
```rust
#[derive(Default)]
pub struct App {
    // ... 现有字段 ...
    
    dlna_state: crate::dlna::DlnaState,
}
```

**Step 2: 在 main.rs 或 lib.rs 中添加 dlna 模块引用**

确保 `dlna` 模块被正确引入

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 4: Commit**

```bash
git add src/gui/app.rs
git commit -m "feat(dlna): add dlna state to app"
```

---

## 阶段 6: GUI 组件 - 设备选择

### Task 8: 添加设备选择模态框

**Files:**
- Modify: `src/gui/modal.rs`

**Step 1: 添加 DLNA 相关 Modal 变体**

```rust
#[derive(Debug, Clone)]
pub enum Modal {
    // ... 现有变体 ...
    
    DlnaDeviceSelect {
        devices: Vec<crate::dlna::DlnaDevice>,
        current_media: Option<StrictPath>,
    },
    DlnaControl {
        device: crate::dlna::DlnaDevice,
        media: StrictPath,
        position: u64,
        is_paused: bool,
        volume: u8,
    },
}
```

**Step 2: 添加 Modal Event**

```rust
#[derive(Debug, Clone)]
pub enum Event {
    // ... 现有 ...
    
    DlnaDeviceSelected(crate::dlna::DlnaDevice),
    DlnaPlay,
    DlnaPause,
    DlnaStop,
    DlnaSeek(u64),
    DlnaSetVolume(u8),
}
```

**Step 3: 实现 Modal view 方法**

为新模态框添加 view 实现

**Step 4: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 5: Commit**

```bash
git add src/gui/modal.rs
git commit -f feat(dlna): add device selection modal"
```

---

## 阶段 7: GUI 组件 - 投屏按钮

### Task 9: 添加投屏按钮到工具栏

**Files:**
- Modify: `src/gui/player.rs`
- Modify: `src/gui/button.rs`

**Step 1: 在 Icon 枚举中添加投屏图标**

```rust
// src/gui/icon.rs
#[derive(Clone, Copy, Debug)]
pub enum Icon {
    // ... 现有 ...
    Cast,           // 投屏
    CastConnected,  // 已投屏
}
```

**Step 2: 实现 Icon 的 as_char 方法**

```rust
impl Icon {
    pub fn as_char(self) -> char {
        match self {
            // ... 现有 ...
            Self::Cast => '\u{e1fa}',        // 需要查找合适的 Unicode 字符
            Self::CastConnected => '\u{e1fb}',
        }
    }
}
```

**Step 3: 在工具栏添加投屏按钮**

在 player.rs 的底部工具栏视图中添加投屏按钮

**Step 4: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 5: Commit**

```bash
git add src/gui/icon.rs src/gui/player.rs
git commit -m "feat(dlna): add cast button to toolbar"
```

---

## 阶段 8: 实现消息处理逻辑

### Task 10: 实现 DLNA 消息处理

**Files:**
- Modify: `src/gui/app.rs`

**Step 1: 添加 update 方法处理 DLNA 消息**

```rust
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        // ... 现有 ...
        Message::Dlna(msg) => self.update_dlna(msg),
    }
}

fn update_dlna(&mut self, msg: DlnaMessage) -> Task<Message> {
    match msg {
        DlnaMessage::ScanDevices => {
            self.dlna_state = crate::dlna::DlnaState::Scanning;
            // 启动异步扫描任务
            Task::none()
        }
        DlnaMessage::DevicesFound(devices) => {
            if devices.is_empty() {
                self.dlna_state = crate::dlna::DlnaState::Error(
                    crate::dlna::DlnaError::NoDevicesFound
                );
            } else {
                self.dlna_state = crate::dlna::DlnaState::DevicesReady(devices);
            }
            Task::none()
        }
        DlnaMessage::SelectDevice(device) => {
            // 显示设备选择模态框
            self.modals.push(Modal::DlnaDeviceSelect {
                devices: vec![device],
                current_media: None,
            });
            Task::none()
        }
        DlnaMessage::CastMedia { path, device } => {
            // 1. 暂停本地播放
            // 2. 启动媒体服务器
            // 3. 发送播放命令
            Task::none()
        }
        // ... 其他消息处理
    }
}
```

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 3: Commit**

```bash
git add src/gui/app.rs
git commit -m "feat(dlna): implement dlna message handling"
```

---

## 阶段 9: 异步任务集成

### Task 11: 实现异步设备扫描任务

**Files:**
- Modify: `src/gui/app.rs`

**Step 1: 实现设备扫描任务**

在 update 方法中添加:
```rust
DnaMessage::ScanDevices => {
    let timeout = 5;
    self.dlna_state = crate::dlna::DlnaState::Scanning;
    
    // 启动异步扫描任务
    return Task::perform(
        async move {
            crate::dlna::device::discover_devices(timeout).await
        },
        |result| match result {
            Ok(devices) => Message::Dlna(DlnaMessage::DevicesFound(devices)),
            Err(e) => Message::Dlna(DlnaMessage::ScanError(e.to_string())),
        },
    );
}
```

**Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 3: Commit**

```bash
git add src/gui/app.rs
git commit -m "feat(dlna): implement async device scanning"
```

---

## 阶段 10: 完整投屏流程

### Task 12: 实现完整投屏逻辑

**Files:**
- Modify: `src/gui/app.rs`

**Step 1: 实现 CastMedia 处理**

```rust
DlnaMessage::CastMedia { path, device } => {
    // 1. 暂停本地播放
    // 2. 查找字幕文件
    let subtitle = infer_subtitle_from_video(&path);
    
    // 3. 启动媒体服务器
    // 4. 发送到设备播放
    // 5. 更新状态
    
    return Task::perform(
        async move {
            // 实现投屏逻辑
        },
        |result| Message::Dlna(/* 处理结果 */),
    );
}
```

**Step 2: 实现播放控制**

添加 Play, Pause, Stop 消息处理

**Step 3: 验证编译**

Run: `cargo check`
Expected: 无错误

**Step 4: Commit**

```bash
git add src/gui/app.rs
git commit -m "feat(dlna): implement complete casting flow"
```

---

## 阶段 11: 测试与完善

### Task 13: 功能测试

**Step 1: 运行 cargo clippy**

Run: `cargo clippy -- -D warnings`
Expected: 无警告

**Step 2: 运行 cargo test**

Run: `cargo test`
Expected: 所有测试通过

**Step 3: 构建调试版本**

Run: `cargo build`
Expected: 构建成功

**Step 4: Commit**

```bash
git add .
git commit -m "feat(dlna): complete dlna casting feature"
```

---

## 执行方式

**计划完成，保存到 `docs/plans/2026-04-15-dlna-implementation.md`**

两种执行方式：

1. **Subagent-Driven (本会话)** - 我为每个任务调度新的子代理，任务间进行代码审查，快速迭代

2. **Parallel Session (单独会话)** - 在新会话中使用 executing-plans，分批执行并设置检查点

请问您选择哪种方式？
