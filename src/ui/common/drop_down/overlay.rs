// Copyright (C) 2026 zsyo - GNU AGPL v3.0
//
// 本文件移植自 iced_aw 0.14.1 (MIT License) 的 drop_down 组件并修复了
// 下拉层定位问题：iced_aw 0.14 将越界处理从“贴边钳制”改为“整体翻转”，
// 且用 scrollable 可见区域的宽高与窗口绝对坐标比较（见 iced_aw #334/#300），
// 导致位于滚动区域右侧的下拉框被错误地翻转 to 左侧。
// 此处恢复钳制行为，并将 viewport 的 x/y 偏移计入边界计算。

use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget;
use iced::advanced::{Clipboard, Layout, Shell};
use iced::{Element, Event, Point, Rectangle, Size, keyboard, mouse, touch};

/// 下拉框相对触发组件（underlay）的对齐方式
///
/// ```text
/// +-----------+-----------+-----------+
/// | TopStart  |   Top     |  TopEnd   |
/// +-----------+-----------+-----------+
/// |  Start    |           |   End     |
/// +-----------+-----------+-----------+
/// |BottomStart|  Bottom   | BottomEnd |
/// +-----------+-----------+-----------+
/// ```
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    TopStart,
    Top,
    TopEnd,

    End,

    BottomEnd,
    Bottom,
    BottomStart,

    Start,
}

/// 下拉框与触发组件之间的间距偏移
#[derive(Copy, Clone, Debug)]
pub struct Offset {
    /// x 轴偏移
    pub x: f32,
    /// y 轴偏移
    pub y: f32,
}

impl Offset {
    /// 创建 [`Offset`]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<f32> for Offset {
    fn from(float: f32) -> Self {
        Self { x: float, y: float }
    }
}

impl From<[f32; 2]> for Offset {
    fn from(array: [f32; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}

/// 下拉框展开层（overlay）状态与渲染
pub(super) struct DropDownOverlay<'a, 'b, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    state: &'b mut widget::Tree,
    element: &'b mut Element<'a, Message, Theme, Renderer>,
    on_dismiss: Option<&'b Message>,
    width: Option<&'b iced::Length>,
    height: &'b iced::Length,
    alignment: &'b Alignment,
    offset: &'b Offset,
    underlay_bounds: Rectangle,
    position: Point,
    viewport: Rectangle,
}

impl<'a, 'b, Message, Theme, Renderer> DropDownOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        state: &'b mut widget::Tree,
        element: &'b mut Element<'a, Message, Theme, Renderer>,
        on_dismiss: Option<&'b Message>,
        width: Option<&'b iced::Length>,
        height: &'b iced::Length,
        alignment: &'b Alignment,
        offset: &'b Offset,
        underlay_bounds: Rectangle,
        position: Point,
        viewport: Rectangle,
    ) -> Self {
        DropDownOverlay {
            state,
            element,
            on_dismiss,
            width,
            height,
            alignment,
            offset,
            underlay_bounds,
            position,
            viewport,
        }
    }
}

