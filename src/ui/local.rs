use super::AppMessage;
use crate::services::local::Wallpaper;
use crate::utils::config::Config;
use iced::widget::{Id, button, column, row, scrollable, text};
use iced::{Alignment, Color, Element, Font, Length};

#[derive(Debug, Clone)]
pub enum LocalMessage {
    LoadWallpapers,                           // 加载壁纸列表（路径）
    LoadWallpapersSuccess(Vec<String>),       // 加载壁纸路径列表成功
    LoadWallpapersFailed(String),             // 加载壁纸列表失败
    LoadPage,                                 // 加载当前页面的壁纸
    LoadPageSuccess(Vec<(usize, Wallpaper)>), // 加载页面壁纸成功（索引，壁纸）
    LoadPageFailed(String),                   // 加载页面壁纸失败
    WallpaperSelected(Wallpaper),             // 选择壁纸
    ScrollToBottom,                           // 滚动到底部，加载更多
    AnimationTick,                            // 动画定时器
    // 模态窗口相关消息
    ShowModal(usize),             // 显示模态窗口，参数为当前壁纸索引
    CloseModal,                   // 关闭模态窗口
    NextImage,                    // 下一张图片
    PreviousImage,                // 上一张图片
    AnimatedFrameUpdate,          // 更新动态图帧
}

// 定义单个壁纸的加载状态
#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    Loading,           // 正在加载
    Loaded(Wallpaper), // 已加载完成
}

// 定义加载状态
#[derive(Debug)]
pub struct LocalState {
    pub wallpapers: Vec<WallpaperLoadStatus>, // 使用新的加载状态枚举
    pub all_paths: Vec<String>,               // 存储所有壁纸路径
    pub loading_page: bool,                   // 是否正在加载当前页面
    pub current_page: usize,                  // 当前页码
    pub page_size: usize,                     // 每页显示数量
    pub total_count: usize,                   // 总壁纸数量
    pub rotation_angle: f32,                  // 加载动画的旋转角度
    // 模态窗口相关状态
    pub modal_visible: bool,        // 模态窗口是否可见
    pub current_image_index: usize, // 当前显示的图片索引
    pub animated_decoder: Option<crate::utils::animated_image::AnimatedDecoder>, // 动态图解码器
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            wallpapers: Vec::new(),
            all_paths: Vec::new(),
            loading_page: false,
            current_page: 0,
            page_size: 20, // 默认每页20张
            total_count: 0,
            rotation_angle: 0.0,    // 初始旋转角度为0
            modal_visible: false,   // 默认不显示模态窗口
            current_image_index: 0, // 默认当前图片索引为0
            animated_decoder: None, // 默认没有动态图解码器
        }
    }
}

