// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::{IMAGE_HEIGHT, IMAGE_WIDTH, LOADING_TEXT_SIZE};
use crate::ui::style::ThemeConfig;
use crate::ui::{common, style};
use iced::widget::{button, container, text};
use iced::{Alignment, Length};

/// 创建加载占位符
pub(in crate::ui::local) fn create_loading_placeholder<'a>(
    i18n: &'a I18n,
    theme_config: &'a ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let loading_text =
        text(i18n.t("local-list.image-loading"))
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
            let shadow = style::get_card_shadow_by_status(matches!(status, button::Status::Hovered));
            button::Style { shadow, ..base_style }
        })
}