impl<Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
    for DropDownOverlay<'_, '_, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> Node {
        // viewport 是可见区域（滚动容器内会带 x/y 偏移），坐标均为窗口绝对坐标，
        // 因此可用空间与边界判断都必须基于 viewport 矩形而非整个窗口
        let limits = Limits::new(
            Size::ZERO,
            Size::new(self.viewport.width, self.viewport.height),
        )
        .width(
            *self
                .width
                .unwrap_or(&iced::Length::Fixed(self.underlay_bounds.width)),
        )
        .height(*self.height);

        let previous_position = self.position;
        let max = limits.max();

        let height_above = (previous_position.y - self.offset.y).max(0.0);
        let height_below =
            (max.height - previous_position.y - self.underlay_bounds.height - self.offset.y)
                .max(0.0);

        let ref_center_y = previous_position.y + self.underlay_bounds.height / 2.0;
        let max_height_symmetric = (ref_center_y.min(max.height - ref_center_y) * 2.0).max(0.0);

        let limits = match self.alignment {
            Alignment::Top => limits.max_height(height_above),
            Alignment::TopStart | Alignment::TopEnd => {
                limits.max_height((height_above + self.underlay_bounds.height).max(0.0))
            }
            Alignment::Bottom => limits.max_height(height_below),
            Alignment::BottomEnd | Alignment::BottomStart => {
                limits.max_height((height_below + self.underlay_bounds.height).max(0.0))
            }
            Alignment::Start | Alignment::End => limits.max_height(max_height_symmetric),
        };

        let node = self
            .element
            .as_widget_mut()
            .layout(self.state, renderer, &limits);

        let mut new_position = match self.alignment {
            Alignment::TopStart => Point::new(
                previous_position.x - node.bounds().width - self.offset.x,
                previous_position.y - node.bounds().height + self.underlay_bounds.height
                    - self.offset.y,
            ),
            Alignment::Top => Point::new(
                previous_position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                previous_position.y - node.bounds().height - self.offset.y,
            ),
            Alignment::TopEnd => Point::new(
                previous_position.x + self.underlay_bounds.width + self.offset.x,
                previous_position.y - node.bounds().height + self.underlay_bounds.height
                    - self.offset.y,
            ),
            Alignment::End => Point::new(
                previous_position.x + self.underlay_bounds.width + self.offset.x,
                previous_position.y + self.underlay_bounds.height / 2.0
                    - node.bounds().height / 2.0,
            ),
            Alignment::BottomEnd => Point::new(
                previous_position.x + self.underlay_bounds.width + self.offset.x,
                previous_position.y + self.offset.y,
            ),
            Alignment::Bottom => Point::new(
                previous_position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                previous_position.y + self.underlay_bounds.height + self.offset.y,
            ),
            Alignment::BottomStart => Point::new(
                previous_position.x - node.bounds().width - self.offset.x,
                previous_position.y + self.offset.y,
            ),
            Alignment::Start => Point::new(
                previous_position.x - node.bounds().width - self.offset.x,
                previous_position.y + self.underlay_bounds.height / 2.0
                    - node.bounds().height / 2.0,
            ),
        };

        // 越界时贴边钳制到可见区域内（而非整体翻转），并计入 viewport 偏移
        let viewport_left = self.viewport.x;
        let viewport_top = self.viewport.y;
        let viewport_right = self.viewport.x + self.viewport.width;
        let viewport_bottom = self.viewport.y + self.viewport.height;

        if new_position.x + node.bounds().width > viewport_right {
            new_position.x = (viewport_right - node.bounds().width).max(viewport_left);
        }
        if new_position.x < viewport_left {
            new_position.x = viewport_left;
        }
        if new_position.y + node.bounds().height > viewport_bottom {
            new_position.y = (viewport_bottom - node.bounds().height).max(viewport_top);
        }
        if new_position.y < viewport_top {
            new_position.y = viewport_top;
        }

        node.move_to(new_position)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        self.element
            .as_widget()
            .draw(self.state, renderer, theme, style, layout, cursor, &bounds);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        self.underlay_bounds = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.underlay_bounds.width,
            height: self.underlay_bounds.height,
        };

        if let Some(on_dismiss) = self.on_dismiss {
            match &event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                    if key == &keyboard::Key::Named(keyboard::key::Named::Escape) =>
                {
                    shell.publish(on_dismiss.clone());
                }

                Event::Mouse(mouse::Event::ButtonPressed(
                    mouse::Button::Left | mouse::Button::Right,
                ))
                | Event::Touch(touch::Event::FingerPressed { .. })
                    if !cursor.is_over(layout.bounds())
                        && !cursor.is_over(self.underlay_bounds) =>
                {
                    shell.publish(on_dismiss.clone());
                }

                _ => {}
            }
        }

        self.element.as_widget_mut().update(
            self.state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element.as_widget().mouse_interaction(
            self.state,
            layout,
            cursor,
            &self.viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.element
            .as_widget_mut()
            .operate(self.state, layout, renderer, operation);
    }
}
