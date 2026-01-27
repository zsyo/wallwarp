// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::AppMessage;
use super::common;
use crate::services::local::Wallpaper;
use crate::ui::style::{
    ALL_LOADED_TEXT_SIZE, BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, COLOR_MODAL_BG,
    COLOR_OVERLAY_BG, COLOR_OVERLAY_TEXT, EMPTY_STATE_PADDING, EMPTY_STATE_TEXT_SIZE, ERROR_ICON_SIZE, ERROR_PATH_SIZE,
    ERROR_TEXT_SIZE, IMAGE_HEIGHT, IMAGE_SPACING, IMAGE_WIDTH, LOADING_TEXT_SIZE, OVERLAY_HEIGHT, OVERLAY_TEXT_SIZE,
};
use crate::utils::config::Config;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Font, Length};

#[derive(Debug, Clone)]
pub enum LocalMessage {
    LoadWallpapers,
    LoadWallpapersSuccess(Vec<String>),
    LoadWallpapersFailed(String),
    LoadPage,
    LoadPageSuccess(Vec<(usize, Wallpaper)>),
    LoadPageFailed(String),
    WallpaperSelected(Wallpaper),
    ScrollToBottom,
    CheckAndLoadNextPage, // 检查是否需要自动加载下一页
    ShowModal(usize),
    CloseModal,
    NextImage,
    PreviousImage,
    ViewInFolder(usize),
    SetWallpaper(usize),
    ShowDeleteConfirm(usize),
    CloseDeleteConfirm,
    ConfirmDelete(usize),
    ModalImageLoaded(iced::widget::image::Handle),
}

#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    Loading,
    Loaded(Wallpaper),
}

#[derive(Debug)]
pub struct LocalState {
    pub is_visible: bool,
    pub wallpapers: Vec<WallpaperLoadStatus>,
    pub all_paths: Vec<String>,
    pub loading_page: bool,
    pub current_page: usize,
    pub page_size: usize,
    pub total_count: usize,
    pub modal_visible: bool,
    pub current_image_index: usize,
    pub delete_confirm_visible: bool,
    pub delete_target_index: Option<usize>,
    pub modal_image_handle: Option<iced::widget::image::Handle>,
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            is_visible: false,
            wallpapers: Vec::new(),
            all_paths: Vec::new(),
            loading_page: false,
            current_page: 0,
            page_size: 20,
            total_count: 0,
            modal_visible: false,
            current_image_index: 0,
            delete_confirm_visible: false,
            delete_target_index: None,
            modal_image_handle: None,
        }
    }
}

