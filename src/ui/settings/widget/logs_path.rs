// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::{INPUT_PADDING, ROW_SPACING, TEXT_INPUT_SIZE};
use crate::utils::helpers;
use iced::widget::{row, text_input};
use iced::{Alignment, Element, Length};

/// 创建日志目录行（标签+说明在上，输入框与打开按钮占满整行）
pub fn create_logs_path_row<'a>(
    i18n: &I18n,
    label: String,
    description: String,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let logs_path = helpers::get_absolute_path("logs");

    let controls = row![
        text_input("", &logs_path)
            .width(Length::Fill)
            .size(TEXT_INPUT_SIZE)
            .align_x(Alignment::Center)
            .padding(INPUT_PADDING)
            .style(styled_text_input(theme_colors)),
        common::create_icon_button_with_tooltip(
            "\u{F1C5}", // box-arrow-up-right (打开日志目录)
            theme_colors.primary,
            SettingsMessage::OpenLogsPath.into(),
            i18n.t("settings.open-path"),
            theme_colors,
        ),
    ]
    .spacing(ROW_SPACING / 2.0)
    .align_y(Alignment::Center);

    super::create_full_width_row(label, Some(description), controls, theme_colors)
}
