use super::App;
use crate::i18n;
use crate::utils::assets;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

impl App {
    pub fn init_tray(i18n: &i18n::I18n) -> TrayIcon {
        let tray_menu = Menu::new();
        let show_item = MenuItem::with_id("tray_show", i18n.t("menu.tray-show"), true, None);
        let show_settings_item = MenuItem::with_id("tray_settings", i18n.t("menu.tray-settings"), true, None);
        let quit_item = MenuItem::with_id("tray_quit", i18n.t("menu.tray-quit"), true, None);

        tray_menu.append(&show_item).unwrap();
        tray_menu.append(&show_settings_item).unwrap();
        tray_menu.append(&quit_item).unwrap();

        let (rgba, width, height) = assets::get_logo(32);
        let icon = Icon::from_rgba(rgba, width, height).expect("生成 Tray 图标失败");

        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip(i18n.t("app-title"))
            .with_icon(icon) // 需要一个自定义图标加载函数
            .build()
            .unwrap()
    }
}
