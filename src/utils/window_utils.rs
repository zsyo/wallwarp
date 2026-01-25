// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, IsIconic, ShowWindow, SetForegroundWindow,
    SW_RESTORE, SW_SHOW,
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