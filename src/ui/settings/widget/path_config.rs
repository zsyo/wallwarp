// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::{INPUT_PADDING, ROW_SPACING, TEXT_INPUT_SIZE};
use iced::widget::{row, text_input};
use iced::{Alignment, Element, Length};

/// 路径配置行各按钮触发的消息集合
pub struct PathRowActions {
    /// 选择路径
    pub select: AppMessage,
    /// 打开路径
    pub open: AppMessage,
    /// 清空路径（弹出确认）
    pub clear: AppMessage,
    /// 恢复默认路径
    pub restore: AppMessage,
}

/// 创建路径配置行（标签+说明在上，输入框与操作按钮占满整行）
///
/// 操作按钮为带 tooltip 的图标按钮：
/// 选择=folder2-open(f3d8)、查看=box-arrow-up-right(f1c5)、
/// 清空=trash3(f78b)、默认=arrow-counterclockwise(f117)。
///
/// # 参数
/// - `i18n`: 国际化实例
/// - `label`: 标签文本
/// - `description`: 说明文本
/// - `path`: 当前路径（用于展示）
/// - `actions`: 各按钮消息集合
/// - `theme_colors`: 主题颜色
pub fn create_path_config_row<'a>(
    i18n: &I18n,
    label: String,
    description: String,
    path: &str,
    actions: PathRowActions,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let PathRowActions {
        select: select_msg,
        open: open_msg,
        clear: clear_msg,
        restore: restore_msg,
    } = actions;

    let controls = row![
        text_input("", path)
            .width(Length::Fill)
            .size(TEXT_INPUT_SIZE)
            .align_x(Alignment::Center)
            .on_input(|_| SettingsMessage::DataPathSelected("".to_string()).into())
            .padding(INPUT_PADDING)
            .style(styled_text_input(theme_colors)),
        common::create_icon_button_with_tooltip(
            "\u{F3D8}", // folder2-open (选择路径)
            theme_colors.primary,
            select_msg,
            i18n.t("settings.select-path"),
            theme_colors,
        ),
        common::create_icon_button_with_tooltip(
            "\u{F1C5}", // box-arrow-up-right (打开路径)
            theme_colors.primary,
            open_msg,
            i18n.t("settings.open-path"),
            theme_colors,
        ),
        common::create_icon_button_with_tooltip(
            "\u{F78B}", // trash3 (清空路径)
            theme_colors.notification_error_bg,
            clear_msg,
            i18n.t("settings.clear-path"),
            theme_colors,
        ),
        common::create_icon_button_with_tooltip(
            "\u{F117}", // arrow-counterclockwise (恢复默认)
            theme_colors.secondary,
            restore_msg,
            i18n.t("settings.restore-default"),
            theme_colors,
        ),
    ]
    .spacing(ROW_SPACING / 2.0)
    .align_y(Alignment::Center);

    super::create_full_width_row(label, Some(description), controls, theme_colors)
}
