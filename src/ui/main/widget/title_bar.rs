// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::rotated_icon;
use crate::ui::style::{
    BUTTON_COLOR_RED, RADIUS_SM, SEPARATOR_WIDTH, TITLE_BAR_BUTTON_SPACING, TITLE_BAR_HEIGHT,
    TITLE_BAR_ICON_SIZE, TITLE_BAR_TITLE_SIZE, TITLE_BAR_WINDOW_ICON_SIZE, ThemeColors,
    ThemeConfig, darken,
};
use iced::border::{Border, Radius};
use iced::widget::{button, column, container, mouse_area, row, space::Space, text};
use iced::{Alignment, Color, Element, Font, Length, Radians};

/// macOS 原生红绿灯按钮占据的标题栏左侧宽度
/// （fullsize_content_view 模式下红绿灯叠加在自绘标题栏上，需预留空间）
#[cfg(target_os = "macos")]
const TRAFFIC_LIGHT_INSET: f32 = 78.0;
#[cfg(not(target_os = "macos"))]
const TRAFFIC_LIGHT_INSET: f32 = 0.0;

/// 标题栏各按钮触发的消息集合
pub struct TitleBarActions<Message> {
    /// 拖拽窗口
    pub drag: Message,
    /// 最小化
    pub minimize: Message,
    /// 最大化/还原
    pub maximize: Message,
    /// 关闭
    pub close: Message,
}

/// 标题栏窗口按钮样式：悬停淡染填充
///
/// `hover_override` 不为空时（关闭按钮），悬停使用指定的底色与文字色。
fn window_button_style(
    theme_colors: ThemeColors,
    hover_override: Option<(Color, Color)>,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let (bg, text_color) = match status {
            button::Status::Hovered => match hover_override {
                Some((bg, fg)) => (Some(bg), fg),
                None => (Some(theme_colors.hover_fill), theme_colors.text),
            },
            button::Status::Pressed => match hover_override {
                Some((bg, fg)) => (Some(darken(bg, 0.10)), fg),
                None => (Some(theme_colors.hover_fill), theme_colors.text),
            },
            _ => (None, theme_colors.text),
        };
        button::Style {
            text_color,
            background: bg.map(iced::Background::Color),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..button::text(_theme, status)
        }
    }
}

/// 构建 bootstrap 图标文本元素
fn icon_text(glyph: &'static str, size: f32, color: Color) -> text::Text<'static> {
    text(glyph)
        .size(size)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .font(Font::with_name("bootstrap-icons"))
        .color(color)
}

/// 创建标题栏窗口图标按钮
fn window_button<'a, Message>(
    icon: Element<'a, Message>,
    theme_colors: ThemeColors,
    hover_override: Option<(Color, Color)>,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(icon)
        .padding([4, 8])
        .style(window_button_style(theme_colors, hover_override))
        .on_press(message)
}

/// 创建自定义标题栏
///
/// # 参数
/// - `title`: 窗口标题
/// - `is_maximized`: 是否已最大化
/// - `theme_config`: 主题配置
/// - `actions`: 各按钮消息集合
pub fn create_title_bar<'a, Message>(
    title: String,
    is_maximized: bool,
    theme_config: &'a ThemeConfig,
    actions: TitleBarActions<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let TitleBarActions {
        drag: drag_message,
        minimize: minimize_message,
        maximize: maximize_message,
        close: close_message,
    } = actions;
    let theme_colors = theme_config.get_theme_colors();

    // 创建拖拽区域（标题文本区域）
    // 使用 mouse_area 捕获鼠标事件并触发拖拽
    let title_text = text(title)
        .size(TITLE_BAR_TITLE_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let drag_area = mouse_area(
        container(title_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(drag_message);

    // 创建最小化按钮
    let minimize_btn = window_button(
        icon_text("\u{F63B}", TITLE_BAR_ICON_SIZE, theme_colors.text).into(), // bootstrap-icons: dash-lg
        theme_colors,
        None,
        minimize_message,
    );

    // 创建最大化/还原按钮（图标缩小 20%；还原图标旋转 180 度贴近原生窗口按钮观感）
    let maximize_icon = if is_maximized {
        // bootstrap-icons: copy 旋转 180 度
        rotated_icon(
            "\u{F759}",
            TITLE_BAR_WINDOW_ICON_SIZE,
            theme_colors.text,
            Radians::PI,
        )
    } else {
        icon_text("\u{F584}", TITLE_BAR_WINDOW_ICON_SIZE, theme_colors.text).into() // bootstrap-icons: square
    };
    let maximize_btn = window_button(maximize_icon, theme_colors, None, maximize_message);

    // 创建关闭按钮（悬停红色背景 + 白色图标）
    let close_btn = window_button(
        icon_text("\u{F659}", TITLE_BAR_ICON_SIZE, theme_colors.text).into(), // bootstrap-icons: x-lg
        theme_colors,
        Some((BUTTON_COLOR_RED, Color::WHITE)),
        close_message,
    );

    // 创建标题栏内容
    let title_bar_content = row![
        drag_area,
        row![minimize_btn, maximize_btn, close_btn,]
            .spacing(TITLE_BAR_BUTTON_SPACING)
            .align_y(Alignment::Center)
            .height(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(TITLE_BAR_BUTTON_SPACING)
    .width(Length::Fill)
    .height(Length::Fill);

    // 标题栏底部分隔线（替代原先四边的 1px 边框）
    let bottom_line = container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(SEPARATOR_WIDTH))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.separator)),
            ..Default::default()
        });

    container(
        column![title_bar_content, bottom_line]
            .spacing(0)
            .height(TITLE_BAR_HEIGHT),
    )
    .width(Length::Fill)
    .height(TITLE_BAR_HEIGHT)
    .padding(iced::Padding {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: TRAFFIC_LIGHT_INSET,
    })
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.title_bar_bg)),
        ..Default::default()
    })
    .into()
}
