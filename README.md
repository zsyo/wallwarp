# WallWarp

<div align="center">
  <img src="assets/logo.ico" alt="WallWarp Logo" width="128"/>

  一个功能齐全的桌面壁纸管理软件

  [English](README_EN.md)
</div>

---

## 简介

WallWarp 是一款用 Rust 开发的桌面壁纸管理应用程序，采用现代化的 Iced GUI 框架构建。它提供了丰富的壁纸管理功能，包括本地壁纸浏览、在线壁纸搜索、批量下载、自动轮换等。

## 主要功能

- 🖼️ **本地壁纸管理**
  - 浏览本地文件夹中的壁纸
  - 支持多种图像格式（JPG、PNG、BMP、WebP）
  - 壁纸预览和快速设置

- 🌐 **在线壁纸搜索**
  - 集成 Wallhaven API
  - 丰富的筛选选项（分类、纯度、颜色、分辨率等）
  - 高质量壁纸浏览和下载
  - 批量下载支持

- ⚙️ **高级设置**
  - 自动壁纸轮换
  - 自定义轮换间隔
  - 窗口大小和位置记忆
  - 多显示器支持

- 🌍 **国际化支持**
  - 支持中文和英文界面
  - 可扩展的多语言系统

## 技术栈

- **语言**: Rust 2024 Edition
- **GUI 框架**: Iced 0.14（多窗口 daemon）
- **异步运行时**: Tokio
- **图像处理**: Image、fast_image_resize
- **序列化**: Serde、Serde_json
- **国际化**: fluent-bundle
- **网络请求**: Reqwest（native-tls，走系统 TLS 栈与证书库）
- **系统托盘**: tray-icon（Windows / macOS / Linux）

## 平台支持

| 平台 | 架构 | 安装包 | 说明 |
|------|------|--------|------|
| Windows 10+ | x64 | NSIS 安装器 / 便携 zip | 完整功能 |
| Windows 11 | arm64 | NSIS 安装器 / 便携 zip | 完整功能 |
| macOS 10.15+ | Apple Silicon (arm64) | dmg | 完整功能（壁纸铺满方式由系统决定） |
| macOS 10.15+ | Intel (x64) | dmg | 完整功能（同上） |
| Linux (X11) | x64 | AppImage / deb / rpm / pacman | 悬浮球/贴边需 X11 会话 |
| Linux (X11) | arm64 | AppImage / deb / rpm / pacman | 同上 |

> **Linux 桌面环境**：壁纸设置支持 GNOME/KDE/XFCE/Cinnamon/MATE/LXDE/Deepin；
> 托盘图标依赖 StatusNotifier（libayatana-appindicator 已随 AppImage 打包，
> deb/rpm/pacman 安装方式经系统包管理器自动安装该依赖）。
> **Wayland 会话**：主窗口与托盘正常，桌面悬浮球因窗口定位/置顶受限而禁用。
> **macOS**：安装包未签名，首次打开需右键 → 打开。

## 安装

### Linux 包管理器安装

