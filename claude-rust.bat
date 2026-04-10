@echo off
REM Claude Code Rust - 快速启动脚本
REM 用法: claude-rust.bat [参数]

set "SCRIPT_DIR=%~dp0"
set "EXE_PATH=%SCRIPT_DIR%target\debug\claude-code.exe"

if exist "%EXE_PATH%" (
    "%EXE_PATH%" %*
) else (
    echo 错误: 找不到 %EXE_PATH%
    echo 请先运行: cargo build
    exit /b 1
)
