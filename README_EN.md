# WallWarp

<div align="center">
  <img src="assets/logo.ico" alt="WallWarp Logo" width="128"/>

  A feature-rich desktop wallpaper management software

  [中文](README.md)
</div>

---

## Introduction

WallWarp is a desktop wallpaper management application built with Rust and the modern Iced GUI framework. It provides rich wallpaper management features, including local wallpaper browsing, online wallpaper search, batch downloading, and automatic rotation.

## Key Features

- 🖼️ **Local Wallpaper Management**
  - Browse wallpapers in local folders
  - Support for multiple image formats (JPG, PNG, BMP, WebP)
  - Wallpaper preview and quick setup

- 🌐 **Online Wallpaper Search**
  - Integrated Wallhaven API
  - Rich filtering options (category, purity, color, resolution, etc.)
  - High-quality wallpaper browsing and downloading
  - Batch download support

- ⚙️ **Advanced Settings**
  - Automatic wallpaper rotation
  - Custom rotation interval
  - Window size and position memory
  - Multi-monitor support

- 🌍 **Internationalization**
  - Chinese and English interface support
  - Extensible multi-language system

## Tech Stack

- **Language**: Rust 2024 Edition
- **GUI Framework**: Iced 0.14 (multi-window daemon)
- **Async Runtime**: Tokio
- **Image Processing**: Image, fast_image_resize
- **Serialization**: Serde, Serde_json
- **Internationalization**: fluent-bundle
- **Network Requests**: Reqwest (native-tls, uses the system TLS stack and certificate store)
- **System Tray**: tray-icon (Windows / macOS / Linux)

## Platform Support

| Platform | Architecture | Installer | Notes |
|----------|--------------|-----------|-------|
| Windows 10+ | x64 | NSIS installer / portable zip | Full features |
| Windows 11 | arm64 | NSIS installer / portable zip | Full features |
| macOS 10.15+ | Apple Silicon (arm64) | dmg | Full features (wallpaper fit mode decided by the system) |
| macOS 10.15+ | Intel (x64) | dmg | Full features (same as above) |
| Linux (X11) | x64 | AppImage / deb / rpm / pacman | Floating ball requires an X11 session |
| Linux (X11) | arm64 | AppImage / deb / rpm / pacman | Same as above |

> **Linux desktops**: wallpaper setting supports GNOME/KDE/XFCE/Cinnamon/MATE/LXDE/Deepin;
> the tray icon relies on StatusNotifier (libayatana-appindicator is bundled with the AppImage;
> deb/rpm/pacman installs pull it in automatically via the system package manager).
> **Wayland sessions**: the main window and tray work normally; the desktop floating ball is disabled
> because window positioning/always-on-top is restricted.
> **macOS**: the dmg is unsigned — right-click → Open on first launch.

## Installation

### Linux Package Manager Installation

Download the package for your architecture from the
[Releases](https://github.com/zsyo/wallwarp/releases) page (`x86_64` for x64,
`aarch64` for arm64) and install:

```bash
# deb (Debian / Ubuntu and derivatives)
sudo apt install ./wallwarp_1.5.0_amd64.deb

# rpm (Fedora / openSUSE / RHEL family)
sudo rpm -i wallwarp-1.5.0-1.x86_64.rpm
# or dnf / zypper
sudo dnf install ./wallwarp-1.5.0-1.x86_64.rpm

# pacman (Arch / Manjaro etc., installs directly via pacman -U)
sudo pacman -U wallwarp-1.5.0-1-x86_64.pkg.tar.zst

# AppImage (no installation — make executable and run)
chmod +x wallwarp_1.5.0_amd64.AppImage
./wallwarp_1.5.0_amd64.AppImage
```

> File names use the version from the actual Release page; pre-release versions
> (tag contains `_`, e.g. `1.5.1_beta.1`) are marked as Pre-release on the
> Releases page.

### Build from Source

Make sure you have the Rust toolchain installed (Rust 1.85 or higher).

```bash
# Clone the repository
git clone https://github.com/zsyo/wallwarp.git
cd wallwarp

# Build release version
cargo build --release

# Run
cargo run --release

# Build installer package (NSIS / dmg / AppImage, platform default)
cargo packager --release

# Specify format and target
cargo packager --release --formats dmg --target aarch64-apple-darwin
```

**Linux build dependencies**:

```bash
sudo apt install libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libx11-dev \
  libxcb1-dev libxrandr-dev libxi-dev cmake libssl-dev pkg-config libxdo-dev

# Required to run AppImages (Ubuntu 22.04+ ships without fuse2)
sudo apt install libfuse2
```

