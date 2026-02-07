// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{BUTTON_COLOR_BLUE, BUTTON_SPACING, INPUT_PADDING, TEXT_INPUT_SIZE};
use crate::ui::{App, AppMessage};
use iced::border::{Border, Radius};
use iced::widget::{Space, container, row, text_input};
use iced::{Alignment, Color, Element, Length};

/// 创建API配置区块
pub fn create_api_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    super::create_config_section(
        app.i18n.t("settings.api-config"),
        vec![super::create_setting_row(
            app.i18n.t("settings.wallhaven-api-key"),
            row![
                text_input(
                    &app.i18n.t("settings.wallhaven-api-key-placeholder"),
                    &app.settings_state.wallhaven_api_key
                )
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(|s| SettingsMessage::WallhavenApiKeyChanged(s).into())
                .padding(INPUT_PADDING)
                .style(move |_theme: &iced::Theme, _status| text_input::Style {
                    background: iced::Background::Color(theme_colors.text_input_background),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::from(4.0),
                    },
                    icon: theme_colors.light_text_sub,
                    placeholder: theme_colors.light_text_sub,
                    value: theme_colors.light_text,
                    selection: theme_colors.text_input_selection_color,
                }),
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
