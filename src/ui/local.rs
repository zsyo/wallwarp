use super::AppMessage;
use super::common;
use crate::services::local::Wallpaper;
use crate::utils::config::Config;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Font, Length};

// 布局常量
const IMAGE_WIDTH: f32 = 300.0;
const IMAGE_HEIGHT: f32 = 200.0;
const IMAGE_SPACING: f32 = 20.0;
const EMPTY_STATE_PADDING: u16 = 360;
const EMPTY_STATE_TEXT_SIZE: f32 = 24.0;

// 加载文本常量
const LOADING_TEXT_SIZE: f32 = 24.0;

// 错误占位图常量
const ERROR_ICON_SIZE: f32 = 56.0;
const ERROR_TEXT_SIZE: f32 = 18.0;
const ERROR_PATH_SIZE: f32 = 10.0;

// 按钮常量
const LOAD_MORE_TEXT_SIZE: f32 = 16.0;
const LOAD_MORE_PADDING: u16 = 10;
const ALL_LOADED_TEXT_SIZE: f32 = 14.0;

// 透明遮罩常量
const OVERLAY_HEIGHT: f32 = 22.0;
const OVERLAY_TEXT_SIZE: f32 = 12.0;

// 容器样式常量
const BORDER_WIDTH: f32 = 1.0;
const BORDER_RADIUS: f32 = 4.0;

// 颜色常量
const COLOR_BG_LIGHT: Color = Color::from_rgb(0.9, 0.9, 0.9);
const COLOR_TEXT_DARK: Color = Color::from_rgb(0.3, 0.3, 0.3);
const COLOR_MODAL_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.85);
const COLOR_OVERLAY_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);
const COLOR_OVERLAY_TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);

// 模态窗口加载占位符常量
const MODAL_LOADING_TEXT_SIZE: f32 = 20.0;

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
    AnimationTick,
    ShowModal(usize),
    CloseModal,
    NextImage,
    PreviousImage,
    AnimatedFrameUpdate,
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
    pub wallpapers: Vec<WallpaperLoadStatus>,
    pub all_paths: Vec<String>,
    pub loading_page: bool,
    pub current_page: usize,
    pub page_size: usize,
    pub total_count: usize,
    pub modal_visible: bool,
    pub current_image_index: usize,
    pub animated_decoder: Option<crate::utils::animated_image::AnimatedDecoder>,
    pub delete_confirm_visible: bool,
    pub delete_target_index: Option<usize>,
    pub modal_image_handle: Option<iced::widget::image::Handle>,
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            wallpapers: Vec::new(),
            all_paths: Vec::new(),
            loading_page: false,
            current_page: 0,
            page_size: 20,
            total_count: 0,
            modal_visible: false,
            current_image_index: 0,
            animated_decoder: None,
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
) -> Element<'a, AppMessage> {
    let content = if local_state.all_paths.is_empty() {
        column![text(i18n.t("local-list.no-wallpapers")).size(EMPTY_STATE_TEXT_SIZE)]
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
                    WallpaperLoadStatus::Loading => create_loading_placeholder(i18n),
                    WallpaperLoadStatus::Loaded(wallpaper) => {
                        let wallpaper_index = local_state
                            .all_paths
                            .iter()
                            .position(|p| p == &wallpaper.path)
                            .unwrap_or(0);

                        if wallpaper.name == "加载失败" {
                            create_error_placeholder(i18n, wallpaper, wallpaper_index)
                        } else {
                            create_loaded_wallpaper(i18n, wallpaper, wallpaper_index)
                        }
                    }
                };

                row_container = row_container.push(image_element);
            }

            let centered_row = container(row_container).width(Length::Fill).center_x(Length::Fill);
            content = content.push(centered_row);
        }

        if local_state.current_page * local_state.page_size < local_state.total_count && !local_state.loading_page {
            let load_more_button = button(text(i18n.t("local-list.load-more")).size(LOAD_MORE_TEXT_SIZE))
                .padding(LOAD_MORE_PADDING)
                .on_press(super::AppMessage::Local(LocalMessage::ScrollToBottom));
            content = content.push(load_more_button)
        } else if local_state.current_page * local_state.page_size >= local_state.total_count {
            let all_loaded_text = text(i18n.t("local-list.all-loaded")).size(ALL_LOADED_TEXT_SIZE);
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
        .id(iced::widget::Id::new("local_wallpapers_scroll"));

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
        } else if let Some(ref decoder) = local_state.animated_decoder {
            // 动态图
            let current_frame = decoder.current_frame();
            let modal_image = iced::widget::image(current_frame.handle.clone())
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
        let modal_image_content = iced::widget::stack(vec![
            loading_text,
            image_layer,
        ]);

        // 创建底部工具栏按钮
        let prev_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F12E}",
                common::BUTTON_COLOR_BLUE,
                AppMessage::Local(LocalMessage::PreviousImage),
            ),
            i18n.t("local-list.tooltip-prev"),
        );

        let next_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F137}",
                common::BUTTON_COLOR_BLUE,
                AppMessage::Local(LocalMessage::NextImage),
            ),
            i18n.t("local-list.tooltip-next"),
        );

        let set_wallpaper_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F196}",
                common::BUTTON_COLOR_GREEN,
                AppMessage::Local(LocalMessage::SetWallpaper(wallpaper_index)),
            ),
            i18n.t("local-list.tooltip-set-wallpaper"),
        );

        let close_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F659}",
                common::BUTTON_COLOR_RED,
                AppMessage::Local(LocalMessage::CloseModal),
            ),
            i18n.t("local-list.tooltip-close"),
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

