# Madamiru 项目开发指南

## 1. 项目概述

Madamiru 是一个跨平台多媒体播放器，使用 Rust 编写，支持在网格布局中自动随机播放多个视频、图片和音频文件。

### 核心技术栈

| 类别 | 技术 |
|------|------|
| 编程语言 | Rust (Edition 2021) |
| GUI 框架 | Iced (v0.14.0) |
| 视频解码 | GStreamer (1.22.12+) |
| 音频解码 | Rodio + Symphonia |
| 配置格式 | YAML / JSON |
| 国际化 | Fluent (Mozilla) |
| 构建工具 | Cargo |

### 支持的媒体格式

- **视频**: AVI, M4V, MKV, MOV, MP4, WebM 及 GStreamer 支持的其他格式
- **图片**: BMP, GIF, ICO, JPEG, PNG/APNG, TIFF, SVG, WebP
- **音频**: FLAC, M4A, MP3, WAV
- **字幕**: MKV 内嵌字幕（不支持独立文件）

---

## 2. 项目结构

```
src/
├── main.rs              # 入口点，日志初始化、命令行/GUI 分流
├── cli.rs               # CLI 命令处理模块
├── gui.rs               # GUI 入口
├── media.rs             # 媒体源、媒体项、分组管理
├── metadata.rs          # 元数据提取
├── path.rs              # 路径处理 (StrictPath 类型)
├── prelude.rs           # 全局常量和错误类型
├── lang.rs              # 国际化系统
├── resource.rs          # 资源文件 trait 定义
├── testing.rs           # 测试工具
├── cli/
│   └── parse.rs         # CLI 参数解析 (clap)
├── gui/
│   ├── app.rs           # 主应用状态管理
│   ├── player.rs        # 播放器组件
│   ├── grid.rs         # 网格布局
│   ├── style.rs        # 样式主题
│   ├── shortcuts.rs    # 键盘快捷键
│   ├── modal.rs        # 模态对话框
│   ├── button.rs       # 按钮组件
│   ├── dropdown.rs     # 下拉选择
│   ├── widget.rs       # 通用组件
│   ├── common.rs       # 通用工具
│   ├── font.rs         # 字体处理
│   ├── icon.rs         # 图标处理
│   └── undoable.rs     # 撤销/重做功能
└── resource/
    ├── config.rs       # 配置管理 (config.yaml)
    ├── playlist.rs     # 播放列表管理 (playlist.yaml)
    └── cache.rs        # 缓存管理
```

---

## 3. 构建与运行

### 开发环境准备

1. **安装 Rust**: 使用 [rustup](https://rustup.rs/) 安装最新稳定版
2. **安装 GStreamer**:
   - **Linux**: `sudo apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-good`
   - **macOS**: `brew install gstreamer`
   - **Windows**: 安装 gstreamer runtime
3. **系统依赖** (Linux):
   ```bash
   sudo apt-get install gcc libxcb-composite0-dev libgtk-3-dev libasound2-dev
   ```

### 常用命令

| 命令 | 说明 |
|------|------|
| `cargo run` | 运行程序（开发模式） |
| `cargo test` | 运行测试 |
| `cargo build --release` | 发布构建 |
| `cargo fmt` | 代码格式化 |
| `cargo clippy` | 代码检查 |

### 预提交钩子

```bash
pip install --user pre-commit
pre-commit install
```

### 环境变量

- `MADAMIRU_VERSION`: 覆盖版本号显示（用于 CI）
- `MADAMIRU_DEBUG`: 启用调试模式（Windows 保持控制台）

---

## 4. 开发规范

### 代码风格

- 遵循 `rustfmt.toml` 配置
- 使用 Clippy 进行 lint 检查（`--deny warnings`）
- 启用所有 Clippy lint

### 测试

- 单元测试使用 `#[test]` 和 `test-case` crate
- 集成测试位于 `src/testing.rs`
- 测试断言使用 `pretty_assertions`

### 错误处理

- 使用 `prelude::Error` 枚举定义应用级错误
- 避免使用裸 `except:`
- 配置文件错误会存档到 `config.invalid.yaml`

### 国际化

- 翻译文件位于 `lang/*.ftl`
- 使用 Fluent 消息格式
- 支持语言: en-US, fr-FR, de-DE, pl-PL, pt-BR

---

## 5. 关键模块说明

### StrictPath (`src/path.rs`)

项目自定义的路径类型，提供：
- 跨平台路径处理
- 相对/绝对路径转换
- URI 支持
- 占位符替换 (`<playlist>`)

### 配置系统 (`src/resource/config.rs`)

```yaml
# config.yaml 结构
release:
  check: true          # 检查新版本
view:
  language: en-US
  theme: dark          # light/dark
  confirm_discard_playlist: true
playback:
  muted: false
  volume: 1.0
  image_duration: 10   # 图片显示秒数
  pause_on_unfocus: false
  synchronized: false  # 同步播放
```

### 媒体分组 (`src/media.rs`)

- `Source`: 媒体来源（路径或 glob 模式）
- `Group`: 媒体分组，包含多个 Source
- `RefreshContext`: 刷新上下文（Launch/Edit/Playlist/Automatic/Manual）

---

## 6. GUI 架构 (Iced)

### 消息循环

```
User Input → Message → update() → Command → View
```

### 状态管理

- `Flags`: 启动参数（媒体源）
- `app.rs`: 主应用状态（配置、分组、播放列表）
- `player.rs`: 播放器状态（播放/暂停/进度）

### 样式系统

- `style.rs`: 定义主题（Light/Dark）
- 使用 `iced::theme` 基础上自定义

---

## 7. 发布流程

使用 `invoke` 工具（参考 CONTRIBUTING.md）:

```bash
invoke prerelease    # 预发布检查
invoke release       # 创建发布
invoke release-flatpak  # Flatpak 发布
invoke release-winget   # WinGet 发布
```

---

## 8. 注意事项

1. **Windows 控制台**: 程序会自动分离控制台（双击运行时），使用 `MADAMIRU_DEBUG` 保持控制台
2. **GStreamer 版本**: 必须 1.14+，推荐 1.22.12
3. **平台特性**:
   - Windows: 使用 Windows API 处理控制台分离
   - macOS: 使用 system-deps 处理系统依赖
   - Linux: 检测 Steam Deck 设备
4. **图片格式**: SVG 需要 `rsvg` 依赖
