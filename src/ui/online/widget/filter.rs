// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::source::SearchCapability;
use crate::services::wallhaven::{Category, Purity, Sorting};
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::icon_button::solid_icon_button_style;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use crate::utils::config::Config;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

/// 筛选栏切换按钮样式：选中=语义色实底，未选中=中性底+悬停反馈
fn toggle_chip_style(
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

/// 筛选栏外层容器（各搜索能力档位共用的外观包装）
fn wrap_filter_bar<'a>(
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

/// 创建筛选栏
pub fn create_filter_bar<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    config: &'a Config,
    app_groups: &'a [crate::services::database::WallpaperGroupDB],
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 功能按钮（图源无关，始终展示）
    let refresh_button = common::create_icon_button_with_size(
        "\u{F130}", // arrow-repeat
        theme_colors.light_text,
        20,
        OnlineMessage::Refresh.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT))
    .style(solid_icon_button_style(
        theme_colors.light_button,
        theme_colors.light_text,
    ));

    // 壁纸组选择器（组含图源与筛选快照，应用后整体切换）
    let group_picker = super::create_group_picker(i18n, app_groups, state, theme_colors);

    // 图源选择器（顶栏最前，各能力档位共用）
    let source_picker = super::create_source_picker(i18n, state, theme_colors);

    // 无搜索维度的图源（如 Bing 每日一图），仅展示说明与刷新按钮
    if state.source.search_capability() == SearchCapability::Bare {
        let hint_row = row![
            source_picker,
            Space::new().width(4),
            text(i18n.t("online-wallpapers.bing-daily-hint"))
                .size(14)
                .color(theme_colors.light_text_sub),
            Space::new().width(Length::Fill),
            group_picker,
            refresh_button,
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        return wrap_filter_bar(hint_row, theme_colors);
    }

    // 搜索框（放在最前面）
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

    let search_container = row![search_input, search_button]
        .spacing(2)
        .align_y(Alignment::Center);

    // 分辨率选择器 - 使用 DropDown 组件
    let resolution_picker = super::create_resolution_picker(i18n, state, theme_colors);

    // 比例选择器 - 使用 DropDown 组件（支持多选）
    let ratio_picker = super::create_ratio_picker(i18n, state, theme_colors);

    // 颜色选择器 - 使用 DropDown 组件
    let color_picker = super::create_color_picker(i18n, state, theme_colors);

    let sorting_picker = super::create_sorting_picker(i18n, state, theme_colors);

    // Basic 档图源：支持关键词搜索与排序，其余筛选维度由图源 service 层映射或忽略
    if state.source.search_capability() == SearchCapability::Basic {
        let basic_row = row![
            source_picker,
            Space::new().width(2),
            search_container,
            Space::new().width(2),
            sorting_picker,
            group_picker,
            refresh_button,
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        return wrap_filter_bar(basic_row, theme_colors);
    }

    let time_range_picker = super::create_time_range_picker(i18n, state, theme_colors);

    // 分类切换按钮（选中=强调色实底）
    let category_button = |label: String, category: Category, message: OnlineMessage| {
        let is_checked = (state.categories & category.bit_value()) != 0;
        button(text(label).size(14))
            .on_press(message.into())
            .padding(6)
            .style(toggle_chip_style(
                theme_colors,
                is_checked,
                theme_colors.primary,
                Color::WHITE,
            ))
    };

    // 纯净度切换按钮（选中=纯净度语义色实底）
    let purity_button = |label: String,
                         purity: Purity,
                         accent: Color,
                         checked_text: Color,
                         message: OnlineMessage| {
        let is_checked = (state.purities & purity.bit_value()) != 0;
        button(text(label).size(14))
            .on_press(message.into())
            .padding(6)
            .style(toggle_chip_style(
                theme_colors,
                is_checked,
                accent,
                checked_text,
            ))
    };

    // 组合所有元素
    let filter_row = row![
        source_picker,
        Space::new().width(2),
        search_container,
        Space::new().width(2),
        category_button(
            i18n.t("online-wallpapers.category-general"),
            Category::General,
            OnlineMessage::CategoryToggled(Category::General),
        ),
        category_button(
            i18n.t("online-wallpapers.category-anime"),
            Category::Anime,
            OnlineMessage::CategoryToggled(Category::Anime),
        ),
        category_button(
            i18n.t("online-wallpapers.category-people"),
            Category::People,
            OnlineMessage::CategoryToggled(Category::People),
        ),
        Space::new().width(2),
        purity_button(
            i18n.t("online-wallpapers.purity-sfw"),
            Purity::SFW,
            COLOR_SFW,
            Color::WHITE,
            OnlineMessage::PurityToggled(Purity::SFW),
        ),
        purity_button(
            i18n.t("online-wallpapers.purity-sketchy"),
            Purity::Sketchy,
            COLOR_SKETCHY,
            Color::BLACK,
            OnlineMessage::PurityToggled(Purity::Sketchy),
        ),
        // NSFW 按钮：只在 API Key 不为空时显示
        if !config.wallhaven.api_key.is_empty() {
            Some(purity_button(
                i18n.t("online-wallpapers.purity-nsfw"),
                Purity::NSFW,
                COLOR_NSFW,
                Color::WHITE,
                OnlineMessage::PurityToggled(Purity::NSFW),
            ))
        } else {
            None
        },
        Space::new().width(2),
        resolution_picker,
        ratio_picker,
        color_picker,
        sorting_picker,
        // 时间范围选择器：仅在排序为 TopList 时显示
        if state.sorting == Sorting::TopList {
            Some(time_range_picker)
        } else {
            None
        },
        group_picker,
        refresh_button,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    wrap_filter_bar(filter_row, theme_colors)
}
