// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::DownloadTask;
use crate::ui::style::TABLE_TEXT_SIZE;
use crate::ui::style::ThemeConfig;
use crate::utils::helpers::format_file_size;
use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建表格行
///
/// 整行作为按钮：悬停淡染、点击切换选中；行内的复选框与操作按钮
/// 自行消费点击事件，不会触发整行切换。
pub fn create_table_row<'a>(
    i18n: &'a I18n,
    task: &'a DownloadTask,
    is_selected: bool,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let row_content = row![
        // 选中框列
        super::create_task_checkbox(task.id, is_selected, theme_config),
        // 文件名列
        container(text(&task.file_name).size(TABLE_TEXT_SIZE).style(
            move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }
        ))
        .width(Length::FillPortion(3))
        .padding(5),
        // 大小列
        container(
            text(format_file_size(task.total_size))
                .size(TABLE_TEXT_SIZE)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.light_text),
                })
        )
        .width(Length::Fixed(100.0))
        .padding(5),
        // 状态列
        container(super::create_status_display(i18n, task, theme_config))
            .width(Length::Fixed(220.0))
            .padding(5),
        // 下载列
        container(super::create_download_display(i18n, task, theme_config))
            .width(Length::Fixed(100.0))
            .padding(5),
        // 添加时间列
        container(
            text(task.created_at.format("%Y-%m-%d %H:%M:%S").to_string())
                .size(TABLE_TEXT_SIZE)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.light_text),
                })
        )
        .width(Length::Fixed(150.0))
        .padding(5),
        // 操作列（最后一列）
        container(super::create_operation_buttons(i18n, task, theme_colors))
            .width(Length::Fill)
            .padding(5),
    ]
    .width(Length::Fill)
    .padding(5)
    .align_y(Alignment::Center);

    button(row_content)
        .padding(0)
        .width(Length::Fill)
        .on_press(AppMessage::Download(
            crate::ui::download::message::DownloadMessage::ToggleTaskSelection(task.id),
        ))
        .style(move |_theme: &iced::Theme, status| {
            use crate::ui::style::tint;
            let bg = match status {
                button::Status::Hovered => theme_colors.hover_fill,
                button::Status::Pressed => tint(theme_colors.primary, 0.08),
                _ => iced::Color::TRANSPARENT,
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: theme_colors.text,
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..iced::widget::button::text(_theme, status)
            }
        })
        .into()
}
