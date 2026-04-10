# Claude Code GUI - 使用说明

## 📋 项目状态

已创建完整的 GUI 复刻版文件，位于 `gui-tauri/` 目录。

## 🎨 文件结构

```
gui-tauri/
├── src/
│   ├── index.html          # 主 HTML 界面
│   ├── styles.css          # 完整深色主题样式
│   └── app.js             # 前端应用逻辑
├── src-tauri/
│   ├── src/
│   │   └── main.rs        # Rust 后端
│   ├── Cargo.toml         # Rust 依赖
│   └── tauri.conf.json    # Tauri 配置
├── package.json            # npm 配置
└── README.md               # 文档
```

## 🚀 快速预览

由于 Tauri 配置比较复杂，你可以直接在浏览器中预览界面：

1. 用浏览器打开文件：
   ```
   file:///C:/Users/user/claude-code-rust/gui-tauri/src/index.html
   ```

2. 或者用本地服务器预览：
   ```bash
   cd gui-tauri/src
   python -m http.server 8080
   # 然后访问 http://localhost:8080
   ```

## 🎯 复刻的原版特性

### ✅ 界面元素

- **侧边栏导航**：Chat / Files / Git / MCP / Settings
- **深色主题**：紫色 (#9370db) + 橙色 (#ff8c42) 配色
- **聊天界面**：消息气泡、头像、打字指示器
- **欢迎界面**：快速操作按钮
- **设置面板**：API 配置、模型选择等

### ✅ 功能面板

1. **Chat** - 对话界面
   - Markdown 渲染
   - 代码高亮
   - 打字动画
   - 快速操作

2. **Files** - 文件浏览器
   - 目录树
   - 文件图标
   - 大小显示

3. **Git** - Git 面板
   - 状态显示
   - 提交历史

4. **MCP** - MCP 服务器
   - 配置管理

5. **Settings** - 设置
   - API Key
   - Base URL
   - 模型选择
   - 温度设置
   - 主题切换

## 🔧 完整 Tauri 构建

如需完整的 Tauri 应用，建议：

1. 查看 Tauri 官方文档：https://tauri.app/
2. 或使用 tauri init 命令创建标准项目：
   ```bash
   cd gui-tauri
   cargo-tauri init
   # 然后按提示操作
   ```

## 📝 注意

- 后端 Rust 代码已完整 (`src-tauri/src/main.rs`)
- 前端界面已完整 (`src/index.html` + `styles.css` + `app.js`)
- 配色严格复刻原版 Claude Code
