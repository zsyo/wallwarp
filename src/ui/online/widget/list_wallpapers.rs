// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::widget::{Id, column, scrollable, text};
use iced::{Alignment, Element, Length};

/// 创建壁纸列表内容
pub fn create_wallpaper_list<'a>(
    i18n: &'a I18n,
    window_width: u32,
    online_state: &'a OnlineState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let content: Element<'a, AppMessage> =
        if !online_state.has_loaded && !online_state.loading_page {
            // 初始状态，还未开始加载
            column![text(i18n.t("online-wallpapers.loading")).size(LOADING_TEXT_SIZE)]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding(EMPTY_STATE_PADDING)
                .into()
        } else if online_state.wallpapers.is_empty() && online_state.loading_page {
            // 正在加载中
            let theme_colors = theme_config.get_theme_colors();
            column![text(i18n.t("online-wallpapers.loading")).size(LOADING_TEXT_SIZE).style(
                move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                }
            )]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(EMPTY_STATE_PADDING)
            .into()
        } else if online_state.wallpapers.is_empty() && online_state.has_loaded {
            // 已加载但无数据
            let theme_colors = theme_config.get_theme_colors();
            column![
                text(i18n.t("online-wallpapers.no-data"))
                    .size(EMPTY_STATE_TEXT_SIZE)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme_colors.text),
                    }),
                text(i18n.t("online-wallpapers.no-data-hint"))
                    .size(14)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme_colors.light_text_sub),
                    }),
            ]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(EMPTY_STATE_PADDING)
            .spacing(10)
            .into()
        } else {
            super::create_wallpaper_grid(i18n, window_width, online_state, theme_config)
        };

    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(Id::new("online_wallpapers"))
        .on_scroll(|viewport| {
            // 检查是否滚动到底部
            let content_height = viewport.content_bounds().height;
            let view_height = viewport.bounds().height;
            let scroll_position = viewport.absolute_offset().y;

            // 计算可滚动的总距离
            let scrollable_height = content_height - view_height;

            if scrollable_height > 0.0 {
                // 有滚动条的情况：计算当前滚动百分比（0.0 到 1.0）
                let scroll_percentage = scroll_position / scrollable_height;

                // 当滚动到 95% 以上时触发加载
                let is_near_bottom = scroll_percentage >= 0.95;

                if is_near_bottom {
                    OnlineMessage::ScrollToBottom.into()
                } else {
                    AppMessage::None
                }
            } else {
                // 没有滚动条的情况：检测是否有滚轮事件
                let relative_offset = viewport.relative_offset().y;

                // 只有当向下滚动（relative_offset > 0）且在底部时才触发加载
                if relative_offset > 0.0 {
                    OnlineMessage::ScrollToBottom.into()
                } else {
                    AppMessage::None
                }
            }
        })
        .into()
}
