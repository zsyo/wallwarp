// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! macOS 平台实现：AppKit 窗口几何与工作区
//!
//! 坐标系为 AppKit 全局坐标（左下原点、y 向上、点单位），
//! 与平台内其他几何查询/移动调用自洽。

use super::WindowAnchor;
use objc2::rc::Retained;
use objc2_app_kit::{NSScreen, NSView, NSWindow};

/// 从 iced 窗口提取 NSView 指针（作为菜单弹出锚点）
pub fn window_anchor(mw: &dyn iced::window::Window) -> WindowAnchor {
    use iced::wgpu::rwh::RawWindowHandle;

    match mw.window_handle().map(|h| h.as_raw()) {
        Ok(RawWindowHandle::AppKit(handle)) => {
            WindowAnchor::MacOs(handle.ns_view.as_ptr() as usize)
        }
        _ => WindowAnchor::Unsupported,
    }
}

/// 查询窗口几何信息（AppKit 全局坐标）
pub fn window_geometry(mw: &dyn iced::window::Window) -> Option<super::WindowGeometry> {
    let window = ns_window(mw)?;
    let frame = window.frame();
    // CGFloat 为 f64，iced 几何类型为 f32
    Some(super::WindowGeometry {
        x: frame.origin.x as f32,
        y: frame.origin.y as f32,
        size: frame.size.width.max(frame.size.height) as f32,
    })
}

/// 查询窗口所在屏幕的可见区域（visibleFrame，排除 Dock 与菜单栏）
pub fn work_area(mw: &dyn iced::window::Window) -> Option<iced::Rectangle> {
    let window = ns_window(mw)?;
    let screen: Retained<NSScreen> = window.screen()?;
    let visible = screen.visibleFrame();
    Some(iced::Rectangle::new(
        iced::Point::new(visible.origin.x as f32, visible.origin.y as f32),
        iced::Size::new(visible.size.width as f32, visible.size.height as f32),
    ))
}

/// 以 AppKit 全局坐标移动窗口（保持尺寸不变）
pub fn move_window_to(mw: &dyn iced::window::Window, x: f32, y: f32) {
    if let Some(window) = ns_window(mw) {
        window.setFrameOrigin(objc2_foundation::NSPoint::new(x as f64, y as f64));
    }
}

/// 移除窗口的系统边框/非客户区修饰（macOS 用原生 fullsize content view，无需处理）
pub fn remove_dwm_frame(_mw: &dyn iced::window::Window) {}

/// 为无边框窗口启用系统级边缘缩放（macOS 用原生 fullsize content view，无需处理）
pub fn enable_resize_border(_mw: &dyn iced::window::Window) {}

/// 悬浮球窗口支持（macOS 全平台支持）
pub fn supports_floating_ball() -> bool {
    true
}

/// 通过 raw window handle 获取 NSView → NSWindow
///
/// winit 在 macOS 上创建的是标准 NSView 层级，
/// NSView::window 即可拿到宿主 NSWindow（AppKit 对象只能在主线程访问，
/// 本模块所有函数均由 window::run 在主线程调用）。
fn ns_window(mw: &dyn iced::window::Window) -> Option<Retained<NSWindow>> {
    let view = ns_view(mw)?;
    view.window()
}

/// 从 iced 窗口提取 NSView（指针安全性由主线程调用约定保证，
/// retain_autoreleased 失败时返回 None）
fn ns_view(mw: &dyn iced::window::Window) -> Option<Retained<NSView>> {
    let WindowAnchor::MacOs(ptr) = super::window_anchor(mw) else {
        return None;
    };
    if ptr == 0 {
        return None;
    }
    // 指针来自 winit 持有的 NSView，借用期间其生命周期由宿主窗口保证
    let view: *mut NSView = ptr as *mut NSView;
    unsafe { Some(Retained::retain_autoreleased(view)?) }
}
