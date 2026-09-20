// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Basic 档图源（Pixabay/Unsplash/Danbooru）的筛选栏行
//!
//! 公共控件：图源 + 搜索框 + 排序 + 壁纸组 + 刷新；筛选项按图源能力追加：
//! Pixabay 有方向/分类/颜色/精选能力，Unsplash 有方向能力（颜色见
//! Unsplash service 映射就绪后补充），Danbooru 有 rating 分级能力
//! （纯净度 chip 写 purities 位掩码，由 service 层映射为 rating metatag）

use crate::i18n::I18n;
use crate::services::source::SourceKind;
use crate::services::wallhaven::Purity;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState};
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
        SourceKind::Pixabay => {
            source_filters.push(super::filter_chips::orientation_chip(
                i18n,
                state.ratio_landscape_selected,
                "online-wallpapers.ratio-landscape",
                OnlineMessage::RatioLandscapeToggled,
                theme_colors,
            ));
            source_filters.push(super::filter_chips::orientation_chip(
                i18n,
                state.ratio_portrait_selected,
                "online-wallpapers.ratio-portrait",
                OnlineMessage::RatioPortraitToggled,
                theme_colors,
            ));
            source_filters.push(super::create_pixabay_category_picker(
                i18n,
                state,
                theme_colors,
            ));
            source_filters.push(super::create_source_color_picker(i18n, state, theme_colors));
            source_filters.push(super::filter_chips::editors_choice_chip(
                i18n,
                state,
                theme_colors,
            ));
        }
        SourceKind::Unsplash => {
            source_filters.push(super::filter_chips::orientation_chip(
                i18n,
                state.ratio_landscape_selected,
                "online-wallpapers.ratio-landscape",
                OnlineMessage::RatioLandscapeToggled,
                theme_colors,
            ));
            source_filters.push(super::filter_chips::orientation_chip(
                i18n,
                state.ratio_portrait_selected,
                "online-wallpapers.ratio-portrait",
                OnlineMessage::RatioPortraitToggled,
                theme_colors,
            ));
        }
        SourceKind::Danbooru => {
            for purity in Purity::all() {
                source_filters.push(super::filter_chips::purity_chip(
                    i18n,
                    state,
                    purity,
                    theme_colors,
                ));
            }
        }
        // Bing（Bare）与 Wallhaven（Full）不经本函数，由 create_filter_bar 分派
        SourceKind::Bing | SourceKind::Wallhaven => unreachable!(),
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
