// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::local::message::LocalMessage;
use crate::ui::local::state::LocalState;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, COLOR_MODAL_BG};
use iced::widget::{Space, column, container, row, tooltip};
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
        container(Space::new()).width(Length::Fill).height(Length::Fill).into()
    };

    // 使用 stack 将图片层叠加在加载文字之上
    let modal_image_content = iced::widget::stack(vec![loading_text, image_layer]);

    // 创建底部工具栏按钮
    let prev_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F12E}",
            BUTTON_COLOR_BLUE,
            AppMessage::Local(LocalMessage::PreviousImage),
        ),
        i18n.t("local-list.tooltip-prev"),
        tooltip::Position::Top,
        theme_config,
    );

    let next_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F137}",
            BUTTON_COLOR_BLUE,
            AppMessage::Local(LocalMessage::NextImage),
        ),
        i18n.t("local-list.tooltip-next"),
        tooltip::Position::Top,
        theme_config,
    );

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}",
            BUTTON_COLOR_GREEN,
            AppMessage::Local(LocalMessage::SetWallpaper(wallpaper_index)),
        ),
        i18n.t("local-list.tooltip-set-wallpaper"),
        tooltip::Position::Top,
        theme_config,
    );

    let close_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F659}",
            BUTTON_COLOR_RED,
            AppMessage::Local(LocalMessage::CloseModal),
        ),
        i18n.t("local-list.tooltip-close"),
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

    container(
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
    })
    .into()
}
