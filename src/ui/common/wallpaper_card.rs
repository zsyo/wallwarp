// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸卡片通用组件：内容区 + 底部信息遮罩 + 卡片按钮样式
//!
//! 本地页与在线页的壁纸卡片（含加载占位符）共用此骨架，
//! 差异仅在于内容区元素、居中分辨率文本与右侧操作按钮

use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{
    IMAGE_HEIGHT, IMAGE_WIDTH, LOADING_TEXT_SIZE, OVERLAY_HEIGHT, OVERLAY_TEXT_SIZE,
};
use iced::widget::{Space, button, container, row, stack, text};
use iced::{Alignment, Element, Length};

/// 创建壁纸卡片：内容区 + 底部信息遮罩
///
/// - `content`: 卡片主内容（图片或错误占位），需已按卡片尺寸与容器样式包装；
/// - `file_size`: 遮罩左侧文件大小文本（已格式化）；
/// - `resolution`: 遮罩居中显示的分辨率（None 时不显示）；
/// - `actions`: 遮罩右侧操作按钮；
/// - `action_spacing`: 操作按钮水平间距；
/// - `on_press`: 卡片点击消息（None 时卡片不可点击）。
pub fn wallpaper_card<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    file_size: String,
    resolution: Option<String>,
    actions: Vec<Element<'a, Message>>,
    action_spacing: f32,
    on_press: Option<Message>,
    theme_colors: crate::ui::style::ThemeColors,
) -> Element<'a, Message> {
    let text_color = theme_colors.overlay_text;

    let file_size_text =
        text(file_size)
            .size(OVERLAY_TEXT_SIZE)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(text_color),
            });

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：操作按钮
    let right_area = row(actions)
        .spacing(action_spacing)
        .align_y(Alignment::Center);

    // 底层：左中右三部分布局；顶层：分辨率居中显示（不受两侧内容影响）
    let mut overlay_layers = vec![
        container(
            row![
                left_area,
                // 中间占位，让分辨率在顶层居中
                container(Space::new()).width(Length::Fill),
                right_area,
            ]
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .padding([0, 8])
        .into(),
    ];

    if let Some(resolution) = resolution {
        overlay_layers.push(
            container(text(resolution).size(OVERLAY_TEXT_SIZE).style(
                move |_theme: &iced::Theme| text::Style {
                    color: Some(text_color),
                },
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
        );
    }

    // 创建遮罩层（遮罩底色与文字颜色统一走主题色）
    let overlay_bg = theme_colors.overlay_bg;
    let overlay = container(stack(overlay_layers))
        .width(Length::Fill)
        .height(Length::Fixed(OVERLAY_HEIGHT))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(overlay_bg)),
            ..Default::default()
        });

    // 使用 stack 将遮罩覆盖在内容区内部下方
    let card_content = stack(vec![
        content,
        container(overlay)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::End)
            .into(),
    ]);

    let mut card = button(card_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(common::wallpaper_card_button_style(theme_colors));
    if let Some(on_press) = on_press {
        card = card.on_press(on_press);
    }
    card.into()
}

/// 创建壁纸加载占位符卡片（本地/在线共用，仅文案不同）
pub fn wallpaper_loading_placeholder<'a, Message: Clone + 'a>(
    label: String,
    theme_config: &ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = theme_config.get_theme_colors();

    let loading_text = text(label)
        .size(LOADING_TEXT_SIZE)
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let placeholder_content = container(loading_text)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(common::wallpaper_image_container_style(theme_colors));

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(common::wallpaper_card_button_style(theme_colors))
        .into()
}