pub fn local_view<'a>(
    i18n: &'a crate::i18n::I18n,
    _config: &'a Config,
    window_width: u32,
    local_state: &'a LocalState,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, AppMessage> {
    let content = if local_state.all_paths.is_empty() {
        let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());
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
    } else {
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
                    WallpaperLoadStatus::Loading => create_loading_placeholder(i18n, theme_config),
                    WallpaperLoadStatus::Loaded(wallpaper) => {
                        let wallpaper_index = local_state
                            .all_paths
                            .iter()
                            .position(|p| p == &wallpaper.path)
                            .unwrap_or(0);

                        if wallpaper.name == "加载失败" {
                            create_error_placeholder(i18n, wallpaper, wallpaper_index, theme_config)
                        } else {
                            create_loaded_wallpaper(i18n, wallpaper, wallpaper_index, theme_config)
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
            let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());
            let all_loaded_text =
                text(i18n.t("local-list.all-loaded"))
                    .size(ALL_LOADED_TEXT_SIZE)
                    .style(move |_theme: &iced::Theme| text::Style {
                        color: Some(theme_colors.text),
                    });
            content = content.push(all_loaded_text)
        }

        column![
            iced::widget::Space::new().height(IMAGE_SPACING),
            content,
            iced::widget::Space::new().height(IMAGE_SPACING)
        ]
    };

    let base_layer = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(iced::widget::Id::new("local_wallpapers_scroll"))
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
                    super::AppMessage::Local(LocalMessage::ScrollToBottom)
                } else {
                    super::AppMessage::None
                }
            } else {
                // 没有滚动条的情况：检测是否有滚轮事件
                // 当内容高度小于等于视图高度时，通过 relative_offset().y 检测滚轮事件
                // 如果 relative_offset().y > 0 表示向下滚动
                let relative_offset = viewport.relative_offset().y;

                // 只有当向下滚动（relative_offset > 0）且在底部时才触发加载
                if relative_offset > 0.0 {
                    super::AppMessage::Local(LocalMessage::ScrollToBottom)
                } else {
                    super::AppMessage::None
                }
            }
        });

    let mut layers = vec![base_layer.into()];

    // 图片预览模态窗口
    if local_state.modal_visible && !local_state.all_paths.is_empty() {
        let wallpaper_index = local_state.current_image_index;

        // 创建背景加载文字
        let loading_text = create_modal_loading_placeholder(i18n);

        // 创建图片层（加载完成后显示）
        let image_layer: Element<_> = if let Some(ref handle) = local_state.modal_image_handle {
            // 使用预加载的图片数据
            let modal_image = iced::widget::image(handle.clone())
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill);
            modal_image.into()
        } else {
            // 图片未加载完成，显示透明占位符（让背景文字可见）
            container(iced::widget::Space::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        // 使用 stack 将图片层叠加在加载文字之上
        let modal_image_content = iced::widget::stack(vec![loading_text, image_layer]);

        // 创建底部工具栏按钮
        let prev_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F12E}",
                BUTTON_COLOR_BLUE,
                AppMessage::Local(LocalMessage::PreviousImage),
            ),
            i18n.t("local-list.tooltip-prev"),
            iced::widget::tooltip::Position::Top,
            theme_config,
        );

        let next_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F137}",
                BUTTON_COLOR_BLUE,
                AppMessage::Local(LocalMessage::NextImage),
            ),
            i18n.t("local-list.tooltip-next"),
            iced::widget::tooltip::Position::Top,
            theme_config,
        );

        let set_wallpaper_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F429}",
                BUTTON_COLOR_GREEN,
                AppMessage::Local(LocalMessage::SetWallpaper(wallpaper_index)),
            ),
            i18n.t("local-list.tooltip-set-wallpaper"),
            iced::widget::tooltip::Position::Top,
            theme_config,
        );

        let close_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F659}",
                BUTTON_COLOR_RED,
                AppMessage::Local(LocalMessage::CloseModal),
            ),
            i18n.t("local-list.tooltip-close"),
            iced::widget::tooltip::Position::Top,
            theme_config,
        );

        // 底部工具栏
        let toolbar = container(
            row![
                container(iced::widget::Space::new()).width(Length::Fill),
                prev_button,
                next_button,
                set_wallpaper_button,
                close_button,
                container(iced::widget::Space::new()).width(Length::Fill),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .spacing(50.0),
        )
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.7,
            })),
            ..Default::default()
        });

        let modal_content = container(
            column![
                container(modal_image_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20),
                toolbar,
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_MODAL_BG)),
            ..Default::default()
        });

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

fn create_loading_placeholder<'a>(
    i18n: &'a crate::i18n::I18n,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let loading_text =
        text(i18n.t("local-list.image-loading"))
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
}

fn create_error_placeholder<'a>(
    i18n: &'a crate::i18n::I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let error_image = text("\u{F428}")
        .font(Font::with_name("bootstrap-icons"))
        .color(Color::BLACK)
        .size(ERROR_ICON_SIZE);

    let error_text =
        text(i18n.t("local-list.loading-error"))
            .size(ERROR_TEXT_SIZE)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            });

    let error_path = text(&wallpaper.path)
        .size(ERROR_PATH_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let inner_content = container(
        column![error_image, error_text, error_path]
            .width(Length::Fill)
            .align_x(Alignment::Center),
    )
    .width(Length::Fixed(IMAGE_WIDTH))
    .height(Length::Fixed(IMAGE_HEIGHT))
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    let error_content = container(inner_content)
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

    // 创建遮罩层内容（不显示分辨率）
    let file_size_text = text(crate::utils::helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    let view_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F341}",
            BUTTON_COLOR_YELLOW,
            super::AppMessage::Local(LocalMessage::ViewInFolder(index)),
        ),
        i18n.t("local-list.tooltip-locate"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    let delete_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F78B}",
            BUTTON_COLOR_RED,
            super::AppMessage::Local(LocalMessage::ShowDeleteConfirm(index)),
        ),
        i18n.t("local-list.tooltip-delete"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：操作按钮
    let right_area = row![view_button, delete_button].spacing(2.0).align_y(Alignment::Center);

    // 遮罩层内容（左中右布局，中间为空，因为没有分辨率）
    let overlay_content = row![
        left_area,
        container(iced::widget::Space::new()).width(Length::Fill),
        right_area,
    ]
    .align_y(Alignment::Center)
    .padding([0, 8]);

    // 创建遮罩层
    let overlay = container(overlay_content)
        .width(Length::Fill)
        .height(Length::Fixed(OVERLAY_HEIGHT))
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_OVERLAY_BG)),
            ..Default::default()
        });

    // 使用 stack 将遮罩覆盖在错误占位图内部下方
    let card_content = iced::widget::stack(vec![
        error_content.into(),
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
        .style(|_theme, status| {
            let base_style = iced::widget::button::text(_theme, status);
            let shadow = match status {
                iced::widget::button::Status::Hovered => iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                    offset: iced::Vector { x: 0.0, y: 4.0 },
                    blur_radius: 12.0,
                },
                _ => iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: iced::Vector { x: 0.0, y: 2.0 },
                    blur_radius: 8.0,
                },
            };
            iced::widget::button::Style { shadow, ..base_style }
        })
}

