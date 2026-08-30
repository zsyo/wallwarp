// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Linux 菜单运行时：专用 GTK 线程
//!
//! muda 的 `Menu`/`MenuItem` 基于 `Rc`（非 Send），libappindicator 托盘
//! 又依赖运行中的 GTK/GLib 事件循环，因此托盘图标、托盘菜单与悬浮球菜单
//! 全部在一条专用线程上创建，主线程经本模块的命令通道操控。
//! 菜单激活事件仍通过 muda 全局 `MenuEvent::receiver()` 流入订阅，无需适配。

use super::super::menu::MenuKind;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tray_icon::menu::{IsMenuItem, Menu as MudaMenu, MenuItem, PredefinedMenuItem};

/// 主线程 → GTK 线程 的菜单命令
pub(super) enum MenuCommand {
    BuildMenu(MenuKind, Vec<(String, String, bool)>, Vec<usize>),
    AttachTray {
        rgba: Vec<u8>,
        width: u32,
        height: u32,
        tooltip: String,
    },
    SetText(MenuKind, String, String),
    SetEnabled(MenuKind, String, bool),
    SetTooltip(String),
    PopupBall,
}

static COMMAND_TX: OnceLock<Sender<MenuCommand>> = OnceLock::new();

/// 菜单命令通道（首次调用时启动 GTK 线程）
pub(super) fn command_sender() -> &'static Sender<MenuCommand> {
    COMMAND_TX.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::Builder::new()
            .name("wallwarp-tray".into())
            .spawn(move || gtk_thread(rx))
            .expect("启动托盘 GTK 线程失败");
        tx
    })
}

pub(super) fn build_menu(
    kind: MenuKind,
    items: Vec<(String, String, bool)>,
    separator_after: &[usize],
) -> super::super::menu::Menu {
    let _ = command_sender().send(MenuCommand::BuildMenu(
        kind,
        items,
        separator_after.to_vec(),
    ));
    super::super::menu::Menu {
        kind,
        tx: command_sender().clone(),
    }
}

pub(super) fn attach_tray(
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    tooltip: &str,
) -> Result<super::super::menu::TrayIconHandle, String> {
    let _ = command_sender().send(MenuCommand::AttachTray {
        rgba,
        width,
        height,
        tooltip: tooltip.to_string(),
    });
    Ok(super::super::menu::TrayIconHandle {
        tx: command_sender().clone(),
    })
}

/// GTK 线程主循环：初始化 GTK → 轮询命令 → 运行 GLib 主循环
fn gtk_thread(rx: Receiver<MenuCommand>) {
    use gtk::prelude::*;

    if let Err(e) = gtk::init() {
        tracing::error!("[托盘] [GTK] 初始化失败，托盘与悬浮球菜单不可用: {e}");
        return;
    }

    let mut menus: HashMap<MenuKind, (MudaMenu, HashMap<String, MenuItem>)> = HashMap::new();
    let mut tray: Option<tray_icon::TrayIcon> = None;
    // 弹出悬浮球菜单的锚点窗口（1x1、无装饰，弹出期间临时显示）
    let mut anchor: Option<gtk::Window> = None;

    let main_loop = gtk::glib::MainLoop::new(None, false);
    let loop_handle = main_loop.clone();
    gtk::glib::timeout_add_local(Duration::from_millis(50), move || {
        while let Ok(cmd) = rx.try_recv() {
            handle_command(cmd, &mut menus, &mut tray, &mut anchor);
        }
        let _ = &loop_handle;
        gtk::glib::ControlFlow::Continue
    });
    main_loop.run();
    unreachable!("GTK 主循环不会主动退出");
}

