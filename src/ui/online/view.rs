// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::OnlineState;
use super::widget;
use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::ThemeConfig;
use crate::utils::config::Config;
use iced::widget::{column, stack};
use iced::{Element, Length};

/// 在线壁纸页面视图
pub fn online_view<'a>(
    i18n: &'a I18n,
    window_width: u32,
    online_state: &'a OnlineState,
    config: &'a Config,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    // 创建筛选栏
    let filter_bar = widget::create_filter_bar(i18n, online_state, config, theme_config);

    // 创建壁纸列表
    let wallpaper_list = widget::create_wallpaper_list(i18n, window_width, online_state, theme_config);

    let main_content = column![filter_bar, wallpaper_list]
        .width(Length::Fill)
        .height(Length::Fill);

    let mut layers = vec![main_content.into()];

    // 图片预览模态窗口
    if online_state.modal_visible && !online_state.wallpapers.is_empty() {
        layers.push(widget::create_modal(i18n, online_state, theme_config));
    }

    stack(layers).width(Length::Fill).height(Length::Fill).into()
}
