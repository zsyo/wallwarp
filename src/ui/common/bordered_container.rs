// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::ThemeConfig;
use crate::ui::style::{BORDER_RADIUS, BORDER_WIDTH, shadows::CARD_SHADOW};
use iced::border::{Border, Radius};
use iced::widget::container;

/// 创建带边框的容器样式（带背景色）
///
/// 用于设置页配置区块卡片与壁纸卡片：主题背景色 + 1px 边框 + 柔和投影。
///
/// # 参数
/// - `theme_config`: 主题配置
pub fn create_bordered_container_style_with_bg(
    theme_config: &ThemeConfig,
) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    let theme_colors = theme_config.get_theme_colors();

    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.sidebar_bg)),
        border: Border {
            color: theme_colors.border,
            width: BORDER_WIDTH,
            radius: Radius::from(BORDER_RADIUS),
        },
        shadow: CARD_SHADOW,
        ..Default::default()
    }
}
