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
  - 浏览壁纸库（数据目录）中的壁纸，支持 JPG、PNG、BMP、WebP 格式
  - 缩略图网格浏览、大图预览、一键设为壁纸、删除
  - 托盘 / 热键"保存当前壁纸到库"，快速归集壁纸

- 🌐 **在线壁纸搜索**
  - 集成 Wallhaven API
  - 丰富的筛选选项（分类、纯度、排序、颜色、比例、分辨率、时间范围）
  - 高质量壁纸浏览和下载，可直接设为壁纸

- 📥 **下载任务**
  - 批量下载与队列管理（暂停 / 恢复 / 重试 / 取消）
  - 手动添加下载链接、批量操作、按状态筛选
  - 下载记录持久化，重启后不丢失

- ⭐ **收藏夹与壁纸历史**
  - 收藏喜爱的壁纸，随时回顾与应用
  - 壁纸切换历史自动记录，可重新应用

- 🔄 **自动轮换**
  - 本地 / 在线两种轮换来源
  - 预设与自定义轮换间隔

- 🎈 **托盘与悬浮球**
  - 最小化到托盘常驻后台，关闭行为可配置（每次询问 / 最小化到托盘 / 直接退出）
  - 托盘菜单快速切换上/下一张、保存当前壁纸
  - 桌面悬浮球：可拖动、自动贴边、左右键弹出快捷菜单

- ⌨️ **全局热键**
  - 切换上/下一张壁纸、显示主窗口、保存当前壁纸（Windows / macOS / Linux X11）

- 🎨 **界面与主题**
  - 深色 / 浅色 / 跟随系统主题
  - 多显示器支持、窗口大小和位置记忆

- 🌍 **国际化支持**
  - 支持中文和英文界面
  - 可扩展的多语言系统（自定义翻译文件，无需重新编译）

- ⚙️ **其他**
  - 网络代理（自动读取系统代理，支持手动配置）
  - 开机自启动、单实例运行
  - 运行日志（可开关、等级可调）

## 技术栈

- **语言**: Rust 2024 Edition
- **GUI 框架**: Iced 0.14（多窗口 daemon）
- **异步运行时**: Tokio
- **图像处理**: Image、fast_image_resize
- **数据持久化**: SQLite（rusqlite）
- **序列化**: Serde（serde_json / toml）
- **国际化**: fluent-bundle
- **网络请求**: Reqwest（native-tls，走系统 TLS 栈与证书库）
- **系统托盘**: tray-icon（Windows / macOS / Linux）
- **全局热键**: global-hotkey

## 平台支持

| 平台 | 架构 | 安装包 | 说明 |
|------|------|--------|------|
| Windows 10+ | x64 | NSIS 安装器 / 便携 zip | 完整功能 |
| Windows 11 | arm64 | NSIS 安装器 / 便携 zip | 完整功能 |
| macOS 10.15+ | Apple Silicon (arm64) | dmg | 完整功能（壁纸铺满方式由系统决定） |
| macOS 10.15+ | Intel (x64) | dmg | 完整功能（同上） |
| Linux (X11) | x64 / arm64 | AppImage / deb / rpm / pacman | 完整功能 |
| Linux (Wayland) | x64 / arm64 | AppImage / deb / rpm / pacman | 悬浮球与全局热键不可用，其余功能正常 |

> **Linux 桌面环境**：壁纸设置支持 GNOME/KDE/XFCE/Cinnamon/MATE/LXDE/Deepin；
> 托盘图标依赖 StatusNotifier（libayatana-appindicator 已随 AppImage 打包，
> deb/rpm/pacman 安装方式经系统包管理器自动安装该依赖）。
> **Wayland 会话**：主窗口与托盘正常，KDE Plasma 下支持最小化到托盘（自动
> 隐藏任务栏条目）；桌面悬浮球与全局热键因协议限制禁用。
> **macOS**：安装包未签名，首次打开需右键 → 打开。

## 安装

### Linux 包管理器安装

