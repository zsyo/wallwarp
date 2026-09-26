// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::i18n::I18n;
use crate::services::source::SourceColor;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{
    self, Displayable, dropdown_picker, flat_dropdown_trigger_button,
};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::{Element, Length};

/// 创建 Basic 档图源共用色板选择器
///
/// 取值并集覆盖 Pixabay colors 与 Unsplash color，各图源 service 层
/// 按自己的参数表映射（无对应取值的图源忽略该筛选）
pub fn create_source_color_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let options: Vec<Displayable<SourceColor>> = SourceColor::all()
        .iter()
        .map(|c| Displayable {
            value: *c,
            display: i18n.t(c.display_name()),
        })
        .collect();

    // 触发按钮（underlay）
    let trigger = flat_dropdown_trigger_button(
        i18n.t(state.source_color.display_name()),
        90.0,
        theme_colors,
        OnlineMessage::SourceColorPickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    dropdown_picker(
        trigger.into(),
        options,
        |v| v == state.source_color,
        |v| OnlineMessage::SourceColorChanged(v).into(),
        OnlineMessage::SourceColorPickerDismiss.into(),
        state.source_color_picker_expanded,
        120.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    )
}
