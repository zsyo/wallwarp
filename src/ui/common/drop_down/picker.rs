// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 通用下拉选择器：选项列表驱动的 DropDown 组装

use super::{Alignment, DropDown, dropdown_option_style, dropdown_panel_style};
use crate::ui::style::ThemeColors;
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 显示用包装类型：枚举值 + 翻译后的显示文本（下拉选择器通用）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Displayable<V> {
    pub value: V,
    pub display: String,
}

impl<V> std::fmt::Display for Displayable<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// 通用下拉选择器（选项列表 + 面板 + 展开/收起状态接线）
///
/// - `underlay`: 触发按钮（已含展开消息，可按页面需要包 tooltip 或改样式）；
/// - `options`: 选项列表；
/// - `is_selected`: 选项选中判断（决定选项高亮态）；
/// - `on_select`: 选项选中消息构造；
/// - `dismiss_msg`: 点击面板外部收起消息；
/// - `expanded`: 展开状态；
/// - `panel_width`: 下拉面板宽度。
#[allow(clippy::too_many_arguments)] // 选择器各维度独立传参，避免调用处拼装结构体
pub fn dropdown_picker<'a, Message, V>(
    underlay: Element<'a, Message>,
    options: Vec<Displayable<V>>,
    is_selected: impl Fn(V) -> bool,
    on_select: impl Fn(V) -> Message,
    dismiss_msg: Message,
    expanded: bool,
    panel_width: f32,
    alignment: Alignment,
    theme_colors: ThemeColors,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
    V: Copy + 'a,
{
    let options_content = column(options.iter().map(|option| {
        let is_selected = is_selected(option.value);
        button(text(option.display.clone()).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(on_select(option.value))
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(options_content)
        .padding(8)
        .width(Length::Fixed(panel_width))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(underlay, opaque(picker_content), expanded)
        .width(Length::Shrink)
        .on_dismiss(dismiss_msg)
        .alignment(alignment)
        .into()
}
