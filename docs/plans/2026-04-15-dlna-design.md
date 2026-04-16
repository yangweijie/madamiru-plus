# DLNA 投屏功能设计文档

**日期**: 2026-04-15
**功能**: DLNA 投屏到电视/设备

---

## 1. 功能需求

| 需求 | 描述 |
|------|------|
| 媒体类型 | 视频 + 音频 |
| 设备选择 | 扫描网络，显示设备列表让用户选择 |
| 本地行为 | 投屏时暂停本地播放 |
| UI 入口 | 工具栏按钮 + 右键菜单 |
| 字幕支持 | 自动检测字幕文件 |
| 远程控制 | 播放/暂停/停止 + 进度拖动 + 音量调节 |

---

## 2. 架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                        Madamiru GUI                          │
├─────────────────────────────────────────────────────────────┤
│  播放栏工具栏          │  设备选择模态框                     │
│  [▶][⏸][⏹][📺]      │  ┌─────────────────────────────┐  │
│        ↑              │  │ 🎬 设备 A (TV-Living)       │  │
│     投屏按钮           │  │ 🎬 设备 B (Kodi)           │  │
│                       │  │ 🎬 设备 C (Chromecast)      │  │
│                       │  └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     DLNA 模块 (新)                          │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │
│  │ DeviceFinder │  │MediaServer   │  │  RemoteControl  │ │
│  │  (设备发现)   │  │  (HTTP流媒体) │  │   (播放控制)     │ │
│  └──────────────┘  └──────────────┘  └──────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 数据结构

### 3.1 投屏状态

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum DlnaState {
    Idle,
    Scanning,
    DevicesReady(Vec<DlnaDevice>),
    Connecting(DlnaDevice),
    Playing {
        device: DlnaDevice,
        media_url: url::Url,
        position: Duration,
        is_paused: bool,
    },
    Error(DlnaError),
}
```

### 3.2 设备结构

```rust
#[derive(Debug, Clone)]
pub struct DlnaDevice {
    pub name: String,
    pub location: String,
    pub udn: String,
}
```

---

## 4. 消息类型

```rust
#[derive(Debug, Clone)]
pub enum DlnaMessage {
    ScanDevices,
    DevicesFound(Vec<DlnaDevice>),
    ScanError(String),
    SelectDevice(DlnaDevice),
    CastMedia { path: StrictPath, device: DlnaDevice },
    Play,
    Pause,
    Stop,
    Seek(Duration),
    SetVolume(u8),
    StopCast,
}
```

---

## 5. UI 组件

### 5.1 投屏按钮
- 工具栏图标，根据状态显示不同图标
- 点击触发设备扫描

### 5.2 设备选择模态框
- 显示扫描到的 DLNA 设备列表
- 每个设备显示名称和 IP
- 支持重新扫描

### 5.3 投屏控制栏
- 显示当前投屏设备
- 播放/暂停/停止按钮
- 进度条（可拖动）
- 音量滑块

---

## 6. 模块设计

| 文件 | 职责 |
|------|------|
| `src/dlna/mod.rs` | 模块入口，导出 |
| `src/dlna/device.rs` | 设备发现与封装 |
| `src/dlna/server.rs` | HTTP 媒体服务器 |
| `src/dlna/control.rs` | 播放控制命令 |
| `src/dlna/state.rs` | 状态机管理 |
| `src/dlna/error.rs` | 错误类型定义 |

---

## 7. 依赖

```toml
[dependencies]
crab-dlna = "0.2"
tokio = { version = "1", features = ["full"] }
```

---

## 8. 交互流程

```
用户点击投屏按钮
        │
        ▼
    显示扫描中...
        │
        ▼
   ┌────┴────┐
   │ 设备列表 │
   └────┬────┘
        │
        ▼ 选择设备
   ┌────┴────┐
   │ 开始投屏 │
   │ 暂停本地 │
   └────┬────┘
        │
        ▼ 成功
   ┌────┴────┐
   │ 显示状态 │
   │ +控制栏  │
   └─────────┘
```
