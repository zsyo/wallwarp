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
  - Browse wallpapers from the wallpaper library (data directory), supporting JPG, PNG, BMP, WebP formats
  - Thumbnail grid browsing, full-size preview, one-click set as wallpaper, delete
  - Tray / hotkey "Save current wallpaper to library" for quick collection

- 🌐 **Online Wallpaper Search**
  - Integrated Wallhaven API
  - Rich filtering options (category, purity, sorting, color, ratio, resolution, time range)
  - High-quality wallpaper browsing and downloading, can be set as wallpaper directly

- 📥 **Download Tasks**
  - Batch download and queue management (pause / resume / retry / cancel)
  - Manually add download links, batch operations, status filtering
  - Download records are persisted and survive restarts

- ⭐ **Favorites & Wallpaper History**
  - Favorite wallpapers you like and re-apply them anytime
  - Wallpaper switching history is recorded automatically and can be re-applied

- 🔄 **Auto Rotation**
  - Local / online rotation sources
  - Preset and custom rotation intervals

- 🎈 **Tray & Floating Ball**
  - Minimize to tray and stay in the background; close behavior configurable (ask / minimize to tray / exit)
  - Tray menu for quick previous/next wallpaper switching and saving the current one
  - Desktop floating ball: draggable, auto edge-snapping, left/right click popup menu

- ⌨️ **Global Hotkeys**
  - Switch previous/next wallpaper, show main window, save current wallpaper (Windows / macOS / Linux X11)

- 🎨 **UI & Themes**
  - Dark / light / follow system theme
  - Multi-monitor support, window size and position memory

- 🌍 **Internationalization**
  - Chinese and English interface support
  - Extensible multi-language system (custom translation files, no recompilation needed)

- ⚙️ **Others**
  - Network proxy (auto system proxy detection, manual configuration supported)
  - Auto-start on boot, single instance
  - Runtime logging (toggleable, adjustable level)

## Tech Stack

- **Language**: Rust 2024 Edition
- **GUI Framework**: Iced 0.14 (multi-window daemon)
- **Async Runtime**: Tokio
- **Image Processing**: Image, fast_image_resize
- **Data Persistence**: SQLite (rusqlite)
- **Serialization**: Serde (serde_json / toml)
- **Internationalization**: fluent-bundle
- **Network Requests**: Reqwest (native-tls, uses the system TLS stack and certificate store)
- **System Tray**: tray-icon (Windows / macOS / Linux)
- **Global Hotkeys**: global-hotkey

## Platform Support

| Platform | Architecture | Installer | Notes |
|----------|--------------|-----------|-------|
| Windows 10+ | x64 | NSIS installer / portable zip | Full features |
| Windows 11 | arm64 | NSIS installer / portable zip | Full features |
| macOS 10.15+ | Apple Silicon (arm64) | dmg | Full features (wallpaper fit mode decided by the system) |
| macOS 10.15+ | Intel (x64) | dmg | Full features (same as above) |
| Linux (X11) | x64 / arm64 | AppImage / deb / rpm / pacman | Full features |
| Linux (Wayland) | x64 / arm64 | AppImage / deb / rpm / pacman | Floating ball and global hotkeys unavailable, other features work |

> **Linux desktops**: wallpaper setting supports GNOME/KDE/XFCE/Cinnamon/MATE/LXDE/Deepin;
> the tray icon relies on StatusNotifier (libayatana-appindicator is bundled with the AppImage;
> deb/rpm/pacman installs pull it in automatically via the system package manager).
> **Wayland sessions**: the main window and tray work normally; minimize-to-tray is also
> supported on KDE Plasma (taskbar entry is hidden automatically). The desktop floating ball
> and global hotkeys are disabled due to protocol restrictions.
> **macOS**: the dmg is unsigned — right-click → Open on first launch.

## Installation

### Linux Package Manager Installation

