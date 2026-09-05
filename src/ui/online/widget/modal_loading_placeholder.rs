// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::progress_ring_image;
use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::online::OnlineState;
use crate::ui::style::*;
use crate::utils::helpers;
use iced::widget::{column, container, image, stack, text};
use iced::{Alignment, Element, Length};

/// 环形进度指示器边长
const PROGRESS_RING_SIZE: f32 = 96.0;

/// 创建模态窗口加载占位符（进度环 + 百分比 + 字节说明）
pub fn create_modal_loading_placeholder<'a>(
    i18n: &'a I18n,
    online_state: &'a OnlineState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 环形进度（缓存命中或进度未知时仅显示轨道圈）
    let ring = image(progress_ring_image(
        online_state.modal_download_progress,
        theme_colors.primary,
        with_alpha(COLOR_OVERLAY_TEXT, 0.25),
    ))
    .width(Length::Fixed(PROGRESS_RING_SIZE))
    .height(Length::Fixed(PROGRESS_RING_SIZE));

    // 环心百分比（进度 > 0 时叠加显示）
    let ring_content: Element<'_, AppMessage> = if online_state.modal_download_progress > 0.0 {
        let percent = (online_state.modal_download_progress * 100.0).round() as i32;
        let percent_text = container(
            text(format!("{}%", percent))
                .size(16)
                .color(COLOR_OVERLAY_TEXT),
        )
        // 与环同尺寸，避免 Fill 把 stack 撑满整个模态区域将环挤出可视区
        .width(Length::Fixed(PROGRESS_RING_SIZE))
        .height(Length::Fixed(PROGRESS_RING_SIZE))
        .center_x(Length::Fill)
        .center_y(Length::Fill);
        stack(vec![ring.into(), percent_text.into()]).into()
    } else {
        ring.into()
    };

    // 下方说明文本：有进度时仅显示字节（环与环心百分比已表达加载中），
    // 无进度（下载尚未开始/缓存命中）时显示加载中文案
    let detail_text = if online_state.modal_download_progress > 0.0 {
        if online_state.modal_total_bytes > 0 {
            format!(
                "{} / {}",
                helpers::format_file_size(online_state.modal_downloaded_bytes),
                helpers::format_file_size(online_state.modal_total_bytes)
            )
        } else {
            helpers::format_file_size(online_state.modal_downloaded_bytes)
        }
    } else {
        i18n.t("online-wallpapers.image-loading").to_string()
    };

    let content = column![
        ring_content,
        text(detail_text).size(16).color(COLOR_OVERLAY_TEXT),
    ]
    .spacing(14)
    .align_x(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
