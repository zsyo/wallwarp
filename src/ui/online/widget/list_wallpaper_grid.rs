// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineState, WallpaperLoadStatus};
use crate::ui::style::*;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建壁纸网格内容
pub fn create_wallpaper_grid<'a>(
    i18n: &'a I18n,
    window_width: u32,
    online_state: &'a OnlineState,
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

    // 按页渲染数据，实现类似PDF的分页效果
    let mut start_index = 0;
    for (_page_idx, page_info) in online_state.page_info.iter().enumerate() {
        // 获取当前页的数据范围
        let end_index = page_info.end_index;
        let page_wallpapers = &online_state.wallpapers[start_index..end_index];

        // 渲染当前页的壁纸
        for chunk in page_wallpapers.chunks(items_per_row) {
            let mut row_container = row![].spacing(IMAGE_SPACING).align_y(Alignment::Center);

            for wallpaper_status in chunk {
                let image_element = match wallpaper_status {
                    WallpaperLoadStatus::Loading => super::create_loading_placeholder(i18n, theme_config),
                    WallpaperLoadStatus::ThumbLoaded(wallpaper, handle) => {
                        let wallpaper_index = online_state
                            .wallpapers
                            .iter()
                            .position(|w| matches!(w, WallpaperLoadStatus::ThumbLoaded(wp, _) if wp.id == wallpaper.id))
                            .unwrap_or(0);
                        super::create_loaded_wallpaper_with_thumb(
                            i18n,
                            wallpaper,
                            Some(handle.clone()),
                            wallpaper_index,
                            theme_config,
                        )
                    }
                    WallpaperLoadStatus::Loaded(wallpaper) => {
                        let wallpaper_index = online_state
                            .wallpapers
                            .iter()
                            .position(|w| matches!(w, WallpaperLoadStatus::Loaded(wp) if wp.id == wallpaper.id))
                            .unwrap_or(0);
                        super::create_loaded_wallpaper_with_thumb(i18n, wallpaper, None, wallpaper_index, theme_config)
                    }
                };

                row_container = row_container.push(image_element);
            }

            let centered_row = container(row_container).width(Length::Fill).center_x(Length::Fill);
            content = content.push(centered_row);
        }

        // 在当前页数据后添加分页分隔线
        content = content.push(super::create_page_separator(
            i18n,
            page_info.page_num,
            online_state.total_pages,
        ));

        // 更新下一页的起始索引
        start_index = end_index;
    }

    // 如果是最后一页，显示"已加载全部"
    if online_state.last_page {
        let all_loaded_text = text(i18n.t("online-wallpapers.all-loaded")).size(ALL_LOADED_TEXT_SIZE);
        content = content.push(all_loaded_text)
    }

    column![
        Space::new().height(IMAGE_SPACING),
        content,
        Space::new().height(IMAGE_SPACING)
    ]
    .into()
}
