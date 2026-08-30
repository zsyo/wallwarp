// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{IMAGE_HEIGHT, IMAGE_WIDTH, LOADING_TEXT_SIZE};
use iced::widget::{button, container, text};
use iced::{Alignment, Length};

/// 创建加载占位符
pub(in crate::ui::local) fn create_loading_placeholder<'a>(
    i18n: &'a I18n,
    theme_config: &'a ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let loading_text = text(i18n.t("local-list.image-loading"))
        .size(LOADING_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let placeholder_content = container(loading_text)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(common::wallpaper_image_container_style(theme_colors));

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(common::wallpaper_card_button_style(theme_colors))
}
