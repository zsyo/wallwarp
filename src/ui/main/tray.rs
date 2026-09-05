// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 系统托盘：图标 + 6 项菜单（显示主窗口/上一张/下一张/保存当前/设置/退出）
//!
//! 菜单操作经由 `platform::menu` 跨平台封装：
//! Windows/macOS 主线程直连，Linux 在专用 GTK 线程中执行。
//! 菜单激活事件经 muda 全局 channel 流入 subscription 轮询。
//! 菜单项定义与悬浮球菜单共享（见 [`crate::ui::main::menu_defs`]）。

use crate::i18n::I18n;
use crate::platform::menu::{self, MenuKind};
use crate::ui::main::menu_defs;
use crate::utils::assets;
use std::collections::HashMap;

pub struct TrayManager {
    tray_icon: menu::TrayIconHandle,
    menu: menu::Menu,
    /// 菜单事件 id → i18n key（语言切换时刷新文本）
    kv: HashMap<String, String>,
}

impl TrayManager {
    pub fn new(i18n: &I18n) -> Self {
        // 5 个公共动作项 + 退出程序
        let (items, kv) =
            menu_defs::build_menu_defs(menu_defs::TRAY_ID_PREFIX, ("tray_quit", "menu.tray-quit"), i18n);

        let menu = menu::build_menu(MenuKind::Tray, items, menu_defs::MENU_SEPARATOR_AFTER);

        let (rgba, width, height) = assets::get_logo(32);
        let tray_icon = menu::attach_tray(rgba, width, height, menu.clone(), &i18n.t("app-title"))
            .unwrap_or_else(|e| {
                tracing::error!("[托盘] 附加托盘图标失败: {e}");
                menu::disabled_tray()
            });

        Self {
            tray_icon,
            menu,
            kv,
        }
    }

    pub fn update_switch_previous_item(&mut self, history_count: usize) {
        let id = menu_defs::menu_item_id(
            menu_defs::TRAY_ID_PREFIX,
            menu_defs::MenuAction::SwitchPrevious,
        );
        self.menu.set_enabled(&id, history_count >= 2);
    }

    pub fn update_save_current_item(&mut self, can_save: bool) {
        let id = menu_defs::menu_item_id(
            menu_defs::TRAY_ID_PREFIX,
            menu_defs::MenuAction::SaveCurrent,
        );
        self.menu.set_enabled(&id, can_save);
    }

    pub fn update_i18n(&mut self, i18n: &I18n) {
        for (id, lang_key) in self.kv.iter() {
            self.menu.set_text(id, i18n.t(lang_key));
        }
        self.tray_icon.set_tooltip(&i18n.t("app-title"));
    }
}
