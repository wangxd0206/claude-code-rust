# Claude Code Rust - 深度复刻版

基于对原版 Claude Code 源代码的深入分析，创建的完整 Rust 实现。

## 📋 原版架构分析

### 核心架构

```
原版 Claude Code (TypeScript)
├── UI Layer: Ink (React for Terminal)
│   ├── 主界面布局
│   ├── Gravy 宠物精灵
│   ├── 侧边栏导航
│   └── 聊天区域
├── 业务层
│   ├── 会话管理
│   ├── 工具系统
│   ├── MCP 协议
│   └── 权限管理
├── I/O 层
│   ├── StructuredIO (结构化 I/O)
│   ├── RemoteIO (远程 I/O)
│   └── Print.ts (终端渲染)
└── 通信层
    ├── SDK 协议
    ├── JSON-RPC
    └── WebSocket
```

### 关键模块

- **structuredIO.ts**: 结构化的 stdio 处理，用于 SDK 通信
- **print.ts**: 218KB 的终端渲染引擎，处理所有 UI
- **companion.ts**: Gravy 宠物精灵系统
- **sessionRunner.ts**: 会话执行引擎
- **bridgeMain.ts**: Bridge 模式（远程控制）

## 🚀 Rust 版本实现

### 架构

```
Claude Code Rust
├── UI Layer: Ratatui (TUI Library)
│   ├── widgets/
│   │   ├── Sidebar (侧边栏)
│   │   ├── ChatPanel (聊天区)
│   │   ├── CompanionWidget (Gravy)
│   │   └── StatusBar (状态栏)
│   └── app.rs (主应用)
├── Core Layer
│   ├── session/ (会话管理)
│   ├── tools/ (工具系统)
│   ├── mcp/ (MCP 协议)
│   └── permissions/ (权限管理)
├── I/O Layer
│   ├── structured_io.rs
│   └── terminal_renderer.rs
└── Event Layer
    ├── event_bus.rs
    └── state_manager.rs
```

### 技术栈

- **UI**: Ratatui + Crossterm
- **异步**: Tokio
- **状态管理**: Dashmap (并发安全)
- **事件总线**: Tokio sync channels
- **配置**: Config + Toml
- **HTTP**: Reqwest
- **JSON**: Serde

## 📁 目录结构

```
src/
├── main.rs                 # 入口
├── app.rs                  # Ratatui 主应用
├── state/                  # 状态管理
│   ├── mod.rs
│   ├── app_state.rs
│   └── session.rs
├── ui/                     # UI 组件
│   ├── mod.rs
│   ├── widgets/
│   │   ├── sidebar.rs
│   │   ├── chat_panel.rs
│   │   ├── companion.rs
│   │   └── status_bar.rs
│   └── theme.rs
├── core/                   # 核心逻辑
│   ├── mod.rs
│   ├── session_manager.rs
│   ├── tool_system.rs
│   ├── mcp_server.rs
│   └── permissions.rs
├── io/                     # I/O 层
│   ├── mod.rs
│   ├── structured_io.rs
│   └── terminal_renderer.rs
├── events/                 # 事件系统
│   ├── mod.rs
│   ├── event_bus.rs
│   └── types.rs
└── utils/                  # 工具函数
    ├── mod.rs
    └── config.rs
```

## ✨ 功能特性

### UI 界面

- ✅ 原版深色主题
- ✅ 紫色/橙色配色方案
- ✅ Gravy 宠物精灵动画
- ✅ 侧边栏导航
- ✅ 聊天区域（Markdown 渲染）
- ✅ 代码高亮
- ✅ 打字动画效果
- ✅ 状态显示

### 核心功能

- ✅ 会话管理
- ✅ 工具系统
- ✅ MCP 服务器支持
- ✅ 权限管理
- ✅ Git 集成
- ✅ 文件操作
- ✅ 终端执行
- ✅ 记忆系统

### 通信

- ✅ Structured I/O 协议
- ✅ 事件驱动架构
- ✅ 前后端分离
- ✅ 点击反馈
- ✅ 状态同步

## 🎨 界面设计

### 配色方案

```rust
// 原版配色
const PRIMARY: Color = Color::Rgb(147, 112, 219);    // 紫色 #9370db
const ACCENT: Color = Color::Rgb(255, 140, 66);      // 橙色 #ff8c42
const BG_DARKEST: Color = Color::Rgb(13, 13, 15);
const BG_DARK: Color = Color::Rgb(18, 18, 23);
const BG_MEDIUM: Color = Color::Rgb(26, 26, 36);
const TEXT_PRIMARY: Color = Color::White;
const TEXT_SECONDARY: Color = Color::Rgb(196, 196, 212);
```

### 布局

```
┌─────────────────────────────────────────────────────────────┐
│  Claude Code v999.0.0-restored  [🗑️] [⚙️] [☁️] [📥]   │
├──────────────┬───────────────────────────────────────────────┤
│  Welcome     │  Tips for getting started                     │
│  back!       │  Run /init to create a CLAUDE.md file...    │
│              ├───────────────────────────────────────────────┤
│   🐷        │  Recent activity                              │
│              │  No recent activity                           │
│              │                                               │
│  Haiku 4.5 · │                                               │
│  API Usage    │                                               │
│  Billing      │                                               │
│  ~/Development/│                                               │
├──────────────┴───────────────────────────────────────────────┤
│ ↑ Opus now defaults to 1M context · 5x more room, same pricing │
├───────────────────────────────────────────────────────────────┤
│ ⮕ Try "how does <filepath> work?"                          │
├───────────────────────────────────────────────────────────────┤
│ ? for shortcuts  Claude Code has switched from npm to native │
│ installer. Run `claude inst...                             │
└───────────────────────────────────────────────────────────────┘
```

## 快速开始

```bash
# 编译
cargo build --release

# 运行
./target/release/claude-code-rust
```

## 开发

```bash
# 开发模式
cargo run

# 测试
cargo test

# 格式化
cargo fmt

# Clippy
cargo clippy
```
