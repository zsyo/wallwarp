# WallWarp

<div align="center">
  <img src="assets/logo.ico" alt="WallWarp Logo" width="128"/>

  一个功能齐全的桌面壁纸管理软件

  [English](README_EN.md)
</div>

---

## 简介

WallWarp 是一款用 Rust 开发的跨平台桌面壁纸管理应用程序，采用现代化的 Iced GUI 框架构建。它提供了丰富的壁纸管理功能，包括本地壁纸浏览、在线壁纸搜索、批量下载、自动轮换等。

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
- **GUI 框架**: Iced 0.14
- **异步运行时**: Tokio
- **图像处理**: Image、fast_image_resize
- **序列化**: Serde、Serde_json
- **国际化**: fluent-bundle
- **网络请求**: Reqwest
- **系统托盘**: tray-icon (Windows)

## 安装

### 从源码编译

确保你的系统已安装 Rust 工具链（Rust 1.70 或更高版本）。

```bash
# 克隆仓库
git clone https://github.com/zsyo/wallwarp.git
cd wallwarp

# 编译发布版本
cargo build --release

# 运行
cargo run --release

# 构建安装包
cargo packager --release
```

### 下载预编译版本

访问 [Releases](https://github.com/zsyo/wallwarp/releases) 页面下载适合你系统的预编译版本。

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

WallWarp 会在程序同级目录创建 `config.toml` 配置文件，用于保存用户设置：

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
│   ├── main.rs           # 应用入口
│   ├── lib.rs            # 库入口
│   ├── ui/               # 用户界面模块
│   │   ├── app.rs        # 主应用逻辑
│   │   ├── common.rs     # 公共UI组件
│   │   ├── local.rs      # 本地壁纸页面
│   │   ├── online.rs     # 在线壁纸页面
│   │   ├── settings.rs   # 设置页面
│   │   └── ...
│   ├── services/         # 业务逻辑服务
│   │   ├── local.rs      # 本地壁纸服务
│   │   ├── download.rs   # 下载服务
│   │   └── wallhaven/    # Wallhaven API 集成
│   ├── utils/            # 工具函数
│   │   ├── config.rs     # 配置管理
│   │   ├── logger.rs     # 日志系统
│   │   └── ...
│   └── i18n.rs           # 国际化
├── locales/              # 语言文件
│   ├── zh-cn.ftl
│   └── en.ftl
├── assets/               # 资源文件
│   ├── icons.ttf
│   └── logo.ico
└── Cargo.toml
```

## 开发

### 构建要求

- Rust 1.70 或更高版本
- Windows 10 或更高版本（当前主要支持 Windows）
- **最低 CPU 要求**: 支持 x86-64-v3 指令集的处理器（约 2013 年及以后的 Intel/AMD CPU）

### 编译优化

本项目使用 `x86-64-v3` 目标 CPU 进行编译优化，以获得更好的性能：

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
