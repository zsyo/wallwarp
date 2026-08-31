// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Linux 平台实现：X11 窗口几何与工作区（x11rb）
//!
//! Wayland 协议不允许客户端自由定位/置顶窗口，悬浮球整体禁用；
//! 主窗口拖动（window::drag）与边缘缩放（window::drag_resize）由
//! winit 的 xdg_shell 协议支持，不受影响。

use super::WindowAnchor;
use std::sync::OnceLock;
use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt as RandrExt;
use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt};
use x11rb::rust_connection::RustConnection;

/// X11 连接（进程级复用；连接失败说明无 X11 会话，悬浮球能力判定为 false）
static CONNECTION: OnceLock<Option<RustConnection>> = OnceLock::new();

/// 是否处于 Wayland 会话
pub fn is_wayland() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|t| t.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false)
}

/// 悬浮球窗口支持：仅 X11 会话（Wayland 禁止客户端窗口定位/置顶）
pub fn supports_floating_ball() -> bool {
    !is_wayland() && connection().is_some()
}

/// 从 iced 窗口提取 X11 窗口 id
pub fn window_anchor(mw: &dyn iced::window::Window) -> WindowAnchor {
    use iced::wgpu::rwh::RawWindowHandle;

    match mw.window_handle().map(|h| h.as_raw()) {
        Ok(RawWindowHandle::Xlib(handle)) => WindowAnchor::X11(handle.window as u32),
        Ok(RawWindowHandle::Xcb(handle)) => WindowAnchor::X11(handle.window.get()),
        // Wayland：菜单锚点由 GTK 运行时内部窗口承担，几何查询不可用
        Ok(RawWindowHandle::Wayland(_)) => WindowAnchor::Gtk,
        _ => WindowAnchor::Unsupported,
    }
}

/// 查询窗口几何信息（物理像素，左上原点）
pub fn window_geometry(mw: &dyn iced::window::Window) -> Option<super::WindowGeometry> {
    let conn = connection()?;
    let xid = xid_of(mw)?;

    let geometry = conn.get_geometry(xid).ok()?.reply().ok()?;
    let root = conn.setup().roots.first()?.root;
    let coords = conn
        .translate_coordinates(xid, root, 0, 0)
        .ok()?
        .reply()
        .ok()?;

    Some(super::WindowGeometry {
        x: coords.dst_x as f32,
        y: coords.dst_y as f32,
        size: (geometry.width as f32).max(geometry.height as f32),
    })
}

/// 查询窗口所在显示器的工作区
///
/// RandR 显示器几何（含窗口的那块屏）与 EWMH `_NET_WORKAREA`
/// （全局工作区，排除面板）求交集作为近似工作区
pub fn work_area(mw: &dyn iced::window::Window) -> Option<iced::Rectangle> {
    let conn = connection()?;
    let xid = xid_of(mw)?;
    let root = conn.setup().roots.first()?.root;

    let geometry = conn.get_geometry(xid).ok()?.reply().ok()?;
    let coords = conn
        .translate_coordinates(xid, root, 0, 0)
        .ok()?
        .reply()
        .ok()?;
    let center_x = coords.dst_x as f32 + geometry.width as f32 / 2.0;
    let center_y = coords.dst_y as f32 + geometry.height as f32 / 2.0;

    // 包含窗口中心的显示器（注意 x/y 为 i16、width/height 为 u16，需分别转换）
    let monitors = conn.randr_get_monitors(root, true).ok()?.reply().ok()?;
    let monitor = monitors.monitors.iter().find(|m| {
        center_x >= m.x as f32
            && center_x < (m.x as f32 + m.width as f32)
            && center_y >= m.y as f32
            && center_y < (m.y as f32 + m.height as f32)
    })?;

    let (mx, my, mw_, mh) = (
        monitor.x as f32,
        monitor.y as f32,
        monitor.width as f32,
        monitor.height as f32,
    );
    // 与全局工作区求交集（取不到 _NET_WORKAREA 时退回显示器全区域）
    let mut rect = iced::Rectangle::new(iced::Point::new(mx, my), iced::Size::new(mw_, mh));
    if let Some(work) = net_workarea(conn, root) {
        let x0 = mx.max(work.x);
        let y0 = my.max(work.y);
        let x1 = (mx + mw_).min(work.x + work.width);
        let y1 = (my + mh).min(work.y + work.height);
        if x1 > x0 && y1 > y0 {
            rect =
                iced::Rectangle::new(iced::Point::new(x0, y0), iced::Size::new(x1 - x0, y1 - y0));
        }
    }
    Some(rect)
}

/// 以物理坐标移动窗口（保持尺寸不变）
///
/// X11 没有独立的 MoveWindow 请求，移动 = ConfigureWindow 携带 X/Y
pub fn move_window_to(mw: &dyn iced::window::Window, x: f32, y: f32) {
    if let (Some(conn), Some(xid)) = (connection(), xid_of(mw)) {
        let value_list = ConfigureWindowAux::new().x(x as i32).y(y as i32);
        let _ = conn.configure_window(xid, &value_list);
        let _ = conn.flush();
    }
}

/// 移除窗口的系统边框/非客户区修饰（Linux 无 DWM，无需处理）
pub fn remove_dwm_frame(_mw: &dyn iced::window::Window) {}

/// 为无边框窗口启用系统级边缘缩放（Linux 用自绘边缘感应层，无需处理）
pub fn enable_resize_border(_mw: &dyn iced::window::Window) {}

fn connection() -> Option<&'static RustConnection> {
    CONNECTION
        .get_or_init(|| x11rb::connect(None).ok().map(|(conn, _)| conn))
        .as_ref()
}

fn xid_of(mw: &dyn iced::window::Window) -> Option<u32> {
    match super::window_anchor(mw) {
        WindowAnchor::X11(xid) => Some(xid),
        _ => None,
    }
}

/// 读取根窗口的 `_NET_WORKAREA`（x, y, width, height 四个 CARDINAL）
fn net_workarea(conn: &RustConnection, root: u32) -> Option<iced::Rectangle> {
    let atom = conn
        .intern_atom(false, b"_NET_WORKAREA")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let reply = conn
        .get_property(
            false,
            root,
            atom,
            x11rb::protocol::xproto::AtomEnum::CARDINAL,
            0,
            4,
        )
        .ok()?
        .reply()
        .ok()?;
    let values: Vec<u32> = reply.value32()?.collect();
    if values.len() < 4 {
        return None;
    }
    Some(iced::Rectangle::new(
        iced::Point::new(values[0] as f32, values[1] as f32),
        iced::Size::new(values[2] as f32, values[3] as f32),
    ))
}
