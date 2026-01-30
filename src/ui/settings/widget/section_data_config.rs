// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Element;

/// 创建数据配置区块
pub fn create_data_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());
    super::create_config_section(
        app.i18n.t("settings.data-config"),
        vec![
            super::create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.data-path"),
                &helpers::get_absolute_path(&app.config.data.data_path),
                SettingsMessage::DataPathSelected("SELECT_DATA_PATH".to_string()).into(),
                SettingsMessage::OpenPath("data".to_string()).into(),
                SettingsMessage::ShowPathClearConfirmation("data".to_string()).into(),
                SettingsMessage::RestoreDefaultPath("data".to_string()).into(),
                theme_colors,
            ),
            super::create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.cache-path"),
                &helpers::get_absolute_path(&app.config.data.cache_path),
                SettingsMessage::CachePathSelected("SELECT_CACHE_PATH".to_string()).into(),
                SettingsMessage::OpenPath("cache".to_string()).into(),
                SettingsMessage::ShowPathClearConfirmation("cache".to_string()).into(),
                SettingsMessage::RestoreDefaultPath("cache".to_string()).into(),
                theme_colors,
            ),
            super::create_logs_path_row(&app.i18n, app.i18n.t("settings.logs-path"), theme_colors),
        ],
        &app.theme_config,
    )
}
