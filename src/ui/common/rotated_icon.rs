// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 旋转绘制的 bootstrap 图标
//!
//! iced 0.14 的 text 组件不支持旋转，此处经 canvas 文本管线（fill_text）
//! 以指定角度绘制单个图标字形，用于还原图标旋转 180° 贴近原生窗口按钮样式。

use iced::mouse;
use iced::widget::{canvas, text};
use iced::{Color, Element, Font, Length, Point, Radians, Rectangle, Renderer, Theme, Vector};

/// 旋转图标绘制程序：以指定角度绘制一个 bootstrap 图标字形
struct RotatedIconGlyph {
    glyph: &'static str,
    size: f32,
    color: Color,
    angle: Radians,
}

impl<Message> canvas::Program<Message> for RotatedIconGlyph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // 平移到中心后旋转，再以中心对齐绘制字形，实现绕中心旋转
        frame.translate(Vector::new(bounds.width / 2.0, bounds.height / 2.0));
        frame.rotate(self.angle);
        frame.fill_text(canvas::Text {
            content: self.glyph.to_string(),
            position: Point::ORIGIN,
            max_width: f32::INFINITY,
            color: self.color,
            size: self.size.into(),
            line_height: text::LineHeight::default(),
            font: Font::with_name("bootstrap-icons"),
            align_x: text::Alignment::Center,
            align_y: iced::alignment::Vertical::Center,
            shaping: text::Shaping::default(),
        });

        vec![frame.into_geometry()]
    }
}

/// 创建按指定角度旋转的 bootstrap 图标元素
///
/// # 参数
/// - `glyph`: 图标 Unicode 码点字符串
/// - `size`: 图标尺寸（逻辑像素），同时决定元素边长
/// - `color`: 图标颜色
/// - `angle`: 旋转角度（如 `Radians::PI` 表示旋转 180 度）
pub fn rotated_icon<'a, Message>(
    glyph: &'static str,
    size: f32,
    color: Color,
    angle: impl Into<Radians>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    canvas::Canvas::new(RotatedIconGlyph {
        glyph,
        size,
        color,
        angle: angle.into(),
    })
    // Canvas 默认固定 100x100，必须按图标尺寸重设边长
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .into()
}
