// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::settings::SettingsMessage;
use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Element;

/// 创建数据配置区块
pub fn create_data_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    super::create_config_section(
        "\u{F412}", // hdd
        app.i18n.t("settings.category-data"),
        vec![
            super::create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.data-path"),
                app.i18n.t("settings.data-path-desc"),
                &helpers::get_absolute_path(&app.config.data.data_path),
                super::PathRowActions {
                    select: SettingsMessage::DataPathSelected("SELECT_DATA_PATH".to_string())
                        .into(),
                    open: SettingsMessage::OpenPath("data".to_string()).into(),
                    clear: SettingsMessage::ShowPathClearConfirmation("data".to_string()).into(),
                    restore: SettingsMessage::RestoreDefaultPath("data".to_string()).into(),
                },
                theme_colors,
            ),
            super::create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.cache-path"),
                app.i18n.t("settings.cache-path-desc"),
                &helpers::get_absolute_path(&app.config.data.cache_path),
                super::PathRowActions {
                    select: SettingsMessage::CachePathSelected("SELECT_CACHE_PATH".to_string())
                        .into(),
                    open: SettingsMessage::OpenPath("cache".to_string()).into(),
                    clear: SettingsMessage::ShowPathClearConfirmation("cache".to_string()).into(),
                    restore: SettingsMessage::RestoreDefaultPath("cache".to_string()).into(),
                },
                theme_colors,
            ),
            super::create_logs_path_row(
                &app.i18n,
                app.i18n.t("settings.logs-path"),
                app.i18n.t("settings.logs-path-desc"),
                theme_colors,
            ),
        ],
        &app.theme_config,
    )
}
