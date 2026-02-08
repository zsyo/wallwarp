// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::DownloadStateFull;
use crate::ui::style::ThemeConfig;
use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

/// 创建工具栏
pub fn create_toolbar<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 创建批量操作按钮
    let batch_buttons = super::create_batch_operation_buttons(i18n, download_state, theme_config);

    // 创建删除所有已完成任务按钮
    let clear_completed_button = super::create_clear_completed_button(i18n, download_state, theme_config);

    // 创建状态筛选下拉框
    let filter_dropdown = super::create_status_filter_dropdown(i18n, download_state, theme_config);

    // 工具栏内容
    let toolbar_content = row![
        // 批量操作区域
        container(batch_buttons).padding(10).width(Length::Shrink),
        // 分隔线
        container(text(""))
            .width(Length::Fixed(1.0))
            .height(Length::Fixed(30.0))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme_colors.border)),
                ..Default::default()
            }),
        // 删除所有已完成任务区域
        container(clear_completed_button).padding(10).width(Length::Shrink),
        // 分隔线
        container(text(""))
            .width(Length::Fixed(1.0))
            .height(Length::Fixed(30.0))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme_colors.border)),
                ..Default::default()
            }),
        // 筛选区域
        container(
            row![
                text(i18n.t("download-tasks.filter-label"))
                    .size(14)
                    .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(theme_colors.text),
                    }),
                container(filter_dropdown).width(Length::Fixed(150.0)),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
        )
        .padding(10)
        .width(Length::Shrink),
        // 空白占位（让筛选区域靠右）
        container(text("")).width(Length::Fill),
    ]
    .width(Length::Fill)
    .height(Length::Fixed(50.0))
    .align_y(Alignment::Center);

    container(toolbar_content).width(Length::Fill).padding([5, 10]).into()
}
