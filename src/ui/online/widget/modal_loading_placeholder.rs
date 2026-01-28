// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::online::OnlineState;
use crate::ui::style::*;
use crate::utils::helpers;
use iced::widget::{container, text};
use iced::{Element, Length};

/// 创建模态窗口加载占位符
pub fn create_modal_loading_placeholder<'a>(i18n: &'a I18n, online_state: &'a OnlineState) -> Element<'a, AppMessage> {
    // 如果正在下载，显示进度
    if online_state.modal_download_progress > 0.0 && online_state.modal_image_handle.is_none() {
        let progress_percent = (online_state.modal_download_progress * 100.0) as i32;
        let progress_text = format!(
            "{}: {}% ({}/{})",
            i18n.t("online-wallpapers.image-loading"),
            progress_percent,
            helpers::format_file_size(online_state.modal_downloaded_bytes),
            helpers::format_file_size(online_state.modal_total_bytes)
        );

        let loading_text = text(progress_text).size(18).style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

        container(loading_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    } else {
        // 普通加载状态
        let loading_text = text(i18n.t("online-wallpapers.image-loading"))
            .size(24)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(COLOR_OVERLAY_TEXT),
            });

        container(loading_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
