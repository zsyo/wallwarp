// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 本地壁纸模态预览的信息浮层（左上角半透明胶囊）

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common::{modal_info_pill, modal_info_row};
use crate::ui::style::ThemeConfig;
use crate::utils::helpers::format_file_size;
use iced::widget::column;
use iced::{Alignment, Element};

/// 创建本地壁纸信息浮层
pub fn create_modal_info<'a>(
    i18n: &'a I18n,
    wallpaper: &'a Wallpaper,
    _theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let info_column = column![
        modal_info_row(
            i18n.t("wallpaper-info.resolution").as_str(),
            format!("{} x {}", wallpaper.width, wallpaper.height)
        ),
        modal_info_row(
            i18n.t("wallpaper-info.file-size").as_str(),
            format_file_size(wallpaper.file_size)
        ),
    ]
    .spacing(4)
    .align_x(Alignment::Start);

    modal_info_pill(info_column.into())
}
