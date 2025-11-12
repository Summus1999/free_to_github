# free_to_github
Free and convenient tools for everyone to use GitHub

> 🚀 **快速使用**: 
> 1. 安装 Rust: https://www.rust-lang.org/tools/install
> 2. 双击 `build_gui.bat` 编译
> 3. 右键 `target\release\free_to_github_gui.exe` -> 以管理员身份运行
> 4. 点击“启用加速”按钮即可!

## 简介

一个基于本地的 GitHub 访问加速工具,无需第三方服务器,通过修改 hosts 文件实现快速访问 GitHub。

- ✅ 完全本地运行,无需服务器
- ✅ 一键启用/禁用
- ✅ 支持 Windows/Linux/macOS
- ✅ 开源免费

## 快速开始

### 1. 安装 Rust (如果还没安装)

访问 https://www.rust-lang.org/tools/install 下载安装

### 2. 编译项目

#### 编译 GUI 版本 (推荐)

双击运行 `build_gui.bat` 或在命令行执行:

```bash
cargo build --release --bin free_to_github_gui
```

编译成功后会生成: `target\release\free_to_github_gui.exe`

#### 编译 CLI 版本

```bash
cargo build --release --bin free_to_github_cli
```

### 3. 运行程序

#### GUI 版本 (图形界面)

**重要**: 必须以管理员身份运行!

- 方式1: 右键 `target\release\free_to_github_gui.exe` -> "以管理员身份运行"
- 方式2: 右键 `run_gui_admin.bat` -> "以管理员身份运行"

**界面功能**:
- 显示当前加速状态 (已启用/未启用)
- 一键启用/禁用 GitHub 加速
- 一键刷新 DNS 缓存
- 权限检查和错误提示

#### CLI 版本 (命令行)

```bash
# 启用 GitHub 加速
cargo run --release -- enable

# 禁用 GitHub 加速  
cargo run --release -- disable

# 查看当前状态
cargo run --release -- status
```

### 刷新 DNS 缓存

启用或禁用后建议刷新 DNS:

```bash
# Windows
ipconfig /flushdns

# Linux
sudo systemd-resolve --flush-caches

# macOS
sudo dscacheutil -flushcache
```

## 工作原理

本工具通过修改系统 hosts 文件,将 GitHub 相关域名映射到可访问的 IP 地址:

- Windows: `C:\Windows\System32\drivers\etc\hosts`
- Linux/macOS: `/etc/hosts`

完全基于本地,不使用任何第三方代理或服务器,安全可靠。

## 技术栈

- **Rust** - 系统编程语言,安全高效
- **egui/eframe** - 跨平台 GUI 框架
- **标准库** - 核心功能无外部依赖

## 项目特点

✅ **图形界面** - 直观易用的 GUI,一键启停  
✅ **命令行版** - 支持 CLI 操作,适合自动化  
✅ **完全本地** - 无需服务器,修改 hosts 文件  
✅ **跨平台** - 支持 Windows/Linux/macOS  
✅ **体积小** - Release 编译优化,体积极小  
✅ **权限检测** - 自动检测管理员权限

## 许可证

Apache License 2.0

## Features
我想做一个基于本地的,不使用服务器的,可以方便连接github的工具。
打开就可以让国内用户快速访问github,关闭就可以关闭这个快速访问github的功能。不使用第三方服务器。
