// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::{DownloadStatus, DownloadTask};
use crate::ui::style::ThemeColors;
use crate::ui::style::ThemeConfig;
use crate::utils::helpers::format_file_size;
use iced::Element;
use iced::widget::text;

/// 创建下载显示
pub fn create_download_display<'a>(
    _i18n: &'a I18n,
    task: &'a DownloadTask,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    let speed_text = match &task.status {
        DownloadStatus::Downloading => format!("{}/s", format_file_size(task.speed)),
        _ => "0 B/s".to_string(),
    };

    text(speed_text)
        .size(12)
        .style(move |_| text::Style {
            color: Some(theme_colors.light_text_sub),
        })
        .into()
}
