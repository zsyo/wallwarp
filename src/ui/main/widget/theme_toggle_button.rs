// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::main::MainMessage;
use crate::ui::style::BUTTON_COLOR_YELLOW;
use crate::ui::style::ThemeColors;
use crate::ui::{App, AppMessage};
use crate::utils::config::Theme;
use iced::border::{Border, Radius};
use iced::widget::{button, text, tooltip};
use iced::{Element, Length};

/// 创建主题切换按钮
pub fn create_theme_toggle_button(app: &App) -> Element<'_, AppMessage> {
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());

    let (icon_char, tooltip_text, target_theme) = if app.theme_config.is_dark() {
        ("\u{F5A1}", app.i18n.t("theme.switch-to-light"), Theme::Light)
    } else {
        ("\u{F494}", app.i18n.t("theme.switch-to-dark"), Theme::Dark)
    };

    let btn = button(
        text(icon_char)
            .color(BUTTON_COLOR_YELLOW)
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(20),
    )
    .on_press(MainMessage::ThemeSelected(target_theme).into())
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .style(move |_theme: &iced::Theme, _status| button::Style {
        background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
        text_color: theme_colors.text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(20.0),
        },
        ..Default::default()
    });

    common::create_button_with_tooltip(btn, tooltip_text, tooltip::Position::Top, &app.theme_config)
}
