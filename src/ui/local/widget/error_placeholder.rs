// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::wallpaper_card;
use crate::ui::local::LocalMessage;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{
    BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, ERROR_ICON_SIZE, ERROR_PATH_SIZE, ERROR_TEXT_SIZE,
    IMAGE_HEIGHT, IMAGE_WIDTH,
};
use crate::utils::helpers;
use iced::widget::{column, container, text, tooltip};
use iced::{Alignment, Element, Font, Length};

/// 创建错误占位符
pub fn create_error_placeholder<'a>(
    i18n: &'a I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let error_image = text("\u{F428}") // image-alt
        .font(Font::with_name("bootstrap-icons"))
        .color(theme_colors.disabled_color)
        .size(ERROR_ICON_SIZE);

    let error_text = text(i18n.t("local-list.loading-error"))
        .size(ERROR_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let error_path =
        text(&wallpaper.path)
            .size(ERROR_PATH_SIZE)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            });

    let error_content = container(
        column![error_image, error_text, error_path]
            .width(Length::Fill)
            .align_x(Alignment::Center),
    )
    .width(Length::Fixed(IMAGE_WIDTH))
    .height(Length::Fixed(IMAGE_HEIGHT))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(common::wallpaper_image_container_style(theme_colors));

    let view_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F3D8}", // folder2-open
            BUTTON_COLOR_YELLOW,
            LocalMessage::ViewInFolder(index).into(),
        ),
        i18n.t("local-list.tooltip-locate"),
        tooltip::Position::Top,
        theme_config,
    );

    let delete_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F78B}", // trash3
            BUTTON_COLOR_RED,
            LocalMessage::ShowDeleteConfirm(index).into(),
        ),
        i18n.t("local-list.tooltip-delete"),
        tooltip::Position::Top,
        theme_config,
    );

    // 失败卡片不显示分辨率，也不响应点击
    wallpaper_card(
        error_content.into(),
        helpers::format_file_size(wallpaper.file_size),
        None,
        vec![view_button, delete_button],
        2.0,
        None,
        theme_colors,
    )
}
