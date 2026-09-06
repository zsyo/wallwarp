// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 托盘图标与原生菜单的跨平台封装
//!
//! Windows/macOS：muda 对象在主线程（iced 事件循环线程）直接创建与修改。
//! Linux：muda 对象基于 `Rc`（非 Send），且 libappindicator 托盘需要
//! 运行中的 GTK 事件循环，因此全部菜单与托盘图标在专用 GTK 线程中创建，
//! 主线程通过命令通道（见 [`menu_linux`]）间接操控，公共 API 完全一致。

use super::WindowAnchor;

#[cfg(target_os = "linux")]
mod menu_linux;
#[cfg(target_os = "linux")]
use menu_linux::MenuCommand;
#[cfg(target_os = "linux")]
use std::sync::mpsc::Sender;

#[cfg(not(target_os = "linux"))]
use std::collections::HashMap;

/// 菜单归属（托盘 / 悬浮球）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MenuKind {
    Tray,
    Ball,
}

/// 单个菜单项的定义
pub struct MenuItemDef {
    /// 菜单事件 id（与现有 TrayMenuEvent 处理器中的 id 一致）
    pub id: String,
    /// 显示文本（i18n 已翻译）
    pub text: String,
    /// 初始可用状态
    pub enabled: bool,
}

/// 菜单句柄（可克隆；Windows/macOS 上为 Rc 共享，Linux 上为命令通道）
#[derive(Clone)]
pub struct Menu {
    kind: MenuKind,
    #[cfg(not(target_os = "linux"))]
    inner: std::rc::Rc<MenuInner>,
    #[cfg(target_os = "linux")]
    tx: Sender<MenuCommand>,
}

/// 托盘图标句柄
pub struct TrayIconHandle {
    #[cfg(not(target_os = "linux"))]
    tray: Option<tray_icon::TrayIcon>,
    #[cfg(target_os = "linux")]
    tx: Sender<MenuCommand>,
}

/// 菜单构建顺序中的一条目
pub(super) enum MenuEntryPlan {
    /// 第 idx 个菜单项（按定义顺序）
    Item(usize),
    /// 分隔线
    Separator,
}

/// 计算"菜单项 + 分隔线"的交错顺序（纯计算，各平台构建共用）
///
/// `separator_after` 存储项下标：在该下标的项之后插入一条分隔线；
/// 分隔线实例按计划顺序逐条取用，实例数必须与 `separator_after.len()` 一致
pub(super) fn plan_menu_entries(
    item_count: usize,
    separator_after: &[usize],
) -> Vec<MenuEntryPlan> {
    let mut plan = Vec::with_capacity(item_count + separator_after.len());
    for idx in 0..item_count {
        plan.push(MenuEntryPlan::Item(idx));
        if separator_after.contains(&idx) {
            plan.push(MenuEntryPlan::Separator);
        }
    }
    plan
}

/// 构建菜单（separator_after：在这些项的下标之后插入分隔线）
pub fn build_menu(kind: MenuKind, items: Vec<MenuItemDef>, separator_after: &[usize]) -> Menu {
    #[cfg(not(target_os = "linux"))]
    {
        use tray_icon::menu::{IsMenuItem, Menu as MudaMenu, MenuItem, PredefinedMenuItem};

        // 保持定义顺序构建菜单项
        let ordered: Vec<(String, MenuItem)> = items
            .into_iter()
            .map(|def| {
                (
                    def.id.clone(),
                    MenuItem::with_id(def.id, def.text, def.enabled, None),
                )
            })
            .collect();

        let separators: Vec<PredefinedMenuItem> = separator_after
            .iter()
            .map(|_| PredefinedMenuItem::separator())
            .collect();
        let mut next_sep = separators.iter();
        let list: Vec<&dyn IsMenuItem> = plan_menu_entries(ordered.len(), separator_after)
            .into_iter()
            .map(|entry| match entry {
                MenuEntryPlan::Item(idx) => &ordered[idx].1 as &dyn IsMenuItem,
                MenuEntryPlan::Separator => {
                    next_sep.next().expect("分隔线实例数与计划不符") as &dyn IsMenuItem
                }
            })
            .collect();
        let menu = MudaMenu::with_items(&list).expect("创建托盘菜单失败");
        let items = ordered.into_iter().collect();

        Menu {
            kind,
            inner: std::rc::Rc::new(MenuInner { menu, items }),
        }
    }
    #[cfg(target_os = "linux")]
    {
        let texts: Vec<(String, String, bool)> = items
            .into_iter()
            .map(|def| (def.id, def.text, def.enabled))
            .collect();
        menu_linux::build_menu(kind, texts, separator_after)
    }
}

