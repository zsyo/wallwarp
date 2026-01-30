// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use iced::widget::{Space, container, mouse_area, stack};
use iced::{Alignment, Element, Length, mouse, window};

/// 创建带边缘调整大小功能的容器
///
/// # 参数
/// - `content`: 容器内容
/// - `edge_size`: 边缘触发区域大小（像素）
/// - `resize_message`: 生成调整大小消息的函数，接收 Direction 参数
/// - `is_maximized`: 窗口是否已最大化，最大化时禁用边缘调整大小
pub fn create_resizable_container<'a, Message>(
    content: Element<'a, Message>,
    edge_size: f32,
    resize_message: impl Fn(window::Direction) -> Message + Clone + 'a,
    is_maximized: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    // 辅助闭包：创建一个带有明确光标和事件的感应区
    // 当窗口最大化时，禁用所有边缘调整大小功能
    let make_handle = |dir: window::Direction, cursor: mouse::Interaction, w: Length, h: Length| {
        if is_maximized {
            // 最大化时，使用默认光标且不响应任何事件
            mouse_area(Space::new().width(w).height(h)).interaction(mouse::Interaction::default())
        } else {
            // 非最大化时，启用边缘调整大小功能
            mouse_area(Space::new().width(w).height(h))
                .on_press(resize_message.clone()(dir))
                .interaction(cursor)
        }
    };

    // --- 核心逻辑：8个感应层 ---
    // 每个层都通过 width/height(Fill) 撑开，然后内部对齐到相应边缘。
    // 这样可以确保 container 覆盖整个窗口，而内部的 Space 准确位于边缘。

    // 上边
    let top = container(make_handle(
        window::Direction::North,
        mouse::Interaction::ResizingVertically,
        Length::Fill,
        Length::Fixed(edge_size),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_y(Alignment::Start);

    // 下边
    let bottom = container(make_handle(
        window::Direction::South,
        mouse::Interaction::ResizingVertically,
        Length::Fill,
        Length::Fixed(edge_size),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_y(Alignment::End);

    // 左边
    let left = container(make_handle(
        window::Direction::West,
        mouse::Interaction::ResizingHorizontally,
        Length::Fixed(edge_size),
        Length::Fill,
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Start);

    // 右边
    let right = container(make_handle(
        window::Direction::East,
        mouse::Interaction::ResizingHorizontally,
        Length::Fixed(edge_size),
        Length::Fill,
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::End);

    // 左上角
    let top_left = container(make_handle(
        window::Direction::NorthWest,
        mouse::Interaction::ResizingDiagonallyDown,
        Length::Fixed(edge_size),
        Length::Fixed(edge_size),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Start)
    .align_y(Alignment::Start);

    // 右上角
    let top_right = container(make_handle(
        window::Direction::NorthEast,
        mouse::Interaction::ResizingDiagonallyUp,
        Length::Fixed(edge_size),
        Length::Fixed(edge_size),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::End)
    .align_y(Alignment::Start);

    // 左下角
    let bottom_left = container(make_handle(
        window::Direction::SouthWest,
        mouse::Interaction::ResizingDiagonallyUp,
        Length::Fixed(edge_size),
        Length::Fixed(edge_size),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Start)
    .align_y(Alignment::End);

    // 右下角
    let bottom_right = container(make_handle(
        window::Direction::SouthEast,
        mouse::Interaction::ResizingDiagonallyDown,
        Length::Fixed(edge_size),
        Length::Fixed(edge_size),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::End)
    .align_y(Alignment::End);

    // 在 stack 中，后写的在顶层。将 content 放在最底层，感应区放在后面。
    stack![
        content,
        top,
        bottom,
        left,
        right,
        top_left,
        top_right,
        bottom_left,
        bottom_right
    ]
    .into()
}
