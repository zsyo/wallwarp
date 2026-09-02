// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 平台抽象层：窗口几何查询（物理坐标）、原生菜单锚点、平台专属窗口修饰。
//!
//! 每个平台的实现位于独立文件，按 `target_os` 编译期选择：
//! - [`windows`]: Win32（MonitorFromWindow / GetMonitorInfoW / SetWindowPos）
//! - [`macos`]:   AppKit（NSScreen visibleFrame / NSWindow setFrameOrigin）
//! - [`linux`]:   X11（x11rb）；Wayland 下窗口定位受限，悬浮球整体禁用
//!
//! 坐标系约定：所有几何函数使用"窗口所在平台的原生全屏坐标"
//! （Windows/Linux 为物理像素、左上原点；macOS 为点坐标、左下原点），
//! 只要求同一平台内自洽（贴边保存的上下文与后续移动使用同一坐标系）。

pub mod menu;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as imp;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as imp;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as imp;

/// 弹出原生菜单所需的窗口锚点（由基本类型组成，可跨线程传递）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAnchor {
    /// Win32 HWND
    Win32(isize),
    /// macOS NSView 指针
    MacOs(usize),
    /// X11 窗口 id
    X11(u32),
    /// Linux：锚点由菜单运行时内部的 GTK 窗口承担，无需外部句柄
    Gtk,
    /// 未知窗口后端（如 Wayland 下的非悬浮球窗口）
    Unsupported,
}

/// 窗口几何信息（原生全屏坐标系，见模块文档）
#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    /// 窗口左上角（macOS 为左下角）的 x 坐标
    pub x: f32,
    /// 窗口左上角（macOS 为左下角）的 y 坐标
    pub y: f32,
    /// 窗口边长（悬浮球为正方形窗口，取宽高中较大者）
    pub size: f32,
}

/// 当前平台是否支持桌面悬浮球（需要窗口定位与置顶能力）
pub fn supports_floating_ball() -> bool {
    imp::supports_floating_ball()
}

/// 当前 Linux 会话是否为 Wayland（非 Linux 平台恒为 false）
///
/// Wayland 协议限制：客户端无法自行取消最小化/恢复隐藏窗口
/// （winit 的 set_visible 为空操作、set_minimized(false) 被忽略），
/// 依赖"恢复窗口"的场景（托盘唤醒主窗口）需按此分支改走重建窗口流程
pub fn is_wayland() -> bool {
    #[cfg(target_os = "linux")]
    {
        imp::is_wayland()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// 从 iced 窗口提取原生菜单锚点
pub fn window_anchor(mw: &dyn iced::window::Window) -> WindowAnchor {
    imp::window_anchor(mw)
}

/// 查询窗口几何信息（原生全屏坐标系）
pub fn window_geometry(mw: &dyn iced::window::Window) -> Option<WindowGeometry> {
    imp::window_geometry(mw)
}

/// 查询窗口所在显示器的工作区（同坐标系；Windows/Linux 排除任务栏，
/// macOS 使用 NSScreen visibleFrame）
pub fn work_area(mw: &dyn iced::window::Window) -> Option<iced::Rectangle> {
    imp::work_area(mw)
}

/// 以原生坐标系移动窗口（保持尺寸不变）
pub fn move_window_to(mw: &dyn iced::window::Window, x: f32, y: f32) {
    imp::move_window_to(mw, x, y)
}

/// 移除窗口的系统边框/非客户区修饰（仅 Windows 有效，其他平台为空操作）
pub fn remove_dwm_frame(mw: &dyn iced::window::Window) {
    imp::remove_dwm_frame(mw)
}

/// 为无边框窗口启用系统级边缘缩放（仅 Windows 有效：
/// macOS 使用原生 fullsize content view，Linux 使用自绘边缘感应层）
pub fn enable_resize_border(mw: &dyn iced::window::Window) {
    imp::enable_resize_border(mw)
}

/// 获取系统颜色模式（dark-light，三平台通用）
///
/// # 返回
/// `true` 表示深色主题；获取失败默认 `false`（浅色）
pub fn system_color_mode() -> bool {
    match dark_light::detect() {
        Ok(mode) => matches!(mode, dark_light::Mode::Dark),
        Err(_) => false,
    }
}
