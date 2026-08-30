// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 系统托盘：图标 + 6 项菜单（显示主窗口/上一张/下一张/保存当前/设置/退出）
//!
//! 菜单操作经由 `platform::menu` 跨平台封装：
//! Windows/macOS 主线程直连，Linux 在专用 GTK 线程中执行。
//! 菜单激活事件经 muda 全局 channel 流入 subscription 轮询。

use crate::i18n;
use crate::platform::menu::{self, MenuItemDef, MenuKind};
use crate::utils::assets;
use std::collections::HashMap;

/// 菜单项定义：(菜单事件 id, i18n key, 初始可用状态)
const TRAY_ITEMS: [(&str, &str, bool); 6] = [
    ("tray_show", "menu.tray-show", true),
    ("tray_switch_previous", "menu.tray-switch-previous", false),
    ("tray_switch_next", "menu.tray-switch-next", true),
    ("tray_save_current", "menu.tray-save-current", true),
    ("tray_settings", "menu.tray-settings", true),
    ("tray_quit", "menu.tray-quit", true),
];

pub struct TrayManager {
    tray_icon: menu::TrayIconHandle,
    menu: menu::Menu,
    /// 菜单事件 id → i18n key（语言切换时刷新文本）
    kv: HashMap<String, String>,
}

impl TrayManager {
    pub fn new(i18n: &i18n::I18n) -> Self {
        let items: Vec<MenuItemDef> = TRAY_ITEMS
            .iter()
            .map(|(id, key, enabled)| MenuItemDef {
                id,
                text: i18n.t(key),
                enabled: *enabled,
            })
            .collect();
        let kv: HashMap<String, String> = TRAY_ITEMS
            .iter()
            .map(|(id, key, _)| (id.to_string(), key.to_string()))
            .collect();

        // 分隔线：显示主窗口之后、保存当前之后
        let menu = menu::build_menu(MenuKind::Tray, items, &[0, 3]);

        let (rgba, width, height) = assets::get_logo(32);
        let tray_icon = menu::attach_tray(rgba, width, height, menu.clone(), &i18n.t("app-title"))
            .unwrap_or_else(|e| {
                tracing::error!("[托盘] {e}");
                menu::disabled_tray()
            });

        Self {
            tray_icon,
            menu,
            kv,
        }
    }

    pub fn update_switch_previous_item(&mut self, history_count: usize) {
        self.menu
            .set_enabled("tray_switch_previous", history_count >= 2);
    }

    pub fn update_save_current_item(&mut self, can_save: bool) {
        self.menu.set_enabled("tray_save_current", can_save);
    }

    pub fn update_i18n(&mut self, i18n: &i18n::I18n) {
        for (id, lang_key) in self.kv.iter() {
            self.menu.set_text(id, i18n.t(lang_key));
        }
        self.tray_icon.set_tooltip(&i18n.t("app-title"));
    }
}
