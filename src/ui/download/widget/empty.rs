// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{EMPTY_STATE_PADDING, EMPTY_STATE_TEXT_SIZE};
use iced::widget::{column, text};
use iced::{Alignment, Element, Font, Length};

/// 创建空状态界面
pub fn create_empty_state<'a>(i18n: &'a I18n, theme_config: &'a ThemeConfig) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    let icon = text("\u{F30A}")
        .font(Font::with_name("bootstrap-icons"))
        .size(48.0)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.light_text_sub),
        });

    let empty_text = text(i18n.t("download-tasks.no-tasks"))
        .size(EMPTY_STATE_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let hint_text = text(i18n.t("download-tasks.no-tasks-hint"))
        .size(14)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.light_text_sub),
        });

    column![icon, empty_text, hint_text]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(EMPTY_STATE_PADDING)
        .spacing(10)
        .into()
}
