// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::DownloadStateFull;
use crate::ui::style::ThemeConfig;
use iced::widget::column;
use iced::{Element, Length};

/// 创建表格视图
pub fn create_table<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    // 表头
    let header = super::create_table_header(i18n, theme_config);

    // 表格内容
    let mut table = column![header].spacing(0).width(Length::Fill);

    // 添加表头下方的水平分隔线
    table = table.push(super::create_horizontal_separator(theme_config));

    for task_full in &download_state.tasks {
        // 添加表格行
        table = table.push(super::create_table_row(i18n, &task_full.task, theme_config));
        // 添加行下方的水平分隔线
        table = table.push(super::create_horizontal_separator(theme_config));
    }

    table.into()
}
