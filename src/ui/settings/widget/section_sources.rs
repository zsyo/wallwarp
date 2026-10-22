// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{BUTTON_SPACING, INPUT_PADDING, TEXT_INPUT_SIZE};
use crate::ui::{App, AppMessage};
use iced::widget::{row, text_input};
use iced::{Alignment, Element, Length};

/// 创建图源配置区块
///
/// 管理各图源的 API 密钥（当前仅 Wallhaven 需要密钥；wallpaperscraft 与
/// wp.upx8 免密钥）。内容为空时强制显示以便输入；隐藏状态下输入框展示
/// 脱敏内容且不可编辑，点击眼睛图标切换显隐
pub fn create_sources_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let wallhaven_visible = app.settings_state.wallhaven_api_key_visible
        || app.settings_state.wallhaven_api_key.is_empty();
    let wallhaven_display = if wallhaven_visible {
        &app.settings_state.wallhaven_api_key
    } else {
        &app.settings_state.wallhaven_api_key_masked
    };
    let wallhaven_input = text_input(
        &app.i18n.t("settings.wallhaven-api-key-placeholder"),
        wallhaven_display,
    )
    .width(Length::Fill)
    .size(TEXT_INPUT_SIZE)
    .align_x(Alignment::Center)
    .padding(INPUT_PADDING)
    .style(styled_text_input(theme_colors));
    let wallhaven_input = if wallhaven_visible {
        wallhaven_input.on_input(|s| SettingsMessage::WallhavenApiKeyChanged(s).into())
    } else {
        wallhaven_input
    };

    let rows = vec![super::create_setting_row(
        app.i18n.t("settings.wallhaven-api-key"),
        Some(app.i18n.t("settings.wallhaven-api-key-desc")),
        row![
            wallhaven_input,
            common::create_icon_button(
                if wallhaven_visible {
                    "\u{F340}" // eye-slash (点击隐藏)
                } else {
                    "\u{F341}" // eye (点击显示)
                },
                theme_colors.primary,
                SettingsMessage::ToggleWallhavenApiKeyVisible.into(),
            ),
            common::create_colored_button(
                app.i18n.t("settings.get-api-key"),
                crate::ui::style::BUTTON_COLOR_GREEN,
                SettingsMessage::OpenUrl("https://wallhaven.cc/settings/account".to_string())
                    .into(),
            ),
            common::create_colored_button(
                app.i18n.t("settings.save"),
                crate::ui::style::BUTTON_COLOR_BLUE,
                SettingsMessage::SaveWallhavenApiKey.into()
            )
        ]
        .spacing(BUTTON_SPACING),
        theme_colors,
    )];

    super::create_config_section(
        "\u{F42B}", // images
        app.i18n.t("settings.category-sources"),
        rows,
        &app.theme_config,
    )
}
