// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common::create_colored_button_with_text;
use crate::ui::download::message::DownloadMessage;
use crate::ui::download::state::{DownloadStateFull, DownloadStatus};
use crate::ui::style::BUTTON_COLOR_RED;
use iced::widget::{row, text};
use iced::{Alignment, Element};

/// 创建删除所有已完成任务按钮（统一彩色按钮样式，含悬停/按下/禁用态）
pub fn create_clear_completed_button<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
) -> Element<'a, AppMessage> {
    // 检查是否有已完成的任务
    let has_completed = download_state
        .tasks
        .iter()
        .any(|task| matches!(task.task.status, DownloadStatus::Completed));

    let button_content = row![
        text("\u{F78B}") // trash3
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(14),
        text(i18n.t("download-tasks.clear-completed")).size(13),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    create_colored_button_with_text(
        button_content.into(),
        BUTTON_COLOR_RED,
        AppMessage::Download(DownloadMessage::ClearCompleted),
    )
    .on_press_maybe(has_completed.then_some(AppMessage::Download(DownloadMessage::ClearCompleted)))
    .padding([6, 12])
    .into()
}
