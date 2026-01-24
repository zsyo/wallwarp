// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use winreg::RegKey;
#[cfg(target_os = "windows")]
use winreg::enums::*;

const APP_NAME: &str = "WallWarp";
const APP_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

pub fn is_auto_startup_enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        match get_auto_startup_windows() {
            Ok(enabled) => enabled,
            Err(_) => false,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

pub fn set_auto_startup(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        set_auto_startup_windows(enabled)
    }
    #[cfg(not(target_os = "windows"))]
    {
        // 非Windows平台暂时不支持
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn set_auto_startup_windows(enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (startup_key, _) = hkcu.create_subkey(APP_PATH)?;

    let exe_path = std::env::current_exe()?.to_string_lossy().to_string();

    if enable {
        // 格式: "exe_path" --hidden
        let startup_value = format!("\"{}\" --hidden", exe_path);
        startup_key.set_value(APP_NAME, &startup_value)?;
    } else {
        startup_key.delete_value(APP_NAME).ok();
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_auto_startup_windows() -> Result<bool, Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let startup_key = hkcu.open_subkey(APP_PATH)?;

    let startup_value: String = startup_key.get_value(APP_NAME)?;
    let current_exe = std::env::current_exe()?.to_string_lossy().to_string();

    // 解析启动命令，提取 exe_path
    // 支持以下格式：
    // 1. E:\Tool\wallwarp\wallwarp.exe
    // 2. "E:\Tool\wallwarp\wallwarp.exe"
    // 3. E:\Tool\wallwarp\wallwarp.exe --hidden
    // 4. "E:\Tool\wallwarp\wallwarp.exe" --hidden
    let registered_exe = if startup_value.starts_with('"') {
        // 提取第一个引号内的内容
        if let Some(end_quote) = startup_value[1..].find('"') {
            &startup_value[1..end_quote + 1]
        } else {
            // 如果没有结束引号，尝试整个字符串
            startup_value.trim_start_matches('"')
        }
    } else {
        // 如果没有引号，按空格分割取第一部分
        startup_value.split_whitespace().next().unwrap_or(&startup_value)
    };

    Ok(registered_exe == current_exe)
}
