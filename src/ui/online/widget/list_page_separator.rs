// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::*;
use iced::widget::{container, text};
use iced::{Alignment, Element, Length};

/// 创建分页分隔线
pub fn create_page_separator<'a>(i18n: &'a I18n, current_page: usize, total_pages: usize) -> Element<'a, AppMessage> {
    let page_text = i18n
        .t("online-wallpapers.page-separator")
        .replace("{current}", &current_page.to_string())
        .replace("{total}", &total_pages.to_string());

    let separator = container(
        text(page_text)
            .size(PAGE_SEPARATOR_TEXT_SIZE)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(PAGE_SEPARATOR_TEXT_COLOR),
            }),
    )
    .width(Length::Fill)
    .height(Length::Fixed(PAGE_SEPARATOR_HEIGHT))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    container(separator).width(Length::Fill).padding([0, 20]).into()
}
