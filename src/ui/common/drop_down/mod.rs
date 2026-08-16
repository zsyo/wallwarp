// Copyright (C) 2026 zsyo - GNU AGPL v3.0
//
// 本地化的下拉框组件，API 与 iced_aw::drop_down 保持一致。
// 移植原因：iced_aw 0.14 的 overlay 定位存在回归（详见 overlay.rs 头部说明），
// 上游 0.14.1 尚未修复，因此在此维护修正版本。

mod overlay;

pub use overlay::{Alignment, Offset};

use iced::advanced::layout::{Limits, Node};
use iced::advanced::overlay::Element as OverlayElement;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::{Element, Event, Length, Rectangle, Size, Vector, mouse};

/// 下拉菜单组件
///
/// 由触发组件（underlay）与展开内容（overlay）组成，
/// 展开内容以 overlay 层渲染在触发组件上方或下方。
pub struct DropDown<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    underlay: Element<'a, Message, Theme, Renderer>,
    overlay: Element<'a, Message, Theme, Renderer>,
    on_dismiss: Option<Message>,
    width: Option<Length>,
    height: Length,
    alignment: Alignment,
    offset: Offset,
    expanded: bool,
}

impl<'a, Message, Theme, Renderer> DropDown<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    /// 创建 [`DropDown`]
    ///
    /// # 参数
    /// - `underlay`: 触发组件（如下拉触发按钮）
    /// - `overlay`: 展开时显示的内容
    /// - `expanded`: 当前是否展开
    pub fn new<U, B>(underlay: U, overlay: B, expanded: bool) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        B: Into<Element<'a, Message, Theme, Renderer>>,
    {
        DropDown {
            underlay: underlay.into(),
            overlay: overlay.into(),
            expanded,
            on_dismiss: None,
            width: None,
            height: Length::Shrink,
            alignment: Alignment::Bottom,
            offset: Offset::from(5.0),
        }
    }

    /// 设置展开内容的宽度
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// 设置展开内容的高度
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置展开内容相对触发组件的对齐方式
    #[must_use]
    pub fn alignment(mut self, alignment: impl Into<Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// 设置展开内容与触发组件的间距偏移
    #[must_use]
    pub fn offset(mut self, offset: impl Into<Offset>) -> Self {
        self.offset = offset.into();
        self
    }

    /// 展开状态下点击组件外部时发送的消息
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for DropDown<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay]);
    }

    fn operate<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.underlay
            .as_widget_mut()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget_mut().update(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<OverlayElement<'b, Message, Theme, Renderer>> {
        if !self.expanded {
            return self.underlay.as_widget_mut().overlay(
                &mut state.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        Some(OverlayElement::new(Box::new(
            overlay::DropDownOverlay::new(
                &mut state.children[1],
                &mut self.overlay,
                self.on_dismiss.as_ref(),
                self.width.as_ref(),
                &self.height,
                &self.alignment,
                &self.offset,
                layout.bounds(),
                layout.position() + translation,
                *viewport,
            ),
        )))
    }
}

impl<'a, Message, Theme: 'a, Renderer> From<DropDown<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
{
    fn from(drop_down: DropDown<'a, Message, Theme, Renderer>) -> Self {
        Element::new(drop_down)
    }
}
