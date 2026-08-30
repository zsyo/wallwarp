// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Windows 开机自启动：HKCU Run 注册表键

use super::executable_path;
use winreg::RegKey;
use winreg::enums::*;

const APP_NAME: &str = "WallWarp";
const APP_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

pub(super) fn is_enabled() -> Result<bool, Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let startup_key = hkcu.open_subkey(APP_PATH)?;

    let startup_value: String = startup_key.get_value(APP_NAME)?;
    let current_exe = executable_path()?.to_string_lossy().to_string();

    // 解析启动命令，提取 exe_path
    // 支持以下格式：
    // 1. E:\Tool\wallwarp\wallwarp.exe
    // 2. "E:\Tool\wallwarp\wallwarp.exe"
    // 3. E:\Tool\wallwarp\wallwarp.exe --hidden
    // 4. "E:\Tool\wallwarp\wallwarp.exe" --hidden
    let registered_exe = if let Some(unquoted) = startup_value.strip_prefix('"') {
        // 提取第一个引号内的内容，如果没有结束引号则取整个字符串
        match unquoted.find('"') {
            Some(end_quote) => &unquoted[..end_quote],
            None => unquoted,
        }
    } else {
        // 如果没有引号，按空格分割取第一部分
        startup_value
            .split_whitespace()
            .next()
            .unwrap_or(&startup_value)
    };

    Ok(registered_exe == current_exe)
}

pub(super) fn set(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (startup_key, _) = hkcu.create_subkey(APP_PATH)?;

    let exe_path = executable_path()?.to_string_lossy().to_string();

    if enabled {
        // 格式: "exe_path" --hidden
        let startup_value = format!("\"{}\" --hidden", exe_path);
        startup_key.set_value(APP_NAME, &startup_value)?;
    } else {
        startup_key.delete_value(APP_NAME).ok();
    }

    Ok(())
}
