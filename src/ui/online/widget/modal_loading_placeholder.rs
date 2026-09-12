// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::OnlineState;
use crate::ui::style::ThemeConfig;
use iced::Element;

/// 创建模态窗口加载占位符（进度环 + 百分比 + 字节说明）
pub fn create_modal_loading_placeholder<'a>(
    i18n: &'a I18n,
    online_state: &'a OnlineState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    common::create_progress_ring_placeholder(
        online_state.modal_download_progress,
        online_state.modal_downloaded_bytes,
        online_state.modal_total_bytes,
        i18n.t("online-wallpapers.image-loading").to_string(),
        theme_config,
    )
}
