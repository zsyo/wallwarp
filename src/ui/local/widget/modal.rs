// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::local::message::LocalMessage;
use crate::ui::local::state::LocalState;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, COLOR_MODAL_BG,
};
use iced::widget::{Space, container, row, tooltip};
use iced::{Alignment, Element, Length};

/// 创建模态展示区
pub fn create_modal<'a>(
    i18n: &'a I18n,
    local_state: &'a LocalState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let wallpaper_index = local_state.current_image_index;

    // 创建背景加载文字
    let loading_text = super::create_modal_loading_placeholder(i18n);

    // 创建图片层（加载完成后显示）
    let image_layer: Element<_> = if let Some(ref handle) = local_state.modal_image_handle {
        // 使用预加载的图片数据
        let modal_image = iced::widget::image(handle.clone())
            .content_fit(iced::ContentFit::Contain)
            .width(Length::Fill)
            .height(Length::Fill);
        modal_image.into()
    } else {
        // 图片未加载完成，显示透明占位符（让背景文字可见）
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    };

    // 使用 stack 将图片层叠加在加载文字之上
    let modal_image_content = iced::widget::stack(vec![loading_text, image_layer]);

    // 创建底部工具栏按钮
    let prev_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F12E}",
            BUTTON_COLOR_BLUE,
            LocalMessage::PreviousImage.into(),
        ),
        i18n.t("local-list.tooltip-prev"),
        tooltip::Position::Top,
        theme_config,
    );

    let next_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F137}",
            BUTTON_COLOR_BLUE,
            LocalMessage::NextImage.into(),
        ),
        i18n.t("local-list.tooltip-next"),
        tooltip::Position::Top,
        theme_config,
    );

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}",
            BUTTON_COLOR_GREEN,
            LocalMessage::SetWallpaper(wallpaper_index).into(),
        ),
        i18n.t("local-list.tooltip-set-wallpaper"),
        tooltip::Position::Top,
        theme_config,
    );

    let locate_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F3D8}",
            BUTTON_COLOR_YELLOW,
            LocalMessage::ViewInFolder(wallpaper_index).into(),
        ),
        i18n.t("local-list.tooltip-locate"),
        tooltip::Position::Top,
        theme_config,
    );

    let close_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F659}",
            BUTTON_COLOR_RED,
            LocalMessage::CloseModal.into(),
        ),
        i18n.t("local-list.tooltip-close"),
        tooltip::Position::Top,
        theme_config,
    );

    // 底部悬浮工具栏（圆角胶囊）
    let toolbar = container(
        row![
            prev_button,
            next_button,
            set_wallpaper_button,
            locate_button,
            close_button,
        ]
        .align_y(Alignment::Center)
        .spacing(24.0),
    )
    .padding([6, 20])
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.65,
        })),
        border: iced::border::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(crate::ui::style::RADIUS_MD),
        },
        ..Default::default()
    });

    // 工具栏悬浮于图片底部居中
    let toolbar_layer = container(toolbar)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::End)
        .padding(iced::Padding {
            top: 0.0,
            right: 0.0,
            bottom: 24.0,
            left: 0.0,
        });

    container(iced::widget::stack(vec![
        modal_image_content.into(),
        toolbar_layer.into(),
    ]))
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_MODAL_BG)),
        ..Default::default()
    })
    .into()
}
