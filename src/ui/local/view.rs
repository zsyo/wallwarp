// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 本地壁纸视图模块
//!
//! 定义本地壁纸页面的界面渲染逻辑

use super::message::LocalMessage;
use super::state::LocalState;
use super::widget;
use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::utils::config::Config;
use iced::widget::{Id, container, scrollable};
use iced::{Element, Length};

/// 本地壁纸页面视图函数
pub fn local_view<'a>(
    i18n: &'a I18n,
    _config: &'a Config,
    window_width: u32,
    local_state: &'a LocalState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let content = if local_state.all_paths.is_empty() {
        widget::create_empty_content(i18n, theme_config)
    } else {
        widget::create_content(i18n, window_width, local_state, theme_config)
    };

    let base_layer = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(Id::new("local_wallpapers_scroll"))
        .on_scroll(|viewport| {
            // 检查是否滚动到底部
            // 使用 offset 和 content_size 来判断滚动位置
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
                    AppMessage::Local(LocalMessage::ScrollToBottom)
                } else {
                    AppMessage::None
                }
            } else {
                // 没有滚动条的情况：检测是否有滚轮事件
                // 当内容高度小于等于视图高度时，通过 relative_offset().y 检测滚轮事件
                // 如果 relative_offset().y > 0 表示向下滚动
                let relative_offset = viewport.relative_offset().y;

                // 只有当向下滚动（relative_offset > 0）且在底部时才触发加载
                if relative_offset > 0.0 {
                    AppMessage::Local(LocalMessage::ScrollToBottom)
                } else {
                    AppMessage::None
                }
            }
        });

    let mut layers = vec![base_layer.into()];

    // 图片预览模态窗口
    if local_state.modal_visible && !local_state.all_paths.is_empty() {
        let modal_content = widget::create_modal(i18n, local_state, theme_config);
        layers.push(container(iced::widget::opaque(modal_content)).into());
    }

    // 删除确认模态窗口
    if local_state.delete_confirm_visible {
        let delete_confirm_dialog = common::create_confirmation_dialog(
            i18n.t("local-list.delete-confirm-title"),
            i18n.t("local-list.delete-confirm-message"),
            i18n.t("local-list.delete-confirm-confirm"),
            i18n.t("local-list.delete-confirm-cancel"),
            AppMessage::Local(LocalMessage::ConfirmDelete(
                local_state.delete_target_index.unwrap_or(0),
            )),
            AppMessage::Local(LocalMessage::CloseDeleteConfirm),
        );
        layers.push(delete_confirm_dialog);
    }

    iced::widget::stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
