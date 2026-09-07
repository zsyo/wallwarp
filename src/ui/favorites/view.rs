// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面视图（网格卡片布局）

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::favorites::widget;
use crate::ui::favorites::{FavoritesMessage, FavoritesState};
use crate::ui::style::{EMPTY_STATE_TEXT_SIZE, IMAGE_SPACING, ThemeConfig};
use crate::ui::common::grid_items_per_row;
use iced::widget::{Id, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

/// 收藏夹页视图
pub fn favorites_view<'a>(
    i18n: &'a I18n,
    favorites_state: &'a FavoritesState,
    theme_config: &'a ThemeConfig,
    window_width: u32,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let mut content = column![].spacing(8).width(Length::Fill);

    // 顶部工具条（统计 + 分组筛选 + 新建/删除分组 + 刷新）
    content = content.push(widget::create_favorites_toolbar(
        i18n,
        favorites_state,
        theme_config,
    ));

    if favorites_state.entries.is_empty() {
        content = content.push(
            container(
                text(i18n.t("favorites.empty"))
                    .size(EMPTY_STATE_TEXT_SIZE)
                    .color(theme_colors.text),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        );
    } else {
        let items_per_row = grid_items_per_row(window_width);
        for (row_index, chunk) in favorites_state
            .entries
            .chunks(items_per_row)
            .enumerate()
        {
            let mut grid_row = row![].spacing(8);
            for (col_index, entry) in chunk.iter().enumerate() {
                let index = row_index * items_per_row + col_index;
                grid_row = grid_row.push(widget::create_favorite_card(
                    i18n,
                    index,
                    entry,
                    favorites_state
                        .thumbs
                        .get(index)
                        .unwrap_or(&crate::ui::favorites::state::ThumbState::Loading),
                    theme_config,
                ));
            }
            content = content.push(container(grid_row));
        }
    }

    let base_layer = container(
        scrollable(
            column![
                content,
                container(iced::widget::Space::new()).height(IMAGE_SPACING)
            ]
            .width(Length::Fill),
        )
        .id(Id::new("favorites_scroll"))
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20);

    let mut layers = vec![base_layer.into()];

    // 预览模态（共用预览组件）
    if favorites_state.modal_visible {
        let modal_index = favorites_state.modal_index;
        let modal_fav = favorites_state
            .entries
            .get(modal_index)
            .map(|e| &e.fav);

        let info_layer = modal_fav
            .map(|fav| widget::create_favorite_modal_info(i18n, fav, theme_config));

        let modal_content = common::create_preview_modal(
            i18n,
            theme_config,
            favorites_state.modal_handle.as_ref(),
            None,
            modal_index > 0,
            modal_index + 1 < favorites_state.entries.len(),
            common::PreviewModalMessages {
                previous: FavoritesMessage::PreviousImage.into(),
                next: FavoritesMessage::NextImage.into(),
                set_wallpaper: Some(FavoritesMessage::ApplyEntry(modal_index).into()),
                view_in_folder: if modal_fav
                    .is_some_and(|f| f.kind == crate::services::database::KIND_LOCAL)
                {
                    Some(FavoritesMessage::OpenLocation(modal_index).into())
                } else {
                    None
                },
                close: FavoritesMessage::CloseModal.into(),
            },
            common::PreviewModalTexts {
                loading: i18n.t("favorites.loading"),
                previous: i18n.t("favorites.tooltip-prev"),
                next: i18n.t("favorites.tooltip-next"),
                set_wallpaper: i18n.t("favorites.tooltip-set-wallpaper"),
                view_in_folder: i18n.t("favorites.tooltip-open-location"),
                close: i18n.t("favorites.tooltip-close"),
            },
            common::PreviewModalExtras {
                info_layer,
                ..Default::default()
            },
        );
        layers.push(container(iced::widget::opaque(modal_content)).into());
    }

    // 移除确认框
    if favorites_state.remove_target.is_some() {
        layers.push(common::create_confirmation_dialog(
            theme_colors,
            i18n.t("favorites.remove-confirm-title"),
            i18n.t("favorites.remove-confirm-message"),
            i18n.t("favorites.remove"),
            i18n.t("favorites.cancel"),
            FavoritesMessage::RemoveConfirmed.into(),
            FavoritesMessage::RemoveCanceled.into(),
        ));
    }

    // 新建分组对话框（输入 + 确认/取消）
    if favorites_state.create_group_visible {
        layers.push(create_group_dialog(i18n, favorites_state, theme_colors));
    }

    // 删除分组确认框
    if favorites_state.delete_group_confirm_visible {
        layers.push(common::create_confirmation_dialog(
            theme_colors,
            i18n.t("favorites.delete-group-confirm-title"),
            i18n.t("favorites.delete-group-confirm-message"),
            i18n.t("favorites.delete-group"),
            i18n.t("favorites.cancel"),
            FavoritesMessage::DeleteGroupConfirmed.into(),
            FavoritesMessage::DeleteGroupCanceled.into(),
        ));
    }

    iced::widget::stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// 新建分组对话框（标题 + 输入框 + 确认/取消）
fn create_group_dialog<'a>(
    i18n: &'a I18n,
    favorites_state: &'a FavoritesState,
    theme_colors: crate::ui::style::ThemeColors,
) -> Element<'a, AppMessage> {
    let dialog_content = column![
        text(i18n.t("favorites.create-group"))
            .size(16)
            .color(theme_colors.text)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        iced::widget::text_input(
            &i18n.t("favorites.create-group-placeholder"),
            &favorites_state.create_group_name,
        )
        .on_input(|text| FavoritesMessage::CreateGroupNameChanged(text).into())
        .on_submit(FavoritesMessage::CreateGroupConfirmed.into())
        .padding(6)
        .size(14)
        .width(Length::Fixed(280.0))
        .style(common::styled_text_input(theme_colors)),
        row![
            common::create_colored_button(
                i18n.t("favorites.create-group-confirm"),
                crate::ui::style::BUTTON_COLOR_BLUE,
                FavoritesMessage::CreateGroupConfirmed.into(),
            ),
            common::create_colored_button(
                i18n.t("favorites.cancel"),
                crate::ui::style::BUTTON_COLOR_GRAY,
                FavoritesMessage::CreateGroupCanceled.into(),
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    ]
    .padding(20)
    .spacing(12)
    .align_x(Alignment::Center)
    .width(Length::Shrink);

    common::modal_dialog_shell(theme_colors, dialog_content.into())
}
