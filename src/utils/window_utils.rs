// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, IsIconic, SW_RESTORE, SW_SHOW, SetForegroundWindow, ShowWindow,
};

/// 检查窗口是否最小化
pub fn is_window_minimized(hwnd: HWND) -> bool {
    unsafe { IsIconic(hwnd).as_bool() }
}

/// 检查窗口是否在前台
pub fn is_window_foreground(hwnd: HWND) -> bool {
    unsafe { GetForegroundWindow() == hwnd }
}

/// 将窗口从最小化状态恢复并置顶
pub fn restore_and_bring_to_front(hwnd: HWND) -> bool {
    unsafe {
        // 如果窗口最小化，先恢复
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }

        // 将窗口置顶
        SetForegroundWindow(hwnd).as_bool()
    }
}

/// 显示窗口并置顶
pub fn show_and_bring_to_front(hwnd: HWND) -> bool {
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        SetForegroundWindow(hwnd).as_bool()
    }
}

/// 获取系统颜色模式
///
/// # 返回
/// 返回 `true` 表示系统使用深色主题，`false` 表示系统使用浅色主题
/// 如果获取失败，默认返回 `false`（浅色主题）
pub fn get_system_color_mode() -> bool {
    match dark_light::detect() {
        Ok(mode) => match mode {
            dark_light::Mode::Light | dark_light::Mode::Unspecified => false,
            dark_light::Mode::Dark => true,
        },
        Err(_) => false,
    }
}
