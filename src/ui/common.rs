use iced::widget::{button, column, container, row, text, tooltip};
use iced::{Alignment, Color, Element, Font, Length};

// 模态对话框常量
pub const DIALOG_TITLE_SIZE: f32 = 16.0;
pub const DIALOG_MESSAGE_SIZE: f32 = 14.0;
pub const DIALOG_BUTTON_TEXT_SIZE: f32 = 14.0;

pub const DIALOG_MAX_WIDTH: f32 = 500.0;
pub const DIALOG_BORDER_RADIUS: f32 = 8.0;
pub const DIALOG_BORDER_WIDTH: f32 = 1.0;

pub const DIALOG_PADDING: f32 = 20.0;
pub const DIALOG_SPACING: f32 = 15.0;
pub const DIALOG_BUTTON_SPACING: f32 = 10.0;
pub const DIALOG_INNER_PADDING: f32 = 10.0;

pub const MASK_ALPHA: f32 = 0.5;
pub const BORDER_COLOR_GRAY: f32 = 0.8;

// 设置页面常量
pub const SECTION_TITLE_SIZE: f32 = 16.0;
pub const BUTTON_TEXT_SIZE: f32 = 14.0;
pub const TEXT_INPUT_SIZE: f32 = 14.0;
pub const INPUT_HEIGHT: f32 = 30.0;
pub const ROW_SPACING: f32 = 10.0;
pub const SECTION_PADDING: f32 = 15.0;
pub const SECTION_SPACING: f32 = 10.0;
pub const INPUT_PADDING: u16 = 5;
pub const BUTTON_SPACING: f32 = 2.0;

// 图标按钮常量
pub const ICON_BUTTON_TEXT_SIZE: f32 = 14.0;
pub const ICON_BUTTON_PADDING: [u16; 2] = [4, 4];
pub const TOOLTIP_BORDER_RADIUS: f32 = 4.0;
pub const TOOLTIP_BORDER_WIDTH: f32 = 1.0;

// 容器样式常量
pub const BORDER_WIDTH: f32 = 1.0;
pub const BORDER_RADIUS: f32 = 5.0;

// 按钮颜色
pub const BUTTON_COLOR_BLUE: Color = Color::from_rgb8(0, 123, 255);
pub const BUTTON_COLOR_GREEN: Color = Color::from_rgb8(40, 167, 69);
pub const BUTTON_COLOR_RED: Color = Color::from_rgb8(220, 53, 69);
pub const BUTTON_COLOR_GRAY: Color = Color::from_rgb8(108, 117, 125);
pub const BUTTON_COLOR_YELLOW: Color = Color::from_rgb8(255, 204, 0);

// Tooltip 颜色
pub const TOOLTIP_BG_COLOR: Color = Color::WHITE;
pub const TOOLTIP_BORDER_COLOR: Color = Color::BLACK;

/// 创建带颜色的按钮（接收文本字符串）
pub fn create_colored_button<'a, Message>(label: String, color: Color, message: Message) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text(label).size(BUTTON_TEXT_SIZE))
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
pub fn create_bordered_container_style(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: BORDER_WIDTH,
            radius: iced::border::Radius::from(BORDER_RADIUS),
        },
        ..Default::default()
    }
}

/// 创建配置区块
///
/// # 参数
/// - `title`: 区块标题
/// - `rows`: 区块内容行
pub fn create_config_section<'a, Message: 'a>(title: String, rows: Vec<Element<'a, Message>>) -> Element<'a, Message> {
    let mut column_content = column!(
        text(title)
            .size(SECTION_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
    );

    for row in rows {
        column_content = column_content.push(row);
    }

    container(column_content)
        .padding(SECTION_PADDING)
        .width(Length::Fill)
        .style(create_bordered_container_style)
        .into()
}

/// 创建设置行
///
/// # 参数
/// - `label`: 标签文本
/// - `widget`: 控件
pub fn create_setting_row<'a, Message: 'a>(
    label: String,
    widget: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![text(label).width(Length::FillPortion(1)), widget.into(),]
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
pub fn create_info_row<'a, Message: 'a>(label: String, value: String) -> Element<'a, Message> {
    row![text(label), text(value).width(Length::Fill).align_x(Alignment::Center),]
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
            .size(ICON_BUTTON_TEXT_SIZE),
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
