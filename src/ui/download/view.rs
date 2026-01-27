// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理视图模块
//!
//! 定义下载页面的界面渲染逻辑

use super::super::AppMessage;
use super::super::common;
use super::message::DownloadMessage;
use super::state::{DownloadStatus, DownloadStateFull, DownloadTask};
use crate::i18n::I18n;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW,
    EMPTY_STATE_PADDING, EMPTY_STATE_TEXT_SIZE, TABLE_SEPARATOR_WIDTH,
};
use crate::utils::helpers::format_file_size;
use iced::widget::{column, container, progress_bar, row, scrollable, text};
use iced::{Alignment, Element, Font, Length};

/// 下载页面视图函数
pub fn download_view<'a>(
    i18n: &'a I18n,
    _window_width: u32,
    download_state: &'a DownloadStateFull,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let content = if download_state.tasks.is_empty() {
        // 空状态显示
        create_empty_state(i18n, theme_config)
    } else {
        // 表格布局
        create_table_view(i18n, download_state, theme_config)
    };

    let scrollable_content = scrollable(content).width(Length::Fill).height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
}

/// 创建表格视图
fn create_table_view<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    // 表头
    let header = create_table_header(i18n, theme_config);

    // 表格内容
    let mut table = column![header].spacing(0).width(Length::Fill);

    // 添加表头下方的水平分隔线
    table = table.push(create_horizontal_separator(theme_config));

    for task_full in &download_state.tasks {
        // 添加表格行
        table = table.push(create_table_row(i18n, &task_full.task, theme_config));
        // 添加行下方的水平分隔线
        table = table.push(create_horizontal_separator(theme_config));
    }

    table.into()
}

/// 创建水平分隔线
fn create_horizontal_separator(theme_config: &crate::ui::style::ThemeConfig) -> Element<'_, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    container(iced::widget::Space::new())
        .width(Length::Fill)
        .height(TABLE_SEPARATOR_WIDTH)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.table_separator_color)),
            ..Default::default()
        })
        .into()
}

/// 创建表头
fn create_table_header<'a>(
    i18n: &'a I18n,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

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
        create_separator(theme_config),
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
        create_separator(theme_config),
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
        create_separator(theme_config),
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
        create_separator(theme_config),
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

/// 创建表格行
fn create_table_row<'a>(
    i18n: &'a I18n,
    task: &'a DownloadTask,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    row![
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
        create_separator(theme_config),
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
        create_separator(theme_config),
        // 状态列
        container(create_status_display(i18n, task, theme_config))
            .width(Length::Fixed(220.0))
            .padding(5),
        // 分隔线
        create_separator(theme_config),
        // 下载列
        container(create_download_display(i18n, task, theme_config))
            .width(Length::Fixed(100.0))
            .padding(5),
        // 分隔线
        create_separator(theme_config),
        // 操作列（最后一列，不添加分隔线）
        container(create_operation_buttons(i18n, task))
            .width(Length::Fill)
            .padding(5),
    ]
    .width(Length::Fill)
    .padding(5)
    .align_y(Alignment::Center)
    .into()
}

/// 创建状态显示
fn create_status_display<'a>(
    i18n: &'a I18n,
    task: &'a DownloadTask,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    match &task.status {
        DownloadStatus::Downloading => {
            // 下载中：显示进度条和百分比
            let progress_bar = container(progress_bar(0.0..=1.0, task.progress))
                .width(Length::Fixed(160.0))
                .height(Length::Fixed(12.0));
            let progress_text = text(format!("{:.0}%", task.progress * 100.0))
                .size(11)
                .style(move |_| text::Style {
                    color: Some(BUTTON_COLOR_BLUE),
                });
            row![progress_bar, progress_text]
                .spacing(6)
                .align_y(Alignment::Center)
                .into()
        }
        DownloadStatus::Waiting => text(i18n.t("download-tasks.status-waiting"))
            .size(12)
            .style(move |_| text::Style {
                color: Some(theme_colors.light_text_sub),
            })
            .into(),
        DownloadStatus::Paused => text(i18n.t("download-tasks.status-paused"))
            .size(12)
            .style(move |_| text::Style {
                color: Some(BUTTON_COLOR_GRAY),
            })
            .into(),
        DownloadStatus::Completed => text(i18n.t("download-tasks.status-completed"))
            .size(12)
            .style(|_| text::Style {
                color: Some(BUTTON_COLOR_GREEN),
            })
            .into(),
        DownloadStatus::Failed(_msg) => text(i18n.t("download-tasks.status-failed-error"))
            .size(12)
            .style(|_| text::Style {
                color: Some(BUTTON_COLOR_RED),
            })
            .into(),
        DownloadStatus::Cancelled => text(i18n.t("download-tasks.status-cancelled"))
            .size(12)
            .style(|_| text::Style {
                color: Some(BUTTON_COLOR_YELLOW),
            })
            .into(),
    }
}

