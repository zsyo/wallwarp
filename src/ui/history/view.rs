// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸历史页面视图（SnowShot 式列表布局）

use crate::i18n::I18n;
use crate::ui::common;
use crate::ui::history::widget;
use crate::ui::history::{HistoryMessage, HistoryState};
use crate::ui::style::{EMPTY_STATE_TEXT_SIZE, IMAGE_SPACING, ThemeConfig};
use crate::ui::AppMessage;
use iced::widget::{Id, column, container, rule, scrollable, text};
use iced::{Element, Length};

/// 历史页视图
pub fn history_view<'a>(
    i18n: &'a I18n,
    history_state: &'a HistoryState,
    theme_config: &'a ThemeConfig,
    _window_width: u32,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let mut content = column![].spacing(8).width(Length::Fill);

    // 顶部工具条（统计 + 刷新 + 清空）
    content = content.push(widget::create_history_toolbar(i18n, history_state, theme_config));

    if history_state.entries.is_empty() {
        content = content.push(
            container(
                text(i18n.t("history.empty"))
                    .size(EMPTY_STATE_TEXT_SIZE)
                    .color(theme_colors.text),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        );
    } else {
        for (index, entry) in history_state.entries.iter().enumerate() {
            content = content.push(widget::create_history_row(
                i18n,
                index,
                entry,
                history_state.thumbs.get(index).and_then(|t| t.as_ref()),
                history_state.wallpapers.get(index).and_then(|w| w.as_ref()),
                theme_config,
            ));

            // 行间分隔线（最后一条之后不加）
            if index + 1 < history_state.entries.len() {
                content = content.push(rule::horizontal(1));
            }
        }
    }

    let base_layer = container(
        scrollable(
            column![content, container(iced::widget::Space::new()).height(IMAGE_SPACING)]
                .width(Length::Fill),
        )
        .id(Id::new("history_scroll"))
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20);

    let mut layers = vec![base_layer.into()];

    // 预览模态（共用预览组件）
    if history_state.modal_visible {
        let modal_index = history_state.modal_index;
        let modal_content = common::create_preview_modal(
            i18n,
            theme_config,
            history_state.modal_handle.as_ref(),
            history_state
                .wallpapers
                .get(modal_index)
                .and_then(|w| w.as_ref()),
            modal_index > 0,
            modal_index + 1 < history_state.entries.len(),
            common::PreviewModalMessages {
                previous: HistoryMessage::PreviousImage.into(),
                next: HistoryMessage::NextImage.into(),
                set_wallpaper: HistoryMessage::ApplyEntry(modal_index).into(),
                view_in_folder: HistoryMessage::OpenLocation(modal_index).into(),
                close: HistoryMessage::CloseModal.into(),
            },
            common::PreviewModalTexts {
                loading: i18n.t("history.loading"),
                previous: i18n.t("history.tooltip-prev"),
                next: i18n.t("history.tooltip-next"),
                set_wallpaper: i18n.t("history.apply"),
                view_in_folder: i18n.t("history.open-location"),
                close: i18n.t("history.tooltip-close"),
            },
        );
        layers.push(container(iced::widget::opaque(modal_content)).into());
    }

    // 移除确认框
    if history_state.remove_target.is_some() {
        layers.push(common::create_confirmation_dialog(
            theme_colors,
            i18n.t("history.remove-confirm-title"),
            i18n.t("history.remove-confirm-message"),
            i18n.t("history.remove"),
            i18n.t("history.cancel"),
            HistoryMessage::RemoveConfirmed.into(),
            HistoryMessage::RemoveCanceled.into(),
        ));
    }

    // 清空确认框
    if history_state.clear_confirm_visible {
        layers.push(common::create_confirmation_dialog(
            theme_colors,
            i18n.t("history.clear-confirm-title"),
            i18n.t("history.clear-confirm-message"),
            i18n.t("history.clear"),
            i18n.t("history.cancel"),
            HistoryMessage::ClearConfirmed.into(),
            HistoryMessage::ClearCanceled.into(),
        ));
    }

    iced::widget::stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
