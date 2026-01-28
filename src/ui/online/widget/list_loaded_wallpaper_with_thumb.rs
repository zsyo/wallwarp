// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::OnlineWallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::OnlineMessage;
use crate::ui::style::*;
use crate::utils::helpers;
use iced::widget::image::Handle;
use iced::widget::{Space, button, container, row, text, tooltip};
use iced::{Alignment, Element, Length};

/// 创建已加载的壁纸卡片
pub fn create_loaded_wallpaper_with_thumb<'a>(
    i18n: &'a I18n,
    wallpaper: &'a OnlineWallpaper,
    thumb_handle: Option<Handle>,
    index: usize,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    // 使用缩略图创建图片
    let image = if let Some(handle) = thumb_handle {
        iced::widget::image(handle)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .content_fit(iced::ContentFit::Fill)
    } else {
        // 如果没有缩略图，使用占位符
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
            .style(move |_theme| common::create_bordered_container_style_with_bg(theme_config)(_theme))
            .into();
    };

    let styled_image = container(image)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
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

    // 创建透明遮罩内容
    let file_size_text = text(helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.overlay_text),
        });

    let resolution_text = text(&wallpaper.resolution)
        .size(OVERLAY_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.overlay_text),
        });

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}",
            BUTTON_COLOR_GREEN,
            AppMessage::Online(OnlineMessage::SetAsWallpaper(index)),
        ),
        i18n.t("online-wallpapers.tooltip-set-wallpaper"),
        tooltip::Position::Top,
        theme_config,
    );

    let download_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F30A}",
            BUTTON_COLOR_BLUE,
            AppMessage::Online(OnlineMessage::DownloadWallpaper(index)),
        ),
        i18n.t("online-wallpapers.tooltip-download"),
        tooltip::Position::Top,
        theme_config,
    );

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：设为壁纸按钮 + 下载按钮
    let right_area = row![set_wallpaper_button, download_button]
        .spacing(4)
        .align_y(Alignment::Center);

    // 使用 stack 确保分辨率永远居中，不受两侧内容影响
    let overlay_content = iced::widget::stack(vec![
        // 底层：左中右三部分布局
        container(
            row![
                left_area,
                // 中间占位，让分辨率在顶层居中
                container(Space::new()).width(Length::Fill),
                right_area,
            ]
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .padding([0, 8])
        .into(),
        // 顶层：分辨率居中显示
        container(resolution_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ]);

    // 创建遮罩层
    let overlay = container(overlay_content)
        .width(Length::Fill)
        .height(Length::Fixed(OVERLAY_HEIGHT))
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_OVERLAY_BG)),
            ..Default::default()
        });

    // 使用 stack 将遮罩覆盖在图片内部下方
    let card_content = iced::widget::stack(vec![
        styled_image.into(),
        container(overlay)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::End)
            .into(),
    ]);

    button(card_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .on_press(AppMessage::Online(OnlineMessage::ShowModal(index)))
        .style(|_theme, status| {
            let base_style = button::text(_theme, status);
            let shadow = get_card_shadow_by_status(matches!(status, button::Status::Hovered));
            button::Style { shadow, ..base_style }
        })
        .into()
}
