// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::local::LocalMessage;
use crate::ui::style::{
    BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, COLOR_OVERLAY_BG, COLOR_OVERLAY_TEXT, IMAGE_HEIGHT,
    IMAGE_WIDTH, OVERLAY_HEIGHT, OVERLAY_TEXT_SIZE,
};
use crate::ui::style::{ThemeColors, ThemeConfig};
use crate::utils::helpers;
use iced::widget::image::Handle;
use iced::widget::{Space, button, container, row, text, tooltip};
use iced::{Alignment, Color, Length};

/// 创建已加载壁纸卡片
pub fn create_loaded_wallpaper<'a>(
    i18n: &'a I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
    theme_config: &'a ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    let image_handle = Handle::from_path(&wallpaper.thumbnail_path);
    let image = iced::widget::image(image_handle)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .content_fit(iced::ContentFit::Fill);

    let styled_image = container(image)
        .width(Length::Fill)
        .height(Length::Fill)
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

    let resolution_text = text(helpers::format_resolution(wallpaper.width, wallpaper.height))
        .size(OVERLAY_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    let view_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F341}",
            BUTTON_COLOR_YELLOW,
            LocalMessage::ViewInFolder(index).into(),
        ),
        i18n.t("local-list.tooltip-locate"),
        tooltip::Position::Top,
        theme_config,
    );

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F429}", BUTTON_COLOR_GREEN, LocalMessage::SetWallpaper(index).into()),
        i18n.t("local-list.tooltip-set-wallpaper"),
        tooltip::Position::Top,
        theme_config,
    );

    let delete_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F78B}",
            BUTTON_COLOR_RED,
            LocalMessage::ShowDeleteConfirm(index).into(),
        ),
        i18n.t("local-list.tooltip-delete"),
        tooltip::Position::Top,
        theme_config,
    );

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：操作按钮
    let right_area = row![view_button, set_wallpaper_button, delete_button]
        .spacing(2.0)
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
        .style(|_theme, status| {
            let base_style = button::text(_theme, status);
            let shadow = match status {
                button::Status::Hovered => iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                    offset: iced::Vector { x: 0.0, y: 4.0 },
                    blur_radius: 12.0,
                },
                _ => iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: iced::Vector { x: 0.0, y: 2.0 },
                    blur_radius: 8.0,
                },
            };
            button::Style { shadow, ..base_style }
        })
        .on_press(LocalMessage::ShowModal(index).into())
}
