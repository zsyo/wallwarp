// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState, ResolutionMode};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建分辨率选择器
pub fn create_resolution_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    // 计算按钮显示文本和字体大小
    let button_text = match state.resolution_mode {
        ResolutionMode::All => i18n.t("online-wallpapers.resolution-label").to_string(),
        ResolutionMode::AtLeast => {
            if let Some(res) = state.atleast_resolution {
                format!(">={}", res.value())
            } else {
                i18n.t("online-wallpapers.resolution-label").to_string()
            }
        }
        ResolutionMode::Exactly => {
            if state.selected_resolutions.is_empty() {
                i18n.t("online-wallpapers.resolution-label").to_string()
            } else {
                // 找到最小尺寸的分辨率
                let min_res = state
                    .selected_resolutions
                    .iter()
                    .min_by_key(|r| r.get_pixel_count())
                    .unwrap();
                let extra_count = state.selected_resolutions.len() - 1;
                if extra_count > 0 {
                    format!("{}+{}", min_res.value(), extra_count)
                } else {
                    min_res.value().to_string()
                }
            }
        }
    };

    // 根据文本类型确定字体大小
    let is_label_text = button_text == i18n.t("online-wallpapers.resolution-label");
    let font_size = if is_label_text { 14 } else { 12 };

    // 创建触发按钮（underlay）
    let resolution_underlay = row![
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

    let resolution_trigger = button(resolution_underlay)
        .padding(6)
        .width(Length::Fixed(110.0))
        .on_press(AppMessage::Online(OnlineMessage::ResolutionPickerExpanded))
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

    // 创建分辨率选项（overlay）
    let resolution_options = super::create_resolution_grid_options(i18n, state, theme_colors);

    // 使用 DropDown 组件
    DropDown::new(resolution_trigger, resolution_options, state.resolution_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Online(OnlineMessage::ResolutionPickerDismiss))
        .alignment(drop_down::Alignment::Bottom)
        .into()
}
