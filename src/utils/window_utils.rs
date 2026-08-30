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

/// 将指定句柄（isize 形式）的窗口设为前台
///
/// 供弹出原生菜单前调用（如悬浮球菜单），确保菜单能正常响应外部点击关闭
pub fn set_foreground_window_by_isize(hwnd: isize) {
    unsafe {
        let _ = SetForegroundWindow(HWND(hwnd as *mut _));
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

/// 移除窗口的 DWM 系统边框与非客户区渲染
///
/// 用于悬浮球：Windows 11 会为圆角窗口绘制 1px 系统边框，
/// 在透明窗口上表现为圆球外围的一圈方框，需显式禁用。
///
/// # 参数
/// - `hwnd`: 窗口句柄
pub fn remove_dwm_frame(hwnd: HWND) {
    use windows::Win32::Graphics::Dwm::{
        DWMNCRENDERINGPOLICY, DWMNCRP_DISABLED, DWMWA_BORDER_COLOR, DWMWA_COLOR_NONE,
        DWMWA_NCRENDERING_POLICY, DwmSetWindowAttribute,
    };

    unsafe {
        // 禁用非客户区渲染（系统阴影/边框）
        let policy = DWMNCRENDERINGPOLICY(DWMNCRP_DISABLED.0);
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_NCRENDERING_POLICY,
            &policy as *const _ as *const std::ffi::c_void,
            std::mem::size_of::<DWMNCRENDERINGPOLICY>() as u32,
        );

        // Win11 22000+：将边框颜色设为 NONE（旧系统调用失败可忽略）
        let color_none = DWMWA_COLOR_NONE;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_BORDER_COLOR,
            &color_none as *const _ as *const std::ffi::c_void,
            std::mem::size_of::<u32>() as u32,
        );
    }
}