fn create_loading_placeholder<'a>(i18n: &'a crate::i18n::I18n) -> button::Button<'a, AppMessage> {
    let loading_text =
        text(i18n.t("local-list.image-loading"))
            .size(LOADING_TEXT_SIZE)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(COLOR_TEXT_DARK),
            });

    let placeholder_content = container(loading_text)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(create_bordered_container_style);

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
}

fn create_error_placeholder<'a>(
    i18n: &'a crate::i18n::I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
) -> button::Button<'a, AppMessage> {
    let error_image = text("\u{F428}")
        .font(Font::with_name("bootstrap-icons"))
        .color(Color::BLACK)
        .size(ERROR_ICON_SIZE);

    let error_text = text(i18n.t("local-list.loading-error"))
        .size(ERROR_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_TEXT_DARK),
        });

    let error_path = text(&wallpaper.path)
        .size(ERROR_PATH_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_TEXT_DARK),
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
        .style(create_bordered_container_style);

    // 创建遮罩层内容（不显示分辨率）
    let file_size_text = text(crate::utils::helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    let view_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F341}",
            common::BUTTON_COLOR_YELLOW,
            super::AppMessage::Local(LocalMessage::ViewInFolder(index)),
        ),
        i18n.t("local-list.tooltip-locate"),
    );

    let delete_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F78B}",
            common::BUTTON_COLOR_RED,
            super::AppMessage::Local(LocalMessage::ShowDeleteConfirm(index)),
        ),
        i18n.t("local-list.tooltip-delete"),
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
}

fn create_loaded_wallpaper<'a>(
    i18n: &'a crate::i18n::I18n,
    wallpaper: &'a Wallpaper,
    index: usize,
) -> button::Button<'a, AppMessage> {
    let image_handle = iced::widget::image::Handle::from_path(&wallpaper.thumbnail_path);
    let image = iced::widget::image(image_handle)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .content_fit(iced::ContentFit::Fill);

    let styled_image = container(image)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(create_bordered_container_style);

    // 创建透明遮罩内容
    let file_size_text = text(crate::utils::helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    let resolution_text = text(crate::utils::helpers::format_resolution(
        wallpaper.width,
        wallpaper.height,
    ))
    .size(OVERLAY_TEXT_SIZE)
    .style(|_theme: &iced::Theme| text::Style {
        color: Some(COLOR_OVERLAY_TEXT),
    });

    let view_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F341}",
            common::BUTTON_COLOR_YELLOW,
            super::AppMessage::Local(LocalMessage::ViewInFolder(index)),
        ),
        i18n.t("local-list.tooltip-locate"),
    );

    let set_wallpaper_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F196}",
            common::BUTTON_COLOR_GREEN,
            super::AppMessage::Local(LocalMessage::SetWallpaper(index)),
        ),
        i18n.t("local-list.tooltip-set-wallpaper"),
    );

    let delete_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F78B}",
            common::BUTTON_COLOR_RED,
            super::AppMessage::Local(LocalMessage::ShowDeleteConfirm(index)),
        ),
        i18n.t("local-list.tooltip-delete"),
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
        .on_press(super::AppMessage::Local(LocalMessage::ShowModal(index)))
}

fn create_bordered_container_style(theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(COLOR_BG_LIGHT)),
        border: iced::border::Border {
            color: theme.extended_palette().primary.weak.color,
            width: BORDER_WIDTH,
            radius: iced::border::Radius::from(BORDER_RADIUS),
        },
        ..Default::default()
    }
}

// 创建模态窗口加载占位符，静态文字显示
fn create_modal_loading_placeholder<'a>(
    i18n: &'a crate::i18n::I18n,
) -> Element<'a, AppMessage> {
    let loading_text =
        text(i18n.t("local-list.image-loading"))
            .size(MODAL_LOADING_TEXT_SIZE)
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
