// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{
    BORDER_COLOR_GRAY, BUTTON_COLOR_GRAY, BUTTON_COLOR_RED, BUTTON_TEXT_SIZE, DIALOG_BORDER_RADIUS,
    DIALOG_BORDER_WIDTH, DIALOG_BUTTON_SPACING, DIALOG_INNER_PADDING, DIALOG_MAX_WIDTH, DIALOG_MESSAGE_SIZE,
    DIALOG_PADDING, DIALOG_SPACING, DIALOG_TITLE_SIZE, ICON_BUTTON_PADDING, ICON_BUTTON_TEXT_SIZE, INPUT_HEIGHT,
    MASK_ALPHA, ROW_SPACING, SECTION_CONTENT_SPACING, SECTION_PADDING, SECTION_TITLE_SIZE, TOOLTIP_BG_COLOR,
    TOOLTIP_BORDER_COLOR, TOOLTIP_BORDER_RADIUS, TOOLTIP_BORDER_WIDTH,
};
use iced::widget::{button, column, container, mouse_area, row, text, tooltip};
use iced::{Alignment, Color, Element, Font, Length};

/// 创建带颜色的按钮（接收文本字符串）
pub fn create_colored_button<'a, Message>(label: String, color: Color, message: Message) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        text(label)
            .size(BUTTON_TEXT_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(message)
    .style(move |_theme: &iced::Theme, _status| {
        let base = iced::widget::button::text(_theme, _status);
        iced::widget::button::Style {
            background: Some(iced::Background::Color(color)),
            text_color: iced::Color::WHITE,
            ..base
        }
    })
}

/// 创建带颜色的按钮（接收 text 控件，可自定义字体和颜色）
pub fn create_colored_button_with_text<'a, Message>(
    text_element: Element<'a, Message>,
    color: Color,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text_element)
        .on_press(message)
        .style(move |_theme: &iced::Theme, _status| {
            let base = iced::widget::button::text(_theme, _status);
            iced::widget::button::Style {
                background: Some(iced::Background::Color(color)),
                text_color: iced::Color::WHITE,
                ..base
            }
        })
}

/// 创建模态确认对话框
///
/// # 参数
/// - `title`: 对话框标题
/// - `message`: 对话框提示信息
/// - `confirm_label`: 确认按钮文本
/// - `cancel_label`: 取消按钮文本
/// - `confirm_msg`: 确认按钮消息
/// - `cancel_msg`: 取消按钮消息
pub fn create_confirmation_dialog<'a, Message>(
    title: String,
    message: String,
    confirm_label: String,
    cancel_label: String,
    confirm_msg: Message,
    cancel_msg: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let dialog_content = column![
        text(title)
            .size(DIALOG_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        text(message)
            .size(DIALOG_MESSAGE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        row![
            create_colored_button(confirm_label, BUTTON_COLOR_RED, confirm_msg),
            create_colored_button(cancel_label, BUTTON_COLOR_GRAY, cancel_msg),
        ]
        .spacing(DIALOG_BUTTON_SPACING)
        .align_y(Alignment::Center),
    ]
    .padding(DIALOG_PADDING)
    .spacing(DIALOG_SPACING)
    .align_x(Alignment::Center)
    .width(Length::Shrink)
    .max_width(DIALOG_MAX_WIDTH);

    let modal_dialog = container(dialog_content)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(DIALOG_INNER_PADDING)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::border::Border {
                radius: iced::border::Radius::from(DIALOG_BORDER_RADIUS),
                width: DIALOG_BORDER_WIDTH,
                color: iced::Color::from_rgb(BORDER_COLOR_GRAY, BORDER_COLOR_GRAY, BORDER_COLOR_GRAY),
            },
            ..Default::default()
        });

    let modal_content = container(iced::widget::stack(vec![
        container(iced::widget::Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: MASK_ALPHA,
                })),
                ..Default::default()
            })
            .into(),
        container(modal_dialog)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ]))
    .width(Length::Fill)
    .height(Length::Fill);

    iced::widget::opaque(modal_content).into()
}

/// 创建带边框的容器样式
pub fn create_bordered_container_style(_theme: &iced::Theme) -> iced::widget::container::Style {
    use crate::ui::style::COLOR_SIDEBAR_BG;

    iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_SIDEBAR_BG)),
        border: iced::border::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(8.0),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: iced::Vector { x: 0.0, y: 2.0 },
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

