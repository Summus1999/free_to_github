# Free to GitHub

一个基于本地的 GitHub 访问加速工具,无需第三方服务器

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org)

## ✨ 特性

- 🚀 **一键加速** - 图形界面,简单易用
- 🔒 **完全本地** - 修改 hosts 文件,无需代理或 VPN
- 🎯 **精准高效** - 支持 GitHub 全站及相关服务
- 🖥️ **跨平台** - Windows / Linux / macOS
- 🛡️ **安全可靠** - 开源透明,无隐私风险
- ⚡ **体积小巧** - Release 编译后仅 3MB

## 🚀 快速开始

### 方式一: 直接使用(推荐)

1. 从 [Releases](../../releases) 下载最新版本的 `free_to_github_gui.exe`
2. 双击运行(会自动请求管理员权限)
3. 点击「启用加速」按钮
4. 点击「刷新 DNS」按钮(建议)
5. 开始流畅访问 GitHub!

### 方式二: 从源码编译

**前置要求**: [Rust 1.91+](https://www.rust-lang.org/tools/install)

```bash
# 克隆项目
git clone https://github.com/your-username/free_to_github.git
cd free_to_github

# 编译 GUI 版本
cargo build --release --bin free_to_github_gui

# 运行(需要管理员权限)
.\target\release\free_to_github_gui.exe
```

## 📖 使用说明

### GUI 版本

![GUI界面](https://via.placeholder.com/500x600/1a1a1a/00dc78?text=GUI+Screenshot)

**界面功能**:

- 🟢 **启用加速** - 一键开启 GitHub 访问优化
- 🔴 **禁用加速** - 恢复原始 hosts 配置
- 🔄 **刷新 DNS** - 清除系统 DNS 缓存
- 📂 **打开 Hosts** - 快速访问 hosts 文件目录

**注意事项**:

- ✅ 程序会自动请求管理员权限
- ✅ 首次启用后建议刷新 DNS 缓存
- ✅ 支持实时状态显示

### CLI 版本

```bash
# 查看帮助
free_to_github_cli help

# 启用加速
free_to_github_cli enable

# 禁用加速
free_to_github_cli disable

# 查看状态
free_to_github_cli status
```

## ⚙️ 工作原理

本工具通过修改系统 hosts 文件,将 GitHub 相关域名解析到可访问的 IP 地址:

```text
140.82.113.4    github.com
185.199.108.153 assets-cdn.github.com
185.199.108.133 raw.githubusercontent.com
# ... 更多域名
```

**Hosts 文件位置**:

- Windows: `C:\Windows\System32\drivers\etc\hosts`
- Linux/macOS: `/etc/hosts`

**优势**:

- ✅ 无需安装额外软件或配置代理
- ✅ 不经过第三方服务器,保护隐私
- ✅ 对系统其他网络访问无影响
- ✅ 可随时启用/禁用,完全可控

## 🛠️ 技术栈

- **Rust** - 系统级编程语言,安全高效
- **egui + eframe** - 纯 Rust 的即时模式 GUI 框架
- **Rust 标准库** - 核心功能零外部依赖

## 🔧 开发

### 国内镜像加速

配置 Cargo 使用国内镜像(推荐):

```bash
# 创建配置文件
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << EOF
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
EOF
```

### 编译优化

项目已配置 Release 编译优化:

- LTO (Link Time Optimization)
- 代码精简 (strip)
- 最小体积优化 (opt-level = "z")

### 项目结构

```text
free_to_github/
├── src/
│   ├── main.rs          # CLI 入口
│   ├── main_gui.rs      # GUI 入口
│   └── hosts.rs         # Hosts 文件操作核心逻辑
├── build.rs             # 构建脚本(嵌入管理员权限清单)
├── Cargo.toml           # 项目配置
└── *.bat                # Windows 便捷脚本
```

## 📝 许可证

Apache License 2.0 - 详见 [LICENSE](LICENSE) 文件

## 🤝 贡献

欢迎提交 Issue 和 Pull Request!

## ⚠️ 免责声明

本工具仅供学习交流使用,请遵守当地法律法规。