fn create_loaded_wallpaper<'a>(
    i18n: &'a crate::i18n::I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let image_handle = iced::widget::image::Handle::from_path(&wallpaper.thumbnail_path);
    let image = iced::widget::image(image_handle)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .content_fit(iced::ContentFit::Fill);

    let styled_image = container(image)
        .width(Length::Fill)
        .height(Length::Fill)
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
    let file_size_text = text(crate::utils::helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.overlay_text),
        });

    let resolution_text = text(crate::utils::helpers::format_resolution(
        wallpaper.width,
        wallpaper.height,
    ))
    .size(OVERLAY_TEXT_SIZE)
    .style(move |_theme: &iced::Theme| text::Style {
        color: Some(COLOR_OVERLAY_TEXT),
    });

    let view_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F341}",
            BUTTON_COLOR_YELLOW,
            super::AppMessage::Local(LocalMessage::ViewInFolder(index)),
        ),
        i18n.t("local-list.tooltip-locate"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F429}",
            BUTTON_COLOR_GREEN,
            super::AppMessage::Local(LocalMessage::SetWallpaper(index)),
        ),
        i18n.t("local-list.tooltip-set-wallpaper"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    let delete_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F78B}",
            BUTTON_COLOR_RED,
            super::AppMessage::Local(LocalMessage::ShowDeleteConfirm(index)),
        ),
        i18n.t("local-list.tooltip-delete"),
        iced::widget::tooltip::Position::Top,
        theme_config,
    );

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：操作按钮
    let right_area = row![view_button, set_wallpaper_button, delete_button]
        .spacing(2.0)
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
        .style(|_theme, status| {
            let base_style = iced::widget::button::text(_theme, status);
            let shadow = match status {
                iced::widget::button::Status::Hovered => iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                    offset: iced::Vector { x: 0.0, y: 4.0 },
                    blur_radius: 12.0,
                },
                _ => iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: iced::Vector { x: 0.0, y: 2.0 },
                    blur_radius: 8.0,
                },
            };
            iced::widget::button::Style { shadow, ..base_style }
        })
        .on_press(super::AppMessage::Local(LocalMessage::ShowModal(index)))
}

impl LocalState {
    /// 查找下一个有效的图片索引
    pub fn find_next_valid_image_index(&self, start_index: usize, direction: i32) -> Option<usize> {
        if self.all_paths.is_empty() {
            return None;
        }

        let total = self.all_paths.len();
        let mut current_index = start_index;
        let loop_count = total; // 防止无限循环

        for _ in 0..loop_count {
            // 根据方向更新索引
            if direction > 0 {
                // 向前查找
                current_index = if current_index < total - 1 {
                    current_index + 1
                } else {
                    0
                };
            } else {
                // 向后查找
                current_index = if current_index > 0 {
                    current_index - 1
                } else {
                    total - 1
                };
            }

            // 检查是否回到起始位置
            if current_index == start_index {
                return None;
            }

            // 检查当前索引是否有效
            if let Some(wallpaper_status) = self.wallpapers.get(current_index) {
                if let WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                    if wallpaper.name != "加载失败" {
                        return Some(current_index);
                    }
                }
            }
        }

        None
    }
}

// 创建模态窗口加载占位符，静态文字显示
fn create_modal_loading_placeholder<'a>(i18n: &'a crate::i18n::I18n) -> Element<'a, AppMessage> {
    let loading_text = text(i18n.t("local-list.image-loading"))
        .size(24)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    container(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

impl From<LocalMessage> for AppMessage {
    fn from(local_message: LocalMessage) -> AppMessage {
        AppMessage::Local(local_message)
    }
}
