# Claude Code Tauri GUI

Full-featured GUI for Claude Code, built with Tauri + Rust + Vanilla JS.

## Features

- 🎨 **Beautiful Dark Theme** - matching original Claude Code aesthetic
- 💬 **Full Chat Interface** - with markdown support and syntax highlighting
- 📁 **File Explorer** - browse and manage your project files
- 📦 **Git Integration** - git status and operations
- 🔌 **MCP Servers** - Model Context Protocol support
- ⚙️ **Full Settings Panel** - API configuration, theme, and more
- ⚡ **High Performance** - built with Rust for speed

## Quick Start

```bash
cd gui-tauri

# Install dependencies
npm install

# Development mode
npm run tauri:dev

# Build production version
npm run tauri:build
```

## Directory Structure

```
gui-tauri/
├── src/
│   ├── index.html          # Main HTML
│   ├── styles.css          # Complete styling (dark theme)
│   └── app.js             # Frontend application
└── src-tauri/
    ├── src/
    │   └── main.rs        # Rust backend
    ├── Cargo.toml         # Rust dependencies
    └── tauri.conf.json    # Tauri configuration
```

## Technology Stack

- **Backend**: Rust + Tauri
- **Frontend**: Vanilla JavaScript + HTML5 + CSS3
- **Theme**: Dark theme with purple/orange accent colors
- **API**: REST + WebSocket (for streaming)

## Features Implemented

### ✅ Chat Panel
- Message history
- Markdown rendering
- Code syntax highlighting
- Typing indicator
- Quick actions
- File attachment support

### ✅ File Explorer
- Directory browsing
- File icons
- File metadata display
- Git status overlay

### ✅ Settings Panel
- API configuration
- Model selection
- Generation parameters
- Theme switching
- Hotkeys

### ✅ Git Panel
- Status display
- Commit history
- Branch management
- Staging area

### ✅ MCP Panel
- Server configuration
- Tool registration
- Resource management

## Original Claude Code Features

This GUI replicates:
- Dark theme with purple (#9370db) and orange (#ff8c42) accents
- Sidebar navigation with icons
- Clean message bubbles with avatars
- Proper spacing and typography
- Smooth animations and transitions
- Responsive layout