/// 创建配置区块
///
/// # 参数
/// - `title`: 区块标题
/// - `rows`: 区块内容行
pub fn create_config_section<'a, Message: 'a>(
    title: String,
    rows: Vec<Element<'a, Message>>,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    let mut column_content = column!(
        text(title)
            .size(SECTION_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
    )
    .spacing(SECTION_CONTENT_SPACING);
    column_content = column_content.push(iced::widget::Space::new().height(Length::Fixed(20.0)));

    for row in rows {
        column_content = column_content.push(row);
    }

    container(column_content)
        .padding(SECTION_PADDING)
        .width(Length::Fill)
        .style(create_bordered_container_style_with_bg(theme_config))
        .into()
}

/// 创建设置行
///
/// # 参数
/// - `label`: 标签文本
/// - `widget`: 控件
/// - `theme_config`: 主题配置
pub fn create_setting_row<'a, Message: 'a>(
    label: String,
    widget: impl Into<Element<'a, Message>>,
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

    row![
        text(label)
            .width(Length::FillPortion(1))
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        widget.into(),
    ]
    .align_y(Alignment::Center)
    .height(Length::Fixed(INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}

/// 创建信息行
///
/// # 参数
/// - `label`: 标签文本
/// - `value`: 值文本
/// - `theme_colors`: 主题颜色
pub fn create_info_row<'a, Message: 'a>(
    label: String,
    value: String,
    theme_colors: crate::ui::style::ThemeColors,
) -> Element<'a, Message> {
    row![
        text(label).style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        }),
        text(value)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.light_text),
            }),
    ]
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}

/// 获取绝对路径
pub fn get_absolute_path(path: &str) -> String {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let path_buf = std::path::PathBuf::from(path);

    if path_buf.is_absolute() {
        path.to_string()
    } else {
        current_dir.join(path_buf).to_string_lossy().to_string()
    }
}

/// 创建带图标的操作按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F341}"）
/// - `icon_color`: 图标颜色
/// - `message`: 按钮点击消息
pub fn create_icon_button<'a, Message>(
    icon_char: &'static str,
    icon_color: Color,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        text(icon_char)
            .color(icon_color)
            .font(Font::with_name("bootstrap-icons"))
            .size(ICON_BUTTON_TEXT_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(ICON_BUTTON_PADDING)
    .style(|_theme: &iced::Theme, _status| button::Style {
        text_color: iced::Color::WHITE,
        background: None,
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(0.0),
        },
        ..Default::default()
    })
    .on_press(message)
}

/// 创建带图标的操作按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F341}"）
/// - `icon_color`: 图标颜色
/// - `size`: 按钮大小
/// - `message`: 按钮点击消息
pub fn create_icon_button_with_size<'a, Message>(
    icon_char: &'static str,
    icon_color: Color,
    size: impl Into<iced::Pixels>,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        text(icon_char)
            .color(icon_color)
            .font(Font::with_name("bootstrap-icons"))
            .size(size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(ICON_BUTTON_PADDING)
    .style(|_theme: &iced::Theme, _status| button::Style {
        text_color: iced::Color::WHITE,
        background: None,
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(0.0),
        },
        ..Default::default()
    })
    .on_press(message)
}

/// 创建带 tooltip 的图标按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F341}"）
/// - `icon_color`: 图标颜色
/// - `message`: 按钮点击消息
/// - `tooltip_text`: tooltip 文本
pub fn create_icon_button_with_tooltip<'a, Message>(
    icon_char: &'static str,
    icon_color: Color,
    message: Message,
    tooltip_text: String,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let btn = button(
        text(icon_char)
            .color(icon_color)
            .font(Font::with_name("bootstrap-icons"))
            .size(ICON_BUTTON_TEXT_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(ICON_BUTTON_PADDING)
    .style(|_theme: &iced::Theme, _status| button::Style {
        text_color: iced::Color::WHITE,
        background: None,
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(0.0),
        },
        ..Default::default()
    })
    .on_press(message);

    tooltip(btn, text(tooltip_text), tooltip::Position::Top)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(TOOLTIP_BG_COLOR)),
            border: iced::border::Border {
                color: TOOLTIP_BORDER_COLOR,
                width: TOOLTIP_BORDER_WIDTH,
                radius: iced::border::Radius::from(TOOLTIP_BORDER_RADIUS),
            },
            ..Default::default()
        })
        .into()
}

/// 创建带 tooltip 的按钮
///
/// # 参数
/// - `button`: 按钮组件
/// - `tooltip_text`: tooltip 文本
pub fn create_button_with_tooltip<'a, Message>(
    button: button::Button<'a, Message>,
    tooltip_text: String,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    tooltip(button, text(tooltip_text), tooltip::Position::Top)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(TOOLTIP_BG_COLOR)),
            border: iced::border::Border {
                color: TOOLTIP_BORDER_COLOR,
                width: TOOLTIP_BORDER_WIDTH,
                radius: iced::border::Radius::from(TOOLTIP_BORDER_RADIUS),
            },
            ..Default::default()
        })
        .into()
}

