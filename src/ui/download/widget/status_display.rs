// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::{DownloadStatus, DownloadTask};
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW,
};
use crate::ui::style::ThemeConfig;
use iced::widget::{container, progress_bar, row, text};
use iced::{Alignment, Element, Length};

/// 创建状态显示
pub fn create_status_display<'a>(
    i18n: &'a I18n,
    task: &'a DownloadTask,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

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
