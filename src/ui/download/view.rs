// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理视图模块
//!
//! 定义下载页面的界面渲染逻辑

use super::state::DownloadStateFull;
use super::widget;
use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::ThemeConfig;
use iced::widget::{Space, column, container, scrollable};
use iced::{Element, Length};

/// 下载页面视图函数
pub fn download_view<'a>(
    i18n: &'a I18n,
    _window_width: u32,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    // 筛选后的任务列表
    let filtered_tasks: Vec<_> = download_state
        .tasks
        .iter()
        .filter(|task| {
            if let Some(filter_status) = &download_state.status_filter {
                filter_status.matches(&task.task.status)
            } else {
                true
            }
        })
        .collect();

    // 创建主内容
    let content = column![].spacing(10);

    // 添加工具栏
    let content = content.push(widget::create_toolbar(i18n, download_state, theme_config));

    // 添加垂直间距
    let content = content.push(container(Space::new()).height(Length::Fixed(10.0)));

    // 根据筛选后的任务列表显示内容
    let content = if filtered_tasks.is_empty() {
        // 无任务显示（保留表头+文案）
        content.push(widget::create_filtered_empty_state(i18n, download_state, theme_config))
    } else {
        // 表格布局
        content.push(widget::create_filtered_table(i18n, filtered_tasks, theme_config))
    };

    let scrollable_content = scrollable(content).width(Length::Fill).height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
}
