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
        startup_key.set_value(APP_NAME, &exe_path)?;
    } else {
        startup_key.delete_value(APP_NAME).ok();
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_auto_startup_windows() -> Result<bool, Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let startup_key = hkcu.open_subkey(APP_PATH)?;

    let exe_path: String = startup_key.get_value(APP_NAME)?;
    let current_exe = std::env::current_exe()?.to_string_lossy().to_string();

    Ok(exe_path == current_exe)
}