/// 创建托盘图标（图标 RGBA 数据 + 绑定菜单 + 初始 tooltip）
pub fn attach_tray(
    icon_rgba: Vec<u8>,
    width: u32,
    height: u32,
    menu: Menu,
    tooltip: &str,
) -> Result<TrayIconHandle, String> {
    debug_assert_eq!(menu.kind(), MenuKind::Tray, "托盘图标必须绑定 Tray 菜单");

    #[cfg(not(target_os = "linux"))]
    {
        use tray_icon::TrayIconBuilder;

        let icon = tray_icon::Icon::from_rgba(icon_rgba, width, height)
            .map_err(|e| format!("托盘图标解码失败: {e}"))?;
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu.inner.menu.clone()))
            .with_tooltip(tooltip)
            .with_icon(icon)
            .build()
            .map_err(|e| format!("创建托盘图标失败: {e}"))?;
        Ok(TrayIconHandle { tray: Some(tray) })
    }
    #[cfg(target_os = "linux")]
    {
        menu_linux::attach_tray(icon_rgba, width, height, tooltip)
    }
}

/// 空托盘句柄（托盘创建失败时降级使用，所有操作为空操作）
#[must_use]
pub fn disabled_tray() -> TrayIconHandle {
    #[cfg(not(target_os = "linux"))]
    {
        TrayIconHandle { tray: None }
    }
    #[cfg(target_os = "linux")]
    {
        let (tx, _rx) = std::sync::mpsc::channel();
        TrayIconHandle { tx }
    }
}

#[cfg(not(target_os = "linux"))]
struct MenuInner {
    menu: tray_icon::menu::Menu,
    /// 供后续 set_text/set_enabled 修改
    items: HashMap<String, tray_icon::menu::MenuItem>,
}

impl Menu {
    pub fn kind(&self) -> MenuKind {
        self.kind
    }

    /// 修改菜单项可用状态
    pub fn set_enabled(&self, id: &str, enabled: bool) {
        #[cfg(not(target_os = "linux"))]
        if let Some(item) = self.inner.items.get(id) {
            item.set_enabled(enabled);
        }
        #[cfg(target_os = "linux")]
        {
            let _ = self
                .tx
                .send(MenuCommand::SetEnabled(self.kind, id.to_string(), enabled));
        }
    }

    /// 修改菜单项文本
    pub fn set_text(&self, id: &str, text: String) {
        #[cfg(not(target_os = "linux"))]
        if let Some(item) = self.inner.items.get(id) {
            item.set_text(text);
        }
        #[cfg(target_os = "linux")]
        {
            let _ = self
                .tx
                .send(MenuCommand::SetText(self.kind, id.to_string(), text));
        }
    }

    /// 在指定窗口锚点处弹出菜单（悬浮球用），阻塞至菜单关闭
    ///
    /// Windows：TrackPopupMenu（弹出前先前置窗口）；macOS：NSView 弹出；
    /// Linux：命令通道发往 GTK 线程弹出，经 oneshot 回执等待实际结果，
    /// 三平台返回值均表示菜单是否成功弹出
    pub fn popup_at(&self, anchor: WindowAnchor) -> bool {
        #[cfg(not(target_os = "linux"))]
        {
            use tray_icon::menu::ContextMenu;
            match anchor {
                WindowAnchor::Win32(hwnd) => {
                    #[cfg(target_os = "windows")]
                    {
                        super::set_foreground_window(hwnd);
                        unsafe { self.inner.menu.show_context_menu_for_hwnd(hwnd, None) }
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        let _ = hwnd;
                        false
                    }
                }
                WindowAnchor::MacOs(view) => {
                    #[cfg(target_os = "macos")]
                    {
                        unsafe {
                            self.inner
                                .menu
                                .show_context_menu_for_nsview(view as *const std::ffi::c_void, None)
                        }
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        let _ = view;
                        false
                    }
                }
                _ => false,
            }
        }
        #[cfg(target_os = "linux")]
        {
            let _ = anchor;
            // oneshot 回执：等待 GTK 线程回传实际弹出结果
            // （弹出期间阻塞，与 Windows/macOS 的阻塞弹出语义一致）
            let (result_tx, result_rx) = std::sync::mpsc::channel();
            if self.tx.send(MenuCommand::PopupBall(result_tx)).is_err() {
                return false;
            }
            result_rx.recv().unwrap_or(false)
        }
    }
}

impl TrayIconHandle {
    /// 更新托盘 tooltip
    pub fn set_tooltip(&self, text: &str) {
        #[cfg(not(target_os = "linux"))]
        if let Some(tray) = &self.tray {
            tray.set_tooltip(Some(text)).ok();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = self.tx.send(MenuCommand::SetTooltip(text.to_string()));
        }
    }
}
