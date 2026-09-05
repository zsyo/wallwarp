// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 图片预览模态层（本地壁纸页/历史记录页共用）
//!
//! 结构：加载占位 + 原图（等比缩放）+ 左上信息浮层 + 底部胶囊工具栏。
//! 页面通过 `PreviewModalMessages` 注入各自的导航/操作消息，
//! 通过 `PreviewModalTexts` 注入各自的 i18n 文案。

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, COLOR_MODAL_BG,
    COLOR_OVERLAY_TEXT,
};
use crate::utils::helpers::format_file_size;
use iced::widget::image::Handle;
use iced::widget::{Space, container, row, text, tooltip};
use iced::{Alignment, Color, Element, Length};

/// 预览模态各操作对应的页面消息
pub struct PreviewModalMessages {
    pub previous: AppMessage,
    pub next: AppMessage,
    pub set_wallpaper: AppMessage,
    pub view_in_folder: AppMessage,
    pub close: AppMessage,
}

/// 预览模态的界面文案（由页面从 i18n 取好后传入）
pub struct PreviewModalTexts {
    pub loading: String,
    pub previous: String,
    pub next: String,
    pub set_wallpaper: String,
    pub view_in_folder: String,
    pub close: String,
}

/// 创建图片预览模态层
///
/// - `image_handle`: 原图句柄，未加载完成时显示占位文案；
/// - `wallpaper`: 元信息（分辨率/文件大小），可选；
/// - `has_previous`/`has_next`: 是否存在上/下一张（决定导航按钮可用态）。
pub fn create_preview_modal<'a>(
    i18n: &'a I18n,
    theme_config: &'a ThemeConfig,
    image_handle: Option<&Handle>,
    wallpaper: Option<&Wallpaper>,
    has_previous: bool,
    has_next: bool,
    messages: PreviewModalMessages,
    texts: PreviewModalTexts,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 加载占位层
    let loading_layer: Element<_> =
        container(text(texts.loading).size(24).color(COLOR_OVERLAY_TEXT))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();

    // 原图层（等比缩放，未加载完成时透出占位层）
    let image_layer: Element<_> = if let Some(handle) = image_handle {
        iced::widget::image(handle.clone())
            .content_fit(iced::ContentFit::Contain)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    };

    let image_content = iced::widget::stack(vec![loading_layer, image_layer]);

    // 底部胶囊工具栏（不可用导航显示灰色禁用图标）
    let disabled_color = theme_colors.disabled_color;
    let toolbar = container(
        row![
            toolbar_button(
                theme_config,
                disabled_color,
                "\u{F12F}", // arrow-left
                BUTTON_COLOR_BLUE,
                texts.previous,
                has_previous.then_some(messages.previous),
            ),
            toolbar_button(
                theme_config,
                disabled_color,
                "\u{F138}", // arrow-right
                BUTTON_COLOR_BLUE,
                texts.next,
                has_next.then_some(messages.next),
            ),
            toolbar_button(
                theme_config,
                disabled_color,
                "\u{F429}", // image-fill
                BUTTON_COLOR_GREEN,
                texts.set_wallpaper,
                Some(messages.set_wallpaper),
            ),
            toolbar_button(
                theme_config,
                disabled_color,
                "\u{F3D8}", // folder2-open
                BUTTON_COLOR_YELLOW,
                texts.view_in_folder,
                Some(messages.view_in_folder),
            ),
            toolbar_button(
                theme_config,
                disabled_color,
                "\u{F659}", // x-lg
                BUTTON_COLOR_RED,
                texts.close,
                Some(messages.close),
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(24.0),
    )
    .padding([6, 20])
    .style(common::modal_overlay_style);

    // 信息浮层（左上角，有元数据时显示）
    let info_layer: Element<_> = match wallpaper {
        Some(wallpaper) => container(info_overlay(i18n, wallpaper))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::Start)
            .padding(16.0)
            .into(),
        None => container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
    };

    // 工具栏悬浮于底部居中
    let toolbar_layer = container(toolbar)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::End)
        .padding(iced::Padding {
            top: 0.0,
            right: 0.0,
            bottom: 24.0,
            left: 0.0,
        });

    container(iced::widget::stack(vec![
        image_content.into(),
        info_layer.into(),
        toolbar_layer.into(),
    ]))
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_MODAL_BG)),
        ..Default::default()
    })
    .into()
}

/// 工具栏图标按钮（消息为 None 时显示灰色禁用图标）
fn toolbar_button<'a>(
    theme_config: &'a ThemeConfig,
    disabled_color: Color,
    icon: &'static str,
    color: Color,
    tooltip_text: String,
    message: Option<AppMessage>,
) -> Element<'a, AppMessage> {
    let button = match message {
        Some(message) => common::create_icon_button(icon, color, message),
        None => common::create_icon_button_disabled(icon, disabled_color),
    };
    common::create_button_with_tooltip(button, tooltip_text, tooltip::Position::Top, theme_config)
}

/// 左上角信息浮层（分辨率/文件大小）
fn info_overlay<'a>(i18n: &'a I18n, wallpaper: &Wallpaper) -> Element<'a, AppMessage> {
    let info_row = |label: &str, value: String| -> Element<'a, AppMessage> {
        row![
            text(format!("{label}: "))
                .size(12)
                .color(COLOR_OVERLAY_TEXT),
            text(value).size(12).color(COLOR_OVERLAY_TEXT),
        ]
        .spacing(2)
        .align_y(Alignment::Center)
        .into()
    };

    let info_column = iced::widget::column![
        info_row(
            i18n.t("wallpaper-info.resolution").as_str(),
            format!("{} x {}", wallpaper.width, wallpaper.height)
        ),
        info_row(
            i18n.t("wallpaper-info.file-size").as_str(),
            format_file_size(wallpaper.file_size)
        ),
    ]
    .spacing(4)
    .align_x(Alignment::Start);

    container(info_column)
        .padding([8, 12])
        .style(common::modal_overlay_style)
        .into()
}
