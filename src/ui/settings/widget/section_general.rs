// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ROW_SPACING;
use crate::ui::{App, AppMessage};
use crate::utils::config::CloseAction;
use crate::utils::startup;
use iced::widget::{radio, row, toggler};
use iced::{Alignment, Element};

/// 创建通用配置区块
pub fn create_general_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let mut rows = vec![
        super::create_setting_row(
            app.i18n.t("settings.app-language"),
            Some(app.i18n.t("settings.app-language-desc")),
            super::create_language_picker(app),
            theme_colors,
        ),
        super::create_setting_row(
            app.i18n.t("settings.theme-color"),
            Some(app.i18n.t("settings.theme-color-desc")),
            super::create_theme_picker(app),
            theme_colors,
        ),
        super::create_setting_row(
            app.i18n.t("settings.auto-startup"),
            Some(app.i18n.t("settings.auto-startup-desc")),
            toggler(startup::is_auto_startup_enabled())
                .on_toggle(|state| SettingsMessage::AutoStartupToggled(state).into()),
            theme_colors,
        ),
    ];
    // Wayland 会话下窗口定位/置顶受限，悬浮球不可用，隐藏该设置项
    if crate::platform::supports_floating_ball() {
        rows.push(super::create_setting_row(
            app.i18n.t("settings.show-floating-ball"),
            Some(app.i18n.t("settings.show-floating-ball-desc")),
            toggler(app.config.global.show_floating_ball)
                .on_toggle(|state| SettingsMessage::FloatingBallToggled(state).into()),
            theme_colors,
        ));
    }
    rows.push(super::create_setting_row(
        app.i18n.t("settings.close-action"),
        Some(app.i18n.t("settings.close-action-desc")),
        row![
            radio(
                app.i18n.t("close-action-options.ask"),
                CloseAction::Ask,
                Some(app.config.global.close_action),
                |act| SettingsMessage::CloseActionSelected(act).into()
            )
            .style(common::radio_transparent_style(theme_colors.text)),
            radio(
                app.i18n.t("close-action-options.minimize-to-tray"),
                CloseAction::MinimizeToTray,
                Some(app.config.global.close_action),
                |act| SettingsMessage::CloseActionSelected(act).into()
            )
            .style(common::radio_transparent_style(theme_colors.text)),
            radio(
                app.i18n.t("close-action-options.close-app"),
                CloseAction::CloseApp,
                Some(app.config.global.close_action),
                |act| SettingsMessage::CloseActionSelected(act).into()
            )
            .style(common::radio_transparent_style(theme_colors.text)),
        ]
        .spacing(ROW_SPACING),
        theme_colors,
    ));
    rows.push(super::create_setting_row(
        app.i18n.t("settings.enable-logging"),
        Some(app.i18n.t("settings.enable-logging-desc")),
        row![
            toggler(app.config.global.enable_logging)
                .on_toggle(|state| SettingsMessage::LoggingToggled(state).into()),
            super::create_log_level_picker(app),
        ]
        .spacing(ROW_SPACING)
        .align_y(Alignment::Center),
        theme_colors,
    ));

    super::create_config_section(
        "\u{F56B}", // sliders
        app.i18n.t("settings.category-general"),
        rows,
        &app.theme_config,
    )
}
