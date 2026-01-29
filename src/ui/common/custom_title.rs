// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{TITLE_BAR_BUTTON_SPACING, TITLE_BAR_HEIGHT, TITLE_BAR_ICON_SIZE, TITLE_BAR_TITLE_SIZE};
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, mouse_area, row, stack, text, tooltip};
use iced::{Alignment, Color, Element, Font, Length, mouse, window};

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

/// 创建自定义标题栏
///
/// # 参数
/// - `title`: 窗口标题
/// - `is_maximized`: 是否已最大化
/// - `theme_config`: 主题配置
/// - `drag_message`: 拖拽消息
/// - `minimize_to_tray_message`: 最小化到托盘消息
/// - `minimize_message`: 最小化消息
/// - `maximize_message`: 最大化消息
/// - `close_message`: 关闭消息
pub fn create_title_bar<'a, Message>(
    title: String,
    is_maximized: bool,
    theme_config: &'a ThemeConfig,
    drag_message: Message,
    minimize_to_tray_message: Message,
    minimize_to_tray_tooltip: String,
    minimize_message: Message,
    maximize_message: Message,
    close_message: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

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

    // 创建最小化到托盘按钮（带悬停效果）
    let minimize_to_tray_btn = button(
        text("\u{F2EA}") // Bootstrap Icons: dash
            .size(TITLE_BAR_ICON_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .font(Font::with_name("bootstrap-icons"))
            .color(theme_colors.text),
    )
    .padding([4, 8])
    .style(move |_theme: &iced::Theme, status| {
        let base = button::text(_theme, status);
        match status {
            button::Status::Hovered => {
                // 根据主题计算悬停背景色
                let hover_bg = if theme_config.is_dark() {
                    // 深色主题：使用文本颜色的 10% 透明度
                    iced::Color {
                        r: theme_colors.text.r * 0.1,
                        g: theme_colors.text.g * 0.1,
                        b: theme_colors.text.b * 0.1,
                        a: 1.0,
                    }
                } else {
                    // 浅色主题：使用比标题栏背景色稍深的颜色
                    iced::Color {
                        r: (theme_colors.sidebar_bg.r * 255.0 - 20.0) / 255.0,
                        g: (theme_colors.sidebar_bg.g * 255.0 - 20.0) / 255.0,
                        b: (theme_colors.sidebar_bg.b * 255.0 - 20.0) / 255.0,
                        a: 1.0,
                    }
                };

                button::Style {
                    text_color: theme_colors.text,
                    background: Some(iced::Background::Color(hover_bg)),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::from(4.0),
                    },
                    ..base
                }
            }
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..base
            },
        }
    })
    .on_press(minimize_to_tray_message);

    // 为最小化到托盘按钮添加 tooltip（位于按钮下方）
    let minimize_to_tray_btn = super::create_button_with_tooltip(
        minimize_to_tray_btn,
        minimize_to_tray_tooltip,
        tooltip::Position::Bottom,
        theme_config,
    );

    // 创建最小化按钮（带悬停效果）
    let minimize_btn = button(
        text("\u{F63B}") // Bootstrap Icons: dash-lg
            .size(TITLE_BAR_ICON_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .font(Font::with_name("bootstrap-icons"))
            .color(theme_colors.text),
    )
    .padding([4, 8])
    .style(move |_theme: &iced::Theme, status| {
        let base = button::text(_theme, status);
        match status {
            button::Status::Hovered => {
                // 根据主题计算悬停背景色
                let hover_bg = if theme_config.is_dark() {
                    // 深色主题：使用文本颜色的 10% 透明度
                    iced::Color {
                        r: theme_colors.text.r * 0.1,
                        g: theme_colors.text.g * 0.1,
                        b: theme_colors.text.b * 0.1,
                        a: 1.0,
                    }
                } else {
                    // 浅色主题：使用比标题栏背景色稍深的颜色
                    iced::Color {
                        r: (theme_colors.sidebar_bg.r * 255.0 - 20.0) / 255.0,
                        g: (theme_colors.sidebar_bg.g * 255.0 - 20.0) / 255.0,
                        b: (theme_colors.sidebar_bg.b * 255.0 - 20.0) / 255.0,
                        a: 1.0,
                    }
                };

                button::Style {
                    text_color: theme_colors.text,
                    background: Some(iced::Background::Color(hover_bg)),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::from(4.0),
                    },
                    ..base
                }
            }
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..base
            },
        }
    })
    .on_press(minimize_message);

    // 创建最大化/还原按钮（带悬停效果）
    let maximize_icon = if is_maximized {
        "\u{F149}" // Bootstrap Icons: arrows-angle-contract
    } else {
        "\u{F14A}" // Bootstrap Icons: arrows-angle-expand
    };
    let maximize_btn = button(
        text(maximize_icon)
            .size(TITLE_BAR_ICON_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .font(Font::with_name("bootstrap-icons"))
            .color(theme_colors.text),
    )
    .padding([4, 8])
    .style(move |_theme: &iced::Theme, status| {
        let base = button::text(_theme, status);
        match status {
            button::Status::Hovered => {
                // 根据主题计算悬停背景色
                let hover_bg = if theme_config.is_dark() {
                    // 深色主题：使用文本颜色的 10% 透明度
                    iced::Color {
                        r: theme_colors.text.r * 0.1,
                        g: theme_colors.text.g * 0.1,
                        b: theme_colors.text.b * 0.1,
                        a: 1.0,
                    }
                } else {
                    // 浅色主题：使用比标题栏背景色稍深的颜色
                    iced::Color {
                        r: (theme_colors.sidebar_bg.r * 255.0 - 20.0) / 255.0,
                        g: (theme_colors.sidebar_bg.g * 255.0 - 20.0) / 255.0,
                        b: (theme_colors.sidebar_bg.b * 255.0 - 20.0) / 255.0,
                        a: 1.0,
                    }
                };

                button::Style {
                    text_color: theme_colors.text,
                    background: Some(iced::Background::Color(hover_bg)),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::from(4.0),
                    },
                    ..base
                }
            }
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..base
            },
        }
    })
    .on_press(maximize_message);

    // 创建关闭按钮（带悬停效果，悬停时显示红色背景）
    let close_btn = button(
        text("\u{F659}") // Bootstrap Icons: x-lg
            .size(TITLE_BAR_ICON_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .font(Font::with_name("bootstrap-icons"))
            .color(theme_colors.text),
    )
    .padding([4, 8])
    .style(move |_theme: &iced::Theme, status| {
        let base = button::text(_theme, status);
        match status {
            button::Status::Hovered => button::Style {
                text_color: iced::Color::WHITE,
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.86,
                    g: 0.21,
                    b: 0.21,
                    a: 1.0,
                })),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..base
            },
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..base
            },
        }
    })
    .on_press(close_message);

    // 创建标题栏内容
    let title_bar_content = row![
        drag_area,
        row![minimize_to_tray_btn, minimize_btn, maximize_btn, close_btn,]
            .spacing(TITLE_BAR_BUTTON_SPACING)
            .align_y(Alignment::Center)
            .height(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(TITLE_BAR_BUTTON_SPACING)
    .width(Length::Fill)
    .height(TITLE_BAR_HEIGHT);

    container(title_bar_content)
        .width(Length::Fill)
        .height(TITLE_BAR_HEIGHT)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.title_bar_bg)),
            border: Border {
                color: theme_colors.border,
                width: 1.0,
                radius: Radius::from(0.0),
            },
            ..Default::default()
        })
        .into()
}
