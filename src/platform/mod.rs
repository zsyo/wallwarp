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

// KDE Wayland 托盘化的任务栏条目控制，仅 Linux 编译（其余平台为空操作）
#[cfg(target_os = "linux")]
mod kwin_rules;

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

/// 当前平台是否支持全局热键（Windows/macOS 恒支持；Linux 仅 X11）
pub fn supports_global_hotkeys() -> bool {
    !is_wayland()
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

/// 当前 Linux 会话是否为 KDE Plasma（非 Linux 平台恒为 false）
pub fn is_kde_plasma() -> bool {
    #[cfg(target_os = "linux")]
    {
        imp::is_kde_plasma()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// KDE Wayland：最小化到托盘时写入"跳过任务栏"窗口规则并触发 KWin
/// 重载（Wayland 协议无法撤回窗口，最小化后任务栏仍显示条目；规则为
/// Force 等级，对已映射窗口即时生效，条目随之消失）。非该环境为空操作
pub fn on_minimized_to_tray() {
    #[cfg(target_os = "linux")]
    {
        kwin_rules::on_minimized_to_tray();
    }
}

/// KDE Wayland：从托盘恢复（及启动清理）时移除"跳过任务栏"规则，
/// 窗口恢复任务栏条目。非该环境为空操作
///
/// 返回是否实际移除了规则——KWin 的 reconfigure 有 200ms 防抖，
/// 重建主窗口需等重载完成后再创建（见 show_window 的 Wayland 分支）
pub fn on_restored_from_tray() -> bool {
    #[cfg(target_os = "linux")]
    {
        kwin_rules::on_restored_from_tray()
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// KDE Plasma 壁纸设置（仅 Linux 有实现）：经 gdbus 直连 PlasmaShell
/// 同时写入铺满模式与壁纸路径。wallpaper crate 的 KDE 分支依赖 qdbus
/// 命令（Fedora 等发行版默认缺失），此实现用 glib2 自带的 gdbus
pub fn set_wallpaper_kde(path: &str, mode: wallpaper::Mode) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        imp::set_wallpaper_kde(path, mode)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (path, mode);
        Err("KDE Plasma 壁纸设置仅支持 Linux".into())
    }
}

/// 显示器信息（原生标识与几何，供多显示器独立壁纸使用）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorInfo {
    /// 平台原生显示器标识（Windows: IDesktopWallpaper 设备路径；
    /// macOS: NSScreenNumber；Linux X11: RandR 显示器名）
    pub id: String,
    /// 显示名称（Windows: 设备名如 \\.\DISPLAY1；macOS: localizedName；
    /// Linux: RandR 名称）
    pub name: String,
    /// 显示器左上角横坐标（原生全屏坐标系）
    pub x: i32,
    /// 显示器左上角纵坐标（原生全屏坐标系）
    pub y: i32,
    /// 显示器宽度（物理像素）
    pub width: u32,
    /// 显示器高度（物理像素）
    pub height: u32,
    /// 是否主显示器
    pub primary: bool,
}

/// 枚举系统所有显示器
///
/// Wayland 会话或查询失败时返回空列表（调用方按不支持处理）
pub fn enumerate_monitors() -> Vec<MonitorInfo> {
    imp::enumerate_monitors()
}

/// 当前环境是否支持按显示器独立设置壁纸
///
/// Windows（IDesktopWallpaper）与 macOS（NSWorkspace）恒支持；
/// Linux 仅 KDE Plasma（X11/Wayland 经 PlasmaShell 按桌面索引设置）支持
pub fn supports_per_monitor_wallpaper() -> bool {
    imp::supports_per_monitor_wallpaper()
}

/// 为指定显示器设置壁纸（monitor_id 为 [`enumerate_monitors`] 返回的 id）
///
/// # 参数
/// - `monitor_id`: 目标显示器的平台原生标识
/// - `image_path`: 壁纸图片路径（绝对路径）
/// - `mode`: 铺满模式（macOS 下由系统决定，仅 Windows/KDE 精确生效）
///
/// # 返回
/// 当前环境不支持按显示器设置时返回 Err
pub fn set_wallpaper_for_monitor(
    monitor_id: &str,
    image_path: &str,
    mode: crate::utils::config::WallpaperMode,
) -> Result<(), String> {
    imp::set_wallpaper_for_monitor(monitor_id, image_path, mode)
}

/// 当前 KDE 会话是否正在注销/关机（非 Linux 平台恒为 false）
///
/// 会话管理器（ksmserver）在注销/关机流程第一步 closeSession 即置位该
/// 状态，早于 compositor（KWin closeWaylandWindows）向窗口下发 close 事件。
/// Wayland 下"会话关闭"与用户 Alt+F4 走同一 close 通道无法直接区分，
/// 收到窗口关闭请求时查询此状态：会话结束必须退出而非托盘化，否则会话
/// 管理器等待超时后弹出"应用未关闭"确认框阻塞关机
pub fn is_session_shutting_down() -> bool {
    #[cfg(target_os = "linux")]
    {
        imp::is_session_shutting_down()
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

/// 弹出菜单前将窗口前置（仅 Windows 有效：TrackPopupMenu 需要前置窗口
/// 才能点击外部关闭；macOS/Linux 弹出机制无此要求，为空操作）
pub fn set_foreground_window(hwnd: isize) {
    imp::set_foreground_window(hwnd)
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

/// 系统颜色模式监听器（三平台通用）
///
/// 事件驱动：Windows 为注册表变更通知（`RegNotifyChangeKeyValue`）、
/// macOS 为 dark-light 内部线程按固定间隔轮询 NSUserDefaults、
/// Linux 为 XDG desktop portal 的 D-Bus 信号。仅上报变化事件，不含初始状态。
///
/// Drop 时停止底层监听线程
pub struct ColorModeWatcher {
    watcher: dark_light::Watcher,
}

impl ColorModeWatcher {
    /// 阻塞等待下一次颜色模式变化
    ///
    /// # 返回
    /// `Some(true)` 表示深色、`Some(false)` 表示浅色；底层监听结束后返回 `None`
    pub fn recv(&self) -> Option<bool> {
        self.watcher
            .recv()
            .ok()
            .map(|mode| matches!(mode, dark_light::Mode::Dark))
    }
}

/// 订阅系统颜色模式变化（深浅色切换）
///
/// 调用方需在专用线程中 [`ColorModeWatcher::recv`]（阻塞调用），
/// 不应直接在异步执行器线程上等待
pub fn subscribe_color_mode() -> Result<ColorModeWatcher, String> {
    dark_light::subscribe()
        .map(|watcher| ColorModeWatcher { watcher })
        .map_err(|error| error.to_string())
}
