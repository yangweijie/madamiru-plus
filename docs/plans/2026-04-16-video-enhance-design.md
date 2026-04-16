# 视频实时增强功能设计

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为 Madamiru 播放器添加实时视频增强功能，支持超分、滤镜、着色器和对比模式

**Architecture:** 使用 GStreamer OpenGL 着色器实现视频后处理，通过 Iced UI 展示控制面板

**Tech Stack:** Rust, GStreamer (gl), OpenGL Shaders, Iced

---

## 1. 功能概述

### 核心功能

1. **实时视频增强** - GPU 加速的实时视频处理
2. **预设系统** - 6 个内置预设
3. **参数调节** - 亮度、对比度、饱和度、色调、锐化、降噪
4. **对比模式** - 滑块拖动对比原画/增强后
5. **视觉着色器** - 复古效果（CRT、VHS 等）

### 用户交互流程

```
用户点击增强按钮 → 弹出增强面板
    │
    ├─ 选择预设 → 自动应用参数
    │
    ├─ 调节滑块 → 实时预览
    │
    └─ 开启对比模式 → 拖动滑块对比
```

---

## 2. 架构设计

### 2.1 模块结构

```
src/
├── video_enhance/
│   ├── mod.rs          # 模块入口
│   ├── preset.rs       # 预设定义
│   ├── params.rs       # 参数结构
│   ├── shader.rs       # OpenGL 着色器
│   └── error.rs        # 错误类型
├── gui/
│   ├── modal.rs        # 增强面板模态框
│   └── icon.rs         # 增强图标
└── resource/
    └── config.rs       # 配置扩展
```

### 2.2 GStreamer Pipeline

```
# 原始 Pipeline
playbin uri="..." → appsink

# 增强 Pipeline
playbin uri="..." → videoconvert → glfilterbin → appsink
```

---

## 3. 预设定义

### 3.1 参数结构

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhanceParams {
    pub brightness: f32,  // -100 到 100
    pub contrast: f32,   // -100 到 100
    pub saturation: f32, // -100 到 100
    pub hue: f32,        // -180 到 180
    pub sharpen: f32,    // 0 到 100
    pub denoise: f32,   // 0 到 100
}
```

### 3.2 内置预设

| 预设 ID | 名称 | 亮度 | 对比度 | 饱和度 | 色调 | 锐化 | 降噪 |
|--------|------|------|--------|--------|------|------|------|
| anime | 动漫优化 | 0 | 10 | 15 | 0 | 30 | 5 |
| movie | 电影增强 | 5 | 15 | 10 | 0 | 20 | 10 |
| sdhd | 标清转高清 | 0 | 20 | 5 | 0 | 40 | 15 |
| vivid | 鲜艳模式 | 10 | 20 | 30 | 0 | 15 | 0 |
| retro | 复古风格 | -10 | 15 | -20 | -15 | 10 | 20 |
| off | 关闭 | 0 | 0 | 0 | 0 | 0 | 0 |

---

## 4. UI 设计

### 4.1 工具栏按钮

- 位置：播放控制栏
- 图标：✨ (sparkles)
- 状态：启用时高亮显示
- 点击：打开增强面板

### 4.2 增强面板布局

```
┌─────────────────────────────────────────┐
│  视频增强                            ✕  │
├─────────────────────────────────────────┤
│  预设: [动漫优化 ▼]                     │
├─────────────────────────────────────────┤
│  ┌─基础调整──────────────────────────┐  │
│  │ 亮度    ──●───────── +10        │  │
│  │ 对比度  ──────●─────── +20        │  │
│  │ 饱和度  ──●───────── +15        │  │
│  │ 色调    ──────────── 0          │  │
│  └────────────────────────────────┘  │
├─────────────────────────────────────────┤
│  ┌─高级──────────────────────────────┐  │
│  │ 锐化    ──●───────── +30        │  │
│  │ 降噪    ──────●─────── +10        │  │
│  └────────────────────────────────┘  │
├─────────────────────────────────────────┤
│  [✓] 启用增强                          │
│  [↔] 对比模式                         │
│         ◀───────────▶                   │
└─────────────────────────────────────────┘
```

### 4.3 对比模式

- 滑块位置 0%：显示原画
- 滑块位置 100%：显示增强后
- 中间位置：左右分割显示

---

## 5. OpenGL 着色器

### 5.1 片段着色器

```glsl
uniform float brightness;  // -1.0 到 1.0
uniform float contrast;     // 0.0 到 2.0
uniform float saturation;   // 0.0 到 2.0
uniform float hue;        // 0.0 到 2π
uniform float sharpen;    // 0.0 到 1.0
uniform float denoise;    // 0.0 到 1.0

void main() {
    vec4 color = texture2D(tex, vTexCoord);
    
    // 亮度
    color.rgb += brightness;
    
    // 对比度
    color.rgb = (color.rgb - 0.5) * contrast + 0.5;
    
    // 饱和度
    float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    color.rgb = mix(vec3(gray), color.rgb, saturation);
    
    // 色调、锐化、降噪...
}
```

### 5.2 动态参数更新

当用户调整参数时：
1. 保存新参数到 App 状态
2. 通过 GStreamer property 设置更新着色器
3. 无需重建 Pipeline

---

## 6. 兼容性处理

### 6.1 OpenGL 不可用

- 检测 OpenGL 可用性
- 不可用时显示提示
- 禁用增强按钮

### 6.2 平台特定

| 平台 | 处理 |
|------|------|
| macOS | Metal/OpenGL 兼容层 |
| Linux | gst-plugins-gl + Mesa |
| Windows | DirectX/OpenGL |

---

## 7. 配置持久化

```yaml
# config.yaml
enhance:
  enabled: false
  preset_id: "anime"
  custom_values: null
```
