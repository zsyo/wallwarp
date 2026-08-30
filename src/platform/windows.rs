// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Windows 平台实现：Win32 窗口几何、工作区、窗口移动与 DWM 修饰

use super::WindowAnchor;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, SWP_NOSIZE, SetWindowPos};

/// 从 iced 窗口提取 Win32 HWND
pub fn window_anchor(mw: &dyn iced::window::Window) -> WindowAnchor {
    use iced::wgpu::rwh::RawWindowHandle;

    match mw.window_handle().map(|h| h.as_raw()) {
        Ok(RawWindowHandle::Win32(handle)) => WindowAnchor::Win32(handle.hwnd.get()),
        _ => WindowAnchor::Unsupported,
    }
}

/// 查询窗口几何信息（物理像素，左上原点）
pub fn window_geometry(mw: &dyn iced::window::Window) -> Option<super::WindowGeometry> {
    let hwnd = hwnd_of(mw)?;
    let rect = window_rect(hwnd)?;
    Some(super::WindowGeometry {
        x: rect.left as f32,
        y: rect.top as f32,
        size: (rect.right - rect.left).max(rect.bottom - rect.top) as f32,
    })
}

/// 查询窗口所在显示器的工作区（物理像素，排除任务栏）
pub fn work_area(mw: &dyn iced::window::Window) -> Option<iced::Rectangle> {
    let hwnd = hwnd_of(mw)?;
    let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    if unsafe { GetMonitorInfoW(monitor, &mut info) }.as_bool() {
        let rc = info.rcWork;
        return Some(iced::Rectangle::new(
            iced::Point::new(rc.left as f32, rc.top as f32),
            iced::Size::new((rc.right - rc.left) as f32, (rc.bottom - rc.top) as f32),
        ));
    }
    None
}

/// 以物理坐标移动窗口（保持尺寸不变，不激活、不改层级）
pub fn move_window_to(mw: &dyn iced::window::Window, x: f32, y: f32) {
    if let Some(hwnd) = hwnd_of(mw) {
        unsafe {
            let _ = SetWindowPos(hwnd, None, x as i32, y as i32, 0, 0, SWP_NOSIZE);
        }
    }
}

/// 移除窗口的 DWM 系统边框与非客户区渲染
///
/// Windows 11 会为圆角窗口绘制 1px 系统边框，在透明窗口上表现为
/// 圆球外围的一圈方框，需显式禁用
pub fn remove_dwm_frame(mw: &dyn iced::window::Window) {
    use windows::Win32::Graphics::Dwm::{
        DWMNCRENDERINGPOLICY, DWMNCRP_DISABLED, DWMWA_BORDER_COLOR, DWMWA_COLOR_NONE,
        DWMWA_NCRENDERING_POLICY, DwmSetWindowAttribute,
    };

    let Some(hwnd) = hwnd_of(mw) else {
        return;
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

/// 为无边框窗口添加 WS_THICKFRAME/WS_SIZEBOX 样式并启用阴影，
/// 使系统接管窗口边缘缩放（自定义标题栏方案）
pub fn enable_resize_border(mw: &dyn iced::window::Window) {
    use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
    use windows::Win32::UI::Controls::MARGINS;
    use windows::Win32::UI::WindowsAndMessaging::{
        GWL_STYLE, GetWindowLongPtrW, SetWindowLongPtrW, WS_SIZEBOX, WS_THICKFRAME,
    };

    let Some(hwnd) = hwnd_of(mw) else {
        return;
    };
    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        let new_style = style | WS_THICKFRAME.0 as isize | WS_SIZEBOX.0 as isize;
        let _ = SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);

        // 边距设为 -1，整个窗口参与 DWM 阴影
        let margins = MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyTopHeight: -1,
            cyBottomHeight: -1,
        };
        let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
    }
}

/// 弹出菜单前将窗口前置（TrackPopupMenu 需要前置窗口才能点击外部关闭）
pub fn set_foreground_window(hwnd: isize) {
    use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
    unsafe {
        let _ = SetForegroundWindow(HWND(hwnd as *mut _));
    }
}

/// 悬浮球窗口支持（Windows 全平台支持）
pub fn supports_floating_ball() -> bool {
    true
}

fn hwnd_of(mw: &dyn iced::window::Window) -> Option<HWND> {
    match super::window_anchor(mw) {
        WindowAnchor::Win32(hwnd) => Some(HWND(hwnd as *mut _)),
        _ => None,
    }
}

fn window_rect(hwnd: HWND) -> Option<RECT> {
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect) }.ok().map(|_| rect)
}
