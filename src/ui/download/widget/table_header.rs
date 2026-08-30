// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::{DownloadStateFull, SortColumn};
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Element, Font, Length};

/// 排序状态图标（bootstrap-icons，码点已对照 icons.ttf 验证）
const ICON_SORT_ASC: &str = "\u{F286}"; // chevron-up
const ICON_SORT_DESC: &str = "\u{F282}"; // chevron-down
const ICON_SORT_NONE: &str = "\u{F283}"; // chevron-expand

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
        // 文件名列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-filename",
            download_state,
            SortColumn::FileName,
            theme_colors,
            Length::FillPortion(3),
        ),
        // 大小列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-size",
            download_state,
            SortColumn::Size,
            theme_colors,
            Length::Fixed(100.0),
        ),
        // 状态列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-status",
            download_state,
            SortColumn::Status,
            theme_colors,
            Length::Fixed(220.0),
        ),
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
        // 添加时间列（可排序）
        create_sortable_header_cell(
            i18n,
            "download-tasks.header-created-at",
            download_state,
            SortColumn::CreatedAt,
            theme_colors,
            Length::Fixed(150.0),
        ),
        // 操作列（不可排序，最后一列）
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

/// 创建可排序的表头单元格：悬停变强调色，排序列显示方向箭头
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
            ICON_SORT_DESC
        } else {
            ICON_SORT_ASC
        }
    } else {
        ICON_SORT_NONE
    };

    let header_text = text(i18n.t(translation_key))
        .size(14)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme_colors.text),
        });

    let sort_icon_elem = text(sort_icon)
        .font(Font::with_name("bootstrap-icons"))
        .size(12)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: if is_current_column {
                Some(theme_colors.primary)
            } else {
                Some(theme_colors.light_text_sub)
            },
        });

    // 列名靠左，图标靠右，中间用Fill占位
    let content = row![
        header_text,
        Space::new().width(Length::Fill),
        sort_icon_elem,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let button_elem = button(content)
        .on_press(if is_sorting {
            AppMessage::None // 排序中禁止点击
        } else {
            AppMessage::Download(crate::ui::download::message::DownloadMessage::ToggleSort(
                sort_column,
            ))
        })
        .padding(5) // 与不可排序列一致
        .width(Length::Fill) // 按钮填满容器宽度
        .style(
            move |_theme: &iced::Theme, status: iced::widget::button::Status| {
                let text_color = match status {
                    button::Status::Active | button::Status::Disabled => theme_colors.text,
                    _ => theme_colors.primary,
                };
                let bg = match status {
                    button::Status::Hovered | button::Status::Pressed => theme_colors.hover_fill,
                    _ => iced::Color::TRANSPARENT,
                };
                iced::widget::button::Style {
                    text_color,
                    background: Some(iced::Background::Color(bg)),
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: crate::ui::style::RADIUS_SM.into(),
                    },
                    ..iced::widget::button::text(_theme, status)
                }
            },
        );

    container(button_elem).width(width).into()
}
