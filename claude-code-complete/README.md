# Claude Code Complete - 深度复刻版

基于对原版 Claude Code 源代码的深入分析，完整复刻的 Rust 实现。

## 📊 架构分析

### 原版架构 (TypeScript)

```
Claude Code (TypeScript)
├── UI Layer: Ink (React for Terminal)
│   ├── 218KB print.ts - 终端渲染引擎
│   ├── companion.ts - Gravy 宠物
│   └── sidebar.tsx - 侧边栏
├── Core Layer
│   ├── sessionDiscovery.ts - 会话发现
│   ├── sessionHistory.ts - 会话历史
│   └── toolPool.ts - 工具池
├── I/O Layer
│   ├── structuredIO.ts - 结构化 I/O
│   ├── remoteIO.ts - 远程 I/O
│   └── bridgeMain.ts - Bridge 主控
└── Protocol Layer
    ├── MCP SDK - Model Context Protocol
    └── JSON-RPC - 通信协议
```

### Rust 版本架构

```
Claude Code Rust
├── ui/                     # Ratatui 界面层
│   ├── widgets/
│   │   ├── sidebar.rs     # 侧边栏 (复刻原版)
│   │   ├── chat_panel.rs  # 聊天面板
│   │   ├── companion.rs   # Gravy 宠物精灵
│   │   └── status_bar.rs  # 状态栏
│   └── theme.rs           # 原版配色方案
├── core/                   # 核心逻辑
│   ├── session_manager.rs # 会话管理
│   ├── tool_system.rs     # 工具系统
│   ├── mcp_server.rs      # MCP 服务器
│   └── permissions.rs     # 权限管理
├── events/                 # 事件系统
│   ├── event_bus.rs       # 事件总线
│   └── types.rs           # 事件类型
├── state/                  # 状态管理
│   ├── app_state.rs       # 应用状态
│   └── session.rs         # 会话状态
├── io/                     # I/O 层
│   ├── structured_io.rs   # 结构化 I/O
│   └── terminal_renderer.rs
└── utils/                  # 工具函数
    └── config.rs          # 配置管理
```

## 🎨 界面复刻

### 配色方案 (原版精确匹配)

```rust
PRIMARY:     #9370db (紫色)
ACCENT:      #ff8c42 (橙色)
BG_DARKEST:  #0d0d0f (背景)
BG_DARKER:   #121217 (侧边栏)
TEXT_PRIMARY: #ffffff (主要文字)
```

### 布局结构

```
┌────────────────────────────────────────────────────────────────┐
│  Claude Code v999.0.0-restored  [🗑️] [⚙️] [☁️] [📥]        │
├─────────────────────┬──────────────────────────────────────────┤
│                     │  Tips for getting started                │
│   🐷               │  Run /init to create a CLAUDE.md file   │
│                     ├──────────────────────────────────────────┤
│  Welcome back!     │  Recent activity                         │
│                     │  No recent activity                      │
│  Haiku 4.5         │                                          │
│  API Usage Billing │                                          │
│  ~/Development/    │                                          │
│                     │                                          │
│                     │                                          │
├─────────────────────┴──────────────────────────────────────────┤
│ ↑ Opus now defaults to 1M context · 5x more room, same pricing  │
├────────────────────────────────────────────────────────────────┤
│ ⮕ Try "how does <filepath> work?"                             │
├────────────────────────────────────────────────────────────────┤
│ ? for shortcuts  Claude Code has switched from npm to native    │
└────────────────────────────────────────────────────────────────┘
```

## 🚀 使用方法

### 编译

```bash
cd claude-code-complete
cargo build --release
```

### 运行

```bash
# 交互模式
./target/release/claude-code

# 查看版本
./target/release/claude-code --version

# 详细日志
./target/release/claude-code --verbose
```

### 快捷键

| 键 | 功能 |
|----|------|
| `i` | 进入输入模式 |
| `Enter` | 发送消息 |
| `Esc` | 退出输入模式 |
| `h` | 显示帮助 |
| `s` | 打开设置 |
| `q` | 退出 |
| `↑/↓` | 滚动消息 |

## 📦 功能特性

### ✅ 已实现

- [x] 完整的 Ratatui 终端 UI
- [x] 原版配色方案
- [x] Gravy 宠物精灵动画
- [x] 侧边栏导航
- [x] 聊天面板 (Markdown 渲染)
- [x] 状态栏显示
- [x] 事件驱动架构
- [x] 会话管理
- [x] 工具系统框架
- [x] MCP 服务器框架
- [x] 权限管理

### 🔨 待完善

- [ ] API 集成
- [ ] 工具实现 (Bash, ReadFile, etc.)
- [ ] MCP 完整协议
- [ ] Git 集成
- [ ] 文件浏览器
- [ ] Bridge 模式

## 📚 技术栈

| 组件 | 技术 |
|------|------|
| 终端 UI | Ratatui + Crossterm |
| 异步运行时 | Tokio |
| 事件系统 | Tokio channels + broadcast |
| 状态管理 | Dashmap + RwLock |
| 序列化 | Serde + Serde_json |
| 日志 | Tracing |

## 🔗 原版参考

- 原版 Claude Code: `../claude-code-rev-main/`
- 原版使用: TypeScript + Ink + React
- 本版本使用: Rust + Ratatui

## 📄 许可证

MIT License
