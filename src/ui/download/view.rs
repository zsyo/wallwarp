// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理视图模块
//!
//! 定义下载页面的界面渲染逻辑

use super::super::AppMessage;
use super::state::DownloadStateFull;
use super::widget;
use crate::i18n::I18n;
use crate::ui::style::ThemeConfig;
use iced::widget::{container, scrollable};
use iced::{Element, Length};

/// 下载页面视图函数
pub fn download_view<'a>(
    i18n: &'a I18n,
    _window_width: u32,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let content = if download_state.tasks.is_empty() {
        // 空状态显示
        widget::create_empty_state(i18n, theme_config)
    } else {
        // 表格布局
        widget::create_table(i18n, download_state, theme_config)
    };

    let scrollable_content = scrollable(content).width(Length::Fill).height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
}
