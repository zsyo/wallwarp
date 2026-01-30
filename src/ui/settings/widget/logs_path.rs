// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::{BUTTON_COLOR_GREEN, BUTTON_SPACING, INPUT_HEIGHT, INPUT_PADDING, ROW_SPACING, TEXT_INPUT_SIZE};
use crate::utils::helpers;
use iced::border::{Border, Radius};
use iced::widget::{Space, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

pub fn create_logs_path_row<'a>(i18n: &I18n, label: String, theme_colors: ThemeColors) -> Element<'a, AppMessage> {
    let logs_path = helpers::get_absolute_path("logs");

    row![
        text(label)
            .width(Length::FillPortion(1))
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        row![
            text_input("", &logs_path)
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
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
                i18n.t("settings.open-path"),
                BUTTON_COLOR_GREEN,
                SettingsMessage::OpenLogsPath.into()
            )
            .width(Length::Fixed(210.0)),
        ]
        .width(Length::FillPortion(4))
        .spacing(0),
    ]
    .height(Length::Fixed(INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}
