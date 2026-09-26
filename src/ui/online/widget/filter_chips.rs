// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 在线筛选栏的共用零件（各搜索能力档位共享）
//!
//! 含：外层容器、切换 chip（分类/纯净度/横竖屏方向）、搜索框、刷新按钮。
//! chip 选中态语义：分类/方向用强调色，纯净度用语义色（Wallhaven Full 与
//! Danbooru Basic 共用）

use crate::i18n::I18n;
use crate::services::wallhaven::{Category, Purity};
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::icon_button::solid_icon_button_style;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

/// 搜索框 + 搜索按钮（Wallhaven Full 与 Basic 档共用）
pub(super) fn create_search_container<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let search_input = text_input(
        &i18n.t("online-wallpapers.search-placeholder"),
        &state.search_text,
    )
    .on_input(|text| OnlineMessage::SearchTextChanged(text).into())
    .on_submit(OnlineMessage::Search.into())
    .padding(6)
    .size(14)
    .width(Length::Fixed(160.0))
    .style(common::styled_text_input(theme_colors));

    let search_button = common::create_icon_button_with_size(
        "\u{F52A}", // search
        theme_colors.light_text,
        17,
        OnlineMessage::Search.into(),
    )
    .style(solid_icon_button_style(
        theme_colors.light_button,
        theme_colors.light_text,
    ));

    row![search_input, search_button]
        .spacing(2)
        .align_y(Alignment::Center)
        .into()
}

/// 筛选栏外层容器（各搜索能力档位共用的外观包装）
pub(super) fn wrap_filter_bar<'a>(
    content: impl Into<Element<'a, AppMessage>>,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    container(row![
        Space::new().width(Length::Fixed(2.0)),
        container(content)
            .width(Length::Fill)
            .height(Length::Fixed(50.0))
            .padding(8)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme_colors.light_bg)),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(RADIUS_SM),
                },
                shadow: shadows::FILTER_BAR_SHADOW,
                ..Default::default()
            })
    ])
    .into()
}

/// 筛选栏切换按钮样式：选中=语义色实底，未选中=中性底+悬停反馈
pub(super) fn toggle_chip_style(
    theme_colors: ThemeColors,
    is_checked: bool,
    accent: Color,
    checked_text: Color,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let bg_color = if is_checked {
            match status {
                button::Status::Hovered => darken(accent, 0.08),
                button::Status::Pressed => darken(accent, 0.15),
                _ => accent,
            }
        } else {
            match status {
                button::Status::Hovered => theme_colors.hover_fill,
                button::Status::Pressed => tint(theme_colors.primary, 0.08),
                _ => theme_colors.light_button,
            }
        };
        button::Style {
            background: Some(iced::Background::Color(bg_color)),
            text_color: if is_checked {
                checked_text
            } else {
                theme_colors.light_text
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..button::text(_theme, status)
        }
    }
}

/// 分类切换 chip（选中=强调色实底；仅 Wallhaven Full 档使用）
pub(super) fn category_chip<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    category: Category,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let is_checked = (state.categories & category.bit_value()) != 0;
    button(text(i18n.t(category.display_name())).size(14))
        .on_press(OnlineMessage::CategoryToggled(category).into())
        .padding(6)
        .style(toggle_chip_style(
            theme_colors,
            is_checked,
            theme_colors.primary,
            Color::WHITE,
        ))
        .into()
}

/// 纯净度切换 chip（选中=纯净度语义色实底；Wallhaven 与 Danbooru 共用）
pub(super) fn purity_chip<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    purity: Purity,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let is_checked = (state.purities & purity.bit_value()) != 0;
    let (accent, checked_text) = match purity {
        Purity::SFW => (COLOR_SFW, Color::WHITE),
        Purity::Sketchy => (COLOR_SKETCHY, Color::BLACK),
        Purity::NSFW => (COLOR_NSFW, Color::WHITE),
    };
    button(text(i18n.t(purity.display_name())).size(14))
        .on_press(OnlineMessage::PurityToggled(purity).into())
        .padding(6)
        .style(toggle_chip_style(
            theme_colors,
            is_checked,
            accent,
            checked_text,
        ))
        .into()
}

/// 横竖屏方向切换 chip（Basic 档图源的方向筛选，选中态写入 ratios 快捷项）
pub(super) fn orientation_chip<'a>(
    i18n: &'a I18n,
    is_checked: bool,
    label_key: &'static str,
    message: OnlineMessage,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    button(text(i18n.t(label_key)).size(14))
        .on_press(message.into())
        .padding(6)
        .style(toggle_chip_style(
            theme_colors,
            is_checked,
            theme_colors.primary,
            Color::WHITE,
        ))
        .into()
}

/// Pixabay 精选过滤切换 chip（选中=强调色实底；仅 Pixabay service 层映射）
pub(super) fn editors_choice_chip<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    button(text(i18n.t("online-wallpapers.editors-choice")).size(14))
        .on_press(OnlineMessage::EditorsChoiceToggled.into())
        .padding(6)
        .style(toggle_chip_style(
            theme_colors,
            state.editors_choice,
            theme_colors.primary,
            Color::WHITE,
        ))
        .into()
}

/// 顶栏刷新按钮（图源无关，各能力档位共用）
pub(super) fn create_refresh_button(theme_colors: ThemeColors) -> Element<'static, AppMessage> {
    common::create_icon_button_with_size(
        "\u{F130}", // arrow-repeat
        theme_colors.light_text,
        20,
        OnlineMessage::Refresh.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT))
    .style(solid_icon_button_style(
        theme_colors.light_button,
        theme_colors.light_text,
    ))
    .into()
}
