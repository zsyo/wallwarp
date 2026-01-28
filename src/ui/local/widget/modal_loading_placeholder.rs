// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::COLOR_OVERLAY_TEXT;
use iced::widget::{container, text};
use iced::{Element, Length};

/// 创建模态窗口加载占位符
pub(in crate::ui::local) fn create_modal_loading_placeholder<'a>(i18n: &'a I18n) -> Element<'a, AppMessage> {
    let loading_text = text(i18n.t("local-list.image-loading"))
        .size(24)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    container(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
