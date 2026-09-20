// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Pixabay 分类选择器（21 项，下拉面板限高滚动）
//!
//! 选项数超出 [`crate::ui::common::drop_down::dropdown_picker`] 的无滚动
//! 面板承载能力，故按其内部结构自组装：选项列 + scrollable 限高 + 面板样式

use crate::i18n::I18n;
use crate::services::source::PixabayCategory;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{
    self, Displayable, dropdown_option_style, dropdown_panel_style, flat_dropdown_trigger_button,
};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::widget::{button, column, container, opaque, scrollable, text};
use iced::{Element, Length};

/// 分类下拉面板最大高度（21 项超出部分滚动展示）
const PANEL_MAX_HEIGHT: f32 = 320.0;

/// 创建 Pixabay 内容分类选择器
pub fn create_pixabay_category_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let options: Vec<Displayable<PixabayCategory>> = PixabayCategory::all()
        .iter()
        .map(|c| Displayable {
            value: *c,
            display: i18n.t(c.display_name()),
        })
        .collect();

    // 触发按钮（underlay）
    let trigger = flat_dropdown_trigger_button(
        i18n.t(state.pixabay_category.display_name()),
        110.0,
        theme_colors,
        OnlineMessage::PixabayCategoryPickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    // 选项列（选中项高亮，选中即关闭）
    let options_content = column(options.iter().map(|option| {
        let is_selected = option.value == state.pixabay_category;
        button(text(option.display.clone()).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(OnlineMessage::PixabayCategoryChanged(option.value).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(
        scrollable(options_content)
            .height(Length::Fixed(PANEL_MAX_HEIGHT))
            .width(Length::Fill),
    )
    .padding(8)
    .width(Length::Fixed(160.0))
    .style(dropdown_panel_style(theme_colors));

    drop_down::DropDown::new(
        trigger,
        opaque(picker_content),
        state.pixabay_category_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(OnlineMessage::PixabayCategoryPickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
