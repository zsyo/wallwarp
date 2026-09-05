// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 本地壁纸模态预览的信息浮层（左上角半透明胶囊）

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::utils::helpers::format_file_size;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element};

/// 创建本地壁纸信息浮层
pub fn create_modal_info<'a>(
    i18n: &'a I18n,
    wallpaper: &'a Wallpaper,
    _theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let info_row = |label: &str, value: String| -> Element<'a, AppMessage> {
        row![
            text(format!("{label}: ")).size(12).color(iced::Color::WHITE),
            text(value).size(12).color(iced::Color::WHITE),
        ]
        .spacing(2)
        .align_y(Alignment::Center)
        .into()
    };

    let info_column = column![
        info_row(
            i18n.t("wallpaper-info.resolution").as_str(),
            format!("{} x {}", wallpaper.width, wallpaper.height)
        ),
        info_row(
            i18n.t("wallpaper-info.file-size").as_str(),
            format_file_size(wallpaper.file_size)
        ),
    ]
    .spacing(4)
    .align_x(Alignment::Start);

    container(info_column)
        .padding([8, 12])
        .style(common::modal_overlay_style)
        .into()
}
