// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common::wallpaper_loading_placeholder;
use crate::ui::style::ThemeConfig;
use iced::Element;

/// 创建加载占位符
pub(in crate::ui::local) fn create_loading_placeholder<'a>(
    i18n: &'a I18n,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    wallpaper_loading_placeholder(i18n.t("local-list.image-loading").to_string(), theme_config)
}
