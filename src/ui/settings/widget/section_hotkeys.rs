// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Element;

/// 快捷键配置占位图标（bootstrap-icons keyboard，码点已验证）
const HOTKEYS_ICON: &str = "\u{F451}";

/// 创建快捷键配置占位区块
///
/// 功能尚未实现，展示"开发中"占位卡；后续实现时替换为
/// 快捷键动作列表（动作名 + 可编辑按键组合）。
pub fn create_hotkeys_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    super::create_placeholder_section(
        HOTKEYS_ICON,
        app.i18n.t("settings.hotkeys-title"),
        app.i18n.t("settings.hotkeys-desc"),
        &app.theme_config,
    )
}
