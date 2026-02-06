// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::AppMessage;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ROW_SPACING;
use crate::ui::style::ThemeColors;
use iced::widget::{button, row, text};
use iced::{Alignment, Element, Length};

pub fn create_about_link_row<'a>(
    label: String,
    text_value: &'a str,
    url: &'a str,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    row![
        text(label).style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        }),
        button(text(text_value).width(Length::Fill).align_x(Alignment::Center))
            .padding(0)
            .style(move |_theme: &iced::Theme, _status| {
                let palette = _theme.extended_palette();
                button::Style {
                    text_color: palette.primary.base.color,
                    ..button::text(_theme, _status)
                }
            })
            .on_press(SettingsMessage::OpenUrl(url.to_string()).into()),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .spacing(ROW_SPACING)
    .into()
}