从 [Releases](https://github.com/zsyo/wallwarp/releases) 下载对应架构（x64 为
`x86_64`，arm64 为 `aarch64`）的包后安装：

```bash
# deb（Debian / Ubuntu 及衍生版）
sudo apt install ./wallwarp_1.5.0_amd64.deb

# rpm（Fedora / openSUSE / RHEL 系）
sudo rpm -i wallwarp-1.5.0-1.x86_64.rpm
# 或 dnf / zypper
sudo dnf install ./wallwarp-1.5.0-1.x86_64.rpm

# pacman（Arch / Manjaro 等，直接 pacman -U 安装）
sudo pacman -U wallwarp-1.5.0-1-x86_64.pkg.tar.zst

# AppImage（免安装，下载后添加执行权限直接运行）
chmod +x wallwarp_1.5.0_amd64.AppImage
./wallwarp_1.5.0_amd64.AppImage
```

> 文件名中的版本号以实际 Release 页面为准；预发布版本（tag 含 `_`，如
> `1.5.1_beta.1`）在 Release 页面会标记为 Pre-release。

### 从源码编译

确保你的系统已安装 Rust 工具链（Rust 1.85 或更高版本）。

```bash
# 克隆仓库
git clone https://github.com/zsyo/wallwarp.git
cd wallwarp

# 编译发布版本
cargo build --release

# 运行
cargo run --release

# 构建安装包（NSIS / dmg / AppImage，按当前平台默认）
cargo packager --release

# 指定打包格式与目标
cargo packager --release --formats dmg --target aarch64-apple-darwin
```

**Linux 构建依赖**：

```bash
sudo apt install libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libx11-dev \
  libxcb1-dev libxrandr-dev libxi-dev cmake libssl-dev pkg-config libxdo-dev

# 运行 AppImage 所需（Ubuntu 22.04+ 默认不含 fuse2）
sudo apt install libfuse2
```

### 下载预编译版本

访问 [Releases](https://github.com/zsyo/wallwarp/releases) 页面下载适合你系统的预编译版本（每个平台均提供 x64 与 arm64 架构）：

- **Windows**：`*-setup.exe`（NSIS 安装器）或 `*-portable.zip`（便携版，解压即用）
- **macOS**：`.dmg`
- **Linux**：按发行版选择 AppImage / deb / rpm / pacman 包（见上文安装命令）

## 使用说明

### 首次运行

1. 启动 WallWarp
2. 在"设置"页面添加壁纸文件夹
3. 浏览和设置壁纸
4. 在"在线壁纸"页面搜索并下载壁纸

### 在线壁纸搜索

1. 切换到"在线壁纸"页面
2. 设置筛选条件（分类、纯度、颜色、分辨率等）
3. 点击"搜索"按钮
4. 浏览搜索结果
5. 点击下载按钮保存壁纸

### 自动轮换

1. 切换到"设置"页面
2. 启用"自动轮换"功能
3. 设置轮换间隔时间
4. 选择轮换来源（本地/在线）

## 配置文件

WallWarp 的数据目录（`config.toml`、壁纸库、缓存、数据库、日志）按平台存放：

- **Windows**：exe 同级目录（便携式，绿色软件）
- **macOS**：`~/Library/Application Support/WallWarp`
- **Linux**：`~/.config/wallwarp`

`config.toml` 用于保存用户设置：

```toml
[global]
language = "zh-cn"  # 语言设置

[window]
width = 1280
height = 800
```

## 项目结构

```
wallwarp/
├── src/
│   ├── main.rs                      # 应用入口点
│   ├── lib.rs                       # 库入口，声明所有模块
│   ├── i18n/                        # 国际化支持模块
│   ├── platform/                    # 平台抽象层（Windows/macOS/Linux 实现）
│   │   ├── mod.rs                   # 公共接口（窗口几何/工作区/菜单锚点）
│   │   ├── menu.rs                  # 托盘与原生菜单跨平台封装
│   │   ├── menu_linux.rs            # Linux GTK 菜单运行时
│   │   ├── windows.rs               # Win32 实现
│   │   ├── macos.rs                 # AppKit 实现
│   │   └── linux.rs                 # X11 实现
│   ├── ui/                          # 用户界面模块
│   │   ├── app.rs                   # 主应用逻辑
│   │   ├── mod.rs                   # UI模块声明
│   │   ├── types.rs                 # UI类型定义
│   │   ├── update.rs                # UI更新逻辑
│   │   ├── view.rs                  # 界面渲染逻辑
│   │   ├── subscription.rs          # 订阅管理
│   │   ├── auto_change/             # 自动轮换功能模块
│   │   │   ├── message.rs           # 消息定义
│   │   │   ├── mod.rs               # 模块声明
│   │   │   ├── handler/             # 消息处理器
│   │   │   │   ├── mod.rs           # 处理器模块声明
│   │   │   │   └── ...
│   │   │   └── state/               # 状态管理
│   │   │       ├── mod.rs           # 状态模块声明
│   │   │       └── ...
│   │   ├── common/                  # 公共UI组件
│   │   │   ├── mod.rs               # 公共组件模块声明
│   │   │   └── ...
│   │   ├── download/                # 下载管理模块
│   │   │   ├── message.rs           # 消息定义
│   │   │   ├── mod.rs               # 模块声明
│   │   │   ├── view.rs              # 界面渲染
│   │   │   ├── handler/             # 消息处理器
│   │   │   │   ├── mod.rs           # 处理器模块声明
│   │   │   │   └── ...
│   │   │   ├── state/               # 状态管理
│   │   │   │   ├── mod.rs           # 状态模块声明
│   │   │   │   └── ...
│   │   │   └── widget/              # 自定义组件
│   │   │       ├── mod.rs           # 组件模块声明
│   │   │       └── ...
│   │   ├── local/                    # 本地壁纸模块
│   │   │   ├── message.rs           # 消息定义
│   │   │   ├── mod.rs               # 模块声明
│   │   │   ├── state.rs             # 状态定义
│   │   │   ├── view.rs              # 界面渲染
│   │   │   ├── handler/             # 消息处理器
│   │   │   │   ├── mod.rs           # 处理器模块声明
│   │   │   │   └── ...
│   │   │   └── widget/              # 自定义组件
│   │   │       ├── mod.rs           # 组件模块声明
│   │   │       └── ...
│   │   ├── main/                     # 主界面模块
│   │   │   ├── close_confirm.rs
│   │   │   ├── message.rs           # 消息定义
│   │   │   ├── mod.rs               # 模块声明
│   │   │   ├── state.rs             # 状态定义
│   │   │   ├── tray.rs              # 托盘图标
│   │   │   ├── view.rs              # 界面渲染
│   │   │   ├── handler/             # 消息处理器
│   │   │   │   ├── mod.rs           # 处理器模块声明
│   │   │   │   └── ...
│   │   │   └── widget/              # 自定义组件
│   │   │       ├── mod.rs           # 组件模块声明
│   │   │       └── ...
│   │   ├── online/                   # 在线壁纸模块
│   │   │   ├── message.rs           # 消息定义
│   │   │   ├── mod.rs               # 模块声明
│   │   │   ├── types.rs             # 类型定义
│   │   │   ├── view.rs              # 界面渲染
│   │   │   ├── handler/             # 消息处理器
│   │   │   ├── state/               # 状态管理
│   │   │   └── widget/              # 自定义组件
│   │   ├── settings/                # 设置页面模块
│   │   │   ├── message.rs           # 消息定义
│   │   │   ├── mod.rs               # 模块声明
│   │   │   ├── types.rs             # 类型定义
│   │   │   ├── view.rs              # 界面渲染
│   │   │   ├── handler/             # 消息处理器
│   │   │   ├── state/               # 状态管理
│   │   │   └── widget/              # 自定义组件
│   │   └── style/                   # 样式定义模块
│   │       ├── colors.rs            # 颜色定义
│   │       ├── dimensions.rs        # 尺寸定义
│   │       ├── mod.rs               # 样式模块声明
│   │       ├── shadows.rs           # 阴影定义
│   │       └── theme.rs             # 主题定义
│   ├── services/                    # 业务逻辑服务
│   │   ├── mod.rs                   # 服务模块声明
│   │   ├── local.rs                 # 本地壁纸服务
│   │   ├── download.rs              # 下载服务
│   │   ├── request_context.rs       # 请求上下文
│   │   ├── async_task/              # 异步任务模块
│   │   │   ├── mod.rs               # 异步任务模块声明
│   │   │   └── ...
│   │   └── wallhaven/               # Wallhaven API 集成
│   │       ├── mod.rs               # Wallhaven模块声明
│   │       ├── client.rs            # API客户端
│   │       ├── helper.rs            # 辅助函数
│   │       ├── service.rs           # 服务实现
│   │       ├── types.rs             # 类型定义
│   │       └── model/               # 数据模型
│   │           ├── mod.rs           # 模型模块声明
│   │           └── ...
│   └── utils/                        # 工具函数
│       ├── mod.rs                   # 工具模块声明
│       ├── assets.rs                # 资源管理
│       ├── config.rs                # 配置管理
│       ├── helpers.rs               # 辅助函数
│       ├── logger.rs                # 日志系统
│       ├── single_instance.rs       # 单实例控制
│       └── startup/                 # 开机自启动（注册表/plist/desktop 按平台拆分）
├── locales/                         # 语言文件
│   ├── zh-cn.ftl                    # 中文翻译
│   └── en.ftl                       # 英文翻译
├── assets/                          # 资源文件
│   ├── icons.ttf                    # 图标字体
│   ├── logo.ico                     # 应用图标（Windows）
│   └── logo-*.png                   # 应用图标（macOS/Linux 打包用）
├── .github/                         # GitHub 配置
│   └── workflows/
│       └── release.yml              # 发布工作流
├── Cargo.toml                       # 项目依赖配置
├── build.rs                         # 构建脚本
├── README.md                        # 项目说明（中文）
├── README_EN.md                     # 项目说明（英文）
└── LICENSE                          # 许可证
```

## 开发

### 构建要求

- Rust 1.85 或更高版本（Edition 2024）
- Windows 10+ / macOS 10.15+ / Linux（GTK3 开发库，见上文）
- **最低 CPU 要求（x64）**: 支持 x86-64-v3 指令集的处理器（约 2013 年及以后的 Intel/AMD CPU；arm64 无此要求）

### 编译优化

本项目在 CI 中对 x64 构建使用 `x86-64-v3` 目标 CPU 进行编译优化：

```bash
# 设置编译优化标志
RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release
```

**说明**:
- `x86-64-v3` 目标启用了 AVX2、BMI1/2、FMA 等现代指令集
- 放弃了对古董级 CPU（不支持 AVX2 的处理器）的支持
- 如果需要在更老的 CPU 上运行，请移除 `RUSTFLAGS` 环境变量进行编译

### 开发命令

```bash
# 编译
cargo build

# 运行
cargo run

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

### 贡献指南

欢迎贡献代码、报告问题或提出建议！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 [GNU Affero General Public License v3.0](LICENSE) 开源协议。

## 作者

zsyo <zephyr@aico.top>

## 致谢

- [Iced](https://github.com/iced-rs/iced) - 跨平台 GUI 框架
- [Wallhaven](https://wallhaven.cc/) - 高质量壁纸源
- [Tokio](https://tokio.rs/) - 异步运行时

## 相关链接

- [GitHub 仓库](https://github.com/zsyo/wallwarp)
- [问题反馈](https://github.com/zsyo/wallwarp/issues)
