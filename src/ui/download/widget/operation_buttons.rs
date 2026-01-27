// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::download::DownloadMessage;
use crate::ui::download::state::{DownloadStatus, DownloadTask};
use crate::ui::style::{BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW};
use iced::Element;
use iced::widget::row;

/// 创建操作按钮
pub fn create_operation_buttons<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
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
            row![pause_button, copy_button, cancel_button].spacing(6).into()
        }
        DownloadStatus::Paused => {
            // 暂停中：继续/复制下载链接/取消
            row![resume_button, copy_button, cancel_button].spacing(6).into()
        }
        DownloadStatus::Failed(_) => {
            // 下载失败：重新下载/复制下载链接/删除
            row![retry_button, copy_button, delete_button].spacing(6).into()
        }
        DownloadStatus::Cancelled => {
            // 已取消：重新下载/复制下载链接/删除
            row![retry_button, copy_button, delete_button].spacing(6).into()
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
