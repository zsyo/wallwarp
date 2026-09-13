// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 定时切换"按主题分组选图"设置行（开关 + 浅色/深色选图目录）

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{ROW_SPACING, SETTING_ROW_DESC_SIZE};
use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::widget::{checkbox, row, text};
use iced::{Alignment, Element, Length};

/// 创建"按主题分组选图"设置行组（开关 + 浅色/深色目录选择）
pub fn create_theme_aware_rows<'a>(app: &'a App) -> Vec<Element<'a, AppMessage>> {
    vec![
        super::create_setting_row(
            app.i18n.t("settings.auto-change-theme-aware"),
            Some(app.i18n.t("settings.auto-change-theme-aware-desc")),
            create_theme_aware_toggle(app),
            app.theme_colors,
        ),
        create_theme_dir_row(app, "light"),
        create_theme_dir_row(app, "dark"),
    ]
}

/// 创建"按主题分组选图"开关（启停状态持久化到配置）
fn create_theme_aware_toggle<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let is_enabled = app.settings_state.auto_change_theme_aware;

    checkbox(is_enabled)
        .size(16)
        .on_toggle(move |_state| SettingsMessage::AutoChangeThemeAwareToggled(!is_enabled).into())
        .style(common::checkbox_style(theme_colors, is_enabled))
        .into()
}

/// 创建主题分组选图目录行（dir_type 为 "light"/"dark"）
///
/// 展示当前生效目录（空目录显示回退说明），提供选择/查看/清空操作
fn create_theme_dir_row<'a>(app: &'a App, dir_type: &'static str) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let (label_key, desc_key) = if dir_type == "light" {
        (
            "settings.auto-change-theme-light-dir",
            "settings.auto-change-theme-light-dir-desc",
        )
    } else {
        (
            "settings.auto-change-theme-dark-dir",
            "settings.auto-change-theme-dark-dir-desc",
        )
    };

    let current_dir = theme_dir(app, dir_type);
    let dir_display: String = if current_dir.is_empty() {
        app.i18n.t("settings.auto-change-theme-dir-fallback")
    } else {
        helpers::get_absolute_path(current_dir)
    };

    super::create_setting_row(
        app.i18n.t(label_key),
        Some(app.i18n.t(desc_key)),
        row![
            text(dir_display)
                .size(SETTING_ROW_DESC_SIZE)
                .color(theme_colors.light_text_sub)
                .width(Length::Fill),
            common::create_icon_button_with_tooltip(
                "\u{F3D8}", // folder2-open (选择路径)
                theme_colors.primary,
                SettingsMessage::AutoChangeThemeDirSelected(dir_type.to_string(), "SELECT".into())
                    .into(),
                app.i18n.t("settings.select-path"),
                theme_colors,
            ),
            common::create_icon_button_with_tooltip(
                "\u{F1C5}", // box-arrow-up-right (打开路径)
                theme_colors.primary,
                SettingsMessage::OpenPath(format!("auto-change-{}", dir_type)).into(),
                app.i18n.t("settings.open-path"),
                theme_colors,
            ),
            common::create_icon_button_with_tooltip(
                "\u{F78B}", // trash3 (清空路径)
                theme_colors.notification_error_bg,
                SettingsMessage::AutoChangeThemeDirCleared(dir_type.to_string()).into(),
                app.i18n.t("settings.clear-path"),
                theme_colors,
            ),
        ]
        .spacing(ROW_SPACING / 2.0)
        .align_y(Alignment::Center),
        theme_colors,
    )
}

/// 读取主题分组选图目录配置（dir_type 为 "light"/"dark"）
fn theme_dir<'a>(app: &'a App, dir_type: &str) -> &'a str {
    match dir_type {
        "light" => &app.config.wallpaper.auto_change_light_dir,
        _ => &app.config.wallpaper.auto_change_dark_dir,
    }
}
