use super::AppMessage;
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

// 加载动画常量
const LOADING_DOT_SIZE: f32 = 24.0;
const LOADING_DOT_SPACING: f32 = 3.0;
const LOADING_TEXT_SIZE: f32 = 24.0;
const LOADING_INNER_SPACING: f32 = 5.0;

// 错误占位图常量
const ERROR_ICON_SIZE: f32 = 56.0;
const ERROR_TEXT_SIZE: f32 = 18.0;
const ERROR_PATH_SIZE: f32 = 10.0;

// 按钮常量
const LOAD_MORE_TEXT_SIZE: f32 = 16.0;
const LOAD_MORE_PADDING: u16 = 10;
const ALL_LOADED_TEXT_SIZE: f32 = 14.0;

// 模态窗口常量
const MODAL_BUTTON_PADDING: [u16; 2] = [10, 20];
const MODAL_CLOSE_PADDING: [u16; 2] = [5, 10];
const MODAL_TOP_PADDING: u16 = 10;
const MODAL_BUTTON_SPACING: f32 = 10.0;

// 容器样式常量
const BORDER_WIDTH: f32 = 1.0;
const BORDER_RADIUS: f32 = 4.0;

// 颜色常量
const COLOR_BG_LIGHT: Color = Color::from_rgb(0.9, 0.9, 0.9);
const COLOR_DOT_INACTIVE: Color = Color::from_rgb(0.7, 0.7, 0.7);
const COLOR_TEXT_DARK: Color = Color::from_rgb(0.3, 0.3, 0.3);
const COLOR_MODAL_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.85);

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
    pub rotation_angle: f32,
    pub modal_visible: bool,
    pub current_image_index: usize,
    pub animated_decoder: Option<crate::utils::animated_image::AnimatedDecoder>,
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
            rotation_angle: 0.0,
            modal_visible: false,
            current_image_index: 0,
            animated_decoder: None,
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
                    WallpaperLoadStatus::Loading => {
                        create_loading_placeholder(i18n, local_state.rotation_angle)
                    }
                    WallpaperLoadStatus::Loaded(wallpaper) => {
                        if wallpaper.name == "加载失败" {
                            create_error_placeholder(i18n, wallpaper)
                        } else {
                            let wallpaper_index = local_state
                                .all_paths
                                .iter()
                                .position(|p| p == &wallpaper.path)
                                .unwrap_or(0);
                            create_loaded_wallpaper(wallpaper, wallpaper_index)
                        }
                    }
                };

                row_container = row_container.push(image_element);
            }

            let centered_row = container(row_container)
                .width(Length::Fill)
                .center_x(Length::Fill);
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

    if local_state.modal_visible && !local_state.all_paths.is_empty() {
        let current_path = &local_state.all_paths[local_state.current_image_index];

        let modal_image = if let Some(ref decoder) = local_state.animated_decoder {
            let current_frame = decoder.current_frame();
            iced::widget::image(current_frame.handle.clone())
        } else {
            let image_handle = iced::widget::image::Handle::from_path(current_path);
            iced::widget::image(image_handle)
        };

        let modal_image = modal_image
            .content_fit(iced::ContentFit::Contain)
            .width(Length::Fill)
            .height(Length::Fill);

        let prev_button = button(text("<"))
            .padding(MODAL_BUTTON_PADDING)
            .on_press(AppMessage::Local(LocalMessage::PreviousImage));

        let next_button = button(text(">"))
            .padding(MODAL_BUTTON_PADDING)
            .on_press(AppMessage::Local(LocalMessage::NextImage));

        let close_button = button(text("×"))
            .padding(MODAL_CLOSE_PADDING)
            .on_press(AppMessage::Local(LocalMessage::CloseModal));

        let modal_content = container(column![
            row![
                container(iced::widget::Space::new())
                    .width(Length::Fill)
                    .height(Length::Shrink),
                close_button
            ]
            .padding(MODAL_TOP_PADDING),
            row![
                container(prev_button)
                    .width(Length::Shrink)
                    .center_y(Length::Fill),
                container(modal_image)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
                container(next_button)
                    .width(Length::Shrink)
                    .center_y(Length::Fill),
            ]
            .spacing(MODAL_BUTTON_SPACING)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_MODAL_BG)),
            ..Default::default()
        });

        iced::widget::stack(vec![
            base_layer.into(),
            container(iced::widget::opaque(modal_content)).into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        base_layer.into()
    }
}

fn create_loading_placeholder<'a>(
    i18n: &'a crate::i18n::I18n,
    rotation_angle: f32,
) -> button::Button<'a, AppMessage> {
    let dots = (0..3)
        .map(|i| {
            if i as f32 == (rotation_angle / 120.0).floor() % 3.0 {
                text("●").size(LOADING_DOT_SIZE).into()
            } else {
                text("●")
                    .size(LOADING_DOT_SIZE)
                    .style(|_theme: &iced::Theme| text::Style {
                        color: Some(COLOR_DOT_INACTIVE),
                    })
                    .into()
            }
        })
        .collect::<Vec<_>>();

    let loading_image = row(dots).spacing(LOADING_DOT_SPACING);

    let loading_text = text(i18n.t("local-list.image-loading"))
        .size(LOADING_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_TEXT_DARK),
        });

    let inner_content = container(
        column![loading_image, loading_text]
            .width(Length::Shrink)
            .height(Length::Shrink)
            .align_x(Alignment::Center)
            .spacing(LOADING_INNER_SPACING),
    )
    .width(Length::Fixed(IMAGE_WIDTH))
    .height(Length::Fixed(IMAGE_HEIGHT))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    let placeholder_content = container(inner_content)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(create_bordered_container_style);

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
}

fn create_error_placeholder<'a>(
    i18n: &'a crate::i18n::I18n,
    wallpaper: &'a Wallpaper,
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

    button(error_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
}

fn create_loaded_wallpaper<'a>(
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

    button(styled_image)
        .padding(0)
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

impl From<LocalMessage> for AppMessage {
    fn from(local_message: LocalMessage) -> AppMessage {
        AppMessage::Local(local_message)
    }
}