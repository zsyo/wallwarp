// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 图片预览模态层（本地壁纸页/历史记录页/在线壁纸页共用）
//!
//! 结构：加载占位 + 原图（等比缩放）+ 左上信息浮层 + 底部胶囊工具栏。
//! 页面通过 `PreviewModalMessages` 注入各自的导航/操作消息，
//! 通过 `PreviewModalTexts` 注入各自的 i18n 文案，
//! 通过 `PreviewModalExtras` 注入页面特有内容（进度环占位、富信息浮层、额外按钮）。

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
///
/// - `previous`/`next` 恒为 `AppMessage`，可用性由 `has_previous`/`has_next` 控制；
/// - `set_wallpaper` 为 `None` 时显示灰色禁用图标；
/// - `view_in_folder` 为 `None` 时该按钮不显示（页面无此操作）。
pub struct PreviewModalMessages {
    pub previous: AppMessage,
    pub next: AppMessage,
    pub set_wallpaper: Option<AppMessage>,
    pub view_in_folder: Option<AppMessage>,
    pub close: AppMessage,
}

/// 预览模态的界面文案（由页面从 i18n 取好后传入）
///
/// 注入自定义加载层/隐藏"在文件夹中显示"时，对应文案不生效（传空字符串即可）
pub struct PreviewModalTexts {
    pub loading: String,
    pub previous: String,
    pub next: String,
    pub set_wallpaper: String,
    pub view_in_folder: String,
    pub close: String,
}

/// 预览模态差异项：页面特有内容注入
#[derive(Default)]
pub struct PreviewModalExtras<'a> {
    /// 自定义加载占位层（如在线页的进度环占位）；None 时显示默认占位文案
    pub loading_layer: Option<Element<'a, AppMessage>>,
    /// 自定义信息浮层（如在线页的纯净度/收藏/复制链接胶囊）；
    /// None 时按 `wallpaper` 元数据渲染默认浮层
    pub info_layer: Option<Element<'a, AppMessage>>,
    /// 额外工具栏按钮（插入在"在文件夹中显示"之后、关闭按钮之前）
    pub toolbar_buttons: Vec<Element<'a, AppMessage>>,
}

/// 创建图片预览模态层
///
/// - `image_handle`: 原图句柄，未加载完成时显示占位层；
/// - `wallpaper`: 元信息（分辨率/文件大小），用于默认信息浮层，可选；
/// - `has_previous`/`has_next`: 是否存在上/下一张（决定导航按钮可用态）。
#[allow(clippy::too_many_arguments)] // 页面注入项已尽量收敛为结构体，剩余参数均为独立维度
pub fn create_preview_modal<'a>(
    i18n: &'a I18n,
    theme_config: &'a ThemeConfig,
    image_handle: Option<&Handle>,
    wallpaper: Option<&Wallpaper>,
    has_previous: bool,
    has_next: bool,
    messages: PreviewModalMessages,
    texts: PreviewModalTexts,
    extras: PreviewModalExtras<'a>,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 加载占位层（页面注入或默认文案）
    let loading_layer: Element<_> = match extras.loading_layer {
        Some(layer) => layer,
        None => container(text(texts.loading).size(24).color(COLOR_OVERLAY_TEXT))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    };

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

    // 底部胶囊工具栏（不可用操作显示灰色禁用图标）
    let disabled_color = theme_colors.disabled_color;
    let mut buttons = vec![
        preview_toolbar_button(
            theme_config,
            disabled_color,
            "\u{F12F}", // arrow-left
            BUTTON_COLOR_BLUE,
            texts.previous,
            has_previous.then_some(messages.previous),
        ),
        preview_toolbar_button(
            theme_config,
            disabled_color,
            "\u{F138}", // arrow-right
            BUTTON_COLOR_BLUE,
            texts.next,
            has_next.then_some(messages.next),
        ),
        preview_toolbar_button(
            theme_config,
            disabled_color,
            "\u{F429}", // image-fill
            BUTTON_COLOR_GREEN,
            texts.set_wallpaper,
            messages.set_wallpaper,
        ),
    ];
    if let Some(view_in_folder) = messages.view_in_folder {
        buttons.push(preview_toolbar_button(
            theme_config,
            disabled_color,
            "\u{F3D8}", // folder2-open
            BUTTON_COLOR_YELLOW,
            texts.view_in_folder,
            Some(view_in_folder),
        ));
    }
    buttons.extend(extras.toolbar_buttons);
    buttons.push(preview_toolbar_button(
        theme_config,
        disabled_color,
        "\u{F659}", // x-lg
        BUTTON_COLOR_RED,
        texts.close,
        Some(messages.close),
    ));

    let toolbar = container(row(buttons).align_y(Alignment::Center).spacing(24.0))
        .padding([6, 20])
        .style(common::modal_overlay_style);

    // 信息浮层（左上角：页面注入或按 wallpaper 元数据渲染）
    let info_layer: Element<_> = match extras.info_layer {
        Some(pill) => container(pill)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::Start)
            .padding(16.0)
            .into(),
        None => match wallpaper {
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
        },
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

    let layers: Vec<Element<'_, AppMessage>> =
        vec![image_content.into(), info_layer, toolbar_layer.into()];
    container(iced::widget::stack(layers))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_MODAL_BG)),
            ..Default::default()
        })
        .into()
}

/// 预览工具栏图标按钮（消息为 None 时显示灰色禁用图标）
pub fn preview_toolbar_button<'a>(
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
    let info_column = iced::widget::column![
        common::modal_info_row(
            i18n.t("wallpaper-info.resolution").as_str(),
            format!("{} x {}", wallpaper.width, wallpaper.height)
        ),
        common::modal_info_row(
            i18n.t("wallpaper-info.file-size").as_str(),
            format_file_size(wallpaper.file_size)
        ),
    ]
    .spacing(4)
    .align_x(Alignment::Start);

    common::modal_info_pill(info_column.into())
}
