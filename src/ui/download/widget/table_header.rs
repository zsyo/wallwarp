// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::ThemeConfig;
use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

/// 创建表头
pub fn create_table_header<'a>(i18n: &'a I18n, theme_config: &'a ThemeConfig) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    row![
        // 文件名列
        container(
            text(i18n.t("download-tasks.header-filename"))
                .size(14)
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
            text(i18n.t("download-tasks.header-size"))
                .size(14)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                })
        )
        .width(Length::Fixed(100.0))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 状态列
        container(
            text(i18n.t("download-tasks.header-status"))
                .size(14)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                })
        )
        .width(Length::Fixed(220.0))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 下载列
        container(
            text(i18n.t("download-tasks.header-download"))
                .size(14)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                })
        )
        .width(Length::Fixed(100.0))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 操作列（最后一列，不添加分隔线）
        container(
            text(i18n.t("download-tasks.header-operations"))
                .size(14)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                })
        )
        .width(Length::Fill)
        .padding(5),
    ]
    .width(Length::Fill)
    .padding(5)
    .align_y(Alignment::Center)
    .into()
}