fn handle_command(
    cmd: MenuCommand,
    menus: &mut HashMap<MenuKind, (MudaMenu, HashMap<String, MenuItem>)>,
    tray: &mut Option<tray_icon::TrayIcon>,
    anchor: &mut Option<gtk::Window>,
) {
    use gtk::prelude::*;
    use tray_icon::menu::ContextMenu;

    match cmd {
        MenuCommand::BuildMenu(kind, items, separator_after) => {
            let ordered: Vec<(String, MenuItem)> = items
                .into_iter()
                .map(|(id, text, enabled)| (id.clone(), MenuItem::with_id(id, text, enabled, None)))
                .collect();
            let separators: Vec<PredefinedMenuItem> = separator_after
                .iter()
                .map(|_| PredefinedMenuItem::separator())
                .collect();
            let mut list: Vec<&dyn IsMenuItem> = Vec::new();
            for (idx, (_, item)) in ordered.iter().enumerate() {
                list.push(item);
                if let Some(sep) = separators.get(idx) {
                    list.push(sep);
                }
            }
            match MudaMenu::with_items(&list) {
                Ok(menu) => {
                    menus.insert(kind, (menu, ordered.into_iter().collect()));
                }
                Err(e) => tracing::error!("[托盘] [GTK] 构建菜单 {kind:?} 失败: {e}"),
            }
        }
        MenuCommand::AttachTray {
            rgba,
            width,
            height,
            tooltip,
        } => {
            let Some((menu, _)) = menus.get(&MenuKind::Tray) else {
                tracing::error!("[托盘] [GTK] 托盘菜单尚未构建，无法创建托盘图标");
                return;
            };
            match tray_icon::Icon::from_rgba(rgba, width, height) {
                Ok(icon) => match tray_icon::TrayIconBuilder::new()
                    .with_menu(Box::new(menu.clone()))
                    .with_tooltip(&tooltip)
                    .with_icon(icon)
                    .build()
                {
                    Ok(created) => *tray = Some(created),
                    Err(e) => tracing::error!("[托盘] [GTK] 创建托盘图标失败: {e}"),
                },
                Err(e) => tracing::error!("[托盘] [GTK] 托盘图标解码失败: {e}"),
            }
        }
        MenuCommand::SetText(kind, id, text) => {
            if let Some((_, items)) = menus.get_mut(&kind)
                && let Some(item) = items.get(&id)
            {
                item.set_text(text);
            }
        }
        MenuCommand::SetEnabled(kind, id, enabled) => {
            if let Some((_, items)) = menus.get_mut(&kind)
                && let Some(item) = items.get(&id)
            {
                item.set_enabled(enabled);
            }
        }
        MenuCommand::SetTooltip(text) => {
            if let Some(tray) = tray {
                tray.set_tooltip(Some(&text)).ok();
            }
        }
        MenuCommand::PopupBall => {
            let Some((menu, _)) = menus.get(&MenuKind::Ball) else {
                tracing::warn!("[悬浮球] [GTK] 悬浮球菜单尚未构建，弹出失败");
                return;
            };
            // 锚点窗口必须已 realize（存在 GdkWindow），菜单才能在鼠标处弹出
            let window = anchor.get_or_insert_with(new_anchor_window);
            if let Some((x, y)) = pointer_position() {
                window.move_(x, y);
            }
            window.show_all();
            let shown = menu.show_context_menu_for_gtk_window(window, None);
            window.hide();
            if !shown {
                tracing::warn!("[悬浮球] [GTK] 弹出菜单失败");
            }
        }
    }
}

/// 弹出锚点：1x1 无装饰的临时窗口
fn new_anchor_window() -> gtk::Window {
    gtk::Window::builder()
        .type_(gtk::WindowType::Popup)
        .decorated(false)
        .default_width(1)
        .default_height(1)
        .skip_taskbar_hint(true)
        .accept_focus(false)
        .build()
}

fn pointer_position() -> Option<(i32, i32)> {
    use gtk::prelude::*;
    let display = gtk::gdk::Display::default()?;
    let seat = display.default_seat()?;
    let pointer = seat.pointer()?;
    let (_, x, y) = pointer.position();
    Some((x, y))
}
