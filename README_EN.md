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
- **GUI Framework**: Iced 0.14
- **Async Runtime**: Tokio
- **Image Processing**: Image, fast_image_resize
- **Serialization**: Serde, Serde_json
- **Internationalization**: fluent-bundle
- **Network Requests**: Reqwest
- **System Tray**: tray-icon (Windows)

## Installation

### Build from Source

Make sure you have the Rust toolchain installed (Rust 1.70 or higher).

```bash
# Clone the repository
git clone https://github.com/zsyo/wallwarp.git
cd wallwarp

# Build release version
cargo build --release

# Run
cargo run --release

# Build installer package
cargo packager --release
```

### Download Pre-built Version

Visit the [Releases](https://github.com/zsyo/wallwarp/releases) page to download the pre-built version for your system.

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

WallWarp creates a `config.toml` configuration file in the same directory as the program to save user settings:

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
│   ├── main.rs           # Application entry
│   ├── lib.rs            # Library entry
│   ├── ui/               # User interface modules
│   │   ├── app.rs        # Main application logic
│   │   ├── common.rs     # Common UI components
│   │   ├── local.rs      # Local wallpaper page
│   │   ├── online.rs     # Online wallpaper page
│   │   ├── settings.rs   # Settings page
│   │   └── ...
│   ├── services/         # Business logic services
│   │   ├── local.rs      # Local wallpaper service
│   │   ├── download.rs   # Download service
│   │   └── wallhaven/    # Wallhaven API integration
│   ├── utils/            # Utility functions
│   │   ├── config.rs     # Configuration management
│   │   ├── logger.rs     # Logging system
│   │   └── ...
│   └── i18n.rs           # Internationalization
├── locales/              # Language files
│   ├── zh-cn.ftl
│   └── en.ftl
├── assets/               # Resource files
│   ├── icons.ttf
│   └── logo.ico
└── Cargo.toml
```

## Development

### Build Requirements

- Rust 1.70 or higher
- Windows 10 or higher (currently primarily supports Windows)
- **Minimum CPU Requirement**: Processors supporting x86-64-v3 instruction set (Intel/AMD CPUs from around 2013 and later)

### Compilation Optimization

This project uses the `x86-64-v3` target CPU for compilation optimization to achieve better performance:

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