从 [Releases](https://github.com/zsyo/wallwarp/releases) 下载对应架构（x64 为
`x64`，arm64 为 `arm64`）的包后安装：

```bash
# deb（Debian / Ubuntu 及衍生版）
sudo apt install ./wallwarp_1.6.0_linux_x64.deb

# rpm（Fedora / openSUSE / RHEL 系）
sudo rpm -i wallwarp-1.6.0-1-linux_x64.rpm
# 或 dnf / zypper
sudo dnf install ./wallwarp-1.6.0-1-linux_x64.rpm

# pacman（Arch / Manjaro 等，直接 pacman -U 安装）
sudo pacman -U wallwarp-1.6.0-1-linux_x64.pkg.tar.zst

# AppImage（免安装，下载后添加执行权限直接运行）
chmod +x wallwarp_1.6.0_linux_x64.AppImage
./wallwarp_1.6.0_linux_x64.AppImage
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
  libxcb1-dev libxrandr-dev libxi-dev cmake libssl-dev pkg-config

# 运行 AppImage 所需（Ubuntu 22.04+ 默认不含 fuse2）
sudo apt install libfuse2
```

**WSL + Docker 一键构建 Linux 安装包**（无需在本机安装上述依赖）：

在 WSL 的 zsh 中运行以下脚本，编译环境由 Docker 容器提供，与 CI 发布链路一致，
一次构建出 AppImage / deb / rpm / pacman 四种安装包（默认 x64 全部四种）：

```bash
# 需要 WSL 内有 zsh（Ubuntu 默认无：sudo apt install zsh）与可用的 Docker
zsh packaging/linux/build-packages.sh

# 只构建指定格式（逗号组合）
zsh packaging/linux/build-packages.sh -f deb,rpm

# 构建 arm64（x86 宿主经 qemu 模拟，全量编译较慢）
zsh packaging/linux/build-packages.sh -a arm64

# 覆盖应用内显示版本（产物文件名恒用 Cargo.toml 版本）
zsh packaging/linux/build-packages.sh -v 1.7.0_beta.1
```

产物输出到仓库根 `dist-linux/`。说明：

- 编译缓存位于 `~/.cache/wallwarp-build/<arch>/`（WSL ext4，避免 /mnt 跨盘 IO
  拖慢增量编译），`cargo clean` 不会清除；释放空间时手动删除该目录即可
- Docker Hub / apt / crates.io 的国内加速源已内置并自动探测切换；GitHub 访问
  异常时 AppImage 工具链（linuxdeploy）会自动改由 WSL 侧预下载
- 仅需 Docker 守护进程可用（Docker Desktop 的 WSL 集成或 WSL 内原生 Docker 均可）

### 下载预编译版本

访问 [Releases](https://github.com/zsyo/wallwarp/releases) 页面下载适合你系统的预编译版本（每个平台均提供 x64 与 arm64 架构）：

- **Windows**：`*-setup.exe`（NSIS 安装器）或 `*-portable.zip`（便携版，解压即用）
- **macOS**：`.dmg`
- **Linux**：按发行版选择 AppImage / deb / rpm / pacman 包（见上文安装命令）

## 使用说明

### 首次运行

1. 启动 WallWarp（应用同时驻留系统托盘）
2. 把壁纸文件放入壁纸库目录（可在"设置 → 数据"中查看或更改位置）
3. 在"本地列表"页浏览并设为壁纸；也可在"在线壁纸"页搜索下载

### 在线壁纸搜索

1. 切换到"在线壁纸"页面
2. 设置筛选条件（分类、纯度、颜色、分辨率等）
3. 点击"搜索"按钮
4. 浏览搜索结果
5. 点击下载按钮保存壁纸

### 下载任务与收藏

- "下载任务"页管理下载队列：暂停 / 恢复 / 重试 / 取消、批量操作、手动添加链接
- 浏览壁纸时点击收藏图标加入"收藏夹"，"壁纸历史"自动记录每次切换，均可一键重新应用

### 自动轮换

1. 切换到"设置 → 壁纸"页面
2. 启用"自动轮换"功能
3. 设置轮换间隔时间
4. 选择轮换来源（本地/在线）

## 配置文件

WallWarp 的数据目录（壁纸库、数据库、日志）按平台存放：

- **Windows**：exe 同级目录（便携式，绿色软件）
- **macOS**：`~/Library/Application Support/WallWarp`
- **Linux**：按 XDG 规范细分——数据在 `~/.local/share/wallwarp/`、
  缓存在 `~/.cache/wallwarp/`；`config.toml` 在 `~/.config/wallwarp/`

`config.toml` 用于保存用户设置，按用途分为 `[global]`（语言/主题/关闭行为/
代理/热键/悬浮球）、`[data]`（壁纸库与缓存目录）、`[display]`（窗口大小与
位置）、`[wallhaven]`（图源筛选与 API Key）、`[wallpaper]`（铺满方式与自动
轮换）几组：

```toml
[global]
language = "zh-cn"  # 语言设置

[display]
width = 1280
height = 800
```

## 自定义翻译

WallWarp 的界面文案由 Fluent 翻译文件（`.ftl`）提供，添加新语言无需重新编译：
拷贝一份自带的语言文件，把其中 `=` 右侧的值翻译成目标语言即可。

**1. 找到语言文件目录**：

- **Windows**：exe 同级的 `locales/` 目录
- **Linux**（deb / rpm / pacman）：`/usr/lib/wallwarp/locales/`（需 sudo；
  AppImage 为只读挂载，暂不支持自定义）
- **macOS**：`WallWarp.app/Contents/Resources/locales/`（右键 → 显示包内容）

**2. 创建新语言文件**：复制任一现有文件，以目标语言的代码命名——文件名
（不含 `.ftl`，自动转小写）即语言标识，需为合法的语言代码，例如
`ja.ftl`、`fr.ftl`、`pt-br.ftl`：

```bash
sudo cp /usr/lib/wallwarp/locales/en.ftl /usr/lib/wallwarp/locales/ja.ftl
```

**3. 翻译内容**：用文本编辑器打开，每行格式为 `消息键 = 译文`，只翻译
`=` 右侧的值，左侧的消息键保持原样；形如 `{$name}` 的占位符也必须原样
保留，不要翻译或删改。

**4. 加载**：保存后打开设置页的语言下拉框，新语言会被自动识别（无需重启）；
选中即生效并持久保存。下次启动时若系统语言与新文件匹配，也会自动选用。

## 项目结构

```
wallwarp/
├── src/
│   ├── main.rs                      # 应用入口点（iced::daemon 多窗口运行时）
│   ├── lib.rs                       # 库入口，声明所有模块
│   ├── i18n/                        # 国际化支持模块（目录扫描/加载/翻译/回退）
│   ├── platform/                    # 平台抽象层（三平台实现按 target_os 编译期选择）
│   │   ├── mod.rs                   # 公共接口（窗口几何/工作区/菜单锚点/系统主题监听）
│   │   ├── menu/                    # 托盘与原生菜单跨平台封装
│   │   │   └── menu_linux.rs        # Linux GTK 菜单运行时（专用线程 + 命令通道）
│   │   ├── kwin_rules.rs            # KDE Wayland 最小化到托盘的窗口规则控制
│   │   ├── windows.rs               # Win32 实现
│   │   ├── macos.rs                 # AppKit 实现
│   │   └── linux.rs                 # X11 实现（x11rb）+ KDE 壁纸 gdbus 直连
│   ├── ui/                          # 用户界面模块
│   │   ├── app.rs                   # 主应用逻辑
│   │   ├── mod.rs                   # UI模块声明
│   │   ├── types.rs                 # UI类型定义
│   │   ├── update.rs                # UI更新逻辑
│   │   ├── view.rs                  # 界面渲染分发（按 window::Id）
│   │   ├── subscription.rs          # 订阅管理
│   │   ├── common/                  # 公共UI组件（下拉框/对话框/壁纸卡片等）
│   │   ├── main/                    # 主界面模块（自绘标题栏/托盘/消息处理器）
│   │   │   └── floating_ball/       # 桌面悬浮球
│   │   ├── online/                  # 在线壁纸模块
│   │   ├── local/                   # 本地壁纸模块
│   │   ├── download/                # 下载管理模块
│   │   ├── favorites/               # 收藏夹模块
│   │   ├── history/                 # 壁纸历史模块
│   │   ├── auto_change/             # 自动轮换功能模块
│   │   ├── settings/                # 设置页面模块
│   │   └── style/                   # 样式定义模块（颜色/尺寸/阴影/主题色）
│   ├── services/                    # 业务逻辑服务
│   │   ├── mod.rs                   # 服务模块声明
│   │   ├── local.rs                 # 本地壁纸服务（扫描/缩略图）
│   │   ├── download.rs              # 下载服务
│   │   ├── request_context.rs       # 请求上下文
│   │   ├── proxy.rs / retry.rs      # 代理与重试
│   │   ├── database/                # SQLite 持久化（下载任务/收藏/壁纸历史）
│   │   ├── async_task/              # 异步任务模块
│   │   └── wallhaven/               # Wallhaven API 集成
│   └── utils/                       # 工具函数
│       ├── mod.rs                   # 工具模块声明
│       ├── assets.rs                # 资源管理
│       ├── config.rs                # 配置管理（config.toml）
│       ├── helpers.rs               # 辅助函数
│       ├── logger.rs                # 日志系统
│       ├── single_instance.rs       # 单实例控制
│       ├── hotkey_manager.rs        # 全局热键管理
│       └── startup/                 # 开机自启动（注册表/plist/desktop 按平台拆分）
├── locales/                         # 语言文件（zh-cn.ftl / en.ftl，可自行扩展）
├── assets/                          # 资源文件
│   ├── icons.ttf                    # 图标字体
│   ├── logo.ico                     # 应用图标（Windows）
│   └── logo-*.png                   # 应用图标（macOS/Linux 打包用）
├── packaging/linux/                 # Linux 本地打包脚本（WSL + Docker）
├── .github/                         # GitHub 配置
│   └── workflows/
│       ├── build.yml                # 编译验证工作流
│       ├── package_test.yml         # 打包演练工作流
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
