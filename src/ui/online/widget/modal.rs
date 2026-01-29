// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::widget::{Space, column, container, opaque, row, tooltip};
use iced::{Alignment, Element, Length};

/// 创建图片预览模态窗口
pub fn create_modal<'a>(
    i18n: &'a I18n,
    online_state: &'a OnlineState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let wallpaper_index = online_state.current_image_index;

    // 创建背景加载文字（带进度）
    let loading_text = super::create_modal_loading_placeholder(i18n, online_state);

    // 创建图片层（加载完成后显示）
    let image_layer: Element<_> = if let Some(ref handle) = online_state.modal_image_handle {
        let modal_image = iced::widget::image(handle.clone())
            .content_fit(iced::ContentFit::Contain)
            .width(Length::Fill)
            .height(Length::Fill);
        modal_image.into()
    } else {
        container(Space::new()).width(Length::Fill).height(Length::Fill).into()
    };

    let modal_image_content = iced::widget::stack(vec![loading_text, image_layer]);

    // 创建底部工具栏按钮
    let prev_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F12E}", BUTTON_COLOR_BLUE, OnlineMessage::PreviousImage.into()),
        i18n.t("online-wallpapers.tooltip-prev"),
        tooltip::Position::Top,
        theme_config,
    );

    let next_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F137}", BUTTON_COLOR_BLUE, OnlineMessage::NextImage.into()),
        i18n.t("online-wallpapers.tooltip-next"),
        tooltip::Position::Top,
        theme_config,
    );

    // 设置为壁纸按钮：仅在图片下载完成时可点击
    let set_wallpaper_enabled = online_state.modal_image_handle.is_some();
    let set_wallpaper_button = if set_wallpaper_enabled {
        common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F429}",
                BUTTON_COLOR_GREEN,
                OnlineMessage::SetAsWallpaperFromCache(wallpaper_index).into(),
            ),
            i18n.t("online-wallpapers.tooltip-set-wallpaper"),
            tooltip::Position::Top,
            theme_config,
        )
    } else {
        // 禁用状态的设置为壁纸按钮
        let disabled_button = common::create_icon_button("\u{F429}", BUTTON_COLOR_GRAY, AppMessage::None);
        container(disabled_button)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.7,
                    g: 0.7,
                    b: 0.7,
                    a: 0.5,
                })),
                ..Default::default()
            })
            .into()
    };

    // 下载按钮：仅在图片下载完成时可点击
    let download_enabled = online_state.modal_image_handle.is_some();
    let download_button = if download_enabled {
        common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F30A}",
                BUTTON_COLOR_BLUE,
                OnlineMessage::DownloadFromCache(wallpaper_index).into(),
            ),
            i18n.t("online-wallpapers.tooltip-download"),
            tooltip::Position::Top,
            theme_config,
        )
    } else {
        // 禁用状态的下载按钮
        let disabled_button = common::create_icon_button("\u{F30A}", BUTTON_COLOR_GRAY, AppMessage::None);
        container(disabled_button)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(crate::ui::style::DISABLED_BUTTON_BG)),
                ..Default::default()
            })
            .into()
    };

    let close_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F659}", BUTTON_COLOR_RED, OnlineMessage::CloseModal.into()),
        i18n.t("online-wallpapers.tooltip-close"),
        tooltip::Position::Top,
        theme_config,
    );

    // 底部工具栏
    let toolbar = container(
        row![
            container(Space::new()).width(Length::Fill),
            prev_button,
            next_button,
            set_wallpaper_button,
            download_button,
            close_button,
            container(Space::new()).width(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(50.0),
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fill)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.7,
        })),
        ..Default::default()
    });

    let modal_content = container(
        column![
            container(modal_image_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20),
            toolbar,
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_MODAL_BG)),
        ..Default::default()
    });

    container(opaque(modal_content)).into()
}