/// 创建下载显示
fn create_download_display<'a>(
    _i18n: &'a I18n,
    task: &'a DownloadTask,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let speed_text = match &task.status {
        DownloadStatus::Downloading => format_speed(task.speed),
        _ => "0 B/s".to_string(),
    };

    text(speed_text)
        .size(12)
        .style(move |_| text::Style {
            color: Some(theme_colors.light_text_sub),
        })
        .into()
}

/// 创建操作按钮
fn create_operation_buttons<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    let pause_button = common::create_icon_button_with_tooltip(
        "\u{F4C3}", // pause-fill
        BUTTON_COLOR_YELLOW,
        AppMessage::Download(DownloadMessage::PauseTask(task.id)),
        i18n.t("download-tasks.tooltip-pause"),
    );
    let resume_button = common::create_icon_button_with_tooltip(
        "\u{F4F4}", // play-fill
        BUTTON_COLOR_GREEN,
        AppMessage::Download(DownloadMessage::ResumeTask(task.id)),
        i18n.t("download-tasks.tooltip-resume"),
    );
    let retry_button = common::create_icon_button_with_tooltip(
        "\u{F130}", // play-fill (重新下载)
        BUTTON_COLOR_BLUE,
        AppMessage::Download(DownloadMessage::RetryTask(task.id)),
        i18n.t("download-tasks.tooltip-retry"),
    );
    let copy_button = common::create_icon_button_with_tooltip(
        "\u{F759}", // link-45deg
        BUTTON_COLOR_BLUE,
        AppMessage::Download(DownloadMessage::CopyDownloadLink(task.id)),
        i18n.t("download-tasks.tooltip-copy-url"),
    );
    let cancel_button = common::create_icon_button_with_tooltip(
        "\u{F117}", // x-circle-fill
        BUTTON_COLOR_RED,
        AppMessage::Download(DownloadMessage::CancelTask(task.id)),
        i18n.t("download-tasks.tooltip-cancel"),
    );
    let delete_button = common::create_icon_button_with_tooltip(
        "\u{F78B}", // trash-fill (删除任务)
        BUTTON_COLOR_RED,
        AppMessage::Download(DownloadMessage::DeleteTask(task.id)),
        i18n.t("download-tasks.tooltip-delete"),
    );
    let view_button = common::create_icon_button_with_tooltip(
        "\u{F341}", // folder-fill (查看文件)
        BUTTON_COLOR_YELLOW,
        AppMessage::Download(DownloadMessage::OpenFileLocation(task.id)),
        i18n.t("download-tasks.tooltip-open"),
    );
    let set_wallpaper_button = common::create_icon_button_with_tooltip(
        "\u{F429}", // image-fill (设为壁纸)
        BUTTON_COLOR_GREEN,
        AppMessage::Download(DownloadMessage::SetAsWallpaper(task.id)),
        i18n.t("local-list.tooltip-set-wallpaper"),
    );

    match &task.status {
        DownloadStatus::Downloading => {
            // 下载中：暂停/复制下载链接/取消
            row![pause_button, copy_button, cancel_button]
                .spacing(6)
                .into()
        }
        DownloadStatus::Paused => {
            // 暂停中：继续/复制下载链接/取消
            row![resume_button, copy_button, cancel_button]
                .spacing(6)
                .into()
        }
        DownloadStatus::Failed(_) => {
            // 下载失败：重新下载/复制下载链接/删除
            row![retry_button, copy_button, delete_button]
                .spacing(6)
                .into()
        }
        DownloadStatus::Cancelled => {
            // 已取消：重新下载/复制下载链接/删除
            row![retry_button, copy_button, delete_button]
                .spacing(6)
                .into()
        }
        DownloadStatus::Completed => {
            // 下载完成：查看文件/设为壁纸/复制下载链接/删除(仅删除任务)
            row![view_button, set_wallpaper_button, copy_button, delete_button]
                .spacing(6)
                .into()
        }
        DownloadStatus::Waiting => {
            // 等待中：复制下载链接/取消
            row![copy_button, cancel_button].spacing(6).into()
        }
    }
}

/// 创建空状态界面
fn create_empty_state<'a>(
    i18n: &'a I18n,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let icon = text("\u{F30A}")
        .font(Font::with_name("bootstrap-icons"))
        .size(48.0)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.light_text_sub),
        });

    let empty_text = text(i18n.t("download-tasks.no-tasks"))
        .size(EMPTY_STATE_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let hint_text = text(i18n.t("download-tasks.no-tasks-hint"))
        .size(14)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.light_text_sub),
        });

    column![icon, empty_text, hint_text]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(EMPTY_STATE_PADDING)
        .spacing(10)
        .into()
}

/// 格式化下载速度
fn format_speed(speed: u64) -> String {
    format!("{}/s", format_file_size(speed))
}

/// 创建表格列分隔线
fn create_separator(theme_config: &crate::ui::style::ThemeConfig) -> Element<'_, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    container(iced::widget::Space::new())
        .width(TABLE_SEPARATOR_WIDTH)
        .height(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.table_separator_color)),
            ..Default::default()
        })
        .into()
}

impl From<DownloadMessage> for AppMessage {
    fn from(download_message: DownloadMessage) -> AppMessage {
        AppMessage::Download(download_message)
    }
}