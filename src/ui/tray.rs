use crate::i18n;
use crate::utils::assets;
use std::collections::HashMap;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

pub struct TrayManager {
    _tray_icon: TrayIcon,
    items: HashMap<String, MenuItem>,
}

impl TrayManager {
    pub fn new(i18n: &i18n::I18n) -> Self {
        let mut items = HashMap::new();

        let tray_menu = Menu::new();
        let show_item = MenuItem::with_id("tray_show", i18n.t("menu.tray-show"), true, None);
        let switch_previous_item_id = "tray_switch_previous".to_string();
        let switch_previous_item = MenuItem::with_id(&switch_previous_item_id, i18n.t("menu.tray-switch-previous"), false, None);
        items.insert(switch_previous_item_id, switch_previous_item.clone());
        let switch_next_item = MenuItem::with_id("tray_switch_next", i18n.t("menu.tray-switch-next"), true, None);
        let show_settings_item = MenuItem::with_id("tray_settings", i18n.t("menu.tray-settings"), true, None);
        let quit_item = MenuItem::with_id("tray_quit", i18n.t("menu.tray-quit"), true, None);

        tray_menu.append(&show_item).unwrap();
        tray_menu.append(&switch_previous_item).unwrap();
        tray_menu.append(&switch_next_item).unwrap();
        tray_menu.append(&show_settings_item).unwrap();
        tray_menu.append(&quit_item).unwrap();

        let (rgba, width, height) = assets::get_logo(32);
        let icon = Icon::from_rgba(rgba, width, height).expect("生成 Tray 图标失败");

        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(i18n.t("app-title"))
            .with_icon(icon) // 需要一个自定义图标加载函数
            .build()
            .unwrap();

        Self { _tray_icon, items }
    }

    pub fn update_switch_previous_item(&mut self, history_count: usize) {
        self.items.get_mut("tray_switch_previous").unwrap().set_enabled(history_count >= 2);
    }
}
