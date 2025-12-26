# Free to GitHub

基于本地 hosts 修改的 GitHub 访问加速工具，无需代理或 VPN。

## 快速使用

1. 下载并安装 `Free to GitHub_2.0.0_x64-setup.exe`
2. 启动应用（自动请求管理员权限）
3. 点击「测速」→「启用加速」→「刷新DNS」

## 技术栈

| 组件 | 技术                       |
|------|----------------------------|
| 前端 | Vue 3 + TypeScript + Vite  |
| 后端 | Rust + Tauri 2.0           |
| 构建 | npm + cargo                |

## 编译

**环境要求**: Node.js 18+, Rust 1.70+

**编译步骤**:

```bash
# 1. 安装依赖（首次编译）
npm install

# 2. 一键式编译（生成 NSIS 安装包）
cd tauri-ui && npm run tauri build
```

**编译产物**:

| 文件         | 路径                                                                | 大小   |
|--------------|---------------------------------------------------------------------|--------|
| EXE          | `tauri-ui/src-tauri/target/release/tauri-ui.exe`                    | ~9 MB  |
| NSIS安装包   | `tauri-ui/src-tauri/target/release/bundle/nsis/*-setup.exe`         | ~9 MB  |

## 项目结构

```text
free_to_github/
├── tauri-ui/                # Tauri + Vue 项目
│   ├── src/                 # Vue 前端源码
│   │   └── App.vue          # 主界面
│   └── src-tauri/           # Rust 后端
│       ├── src/
│       │   ├── main.rs      # 入口
│       │   ├── hosts.rs     # hosts 文件操作
│       │   └── network.rs   # IP 测速
│       └── Cargo.toml
└── FreeToGitHub.exe         # 发布版本
```

## 版本

2.0.0

## 许可证

Apache License 2.0
