// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::DownloadTask;
use crate::ui::style::ThemeConfig;
use crate::utils::helpers::format_file_size;
use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

/// 创建表格行
pub fn create_table_row<'a>(
    i18n: &'a I18n,
    task: &'a DownloadTask,
    is_selected: bool,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    row![
        // 选中框列
        super::create_task_checkbox(task.id, is_selected, theme_config),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 文件名列
        container(
            text(&task.file_name)
                .size(13)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                })
        )
        .width(Length::FillPortion(3))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 大小列
        container(
            text(format_file_size(task.total_size))
                .size(12)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.light_text),
                })
        )
        .width(Length::Fixed(100.0))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 状态列
        container(super::create_status_display(i18n, task, theme_config))
            .width(Length::Fixed(220.0))
            .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 下载列
        container(super::create_download_display(i18n, task, theme_config))
            .width(Length::Fixed(100.0))
            .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 添加时间列
        container(
            text(task.created_at.format("%Y-%m-%d %H:%M:%S").to_string())
                .size(12)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.light_text),
                })
        )
        .width(Length::Fixed(150.0))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 操作列（最后一列，不添加分隔线）
        container(super::create_operation_buttons(i18n, task))
            .width(Length::Fill)
            .padding(5),
    ]
    .width(Length::Fill)
    .padding(5)
    .align_y(Alignment::Center)
    .into()
}
