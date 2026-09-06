// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::wallpaper_card;
use crate::ui::local::LocalMessage;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{
    BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, IMAGE_HEIGHT, IMAGE_WIDTH,
};
use crate::utils::helpers;
use iced::widget::image::Handle;
use iced::widget::{container, tooltip};
use iced::{Element, Length};

/// 创建已加载壁纸卡片
pub fn create_loaded_wallpaper<'a>(
    i18n: &'a I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 使用缓存的 image_handle，如果缓存不存在则回退到从路径创建
    let image_handle = wallpaper
        .image_handle
        .clone()
        .unwrap_or_else(|| Handle::from_path(&wallpaper.thumbnail_path));
    let image = iced::widget::image(image_handle)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .content_fit(iced::ContentFit::Fill);

    let styled_image = container(image)
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true)
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

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}", // image-fill
            BUTTON_COLOR_GREEN,
            LocalMessage::SetWallpaper(index).into(),
        ),
        i18n.t("local-list.tooltip-set-wallpaper"),
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

    wallpaper_card(
        styled_image.into(),
        helpers::format_file_size(wallpaper.file_size),
        Some(helpers::format_resolution(
            wallpaper.width,
            wallpaper.height,
        )),
        vec![view_button, set_wallpaper_button, delete_button],
        2.0,
        Some(LocalMessage::ShowModal(index).into()),
        theme_colors,
    )
}
