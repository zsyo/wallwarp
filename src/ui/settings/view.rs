// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::settings::widget;
use crate::ui::style::{SCROLL_PADDING, ThemeColors};
use crate::ui::{App, AppMessage, SettingsCategory};
use iced::widget::{Id, container, scrollable};
use iced::{Element, Length};

/// 设置页视图：当前分类的内容区（分类切换经主侧栏"设置"子菜单）
pub fn settings_view(app: &App) -> Element<'_, AppMessage> {
    container(
        // 卡片跟随窗口宽度撑满，四周等边距（滚动条贴窗口右缘）
        scrollable(
            container(category_content(app))
                .width(Length::Fill)
                .padding(SCROLL_PADDING),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .id(Id::new("settings_scroll")),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(create_content_container_style(app.theme_colors))
    .into()
}

/// 当前分类的内容视图
fn category_content(app: &App) -> Element<'_, AppMessage> {
    match app.settings_state.active_category {
        SettingsCategory::General => widget::create_general_section(app),
        SettingsCategory::Wallpaper => widget::create_wallpaper_config_section(app),
        SettingsCategory::Sources => widget::create_sources_section(app),
        SettingsCategory::Hotkeys => widget::create_hotkeys_section(app),
        SettingsCategory::Network => widget::create_network_section(app),
        SettingsCategory::Data => widget::create_data_config_section(app),
        SettingsCategory::About => widget::create_about_info_section(app),
    }
}

/// 设置内容区容器样式：主背景色
fn create_content_container_style(
    theme_colors: ThemeColors,
) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.background)),
        ..Default::default()
    }
}
