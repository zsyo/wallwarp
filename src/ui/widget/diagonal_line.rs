use iced::mouse;
use iced::widget::canvas;
use iced::{Color, Point, Rectangle, Renderer, Theme};

/// 通用的对角线绘制程序
pub struct DiagonalLine {
    pub color: Color,
    pub width: f32,
    pub padding: f32, // 线条距离边缘的缩进
}

impl Default for DiagonalLine {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(1.0, 0.0, 0.0), // 默认红色
            width: 2.0,
            padding: 0.0,
        }
    }
}

impl<Message> canvas::Program<Message, Theme, Renderer> for DiagonalLine {
    type State = ();

    fn draw(&self, _state: &Self::State, _renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(_renderer, bounds.size());

        // 计算带缩进的起点和终点
        // p1: 左上角 (加上 padding 缩进)
        let p1 = Point::new(self.padding, self.padding);
        // p2: 右下角 (减去 padding 缩进)
        let p2 = Point::new(bounds.width - self.padding, bounds.height - self.padding);

        let path = canvas::Path::line(p1, p2);

        frame.stroke(
            &path,
            canvas::Stroke {
                style: canvas::Style::Solid(self.color),
                width: self.width,
                line_cap: canvas::LineCap::Round, // 让线条两端圆润一点，更好看
                ..Default::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
