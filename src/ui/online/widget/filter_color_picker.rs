// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::ColorOption;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建颜色选择器
pub fn create_color_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let color_button_text = i18n.t("online-wallpapers.color-label");

    let color_button_bg = match state.color {
        ColorOption::Any => theme_colors.light_button,
        ColorOption::Color660000 => COLOR_660000,
        ColorOption::Color990000 => COLOR_990000,
        ColorOption::ColorCC0000 => COLOR_CC0000,
        ColorOption::ColorCC3333 => COLOR_CC3333,
        ColorOption::ColorEA4C88 => COLOR_EA4C88,
        ColorOption::Color993399 => COLOR_993399,
        ColorOption::Color663399 => COLOR_663399,
        ColorOption::Color333399 => COLOR_333399,
        ColorOption::Color0066CC => COLOR_0066CC,
        ColorOption::Color0099CC => COLOR_0099CC,
        ColorOption::Color66CCCC => COLOR_66CCCC,
        ColorOption::Color77CC33 => COLOR_77CC33,
        ColorOption::Color669900 => COLOR_669900,
        ColorOption::Color336600 => COLOR_336600,
        ColorOption::Color666600 => COLOR_666600,
        ColorOption::Color999900 => COLOR_999900,
        ColorOption::ColorCCCC33 => COLOR_CCCC33,
        ColorOption::ColorFFFF00 => COLOR_FFFF00,
        ColorOption::ColorFFCC33 => COLOR_FFCC33,
        ColorOption::ColorFF9900 => COLOR_FF9900,
        ColorOption::ColorFF6600 => COLOR_FF6600,
        ColorOption::ColorCC6633 => COLOR_CC6633,
        ColorOption::Color996633 => COLOR_996633,
        ColorOption::Color663300 => COLOR_663300,
        ColorOption::Color000000 => COLOR_000000,
        ColorOption::Color999999 => COLOR_999999,
        ColorOption::ColorCCCCCC => COLOR_CCCCCC,
        ColorOption::ColorFFFFFF => COLOR_FFFFFF,
        ColorOption::Color424153 => COLOR_424153,
    };

    // 对于浅色背景（白色、浅灰色等），使用深色文字
    let color_button_text_color = theme_colors.light_text;

    // 创建颜色选择器的触发按钮（underlay）
    let color_underlay = row![
        text(color_button_text).size(14).color(color_button_text_color),
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

    let color_trigger = button(color_underlay)
        .padding(6)
        .width(Length::Fixed(80.0))
        .on_press(AppMessage::Online(OnlineMessage::ColorPickerExpanded))
        .style(move |_theme, _status| button::Style {
            background: Some(iced::Background::Color(color_button_bg)),
            text_color: color_button_text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            ..button::text(_theme, _status)
        });

    // 创建颜色网格选项（overlay）
    let color_options = super::create_color_grid_options(i18n, state, theme_colors);

    // 使用 DropDown 组件
    DropDown::new(color_trigger, color_options, state.color_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Online(OnlineMessage::ColorPickerDismiss))
        .alignment(drop_down::Alignment::Bottom)
        .into()
}
