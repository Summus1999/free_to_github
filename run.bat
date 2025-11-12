@echo off
chcp 65001 >nul
echo ================================
echo Free to GitHub - 快速访问工具
echo ================================
echo.

if "%1"=="" (
    echo 请选择操作:
    echo   1. 启用 GitHub 加速
    echo   2. 禁用 GitHub 加速
    echo   3. 查看状态
    echo.
    set /p choice="请输入选项 (1-3): "
    
    if "!choice!"=="1" (
        cargo run --release -- enable
    ) else if "!choice!"=="2" (
        cargo run --release -- disable
    ) else if "!choice!"=="3" (
        cargo run --release -- status
    ) else (
        echo 无效选项!
    )
) else (
    cargo run --release -- %*
)

echo.
pause