Download the package for your architecture from the
[Releases](https://github.com/zsyo/wallwarp/releases) page (`x64` or `arm64`) and install:

```bash
# deb (Debian / Ubuntu and derivatives)
sudo apt install ./wallwarp_1.6.0_linux_x64.deb

# rpm (Fedora / openSUSE / RHEL family)
sudo rpm -i wallwarp-1.6.0-1-linux_x64.rpm
# or dnf / zypper
sudo dnf install ./wallwarp-1.6.0-1-linux_x64.rpm

# pacman (Arch / Manjaro etc., installs directly via pacman -U)
sudo pacman -U wallwarp-1.6.0-1-linux_x64.pkg.tar.zst

# AppImage (no installation — make executable and run)
chmod +x wallwarp_1.6.0_linux_x64.AppImage
./wallwarp_1.6.0_linux_x64.AppImage
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
  libxcb1-dev libxrandr-dev libxi-dev cmake libssl-dev pkg-config

# Required to run AppImages (Ubuntu 22.04+ ships without fuse2)
sudo apt install libfuse2
```

**One-shot Linux package build with WSL + Docker** (no need to install the
dependencies above locally):

Run the following script in zsh inside WSL. The build environment is provided
by a Docker container, identical to the CI release pipeline, and produces all
four packages — AppImage / deb / rpm / pacman — in one go (x64 all four by default):

```bash
# Requires zsh inside WSL (not in Ubuntu by default: sudo apt install zsh) and a working Docker
zsh packaging/linux/build-packages.sh

# Build only the given formats (comma-separated)
zsh packaging/linux/build-packages.sh -f deb,rpm

# Build arm64 (emulated via qemu on an x86 host, full compile is slow)
zsh packaging/linux/build-packages.sh -a arm64

# Override the in-app display version (artifact file names always use the Cargo.toml version)
zsh packaging/linux/build-packages.sh -v 1.7.0_beta.1
```

Artifacts are written to `dist-linux/` at the repository root. Notes:

- The compile cache lives in `~/.cache/wallwarp-build/<arch>/` (WSL ext4, avoiding
  the slow cross-drive IO of /mnt); `cargo clean` does not remove it — delete the
  directory manually to free space
- Acceleration mirrors for Docker Hub / apt / crates.io (mainland China) are built in
  and auto-detected; when GitHub access fails, the AppImage tooling (linuxdeploy) is
  pre-downloaded on the WSL side instead
- Only a working Docker daemon is required (Docker Desktop WSL integration or native
  Docker inside WSL both work)

### Download Pre-built Version

Visit the [Releases](https://github.com/zsyo/wallwarp/releases) page to download the pre-built version for your system (x64 and arm64 builds are provided for every platform):

- **Windows**: `*-setup.exe` (NSIS installer) or `*-portable.zip` (portable, extract and run)
- **macOS**: `.dmg`
- **Linux**: pick AppImage / deb / rpm / pacman by distribution (see install commands above)

## Usage

### First Run

1. Launch WallWarp (the app also stays in the system tray)
2. Put wallpaper files into the wallpaper library directory (view or change it in "Settings → Data")
3. Browse and set wallpapers on the "Local List" page, or search and download on the "Online Wallpapers" page

### Online Wallpaper Search

1. Switch to the "Online Wallpapers" page
2. Set filter conditions (category, purity, color, resolution, etc.)
3. Click the "Search" button
4. Browse search results
5. Click the download button to save wallpapers

### Download Tasks & Favorites

- Manage the download queue on the "Download Tasks" page: pause / resume / retry / cancel,
  batch operations, manual link adding
- Click the favorite icon while browsing to add to "Favorites"; "Wallpaper History" records
  every switch automatically — both can be re-applied with one click

### Auto Rotation

1. Switch to the "Settings → Wallpaper" page
2. Enable "Auto Rotation" feature
3. Set rotation interval
4. Select rotation source (local/online)

## Configuration File

WallWarp stores its data directory (wallpaper library, database, logs) per platform:

- **Windows**: the directory containing the exe (portable)
- **macOS**: `~/Library/Application Support/WallWarp`
- **Linux**: split per XDG — data in `~/.local/share/wallwarp/`,
  cache in `~/.cache/wallwarp/`; `config.toml` lives in `~/.config/wallwarp/`

The `config.toml` file saves user settings, grouped by purpose: `[global]`
(language/theme/close behavior/proxy/hotkeys/floating ball), `[data]` (library
and cache directories), `[display]` (window size and position), `[wallhaven]`
(source filters and API key), `[wallpaper]` (fit mode and auto rotation):

```toml
[global]
language = "zh-cn"  # Language setting

[display]
width = 1280
height = 800
```

## Custom Translations

WallWarp's UI text is provided by Fluent translation files (`.ftl`). Adding a new
language requires no recompilation: copy one of the bundled language files and
translate the values on the right side of `=`.

**1. Find the language file directory**:

- **Windows**: the `locales/` directory next to the exe
- **Linux** (deb / rpm / pacman): `/usr/lib/wallwarp/locales/` (sudo required;
  AppImage is a read-only mount, customization not supported yet)
- **macOS**: `WallWarp.app/Contents/Resources/locales/` (right-click → Show Package Contents)

**2. Create a new language file**: copy any existing file and name it after the
target language code — the file name (without `.ftl`, lowercased) is the language
identifier and must be a valid language code, e.g. `ja.ftl`, `fr.ftl`, `pt-br.ftl`:

```bash
sudo cp /usr/lib/wallwarp/locales/en.ftl /usr/lib/wallwarp/locales/ja.ftl
```

**3. Translate the content**: open it with a text editor. Each line has the form
`message key = translation`; only translate the value on the right side of `=` and
keep the message keys unchanged. Placeholders like `{$name}` must also be kept
as-is — do not translate or alter them.

**4. Load**: after saving, open the language dropdown on the settings page — the new
language is picked up automatically (no restart needed); selecting it takes effect
immediately and persists. On the next launch, if the system language matches the new
file, it is selected automatically as well.

## Project Structure

```
wallwarp/
├── src/
│   ├── main.rs                      # Application entry point (iced::daemon multi-window runtime)
│   ├── lib.rs                       # Library entry, declares all modules
│   ├── i18n/                        # Internationalization support module (scan/load/translate/fallback)
│   ├── platform/                    # Platform abstraction layer (compiled per target_os)
│   │   ├── mod.rs                   # Common API (window geometry/work area/menu anchor/theme watcher)
│   │   ├── menu/                    # Cross-platform tray & native menu wrapper
│   │   │   └── menu_linux.rs        # Linux GTK menu runtime (dedicated thread + command channel)
│   │   ├── kwin_rules.rs            # KDE Wayland minimize-to-tray window rule control
│   │   ├── windows.rs               # Win32 implementation
│   │   ├── macos.rs                 # AppKit implementation
│   │   └── linux.rs                 # X11 implementation (x11rb) + KDE wallpaper via gdbus
│   ├── ui/                          # User interface modules
│   │   ├── app.rs                   # Main application logic
│   │   ├── mod.rs                   # UI module declaration
│   │   ├── types.rs                 # UI type definitions
│   │   ├── update.rs                # UI update logic
│   │   ├── view.rs                  # View dispatch (by window::Id)
│   │   ├── subscription.rs          # Subscription management
│   │   ├── common/                  # Common UI components (drop-down/dialogs/wallpaper cards etc.)
│   │   ├── main/                    # Main window module (custom title bar/tray/handlers)
│   │   │   └── floating_ball/       # Desktop floating ball
│   │   ├── online/                  # Online wallpaper module
│   │   ├── local/                   # Local wallpaper module
│   │   ├── download/                # Download management module
│   │   ├── favorites/               # Favorites module
│   │   ├── history/                 # Wallpaper history module
│   │   ├── auto_change/             # Auto rotation feature module
│   │   ├── settings/                # Settings page module
│   │   └── style/                   # Style definitions (colors/dimensions/shadows/theme colors)
│   ├── services/                    # Business logic services
│   │   ├── mod.rs                   # Service module declaration
│   │   ├── local.rs                 # Local wallpaper service (scan/thumbnails)
│   │   ├── download.rs              # Download service
│   │   ├── request_context.rs       # Request context
│   │   ├── proxy.rs / retry.rs      # Proxy and retry
│   │   ├── database/                # SQLite persistence (download tasks/favorites/history)
│   │   ├── async_task/              # Async task module
│   │   └── wallhaven/               # Wallhaven API integration
│   └── utils/                       # Utility functions
│       ├── mod.rs                   # Utility module declaration
│       ├── assets.rs                # Asset management
│       ├── config.rs                # Configuration management (config.toml)
│       ├── helpers.rs               # Helper functions
│       ├── logger.rs                # Logging system
│       ├── single_instance.rs       # Single instance control
│       ├── hotkey_manager.rs        # Global hotkey management
│       └── startup/                 # Auto-start (registry/plist/desktop per platform)
├── locales/                         # Language files (zh-cn.ftl / en.ftl, extensible)
├── assets/                          # Resource files
│   ├── icons.ttf                    # Icon font
│   ├── logo.ico                     # Application icon (Windows)
│   └── logo-*.png                   # Application icons (macOS/Linux packaging)
├── packaging/linux/                 # Local Linux packaging script (WSL + Docker)
├── .github/                         # GitHub configuration
│   └── workflows/
│       ├── build.yml                # Compile verification workflow
│       ├── package_test.yml         # Package rehearsal workflow
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
