// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线壁纸模态预览的信息浮层（左上角半透明胶囊）

use crate::i18n::I18n;
use crate::services::wallhaven::OnlineWallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::create_icon_button;
use crate::ui::online::OnlineMessage;
use crate::ui::style::{BUTTON_COLOR_BLUE, ThemeConfig};
use crate::utils::helpers::format_file_size;
use iced::widget::{column, container, row, text, tooltip};
use iced::{Alignment, Element};

/// 创建壁纸信息浮层
///
/// `wallpaper_index` 用于"复制原图链接"按钮
pub fn create_modal_info<'a>(
    i18n: &'a I18n,
    wallpaper: &'a OnlineWallpaper,
    wallpaper_index: usize,
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

    let copy_button = common::create_button_with_tooltip(
        create_icon_button(
            "\u{F759}", // copy (复制原图链接)
            BUTTON_COLOR_BLUE,
            OnlineMessage::CopyImageLink(wallpaper_index).into(),
        ),
        i18n.t("download-tasks.tooltip-copy-url"),
        tooltip::Position::Right,
        _theme_config,
    );

    let info_column = column![
        info_row(i18n.t("wallpaper-info.resolution").as_str(), format!("{} x {}", wallpaper.width, wallpaper.height)),
        info_row(i18n.t("wallpaper-info.purity").as_str(), wallpaper.purity.to_uppercase()),
        info_row(
            i18n.t("wallpaper-info.favorites").as_str(),
            wallpaper.favorites.to_string()
        ),
        info_row(
            i18n.t("wallpaper-info.file-size").as_str(),
            format_file_size(wallpaper.file_size)
        ),
        row![copy_button].align_y(Alignment::Center),
    ]
    .spacing(4)
    .align_x(Alignment::Start);

    // 与底部工具栏同款半透明胶囊底色
    container(info_column)
        .padding([8, 12])
        .style(common::modal_overlay_style)
        .into()
}
