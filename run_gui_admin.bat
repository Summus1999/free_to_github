@echo off
:: 检查管理员权限
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo 需要管理员权限！
    echo 请右键此文件 -^> "以管理员身份运行"
    pause
    exit /b 1
)

:: 启动 GUI 程序
start "" "%~dp0target\release\free_to_github_gui.exe"