/// 创建带提示的单选按钮
///
/// # 参数
/// - `label`: 标签文本
/// - `value`: 选项值
/// - `selected_value`: 当前选中的值
/// - `on_selected`: 选中时的回调
/// - `tooltip_text`: 提示文本
/// - `theme_colors`: 主题颜色
pub fn create_radio_with_tooltip<'a, Message, V>(
    label: String,
    value: V,
    selected_value: Option<V>,
    on_selected: impl FnOnce(V) -> Message + 'a,
    tooltip_text: String,
    theme_colors: crate::ui::style::ThemeColors,
) -> Element<'a, Message>
where
    V: Copy + Eq + 'a,
    Message: Clone + 'a,
{
    let radio_button = iced::widget::radio(label, value, selected_value, on_selected)
        .size(16)
        .spacing(8)
        .style(move |theme: &iced::Theme, status| iced::widget::radio::Style {
            text_color: Some(theme_colors.text),
            background: iced::Background::Color(Color::TRANSPARENT),
            ..iced::widget::radio::default(theme, status)
        });

    let content = container(radio_button)
        .height(Length::Fixed(30.0))
        .align_y(Alignment::Center);

    tooltip(content, text(tooltip_text), tooltip::Position::Top)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(TOOLTIP_BG_COLOR)),
            border: iced::border::Border {
                color: TOOLTIP_BORDER_COLOR,
                width: TOOLTIP_BORDER_WIDTH,
                radius: iced::border::Radius::from(TOOLTIP_BORDER_RADIUS),
            },
            ..Default::default()
        })
        .into()
}

/// 创建带边框的容器样式（带背景色）
///
/// # 参数
/// - `theme`: 主题
/// - `bg_color`: 背景颜色
pub fn create_bordered_container_style_with_bg(
    theme_config: &crate::ui::style::ThemeConfig,
) -> impl Fn(&iced::Theme) -> iced::widget::container::Style + '_ {
    use crate::ui::style::{BORDER_RADIUS, BORDER_WIDTH, ThemeColors, shadows::CARD_SHADOW};

    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(theme_colors.sidebar_bg)),
        border: iced::border::Border {
            color: theme_colors.border,
            width: BORDER_WIDTH,
            radius: iced::border::Radius::from(BORDER_RADIUS),
        },
        shadow: CARD_SHADOW,
        ..Default::default()
    }
}

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
    resize_message: impl Fn(iced::window::Direction) -> Message + Clone + 'a,
    is_maximized: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    use iced::widget::{Space, container, mouse_area, stack};
    use iced::{Alignment, Length, mouse, window};

    // 辅助闭包：创建一个带有明确光标和事件的感应区
    // 当窗口最大化时，禁用所有边缘调整大小功能
    let make_handle = |dir: window::Direction, cursor: mouse::Interaction, w: Length, h: Length| {
        if is_maximized {
            // 最大化时，使用默认光标且不响应任何事件
            mouse_area(Space::new().width(w).height(h))
                .interaction(mouse::Interaction::default())
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
    theme_config: &'a crate::ui::style::ThemeConfig,
    drag_message: Message,
    minimize_to_tray_message: Message,
    minimize_message: Message,
    maximize_message: Message,
    close_message: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    use crate::ui::style::{TITLE_BAR_BUTTON_SPACING, TITLE_BAR_HEIGHT, TITLE_BAR_ICON_SIZE, TITLE_BAR_TITLE_SIZE};

    let theme_colors = crate::ui::style::ThemeColors::from_theme(theme_config.get_theme());

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
        text("\u{F1B9}") // Bootstrap Icons: box-arrow-down
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
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..base
                }
            }
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..base
            },
        }
    })
    .on_press(minimize_to_tray_message);

    // 创建最小化按钮（带悬停效果）
    let minimize_btn = button(
        text("\u{F63B}") // Bootstrap Icons: dash
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
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..base
                }
            }
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
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
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..base
                }
            }
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
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
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..base
            },
            _ => button::Style {
                text_color: theme_colors.text,
                background: None,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
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
            border: iced::border::Border {
                color: theme_colors.border,
                width: 1.0,
                radius: iced::border::Radius::from(0.0),
            },
            ..Default::default()
        })
        .into()
}
