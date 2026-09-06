// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::OnlineWallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::wallpaper_card;
use crate::ui::online::OnlineMessage;
use crate::ui::style::*;
use crate::utils::helpers;
use iced::widget::{container, text, tooltip};
use iced::{Alignment, Element, Length};

/// 创建已加载的壁纸卡片
pub fn create_loaded_wallpaper_with_thumb<'a>(
    i18n: &'a I18n,
    wallpaper: &'a OnlineWallpaper,
    index: usize,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 使用缓存的 image_handle
    let image = if let Some(handle) = wallpaper.image_handle.clone() {
        iced::widget::image(handle)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .content_fit(iced::ContentFit::Fill)
    } else {
        // 如果没有缓存的 Handle，使用占位符
        let placeholder = text(i18n.t("online-wallpapers.loading-placeholder"))
            .size(LOADING_TEXT_SIZE)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            });

        return container(placeholder)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_theme| {
                common::create_bordered_container_style_with_bg(theme_config)(_theme)
            })
            .into();
    };

    let styled_image = container(image)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .clip(true)
        .style(common::wallpaper_image_container_style(theme_colors));

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}", // image-fill
            BUTTON_COLOR_GREEN,
            OnlineMessage::SetAsWallpaper(index).into(),
        ),
        i18n.t("online-wallpapers.tooltip-set-wallpaper"),
        tooltip::Position::Top,
        theme_config,
    );

    let download_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F30A}", // download
            BUTTON_COLOR_BLUE,
            OnlineMessage::DownloadWallpaper(index).into(),
        ),
        i18n.t("online-wallpapers.tooltip-download"),
        tooltip::Position::Top,
        theme_config,
    );

    wallpaper_card(
        styled_image.into(),
        helpers::format_file_size(wallpaper.file_size),
        Some(wallpaper.resolution.clone()),
        vec![set_wallpaper_button, download_button],
        4.0,
        Some(OnlineMessage::ShowModal(index).into()),
        theme_colors,
    )
}
