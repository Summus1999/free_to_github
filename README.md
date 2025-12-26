# Free to GitHub

基于本地 hosts 修改的 GitHub 访问加速工具，无需代理或 VPN。

## 快速使用

1. 下载 `FreeToGitHub.exe`
2. 双击运行（自动请求管理员权限）
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
# 1. 进入项目目录
cd tauri-ui

# 2. 安装依赖
npm install

# 3. 执行编译
npm run tauri build

# 4. 复制产物并重命名
copy src-tauri\target\release\tauri-ui.exe ..\FreeToGitHub.exe
```

**编译产物**:

| 文件      | 路径                                                 | 大小   |
|-----------|------------------------------------------------------|--------|
| EXE       | `tauri-ui/src-tauri/target/release/tauri-ui.exe`     | ~9 MB  |
| MSI安装包 | `tauri-ui/src-tauri/target/release/bundle/msi/*.msi` | ~9 MB  |

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

版本2.0

## 许可证

Apache License 2.0
