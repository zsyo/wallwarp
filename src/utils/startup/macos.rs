// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! macOS 开机自启动：LaunchAgent plist
//!
//! 将 plist 写入 `~/Library/LaunchAgents/` 后，下次登录 launchd 会自动
//! 加载（RunAtLoad = true），无需调用 launchctl；关闭即删除该文件。

use super::{executable_path, warn_if_volatile};
use std::path::PathBuf;

const LABEL: &str = "top.aico.wallwarp";

pub(super) fn is_enabled() -> Result<bool, Box<dyn std::error::Error>> {
    let Some(path) = plist_path() else {
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
    let Some(path) = plist_path() else {
        return Err("无法定位 ~/Library/LaunchAgents 目录".into());
    };

    if !enabled {
        if path.exists() {
            std::fs::remove_file(&path)?;
            tracing::info!("[自启动] [macOS] 已移除 LaunchAgent: {}", path.display());
        }
        return Ok(());
    }

    let exe = executable_path()?;
    warn_if_volatile(&exe);
    let exe = exe.to_string_lossy().to_string();

    let content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{LABEL}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe}</string>
        <string>--hidden</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#,
        exe = xml_escape(&exe)
    );

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    tracing::info!("[自启动] [macOS] 已写入 LaunchAgent: {}", path.display());
    Ok(())
}

fn plist_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| {
        home.join("Library")
            .join("LaunchAgents")
            .join(format!("{LABEL}.plist"))
    })
}

/// plist XML 中字符串值的最小转义
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
