// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::message::DownloadMessage;
use crate::ui::download::state::DownloadStateFull;
use crate::ui::style::{BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, ThemeConfig};
use iced::widget::row;
use iced::{Alignment, Element};

/// 创建批量操作按钮
pub fn create_batch_operation_buttons<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 检查按钮是否启用
    let can_start = download_state.can_batch_start();
    let can_pause = download_state.can_batch_pause();
    let can_retry = download_state.can_batch_retry();
    let can_cancel = download_state.can_batch_cancel();
    let can_delete = download_state.can_batch_delete();

    row![
        // 开始（恢复）按钮 - 绿色
        super::create_batch_button(
            i18n.t("download-tasks.batch-start"),
            "\u{F4F4}", // play-fill
            can_start,
            AppMessage::Download(DownloadMessage::BatchStart),
            theme_colors.clone(),
            BUTTON_COLOR_GREEN,
        ),
        // 暂停按钮 - 黄色
        super::create_batch_button(
            i18n.t("download-tasks.batch-pause"),
            "\u{F4C3}", // pause-fill
            can_pause,
            AppMessage::Download(DownloadMessage::BatchPause),
            theme_colors.clone(),
            BUTTON_COLOR_YELLOW,
        ),
        // 重新开始按钮 - 蓝色
        super::create_batch_button(
            i18n.t("download-tasks.batch-retry"),
            "\u{F130}", // play-fill (重新下载)
            can_retry,
            AppMessage::Download(DownloadMessage::BatchRetry),
            theme_colors.clone(),
            BUTTON_COLOR_BLUE,
        ),
        // 取消按钮 - 红色
        super::create_batch_button(
            i18n.t("download-tasks.batch-cancel"),
            "\u{F117}", // x-circle-fill
            can_cancel,
            AppMessage::Download(DownloadMessage::BatchCancel),
            theme_colors.clone(),
            BUTTON_COLOR_RED,
        ),
        // 删除按钮 - 红色
        super::create_batch_button(
            i18n.t("download-tasks.batch-delete"),
            "\u{F78B}", // trash-fill
            can_delete,
            AppMessage::Download(DownloadMessage::BatchDelete),
            theme_colors.clone(),
            BUTTON_COLOR_RED,
        ),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}
