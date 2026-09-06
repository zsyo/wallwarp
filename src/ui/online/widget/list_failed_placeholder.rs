// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{ERROR_ICON_SIZE, ERROR_TEXT_SIZE, IMAGE_HEIGHT, IMAGE_WIDTH};
use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Font, Length};

/// 创建缩略图加载失败占位卡片
pub fn create_load_failed_placeholder<'a>(
    i18n: &'a I18n,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let error_icon = text("\u{F428}") // image-alt
        .font(Font::with_name("bootstrap-icons"))
        .color(theme_colors.disabled_color)
        .size(ERROR_ICON_SIZE);

    let error_text = text(i18n.t("online-wallpapers.load-failed"))
        .size(ERROR_TEXT_SIZE)
        .color(theme_colors.text);

    let placeholder_content = container(
        column![error_icon, error_text]
            .spacing(8)
            .align_x(Alignment::Center),
    )
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
        .into()
}
