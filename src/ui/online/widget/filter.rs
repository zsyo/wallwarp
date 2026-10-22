// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::i18n::I18n;
use crate::services::source::SearchCapability;
use crate::services::wallhaven::{Category, Purity, Sorting};
use crate::ui::AppMessage;
use crate::ui::online::OnlineState;
use crate::ui::style::*;
use crate::utils::config::Config;
use iced::widget::{Space, row, text};
use iced::{Alignment, Element, Length};

/// 创建筛选栏
pub fn create_filter_bar<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    config: &'a Config,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 图源选择器（顶栏最前，各能力档位共用）
    let source_picker = super::create_source_picker(i18n, state, theme_colors);

    // 无搜索维度的图源（如 Bing 每日一图），仅展示说明与刷新按钮
    if state.source.search_capability() == SearchCapability::Bare {
        let hint_row = row![
            source_picker,
            Space::new().width(4),
            text(i18n.t("online-wallpapers.bing-daily-hint"))
                .size(14)
                .color(theme_colors.light_text_sub),
            Space::new().width(Length::Fill),
            super::filter_chips::create_refresh_button(theme_colors),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        return super::filter_chips::wrap_filter_bar(hint_row, theme_colors);
    }

    // Basic 档图源：公共控件 + 按图源能力追加的筛选项（见 filter_basic_row）
    if state.source.search_capability() == SearchCapability::Basic {
        return super::create_basic_filter_row(i18n, state, theme_colors);
    }

    // Standard 档图源（wallpaperscraft）：关键词 + 站点分类 + 分辨率 + 排序
    //
    // 站点无比例与颜色维度（比例只能靠分辨率档位间接表达，如 1920x1200 即
    // 16:10），故不渲染比例选择器，避免出现点击无效的控件
    if state.source.search_capability() == SearchCapability::Standard {
        let filter_row = row![
            source_picker,
            Space::new().width(2),
            super::filter_chips::create_search_container(i18n, state, theme_colors),
            Space::new().width(2),
            super::create_site_category_picker(i18n, state, theme_colors),
            super::create_resolution_picker(i18n, state, theme_colors),
            super::create_sorting_picker(i18n, state, theme_colors),
            super::filter_chips::create_refresh_button(theme_colors),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        return super::filter_chips::wrap_filter_bar(filter_row, theme_colors);
    }

    // 分辨率选择器 - 使用 DropDown 组件
    let resolution_picker = super::create_resolution_picker(i18n, state, theme_colors);

    // 比例选择器 - 使用 DropDown 组件（支持多选）
    let ratio_picker = super::create_ratio_picker(i18n, state, theme_colors);

    // 颜色选择器 - 使用 DropDown 组件
    let color_picker = super::create_color_picker(i18n, state, theme_colors);

    let sorting_picker = super::create_sorting_picker(i18n, state, theme_colors);

    let time_range_picker = super::create_time_range_picker(i18n, state, theme_colors);

    // 组合所有元素
    let filter_row = row![
        source_picker,
        Space::new().width(2),
        super::filter_chips::create_search_container(i18n, state, theme_colors),
        Space::new().width(2),
        super::filter_chips::category_chip(i18n, state, Category::General, theme_colors),
        super::filter_chips::category_chip(i18n, state, Category::Anime, theme_colors),
        super::filter_chips::category_chip(i18n, state, Category::People, theme_colors),
        Space::new().width(2),
        // NSFW 纯净度 chip：只在 API Key 不为空时显示
        super::filter_chips::purity_chip(i18n, state, Purity::SFW, theme_colors),
        super::filter_chips::purity_chip(i18n, state, Purity::Sketchy, theme_colors),
        if !config.wallhaven.api_key.is_empty() {
            Some(super::filter_chips::purity_chip(
                i18n,
                state,
                Purity::NSFW,
                theme_colors,
            ))
        } else {
            None
        },
        Space::new().width(2),
        resolution_picker,
        ratio_picker,
        color_picker,
        sorting_picker,
        // 时间范围选择器：仅在排序为 TopList 时显示
        if state.sorting == Sorting::TopList {
            Some(time_range_picker)
        } else {
            None
        },
        super::filter_chips::create_refresh_button(theme_colors),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    super::filter_chips::wrap_filter_bar(filter_row, theme_colors)
}
