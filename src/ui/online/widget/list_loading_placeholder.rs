// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::style::*;
use iced::widget::{button, container, text};
use iced::{Alignment, Element, Length};

/// 创建加载占位符
pub fn create_loading_placeholder<'a>(i18n: &'a I18n, theme_config: &'a ThemeConfig) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    let loading_text = text(i18n.t("online-wallpapers.image-loading"))
        .size(LOADING_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let placeholder_content = container(loading_text)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_theme| {
            let mut style = common::create_bordered_container_style_with_bg(theme_config)(_theme);
            // 添加阴影效果
            style.shadow = iced::Shadow {
                color: theme_colors.overlay_bg,
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 8.0,
            };
            style
        });

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(|_theme, status| {
            let base_style = button::text(_theme, status);
            let shadow = get_card_shadow_by_status(matches!(status, button::Status::Hovered));
            button::Style { shadow, ..base_style }
        })
        .into()
}
