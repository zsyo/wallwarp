// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_SPACING,
    INPUT_HEIGHT, INPUT_PADDING, ROW_SPACING, TEXT_INPUT_SIZE,
};
use iced::widget::{Space, container, row, text, text_input};
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

/// 创建路径配置行
///
/// # 参数
/// - `i18n`: 国际化实例
/// - `label`: 标签文本
/// - `path`: 当前路径（用于展示）
/// - `actions`: 各按钮消息集合
/// - `theme_colors`: 主题颜色
pub fn create_path_config_row<'a>(
    i18n: &I18n,
    label: String,
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
    row![
        text(label)
            .width(Length::FillPortion(1))
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        row![
            text_input("", path)
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(|_| SettingsMessage::DataPathSelected("".to_string()).into())
                .padding(INPUT_PADDING)
                .style(styled_text_input(theme_colors)),
            container(Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.select-path"),
                BUTTON_COLOR_BLUE,
                select_msg
            ),
            container(Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.open-path"),
                BUTTON_COLOR_GREEN,
                open_msg
            ),
            container(Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.clear-path"),
                BUTTON_COLOR_RED,
                clear_msg
            ),
            container(Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.restore-default"),
                BUTTON_COLOR_GRAY,
                restore_msg
            ),
        ]
        .width(Length::FillPortion(4))
        .spacing(0),
    ]
    .height(Length::Fixed(INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}
