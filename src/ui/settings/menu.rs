// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 主侧栏"设置"项下的子分类菜单
//!
//! 仅在设置页激活时展开，替代设置页内独立导航列，
//! 避免与主侧边栏形成双列导航冲突。

use crate::ui::style::{
    RADIUS_SM, SETTINGS_SUBMENU_ITEM_HEIGHT, SETTINGS_SUBMENU_TEXT_SIZE, tint,
};
use crate::ui::settings::SettingsMessage;
use crate::ui::{App, AppMessage, SettingsCategory};
use iced::border::{Border, Radius};
use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length, Padding};

/// 全部分类，按子菜单顺序排列
pub const ALL_CATEGORIES: [SettingsCategory; 7] = [
    SettingsCategory::General,
    SettingsCategory::Wallpaper,
    SettingsCategory::Sources,
    SettingsCategory::Hotkeys,
    SettingsCategory::Network,
    SettingsCategory::Data,
    SettingsCategory::About,
];

/// 分类子项的 i18n 词条键
fn category_label_key(category: SettingsCategory) -> &'static str {
    match category {
        SettingsCategory::General => "settings.category-general",
        SettingsCategory::Wallpaper => "settings.category-wallpaper",
        SettingsCategory::Sources => "settings.category-sources",
        SettingsCategory::Hotkeys => "settings.category-hotkeys",
        SettingsCategory::Network => "settings.category-network",
        SettingsCategory::Data => "settings.category-data",
        SettingsCategory::About => "settings.category-about",
    }
}

/// 创建设置子分类菜单
///
/// 选中态：强调色淡染底 + 强调色文字（与主菜单视觉语言一致，规格缩小）；
/// 悬停：中性淡染底。子项文字与父项"设置"文字左对齐。
pub fn create_category_submenu(app: &App) -> Element<'_, AppMessage> {
    let items: Vec<Element<'_, AppMessage>> = ALL_CATEGORIES
        .iter()
        .map(|&category| create_sub_item(app, category))
        .collect();

    column(items)
        .spacing(2)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
}

/// 创建单个分类子项按钮
fn create_sub_item<'a>(
    app: &'a App,
    category: SettingsCategory,
) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let is_selected = app.settings_state.active_category == category;

    let text_color = if is_selected {
        theme_colors.primary
    } else {
        theme_colors.light_text_sub
    };

    button(
        container(
            text(app.i18n.t(category_label_key(category)))
                .size(SETTINGS_SUBMENU_TEXT_SIZE)
                .color(text_color),
        )
        .width(Length::Fill)
        .align_x(Alignment::Start),
    )
    .on_press_maybe(if is_selected {
        None
    } else {
        Some(SettingsMessage::SettingsCategorySelected(category).into())
    })
    .padding(Padding {
        top: 4.0,
        right: 10.0,
        bottom: 4.0,
        left: 42.0,
    })
    .height(Length::Fixed(SETTINGS_SUBMENU_ITEM_HEIGHT))
    .width(Length::Fill)
    .style(move |_theme: &iced::Theme, status| {
        let bg_color = if is_selected {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    tint(theme_colors.primary, 0.18)
                }
                _ => theme_colors.sidebar_button_selected,
            }
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => theme_colors.hover_fill,
                _ => theme_colors.sidebar_button_default,
            }
        };

        button::Style {
            background: Some(iced::Background::Color(bg_color)),
            text_color,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..button::text(_theme, status)
        }
    })
    .into()
}
