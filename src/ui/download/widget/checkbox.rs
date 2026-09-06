// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 自定义方形选中框组件
//!
//! 使用 Iced 原生的 checkbox 组件

use crate::ui::AppMessage;
use crate::ui::common::checkbox_style;
use crate::ui::style::ThemeConfig;
use iced::widget::{checkbox, container};
use iced::{Element, Length};

/// 创建表头选中框（方形）
pub fn create_checkbox_header<'a>(
    download_state: &'a crate::ui::download::state::DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();
    let is_checked = download_state.select_all;

    let checkbox_elem = checkbox(is_checked)
        .on_toggle(|_state| {
            AppMessage::Download(crate::ui::download::message::DownloadMessage::ToggleSelectAll)
        })
        .style(checkbox_style(theme_colors, is_checked));

    container(checkbox_elem)
        .width(Length::Fixed(40.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}

/// 创建任务选中框（方形）
pub fn create_task_checkbox<'a>(
    task_id: usize,
    is_selected: bool,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let checkbox_elem = checkbox(is_selected)
        .on_toggle(move |_state| {
            AppMessage::Download(
                crate::ui::download::message::DownloadMessage::ToggleTaskSelection(task_id),
            )
        })
        .style(checkbox_style(theme_colors, is_selected));

    container(checkbox_elem)
        .width(Length::Fixed(40.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}
