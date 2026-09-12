// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面视图（网格卡片布局）

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::favorites::widget;
use crate::ui::favorites::{FavoritesMessage, FavoritesState};
use crate::ui::style::{EMPTY_STATE_TEXT_SIZE, IMAGE_SPACING, ThemeConfig};
use crate::ui::common::grid_items_per_row;
use iced::widget::{Id, Space, column, container, row, scrollable, text};
use iced::{Element, Length};

/// 收藏夹页视图
pub fn favorites_view<'a>(
    i18n: &'a I18n,
    favorites_state: &'a FavoritesState,
    theme_config: &'a ThemeConfig,
    window_width: u32,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let mut content = column![].spacing(IMAGE_SPACING).width(Length::Fill);

    // 顶部工具条（统计 + 类型/时间筛选 + 排序 + 刷新）
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
            let mut grid_row = row![].spacing(IMAGE_SPACING);
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
            // 行居中：与本地页一致，避免不满行时右侧留大片空白
            content = content.push(
                container(grid_row)
                    .width(Length::Fill)
                    .center_x(Length::Fill),
            );
        }

        // 还有未加载批次：网格底部提示（滚动到底部自动追加）
        if favorites_state.has_more() {
            content = content.push(
                container(
                    text(i18n.t("favorites.loading-more"))
                        .size(13)
                        .color(theme_colors.light_text_sub),
                )
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(8),
            );
        }
    }

    let base_layer = container(
        // 水平不留 padding（与本地页一致）：外层 padding 会挤占网格宽度，
        // 导致每行少放一张卡片；垂直留白用上下 Space 表达
        scrollable(
            column![
                Space::new().height(IMAGE_SPACING),
                content,
                container(iced::widget::Space::new()).height(IMAGE_SPACING)
            ]
            .width(Length::Fill),
        )
        .id(Id::new("favorites_scroll"))
        .width(Length::Fill)
        .height(Length::Fill)
        .on_scroll(|viewport| {
            // 滚动到 95% 以上触发追加下一批（与在线页分页阈值一致）
            let content_height = viewport.content_bounds().height;
            let view_height = viewport.bounds().height;
            let scrollable_height = content_height - view_height;
            let near_bottom = scrollable_height > 0.0 && {
                let scroll_percentage = viewport.absolute_offset().y / scrollable_height;
                scroll_percentage >= 0.95
            } || (scrollable_height <= 0.0 && viewport.relative_offset().y > 0.0);
            if near_bottom && favorites_state.has_more() {
                FavoritesMessage::LoadMore.into()
            } else {
                AppMessage::None
            }
        }),
    )
    .width(Length::Fill)
    .height(Length::Fill);

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

        // 按已入库状态区分在线项工具栏：已入库=在文件夹中查看，
        // 未入库=下载到壁纸库（本地项恒为查看）
        let modal_entry = favorites_state.entries.get(modal_index);
        let modal_in_library = modal_entry.is_some_and(|e| e.in_library);
        let modal_is_online =
            modal_fav.is_some_and(|f| f.kind == crate::services::database::KIND_ONLINE);

        let download_toolbar_button = (modal_is_online && !modal_in_library).then(|| {
            common::preview_toolbar_button(
                theme_config,
                theme_colors.disabled_color,
                "\u{F30A}", // download
                crate::ui::style::BUTTON_COLOR_BLUE,
                i18n.t("favorites.tooltip-download").to_string(),
                Some(FavoritesMessage::DownloadEntry(modal_index).into()),
            )
        });

        // 在线项未入库时下载原图：加载占位层显示环形进度
        // （本地项/缓存命中无进度，环为纯轨道圈 + 加载中文案）
        let loading_layer = common::create_progress_ring_placeholder(
            favorites_state.modal_download_progress,
            favorites_state.modal_downloaded_bytes,
            favorites_state.modal_total_bytes,
            i18n.t("favorites.loading").to_string(),
            theme_config,
        );

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
                view_in_folder: if !modal_is_online || modal_in_library {
                    Some(FavoritesMessage::OpenLocation(modal_index).into())
                } else {
                    None
                },
                close: FavoritesMessage::CloseModal.into(),
            },
            common::PreviewModalTexts {
                loading: String::new(), // 加载层由 extras 注入，文案不生效
                previous: i18n.t("favorites.tooltip-prev"),
                next: i18n.t("favorites.tooltip-next"),
                set_wallpaper: i18n.t("favorites.tooltip-set-wallpaper"),
                view_in_folder: i18n.t("favorites.tooltip-open-location"),
                close: i18n.t("favorites.tooltip-close"),
            },
            common::PreviewModalExtras {
                info_layer,
                loading_layer: Some(loading_layer),
                toolbar_buttons: download_toolbar_button.into_iter().collect(),
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

    iced::widget::stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
