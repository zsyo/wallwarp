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

/// 桌面壁纸 COM 对象（IDesktopWallpaper，支持按显示器设置壁纸）
fn desktop_wallpaper() -> Option<windows::Win32::UI::Shell::IDesktopWallpaper> {
    use windows::Win32::System::Com::{
        CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoCreateInstance,
        CoInitializeEx,
    };
    use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};

    unsafe {
        // S_FALSE（本线程已初始化）同样视为成功；RPC_E_CHANGED_MODE 时仍尝试创建
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        let wallpaper: Result<IDesktopWallpaper, _> =
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL);
        wallpaper.ok()
    }
}

/// 枚举系统所有显示器
///
/// IDesktopWallpaper 提供可设置的显示器设备路径，Gdi 枚举提供设备名与
/// 主屏标记，按显示器矩形一一对应合并
pub fn enumerate_monitors() -> Vec<super::MonitorInfo> {
    use windows::Win32::Foundation::{LPARAM, RECT};
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW,
    };
    use windows::core::{BOOL, HSTRING};

    struct GdiMonitor {
        rect: RECT,
        name: String,
        primary: bool,
    }

    unsafe extern "system" fn enum_monitor(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let list = unsafe { &mut *(lparam.0 as *mut Vec<GdiMonitor>) };
        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
        unsafe {
            if GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut MONITORINFO).as_bool() {
                let len = info
                    .szDevice
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(info.szDevice.len());
                // MONITORINFOF_PRIMARY = 1（windows 0.62 未导出该常量）
                list.push(GdiMonitor {
                    rect: info.monitorInfo.rcMonitor,
                    name: String::from_utf16_lossy(&info.szDevice[..len]),
                    primary: (info.monitorInfo.dwFlags & 1u32) != 0,
                });
            }
        }
        true.into()
    }

    let mut gdi_monitors: Vec<GdiMonitor> = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(enum_monitor),
            LPARAM(&mut gdi_monitors as *mut _ as isize),
        );
    }

    let Some(wallpaper) = desktop_wallpaper() else {
        return Vec::new();
    };

    let count = match unsafe { wallpaper.GetMonitorDevicePathCount() } {
        Ok(count) => count,
        Err(_) => return Vec::new(),
    };

    let mut monitors = Vec::new();
    for index in 0..count {
        let Ok(id) = (unsafe { wallpaper.GetMonitorDevicePathAt(index) }) else {
            continue;
        };
        let Ok(id) = (unsafe { id.to_string() }) else {
            continue;
        };
        let Ok(rect) = (unsafe { wallpaper.GetMonitorRECT(&HSTRING::from(&id)) }) else {
            continue;
        };
        let gdi = gdi_monitors.iter().find(|g| g.rect == rect);
        monitors.push(super::MonitorInfo {
            id,
            name: gdi
                .map(|g| g.name.clone())
                .unwrap_or_else(|| format!("Display {}", index + 1)),
            x: rect.left,
            y: rect.top,
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
            primary: gdi.map(|g| g.primary).unwrap_or(index == 0),
        });
    }
    monitors
}

/// 当前环境支持按显示器独立设置壁纸（IDesktopWallpaper 恒可用）
pub fn supports_per_monitor_wallpaper() -> bool {
    true
}

/// 为指定显示器设置壁纸（IDesktopWallpaper SetWallpaper/SetPosition）
pub fn set_wallpaper_for_monitor(
    monitor_id: &str,
    image_path: &str,
    mode: crate::utils::config::WallpaperMode,
) -> Result<(), String> {
    use windows::Win32::UI::Shell::{
        DWPOS_CENTER, DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE,
    };
    use windows::core::{HSTRING, PCWSTR};

    let wallpaper =
        desktop_wallpaper().ok_or_else(|| "初始化 IDesktopWallpaper 失败".to_string())?;
    let id = HSTRING::from(monitor_id);
    let path = HSTRING::from(image_path);

    let position = match mode {
        crate::utils::config::WallpaperMode::Crop => DWPOS_FILL,
        crate::utils::config::WallpaperMode::Fit => DWPOS_FIT,
        crate::utils::config::WallpaperMode::Stretch => DWPOS_STRETCH,
        crate::utils::config::WallpaperMode::Tile => DWPOS_TILE,
        crate::utils::config::WallpaperMode::Center => DWPOS_CENTER,
        crate::utils::config::WallpaperMode::Span => DWPOS_SPAN,
    };

    unsafe {
        wallpaper
            .SetWallpaper(PCWSTR(id.as_ptr()), PCWSTR(path.as_ptr()))
            .map_err(|e| format!("设置显示器壁纸失败: {}", e))?;
        // SetPosition 为全局铺满模式（跨显示器统一），与全局设置行为一致
        wallpaper
            .SetPosition(position)
            .map_err(|e| format!("设置壁纸铺满模式失败: {}", e))?;
    }
    Ok(())
}
