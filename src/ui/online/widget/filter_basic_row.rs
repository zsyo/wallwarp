// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Basic 档图源（wp.upx8）的筛选栏行
//!
//! 公共控件：图源 + 搜索框 + 排序 + 壁纸组 + 刷新；筛选项按图源能力追加：
//! wp.upx8 支持站点分类与分辨率（6 档），其余维度由 service 层忽略

use crate::i18n::I18n;
use crate::services::source::SourceKind;
use crate::ui::AppMessage;
use crate::ui::online::OnlineState;
use crate::ui::style::*;
use iced::widget::{Space, row};
use iced::{Alignment, Element};

/// 创建 Basic 档筛选栏行
pub(super) fn create_basic_filter_row<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    app_groups: &'a [crate::services::database::WallpaperGroupDB],
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let source_picker = super::create_source_picker(i18n, state, theme_colors);
    let group_picker = super::create_group_picker(i18n, app_groups, state, theme_colors);
    let refresh_button = super::filter_chips::create_refresh_button(theme_colors);
    let sorting_picker = super::create_sorting_picker(i18n, state, theme_colors);

    // 搜索框（放在最前面）
    let search_container = super::filter_chips::create_search_container(i18n, state, theme_colors);

    // 按图源能力追加筛选项
    let mut source_filters: Vec<Element<'a, AppMessage>> = Vec::new();
    match state.source {
        SourceKind::Upx8 => {
            source_filters.push(super::create_site_category_picker(i18n, state, theme_colors));
            source_filters.push(super::create_resolution_picker(i18n, state, theme_colors));
        }
        // Bare（Bing）与 Full（Wallhaven）与 Standard（wallpaperscraft）
        // 不经本函数，由 create_filter_bar 分派
        SourceKind::Bing | SourceKind::Wallhaven | SourceKind::Wallpaperscraft => unreachable!(),
    }

    let mut children: Vec<Element<'a, AppMessage>> = vec![source_picker];
    children.push(Space::new().width(2).into());
    children.push(search_container);
    children.push(Space::new().width(2).into());
    children.extend(source_filters);
    children.push(Space::new().width(2).into());
    children.push(sorting_picker);
    children.push(group_picker);
    children.push(refresh_button);

    super::filter_chips::wrap_filter_bar(
        row(children).spacing(4).align_y(Alignment::Center),
        theme_colors,
    )
}
