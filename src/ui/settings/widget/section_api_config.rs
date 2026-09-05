// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{BUTTON_COLOR_BLUE, BUTTON_SPACING, INPUT_PADDING, TEXT_INPUT_SIZE};
use crate::ui::{App, AppMessage};
use iced::widget::{Space, container, row, text_input};
use iced::{Alignment, Element, Length};

/// 创建API配置区块
pub fn create_api_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    // 内容为空时强制显示以便输入；隐藏状态下输入框展示脱敏内容且不可编辑，点击眼睛图标切换显隐
    let api_key_visible = app.settings_state.wallhaven_api_key_visible
        || app.settings_state.wallhaven_api_key.is_empty();
    let api_key_display = if api_key_visible {
        &app.settings_state.wallhaven_api_key
    } else {
        &app.settings_state.wallhaven_api_key_masked
    };
    let api_key_input = text_input(
        &app.i18n.t("settings.wallhaven-api-key-placeholder"),
        api_key_display,
    )
    .width(Length::Fill)
    .size(TEXT_INPUT_SIZE)
    .align_x(Alignment::Center)
    .padding(INPUT_PADDING)
    .style(styled_text_input(theme_colors));
    let api_key_input = if api_key_visible {
        api_key_input.on_input(|s| SettingsMessage::WallhavenApiKeyChanged(s).into())
    } else {
        api_key_input
    };

    super::create_config_section(
        app.i18n.t("settings.api-config"),
        vec![super::create_setting_row(
            app.i18n.t("settings.wallhaven-api-key"),
            row![
                api_key_input,
                container(Space::new()).width(Length::Fixed(BUTTON_SPACING)),
                common::create_icon_button(
                    if api_key_visible {
                        "\u{F340}" // eye-slash (点击隐藏)
                    } else {
                        "\u{F341}" // eye (点击显示)
                    },
                    theme_colors.primary,
                    SettingsMessage::ToggleWallhavenApiKeyVisible.into(),
                ),
                container(Space::new()).width(Length::Fixed(BUTTON_SPACING)),
                common::create_colored_button(
                    app.i18n.t("settings.save"),
                    BUTTON_COLOR_BLUE,
                    SettingsMessage::SaveWallhavenApiKey.into()
                )
            ]
            .width(Length::FillPortion(3))
            .spacing(0),
            &app.theme_config,
        )],
        &app.theme_config,
    )
}
