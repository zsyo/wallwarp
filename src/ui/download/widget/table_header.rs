// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::{DownloadStateFull, SortColumn};
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建表头
pub fn create_table_header<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    row![
        // 全选框列
        super::create_checkbox_header(download_state, theme_config),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 文件名列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-filename",
            download_state,
            SortColumn::FileName,
            theme_colors.clone(),
            Length::FillPortion(3),
        ),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 大小列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-size",
            download_state,
            SortColumn::Size,
            theme_colors.clone(),
            Length::Fixed(100.0),
        ),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 状态列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-status",
            download_state,
            SortColumn::Status,
            theme_colors.clone(),
            Length::Fixed(220.0),
        ),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 下载列（不可排序）
        container(
            text(i18n.t("download-tasks.header-download"))
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme_colors.text),
                })
        )
        .width(Length::Fixed(100.0))
        .padding(5),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 添加时间列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-created-at",
            download_state,
            SortColumn::CreatedAt,
            theme_colors.clone(),
            Length::Fixed(150.0),
        ),
        // 分隔线
        super::create_vertical_separator(theme_config),
        // 操作列（不可排序，最后一列，不添加分隔线）
        container(
            text(i18n.t("download-tasks.header-operations"))
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
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

/// 创建可排序的表头单元格
fn create_sortable_header_cell<'a>(
    i18n: &'a I18n,
    translation_key: &'a str,
    download_state: &'a DownloadStateFull,
    sort_column: SortColumn,
    theme_colors: ThemeColors,
    width: Length,
) -> Element<'a, AppMessage> {
    let is_current_column = download_state.sort_column == Some(sort_column);
    let is_sorting = download_state.is_sorting;

    // 排序图标
    let sort_icon = if is_current_column {
        if download_state.sort_descending {
            text("▼")
        } else {
            text("▲")
        }
    } else {
        text("⇅")
    };

    let header_text = text(i18n.t(translation_key))
        .size(14)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme_colors.text),
        });

    let sort_icon_elem = sort_icon
        .size(10)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: if is_current_column {
                Some(theme_colors.light_text_sub)
            } else {
                Some(theme_colors.text)
            },
        });

    // 列名靠左，图标靠右，中间用Fill占位
    let content = row![header_text, Space::new().width(Length::Fill), sort_icon_elem,].align_y(Alignment::Center);

    let button_elem = button(content)
        .on_press(if is_sorting {
            AppMessage::None // 排序中禁止点击
        } else {
            AppMessage::Download(crate::ui::download::message::DownloadMessage::ToggleSort(sort_column))
        })
        .padding(5) // 添加padding，与不可排序列一致
        .width(Length::Fill) // 按钮填满容器宽度
        .style(
            move |_theme: &iced::Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
                text_color: theme_colors.text,
                background: Some(iced::Background::Color(if is_current_column {
                    theme_colors.background
                } else {
                    iced::Color::TRANSPARENT
                })),
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
                snap: false,
            },
        );

    container(button_elem).width(width).into()
}
