// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建比例选择器
pub fn create_ratio_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    // 计算按钮显示文本和字体大小
    let button_text = if state.ratio_all_selected {
        i18n.t("online-wallpapers.ratio-label").to_string()
    } else {
        // 计算总选中数量
        let extra_count = state.selected_ratios.len() as i32;
        let landscape_count = if state.ratio_landscape_selected { 1 } else { 0 };
        let portrait_count = if state.ratio_portrait_selected { 1 } else { 0 };
        let total_count = extra_count + landscape_count + portrait_count;

        if total_count == 0 {
            i18n.t("online-wallpapers.ratio-label").to_string()
        } else {
            // 确定第一项的显示文本
            let first_item = if state.ratio_landscape_selected {
                i18n.t("online-wallpapers.ratio-mode-landscape").to_string()
            } else if state.ratio_portrait_selected {
                i18n.t("online-wallpapers.ratio-mode-portrait").to_string()
            } else if let Some(first_ratio) = state.selected_ratios.first() {
                first_ratio.value().to_string()
            } else {
                String::new()
            };

            // 计算额外数量（总数减1）
            let extra_display = total_count - 1;
            if extra_display > 0 {
                format!("{}+{}", first_item, extra_display)
            } else {
                first_item
            }
        }
    };

    // 根据文本类型确定字体大小
    let is_label_text = button_text == i18n.t("online-wallpapers.ratio-label");
    let font_size = if is_label_text { 14 } else { 12 };

    // 创建触发按钮（underlay）
    let ratio_underlay = row![
        text(button_text).size(font_size),
        Space::new().width(Length::Fill),
        container(text("⏷").color(theme_colors.light_text_sub))
            .height(Length::Fill)
            .padding(iced::Padding {
                top: -2.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 0.0,
        bottom: 0.0,
        left: 0.0,
        right: -2.0,
    });

    let ratio_trigger = button(ratio_underlay)
        .padding(6)
        .width(Length::Fixed(120.0))
        .on_press(OnlineMessage::RatioPickerExpanded.into())
        .style(move |_theme, _status| button::Style {
            background: Some(iced::Background::Color(theme_colors.light_button)),
            text_color: theme_colors.light_text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            ..button::text(_theme, _status)
        });

    // 创建比例选项（overlay）
    let ratio_options = super::create_ratio_grid_options(i18n, state, theme_colors);

    // 使用 DropDown 组件
    DropDown::new(ratio_trigger, ratio_options, state.ratio_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(OnlineMessage::RatioPickerDismiss.into())
        .alignment(drop_down::Alignment::Bottom)
        .into()
}