pub fn local_view<'a>(
    i18n: &'a crate::i18n::I18n,
    _config: &'a Config,
    window_width: u32,
    local_state: &'a LocalState,
) -> Element<'a, AppMessage> {
    // 正常内容区域
    let content = if local_state.all_paths.is_empty() {
        // 显示空状态
        column![text(i18n.t("local-list.no-wallpapers")).size(24)]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(360)
    } else {
        // 固定图片尺寸为300*200
        let image_width = 300.0;
        let image_height = 200.0;
        let spacing = 20.0; // 图片之间的间距

        // 计算实际可用于显示图片的区域宽度
        // window_width 是整个窗口宽度，需要减去外侧区域宽度
        let available_width = (window_width as f32 - spacing).max(image_width);

        // 计算每行能“完整”容纳的图片数量
        let unit_width = image_width + spacing;
        let items_per_row = (available_width / unit_width).floor() as usize;
        let items_per_row = items_per_row.max(1);

        // 创建图片网格容器
        let mut content = column![]
            .spacing(spacing)
            .width(Length::Fill) // 撑满右侧区域
            .align_x(Alignment::Center); // 使内部的行容器整体居中

        // 按行组织图片 - 使用wallpapers来构建网格
        for chunk in local_state.wallpapers.chunks(items_per_row) {
            let mut row_container = row![].spacing(spacing).align_y(Alignment::Center);

            for wallpaper_status in chunk {
                let image_element = match wallpaper_status {
                    WallpaperLoadStatus::Loading => {
                        // 显示加载中的占位图和动画
                        // 使用简单的点动画作为加载指示器
                        let dots = (0..3)
                            .map(|i| {
                                let dot_element = if i as f32 == (local_state.rotation_angle / 120.0).floor() % 3.0 {
                                    text("●").size(24).into() // 活跃点
                                } else {
                                    text("●")
                                        .size(24)
                                        .style(|_theme: &iced::Theme| {
                                            iced::widget::text::Style {
                                                color: Some([0.7, 0.7, 0.7].into()), // 灰色点
                                            }
                                        })
                                        .into()
                                };
                                dot_element
                            })
                            .collect::<Vec<_>>();

                        let loading_image = row(dots).spacing(3);

                        let loading_text =
                            text(i18n.t("local-list.image-loading"))
                                .size(24)
                                .style(|_theme: &iced::Theme| {
                                    iced::widget::text::Style {
                                        color: Some([0.3, 0.3, 0.3].into()), // 深灰色文字
                                    }
                                });

                        let inner_content = iced::widget::container(
                            column![loading_image, loading_text]
                                .width(Length::Shrink)
                                .height(Length::Shrink)
                                .align_x(Alignment::Center)
                                .spacing(5),
                        )
                        .width(Length::Fixed(image_width))
                        .height(Length::Fixed(image_height))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center);

                        // 使用container包装加载占位图，添加浅灰色背景和深灰色边框
                        let placeholder_content = iced::widget::container(inner_content)
                            .width(Length::Fixed(image_width))
                            .height(Length::Fixed(image_height))
                            .style(|theme: &iced::Theme| iced::widget::container::Style {
                                background: Some(iced::Background::Color([0.9, 0.9, 0.9].into())), // 浅灰色背景
                                border: iced::border::Border {
                                    color: theme.extended_palette().primary.weak.color, // 深灰色边框
                                    width: 1.0,
                                    radius: iced::border::Radius::from(4.0),
                                },
                                ..Default::default()
                            });

                        button(placeholder_content)
                            .padding(0)
                            .width(Length::Fixed(image_width))
                            .height(Length::Fixed(image_height))
                    }
                    WallpaperLoadStatus::Loaded(wallpaper) => {
                        // 检查是否为加载失败的壁纸
                        if wallpaper.name == "加载失败" {
                            // 显示错误占位图
                            let error_image = text("\u{F428}")
                                .font(Font::with_name("bootstrap-icons"))
                                .color(Color::BLACK)
                                .size(56);
                            let error_text =
                                text(i18n.t("local-list.loading-error"))
                                    .size(18)
                                    .style(|_theme: &iced::Theme| {
                                        iced::widget::text::Style {
                                            color: Some([0.3, 0.3, 0.3].into()), // 深灰色文字
                                        }
                                    });
                            let error_path = text(&wallpaper.path).size(10).style(|_theme: &iced::Theme| {
                                iced::widget::text::Style {
                                    color: Some([0.3, 0.3, 0.3].into()), // 深灰色文字
                                }
                            }); // 显示路径中的错误信息

                            let inner_content = iced::widget::container(
                                column![error_image, error_text, error_path]
                                    .width(Length::Fill)
                                    .align_x(Alignment::Center),
                            )
                            .width(Length::Fixed(image_width))
                            .height(Length::Fixed(image_height))
                            .center_x(Length::Fill)
                            .center_y(Length::Fill);

                            // 使用container包装错误占位图，添加浅灰色背景和深灰色边框
                            let error_content = iced::widget::container(inner_content)
                                .width(Length::Fixed(image_width))
                                .height(Length::Fixed(image_height))
                                .style(|theme: &iced::Theme| iced::widget::container::Style {
                                    background: Some(iced::Background::Color([0.9, 0.9, 0.9].into())), // 浅灰色背景
                                    border: iced::border::Border {
                                        color: theme.extended_palette().primary.weak.color, // 深灰色边框
                                        width: 1.0,
                                        radius: iced::border::Radius::from(4.0),
                                    },
                                    ..Default::default()
                                });

                            // 加载失败的图片不响应点击事件
                            button(error_content)
                                .padding(0)
                                .width(Length::Fixed(image_width))
                                .height(Length::Fixed(image_height))
                        } else {
                            // 显示已加载的壁纸
                            let image_handle = iced::widget::image::Handle::from_path(&wallpaper.thumbnail_path);
                            let image = iced::widget::image(image_handle)
                                .width(Length::Fixed(image_width))
                                .height(Length::Fixed(image_height))
                                .content_fit(iced::ContentFit::Fill); // 使用Fill模式使图片拉伸铺满容器

                            // 使用container包装图片，添加浅灰色背景和深灰色边框
                            let styled_image = iced::widget::container(image)
                                .width(Length::Fixed(image_width))
                                .height(Length::Fixed(image_height))
                                .style(|theme: &iced::Theme| iced::widget::container::Style {
                                    background: Some(iced::Background::Color([0.9, 0.9, 0.9].into())), // 浅灰色背景
                                    border: iced::border::Border {
                                        color: theme.extended_palette().primary.weak.color, // 深灰色边框
                                        width: 1.0,
                                        radius: iced::border::Radius::from(4.0),
                                    },
                                    ..Default::default()
                                });

                            // 获取当前壁纸在all_paths中的索引
                            let wallpaper_index = local_state
                                .all_paths
                                .iter()
                                .position(|p| p == &wallpaper.path)
                                .unwrap_or(0);

                            button(styled_image)
                                .padding(0)
                                .on_press(super::AppMessage::Local(LocalMessage::ShowModal(wallpaper_index)))
                        }
                    }
                };

                row_container = row_container.push(image_element);
            }

            // 将行容器包装在一个居中容器中，以实现整体居中
            let centered_row = iced::widget::container(row_container)
                .width(Length::Fill)
                .center_x(Length::Fill);
            content = content.push(centered_row);
        }

        // 如果还有更多壁纸未加载且当前没有在加载中，则添加一个加载更多区域
        if local_state.current_page * local_state.page_size < local_state.total_count && !local_state.loading_page {
            // 滚动到底部触发加载更多
            let load_more_button = button(text(i18n.t("local-list.load-more")).size(16))
                .padding(10)
                .on_press(super::AppMessage::Local(LocalMessage::ScrollToBottom));

            content = content.push(load_more_button)
        } else if local_state.current_page * local_state.page_size >= local_state.total_count {
            // 显示已加载全部壁纸
            let all_loaded_text = text(i18n.t("local-list.all-loaded")).size(14);
            content = content.push(all_loaded_text)
        }
        column![
            iced::widget::Space::new().height(spacing),
            content,
            iced::widget::Space::new().height(spacing)
        ]
    };

    // 将基础内容包装在 scrollable 中，作为 stack 的底层
    let base_layer = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(Id::new("local_wallpapers_scroll"));

    // 模态窗口的实现 - 使用原生iced组件
    if local_state.modal_visible && !local_state.all_paths.is_empty() {
        // 获取当前图片路径
        let current_path = &local_state.all_paths[local_state.current_image_index];

        // 创建图片，使用 content_fit 确保图片比例正确且不超出
        let modal_image = if let Some(ref decoder) = local_state.animated_decoder {
            // 使用动态图解码器的当前帧
            let current_frame = decoder.current_frame();
            iced::widget::image(current_frame.handle.clone())
        } else {
            // 使用静态图片
            let image_handle = iced::widget::image::Handle::from_path(current_path);
            iced::widget::image(image_handle)
        };

        let modal_image = modal_image
            .content_fit(iced::ContentFit::Contain) // 在模态窗口中保持Contain模式以确保全图可见
            .width(Length::Fill)
            .height(Length::Fill);

        // 创建上一张按钮
        let prev_button = button(text("<"))
            .padding([10, 20])
            .on_press(AppMessage::Local(LocalMessage::PreviousImage));

        // 创建下一张按钮
        let next_button = button(text(">"))
            .padding([10, 20])
            .on_press(AppMessage::Local(LocalMessage::NextImage));

        // 创建关闭按钮
        let close_button = button(text("×"))
            .padding([5, 10])
            .on_press(AppMessage::Local(LocalMessage::CloseModal));

        // 构建模态窗口内容
        let modal_content = iced::widget::container(iced::widget::column![
            // 顶部关闭按钮行
            iced::widget::row![
                iced::widget::container(iced::widget::Space::new())
                    .width(Length::Fill)
                    .height(Length::Shrink),
                close_button
            ]
            .padding(10),
            // 图片交互行
            iced::widget::row![
                iced::widget::container(prev_button)
                    .width(Length::Shrink)
                    .center_y(Length::Fill),
                iced::widget::container(modal_image)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
                iced::widget::container(next_button)
                    .width(Length::Shrink)
                    .center_y(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color([0.0, 0.0, 0.0, 0.85].into())), // 半透明黑色背景
            ..Default::default()
        });

        // 使用stack来创建模态效果，将模态内容叠加在基础内容之上
        iced::widget::stack(vec![
            base_layer.into(),
            iced::widget::container(iced::widget::opaque(modal_content)).into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        // 无模态框时，只返回滚动区域
        base_layer.into()
    }
}

impl From<LocalMessage> for AppMessage {
    fn from(local_message: LocalMessage) -> AppMessage {
        AppMessage::Local(local_message)
    }
}
