// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n;
use crate::utils::assets;
use std::collections::HashMap;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem, PredefinedMenuItem},
};

pub struct TrayManager {
    tray_icon: TrayIcon,
    kv: HashMap<String, String>,
    items: HashMap<String, MenuItem>,
}

impl TrayManager {
    pub fn new(i18n: &i18n::I18n) -> Self {
        let mut kv = HashMap::new();
        let mut items = HashMap::new();

        let (key, val) = ("tray_show", "menu.tray-show");
        let show_item = MenuItem::with_id(key, i18n.t(val), true, None);
        kv.insert(key.to_string(), val.to_string());
        items.insert(key.to_string(), show_item.clone());

        let (key, val) = ("tray_switch_previous", "menu.tray-switch-previous");
        let switch_previous_item = MenuItem::with_id(key, i18n.t(val), false, None);
        kv.insert(key.to_string(), val.to_string());
        items.insert(key.to_string(), switch_previous_item.clone());

        let (key, val) = ("tray_switch_next", "menu.tray-switch-next");
        let switch_next_item = MenuItem::with_id(key, i18n.t(&val), true, None);
        kv.insert(key.to_string(), val.to_string());
        items.insert(key.to_string(), switch_next_item.clone());

        let (key, val) = ("tray_save_current", "menu.tray-save-current");
        let save_current_item = MenuItem::with_id(key, i18n.t(&val), true, None);
        kv.insert(key.to_string(), val.to_string());
        items.insert(key.to_string(), save_current_item.clone());

        let (key, val) = ("tray_settings", "menu.tray-settings");
        let show_settings_item = MenuItem::with_id(key, i18n.t(val), true, None);
        kv.insert(key.to_string(), val.to_string());
        items.insert(key.to_string(), show_settings_item.clone());

        let (key, val) = ("tray_quit", "menu.tray-quit");
        let quit_item = MenuItem::with_id(key, i18n.t(&val), true, None);
        kv.insert(key.to_string(), val.to_string());
        items.insert(key.to_string(), quit_item.clone());

        let tray_menu = Menu::with_items(&[
            &show_item,
            &PredefinedMenuItem::separator(),
            &switch_previous_item,
            &switch_next_item,
            &save_current_item,
            &PredefinedMenuItem::separator(),
            &show_settings_item,
            &quit_item,
        ])
        .unwrap();

        let (rgba, width, height) = assets::get_logo(32);
        let icon = Icon::from_rgba(rgba, width, height).expect("生成 Tray 图标失败");

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(i18n.t("app-title"))
            .with_icon(icon) // 需要一个自定义图标加载函数
            .build()
            .unwrap();

        Self { tray_icon, kv, items }
    }

    pub fn update_switch_previous_item(&mut self, history_count: usize) {
        self.items
            .get_mut("tray_switch_previous")
            .unwrap()
            .set_enabled(history_count >= 2);
    }

    pub fn update_save_current_item(&mut self, can_save: bool) {
        self.items.get_mut("tray_save_current").unwrap().set_enabled(can_save);
    }

    pub fn update_i18n(&mut self, i18n: &i18n::I18n) {
        for (id, lang_key) in self.kv.iter() {
            if let Some(item) = self.items.get_mut(id) {
                item.set_text(i18n.t(lang_key));
            }
        }

        self.tray_icon
            .set_tooltip(Some(i18n.t("app-title")))
            .unwrap_or_default();
    }
}