### Download Pre-built Version

Visit the [Releases](https://github.com/zsyo/wallwarp/releases) page to download the pre-built version for your system (x64 and arm64 builds are provided for every platform):

- **Windows**: `*-setup.exe` (NSIS installer) or `*-portable.zip` (portable, extract and run)
- **macOS**: `.dmg`
- **Linux**: pick AppImage / deb / rpm / pacman by distribution (see install commands above)

## Usage

### First Run

1. Launch WallWarp
2. Add wallpaper folders in the "Settings" page
3. Browse and set wallpapers
4. Search and download wallpapers in the "Online Wallpapers" page

### Online Wallpaper Search

1. Switch to the "Online Wallpapers" page
2. Set filter conditions (category, purity, color, resolution, etc.)
3. Click the "Search" button
4. Browse search results
5. Click the download button to save wallpapers

### Auto Rotation

1. Switch to the "Settings" page
2. Enable "Auto Rotation" feature
3. Set rotation interval
4. Select rotation source (local/online)

## Configuration File

WallWarp stores its data directory (`config.toml`, wallpaper library, cache, database, logs) per platform:

- **Windows**: the directory containing the exe (portable)
- **macOS**: `~/Library/Application Support/WallWarp`
- **Linux**: `~/.config/wallwarp`

The `config.toml` file saves user settings:

```toml
[global]
language = "zh-cn"  # Language setting

[window]
width = 1280
height = 800
```

## Project Structure

```
wallwarp/
├── src/
│   ├── main.rs                      # Application entry point
│   ├── lib.rs                       # Library entry, declares all modules
│   ├── i18n/                        # Internationalization support module
│   ├── platform/                    # Platform abstraction layer (Windows/macOS/Linux)
│   │   ├── mod.rs                   # Common API (window geometry/work area/menu anchor)
│   │   ├── menu.rs                  # Cross-platform tray & native menu wrapper
│   │   ├── menu_linux.rs            # Linux GTK menu runtime
│   │   ├── windows.rs               # Win32 implementation
│   │   ├── macos.rs                 # AppKit implementation
│   │   └── linux.rs                 # X11 implementation
│   ├── ui/                          # User interface modules
│   │   ├── app.rs                   # Main application logic
│   │   ├── mod.rs                   # UI module declaration
│   │   ├── types.rs                 # UI type definitions
│   │   ├── update.rs                # UI update logic
│   │   ├── view.rs                  # Interface rendering logic
│   │   ├── subscription.rs          # Subscription management
│   │   ├── auto_change/             # Auto rotation feature module
│   │   │   ├── message.rs           # Message definitions
│   │   │   ├── mod.rs               # Module declaration
│   │   │   ├── handler/             # Message handlers
│   │   │   │   ├── mod.rs           # Handler module declaration
│   │   │   │   └── ...
│   │   │   └── state/               # State management
│   │   │       ├── mod.rs           # State module declaration
│   │   │       └── ...
│   │   ├── common/                  # Common UI components
│   │   │   ├── mod.rs               # Common components module declaration
│   │   │   └── ...
│   │   ├── download/                # Download management module
│   │   │   ├── message.rs           # Message definitions
│   │   │   ├── mod.rs               # Module declaration
│   │   │   ├── view.rs              # Interface rendering
│   │   │   ├── handler/             # Message handlers
│   │   │   │   ├── mod.rs           # Handler module declaration
│   │   │   │   └── ...
│   │   │   ├── state/               # State management
│   │   │   │   ├── mod.rs           # State module declaration
│   │   │   │   └── ...
│   │   │   └── widget/              # Custom components
│   │   │       ├── mod.rs           # Component module declaration
│   │   │       └── ...
│   │   ├── local/                    # Local wallpaper module
│   │   │   ├── message.rs           # Message definitions
│   │   │   ├── mod.rs               # Module declaration
│   │   │   ├── state.rs             # State definitions
│   │   │   ├── view.rs              # Interface rendering
│   │   │   ├── handler/             # Message handlers
│   │   │   │   ├── mod.rs           # Handler module declaration
│   │   │   │   └── ...
│   │   │   └── widget/              # Custom components
│   │   │       ├── mod.rs           # Component module declaration
│   │   │       └── ...
│   │   ├── main/                     # Main interface module
│   │   │   ├── close_confirm.rs
│   │   │   ├── message.rs           # Message definitions
│   │   │   ├── mod.rs               # Module declaration
│   │   │   ├── state.rs             # State definitions
│   │   │   ├── tray.rs              # Tray icon
│   │   │   ├── view.rs              # Interface rendering
│   │   │   ├── handler/             # Message handlers
│   │   │   │   ├── mod.rs           # Handler module declaration
│   │   │   │   └── ...
│   │   │   └── widget/              # Custom components
│   │   │       ├── mod.rs           # Component module declaration
│   │   │       └── ...
│   │   ├── online/                   # Online wallpaper module
│   │   │   ├── message.rs           # Message definitions
│   │   │   ├── mod.rs               # Module declaration
│   │   │   ├── types.rs             # Type definitions
│   │   │   ├── view.rs              # Interface rendering
│   │   │   ├── handler/             # Message handlers
│   │   │   ├── state/               # State management
│   │   │   └── widget/              # Custom components
│   │   ├── settings/                # Settings page module
│   │   │   ├── message.rs           # Message definitions
│   │   │   ├── mod.rs               # Module declaration
│   │   │   ├── types.rs             # Type definitions
│   │   │   ├── view.rs              # Interface rendering
│   │   │   ├── handler/             # Message handlers
│   │   │   ├── state/               # State management
│   │   │   └── widget/              # Custom components
│   │   └── style/                   # Style definition module
│   │       ├── colors.rs            # Color definitions
│   │       ├── dimensions.rs        # Dimension definitions
│   │       ├── mod.rs               # Style module declaration
│   │       ├── shadows.rs           # Shadow definitions
│   │       └── theme.rs             # Theme definitions
│   ├── services/                    # Business logic services
│   │   ├── mod.rs                   # Service module declaration
│   │   ├── local.rs                 # Local wallpaper service
│   │   ├── download.rs              # Download service
│   │   ├── request_context.rs       # Request context
│   │   ├── async_task/              # Async task module
│   │   │   ├── mod.rs               # Async task module declaration
│   │   │   └── ...
│   │   └── wallhaven/               # Wallhaven API integration
│   │       ├── mod.rs               # Wallhaven module declaration
│   │       ├── client.rs            # API client
│   │       ├── helper.rs            # Helper functions
│   │       ├── service.rs           # Service implementation
│   │       ├── types.rs             # Type definitions
│   │       └── model/               # Data models
│   │           ├── mod.rs           # Model module declaration
│   │           └── ...
│   └── utils/                        # Utility functions
│       ├── mod.rs                   # Utility module declaration
│       ├── assets.rs                # Asset management
│       ├── config.rs                # Configuration management
│       ├── helpers.rs               # Helper functions
│       ├── logger.rs                # Logging system
│       ├── single_instance.rs       # Single instance control
│       └── startup/                 # Auto-start (registry/plist/desktop per platform)
├── locales/                         # Language files
│   ├── zh-cn.ftl                    # Chinese translation
│   └── en.ftl                       # English translation
├── assets/                          # Resource files
│   ├── icons.ttf                    # Icon font
│   └── logo.ico                     # Application icon
├── .github/                         # GitHub configuration
│   └── workflows/
│       └── release.yml              # Release workflow
├── Cargo.toml                       # Project dependency configuration
├── build.rs                         # Build script
├── README.md                        # Project documentation (Chinese)
├── README_EN.md                     # Project documentation (English)
└── LICENSE                          # License
```

## Development

### Build Requirements

- Rust 1.85 or higher (Edition 2024)
- Windows 10+ / macOS 10.15+ / Linux (GTK3 development packages, see above)
- **Minimum CPU Requirement (x64)**: Processors supporting x86-64-v3 instruction set (Intel/AMD CPUs from around 2013 and later; no such requirement on arm64)

### Compilation Optimization

The CI uses the `x86-64-v3` target CPU for x64 builds to achieve better performance:

```bash
# Set compilation optimization flags
RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release
```

**Notes**:
- The `x86-64-v3` target enables modern instruction sets such as AVX2, BMI1/2, FMA, etc.
- Support for legacy CPUs (processors without AVX2 support) is dropped
- If you need to run on older CPUs, please compile without the `RUSTFLAGS` environment variable

### Development Commands

```bash
# Build
cargo build

# Run
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Contributing

Contributions are welcome! Please follow these steps:

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the [GNU Affero General Public License v3.0](LICENSE).

## Author

zsyo <zephyr@aico.top>

## Acknowledgments

- [Iced](https://github.com/iced-rs/iced) - Cross-platform GUI framework
- [Wallhaven](https://wallhaven.cc/) - High-quality wallpaper source
- [Tokio](https://tokio.rs/) - Async runtime

## Related Links

- [GitHub Repository](https://github.com/zsyo/wallwarp)
- [Issue Tracker](https://github.com/zsyo/wallwarp/issues)
