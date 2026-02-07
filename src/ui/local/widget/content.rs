// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::local::message::WallpaperLoadStatus;
use crate::ui::local::state::LocalState;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{ALL_LOADED_TEXT_SIZE, EMPTY_STATE_PADDING, EMPTY_STATE_TEXT_SIZE, IMAGE_SPACING, IMAGE_WIDTH};
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建内容展示区
pub fn create_content<'a>(
    i18n: &'a I18n,
    window_width: u32,
    local_state: &'a LocalState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let available_width = (window_width as f32 - IMAGE_SPACING).max(IMAGE_WIDTH);
    let unit_width = IMAGE_WIDTH + IMAGE_SPACING;
    let items_per_row = (available_width / unit_width).floor() as usize;
    let items_per_row = items_per_row.max(1);

    let mut content = column![]
        .spacing(IMAGE_SPACING)
        .width(Length::Fill)
        .align_x(Alignment::Center);

    for chunk in local_state.wallpapers.chunks(items_per_row) {
        let mut row_container = row![].spacing(IMAGE_SPACING).align_y(Alignment::Center);

        for wallpaper_status in chunk {
            let image_element = match wallpaper_status {
                WallpaperLoadStatus::Loading => super::create_loading_placeholder(i18n, theme_config),
                WallpaperLoadStatus::Loaded(wallpaper) => {
                    let wallpaper_index = local_state
                        .all_paths
                        .iter()
                        .position(|p| p == &wallpaper.path)
                        .unwrap_or(0);

                    if wallpaper.name == "加载失败" {
                        super::create_error_placeholder(i18n, wallpaper, wallpaper_index, theme_config)
                    } else {
                        super::create_loaded_wallpaper(i18n, wallpaper, wallpaper_index, theme_config)
                    }
                }
            };

            row_container = row_container.push(image_element);
        }

        let centered_row = container(row_container).width(Length::Fill).center_x(Length::Fill);
        content = content.push(centered_row);
    }

    // 如果已加载全部，显示提示文本
    if local_state.current_page * local_state.page_size >= local_state.total_count {
        let theme_colors = theme_config.get_theme_colors();
        let all_loaded_text =
            text(i18n.t("local-list.all-loaded"))
                .size(ALL_LOADED_TEXT_SIZE)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                });
        content = content.push(all_loaded_text)
    }

    column![
        Space::new().height(IMAGE_SPACING),
        content,
        Space::new().height(IMAGE_SPACING)
    ]
    .into()
}

/// 创建空内容展示区
pub fn create_empty_content<'a>(i18n: &'a I18n, theme_config: &'a ThemeConfig) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();
    column![
        text(i18n.t("local-list.no-wallpapers"))
            .size(EMPTY_STATE_TEXT_SIZE)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            })
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(EMPTY_STATE_PADDING)
    .into()
}
