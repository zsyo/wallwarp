// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::OnlineWallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::{OnlineMessage, OnlineState, WallpaperLoadStatus};
use crate::ui::style::{
    ALL_LOADED_TEXT_SIZE, BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, COLOR_OVERLAY_BG, EMPTY_STATE_PADDING,
    EMPTY_STATE_TEXT_SIZE, IMAGE_HEIGHT, IMAGE_SPACING, IMAGE_WIDTH, LOADING_TEXT_SIZE, OVERLAY_HEIGHT,
    OVERLAY_TEXT_SIZE, PAGE_SEPARATOR_HEIGHT, PAGE_SEPARATOR_TEXT_COLOR, PAGE_SEPARATOR_TEXT_SIZE,
};
use crate::utils::helpers;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

/// 创建壁纸列表内容
pub fn create_wallpaper_list<'a>(
    i18n: &'a I18n,
    window_width: u32,
    online_state: &'a OnlineState,
    theme_config: &'a crate::ui::style::ThemeConfig,
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
            let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());
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
            let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());
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
            create_wallpaper_grid(i18n, window_width, online_state, theme_config)
        };

    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(iced::widget::Id::new("online_wallpapers"))
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
                    AppMessage::Online(OnlineMessage::ScrollToBottom)
                } else {
                    AppMessage::None
                }
            } else {
                // 没有滚动条的情况：检测是否有滚轮事件
                let relative_offset = viewport.relative_offset().y;

                // 只有当向下滚动（relative_offset > 0）且在底部时才触发加载
                if relative_offset > 0.0 {
                    AppMessage::Online(OnlineMessage::ScrollToBottom)
                } else {
                    AppMessage::None
                }
            }
        })
        .into()
}

/// 创建壁纸网格内容
fn create_wallpaper_grid<'a>(
    i18n: &'a I18n,
    window_width: u32,
    online_state: &'a OnlineState,
    theme_config: &'a crate::ui::style::ThemeConfig,
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
                    WallpaperLoadStatus::Loading => create_loading_placeholder(i18n, theme_config),
                    WallpaperLoadStatus::ThumbLoaded(wallpaper, handle) => {
                        let wallpaper_index = online_state
                            .wallpapers
                            .iter()
                            .position(|w| matches!(w, WallpaperLoadStatus::ThumbLoaded(wp, _) if wp.id == wallpaper.id))
                            .unwrap_or(0);
                        create_loaded_wallpaper_with_thumb(
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
                        create_loaded_wallpaper_with_thumb(i18n, wallpaper, None, wallpaper_index, theme_config)
                    }
                };

                row_container = row_container.push(image_element);
            }

            let centered_row = container(row_container).width(Length::Fill).center_x(Length::Fill);
            content = content.push(centered_row);
        }

        // 在当前页数据后添加分页分隔线
        content = content.push(create_page_separator(
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
        iced::widget::Space::new().height(IMAGE_SPACING),
        content,
        iced::widget::Space::new().height(IMAGE_SPACING)
    ]
    .into()
}

/// 创建加载占位符
fn create_loading_placeholder<'a>(
    i18n: &'a I18n,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let loading_text = text(i18n.t("online-wallpapers.image-loading"))
        .size(LOADING_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let placeholder_content = container(loading_text)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_theme| {
            let mut style = common::create_bordered_container_style_with_bg(theme_config)(_theme);
            // 添加阴影效果
            style.shadow = iced::Shadow {
                color: theme_colors.overlay_bg,
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 8.0,
            };
            style
        });

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(|_theme, status| {
            let base_style = iced::widget::button::text(_theme, status);
            let shadow =
                crate::ui::style::get_card_shadow_by_status(matches!(status, iced::widget::button::Status::Hovered));
            iced::widget::button::Style { shadow, ..base_style }
        })
        .into()
}

/// 创建已加载的壁纸卡片
fn create_loaded_wallpaper_with_thumb<'a>(
    i18n: &'a I18n,
    wallpaper: &'a OnlineWallpaper,
    thumb_handle: Option<iced::widget::image::Handle>,
    index: usize,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    // 使用缩略图创建图片
    let image = if let Some(handle) = thumb_handle {
        iced::widget::image(handle)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .content_fit(iced::ContentFit::Fill)
    } else {
        // 如果没有缩略图，使用占位符
        let placeholder = text(i18n.t("online-wallpapers.loading-placeholder"))
            .size(LOADING_TEXT_SIZE)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            });

        return container(placeholder)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_theme| common::create_bordered_container_style_with_bg(theme_config)(_theme))
            .into();
    };

    let styled_image = container(image)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(move |_theme| {
            let mut style = common::create_bordered_container_style_with_bg(theme_config)(_theme);
            // 添加阴影效果
            style.shadow = iced::Shadow {
                color: theme_colors.overlay_bg,
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 8.0,
            };
            style
        });

    // 创建透明遮罩内容
    let file_size_text = text(helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.overlay_text),
        });

    let resolution_text = text(&wallpaper.resolution)
        .size(OVERLAY_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.overlay_text),
        });

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}",
            BUTTON_COLOR_GREEN,
            AppMessage::Online(OnlineMessage::SetAsWallpaper(index)),
        ),
        i18n.t("online-wallpapers.tooltip-set-wallpaper"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    let download_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F30A}",
            BUTTON_COLOR_BLUE,
            AppMessage::Online(OnlineMessage::DownloadWallpaper(index)),
        ),
        i18n.t("online-wallpapers.tooltip-download"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：设为壁纸按钮 + 下载按钮
    let right_area = row![set_wallpaper_button, download_button]
        .spacing(4)
        .align_y(Alignment::Center);

    // 使用 stack 确保分辨率永远居中，不受两侧内容影响
    let overlay_content = iced::widget::stack(vec![
        // 底层：左中右三部分布局
        container(
            row![
                left_area,
                // 中间占位，让分辨率在顶层居中
                container(iced::widget::Space::new()).width(Length::Fill),
                right_area,
            ]
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .padding([0, 8])
        .into(),
        // 顶层：分辨率居中显示
        container(resolution_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ]);

    // 创建遮罩层
    let overlay = container(overlay_content)
        .width(Length::Fill)
        .height(Length::Fixed(OVERLAY_HEIGHT))
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_OVERLAY_BG)),
            ..Default::default()
        });

    // 使用 stack 将遮罩覆盖在图片内部下方
    let card_content = iced::widget::stack(vec![
        styled_image.into(),
        container(overlay)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::End)
            .into(),
    ]);

    button(card_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .on_press(AppMessage::Online(OnlineMessage::ShowModal(index)))
        .style(|_theme, status| {
            let base_style = iced::widget::button::text(_theme, status);
            let shadow =
                crate::ui::style::get_card_shadow_by_status(matches!(status, iced::widget::button::Status::Hovered));
            iced::widget::button::Style { shadow, ..base_style }
        })
        .into()
}

/// 创建分页分隔线
fn create_page_separator<'a>(i18n: &'a I18n, current_page: usize, total_pages: usize) -> Element<'a, AppMessage> {
    let page_text = i18n
        .t("online-wallpapers.page-separator")
        .replace("{current}", &current_page.to_string())
        .replace("{total}", &total_pages.to_string());

    let separator = container(
        text(page_text)
            .size(PAGE_SEPARATOR_TEXT_SIZE)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(PAGE_SEPARATOR_TEXT_COLOR),
            }),
    )
    .width(Length::Fill)
    .height(Length::Fixed(PAGE_SEPARATOR_HEIGHT))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    container(separator).width(Length::Fill).padding([0, 20]).into()
}
