// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Linux 开机自启动：XDG autostart 桌面项
//!
//! 写入 `~/.config/autostart/wallwarp.desktop`，GNOME/KDE/XFCE/Cinnamon/
//! MATE 等主流桌面在登录时执行；关闭即删除该文件。
//! AppImage 运行时通过 `APPIMAGE` 环境变量取真实文件路径（见 mod.rs）。

use super::{executable_path, warn_if_volatile};
use std::path::PathBuf;

pub(super) fn is_enabled() -> Result<bool, Box<dyn std::error::Error>> {
    let Some(path) = desktop_file_path() else {
        return Ok(false);
    };
    if !path.exists() {
        return Ok(false);
    }
    // 校验注册的仍是当前可执行文件（应用移动后视为未开启）
    let content = std::fs::read_to_string(&path)?;
    let exe = executable_path()?.to_string_lossy().to_string();
    Ok(content.contains(&exe))
}

pub(super) fn set(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = desktop_file_path() else {
        return Err("无法定位 XDG 配置目录".into());
    };

    if !enabled {
        if path.exists() {
            std::fs::remove_file(&path)?;
            tracing::info!("[自启动] [Linux] 已移除 autostart 项: {}", path.display());
        }
        return Ok(());
    }

    let exe = executable_path()?;
    warn_if_volatile(&exe);
    let exe = exe.to_string_lossy().to_string();

    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=WallWarp\n\
         Exec=\"{exe}\" --hidden\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n"
    );

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    tracing::info!("[自启动] [Linux] 已写入 autostart 项: {}", path.display());
    Ok(())
}

fn desktop_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("autostart").join("wallwarp.desktop"))
}
