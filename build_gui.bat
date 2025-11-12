@echo off
chcp 65001 >nul
echo ================================
echo 编译 GitHub 加速工具 GUI 版本
echo ================================
echo.

echo 正在编译 Release 版本...
cargo build --release --bin free_to_github_gui

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✓ 编译成功!
    echo.
    echo 生成的文件: target\release\free_to_github_gui.exe
    echo.
    echo 请右键该文件 -^> "以管理员身份运行"
    echo.
) else (
    echo.
    echo ✗ 编译失败，请检查错误信息
    echo.
)

pause
