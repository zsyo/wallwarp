// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 历史记录列表行与顶部工具条（参考 SnowShot 截图历史布局）

use crate::i18n::I18n;
use crate::services::local::Wallpaper;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::history::HistoryMessage;
use crate::ui::history::state::HistoryEntry;
use crate::ui::style::{BUTTON_COLOR_BLUE, BUTTON_COLOR_RED, ThemeColors, ThemeConfig};
use crate::utils::helpers::format_file_size;
use iced::border::Radius;
use iced::widget::image::Handle;
use iced::widget::{button, column, container, image, row, text, tooltip};
use iced::{Alignment, Color, Element, Font, Length, Padding};

/// 行内缩略图显示尺寸（缓存的 256x150 缩略图轻微缩小显示，保证清晰）
const HISTORY_THUMB_WIDTH: f32 = 200.0;
const HISTORY_THUMB_HEIGHT: f32 = 125.0;
/// 行内边距
const ROW_PADDING: f32 = 12.0;
/// 操作按钮文字大小
const ACTION_TEXT_SIZE: f32 = 13.0;
/// 文件名显示的最大字符数（超出截断）
const MAX_NAME_CHARS: usize = 42;

/// 创建历史记录顶部工具条（统计 + 刷新 + 清空）
pub fn create_history_toolbar<'a>(
    i18n: &'a I18n,
    history_state: &'a crate::ui::history::HistoryState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let count_text = text(format!(
        "{}: {}",
        i18n.t("history.count-label"),
        history_state.entries.len()
    ))
    .size(15)
    .color(theme_colors.text);

    let refresh_button = common::create_button_with_tooltip(
        common::create_icon_button_with_size(
            "\u{F130}", // arrow-repeat
            theme_colors.light_text,
            16,
            HistoryMessage::Refresh.into(),
        ),
        i18n.t("history.refresh"),
        tooltip::Position::Left,
        theme_config,
    );

    let clear_button = common::create_button_with_tooltip(
        common::create_icon_button_with_size(
            "\u{F78B}", // trash3
            BUTTON_COLOR_RED,
            16,
            HistoryMessage::ClearRequested.into(),
        ),
        i18n.t("history.clear"),
        tooltip::Position::Left,
        theme_config,
    );

    row![
        count_text,
        container(refresh_button).padding(iced::Padding::new(4.0).left(10.0)),
        clear_button,
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

/// 创建一条历史记录列表行
pub fn create_history_row<'a>(
    i18n: &'a I18n,
    index: usize,
    entry: &HistoryEntry,
    thumb: Option<&Handle>,
    wallpaper: Option<&Wallpaper>,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 标题：序号 + 应用时间
    let applied_at = chrono::DateTime::from_timestamp(entry.applied_at, 0)
        .map(|dt| {
            dt.with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M")
                .to_string()
        })
        .unwrap_or_default();
    let title_text = text(format!(
        "{}. {} {}",
        index + 1,
        i18n.t("history.applied-at"),
        applied_at
    ))
    .size(14)
    .color(theme_colors.text);

    // 元信息：分辨率 · 文件大小 · 文件名
    let file_name = file_name_of(&entry.path);
    let meta_text = match wallpaper {
        Some(w) => format!(
            "{} x {} · {} · {}",
            w.width,
            w.height,
            format_file_size(w.file_size),
            file_name
        ),
        None => file_name,
    };

    // 操作按钮行
    let mut action_buttons = row![common::create_colored_button(
        i18n.t("history.apply"),
        theme_colors.primary,
        HistoryMessage::ApplyEntry(index).into(),
    )]
    .spacing(16)
    .align_y(Alignment::Center);

    // 缓存目录中的文件提供"保存到壁纸库"
    if !entry.in_library {
        action_buttons = action_buttons.push(action_text_button(
            i18n,
            theme_colors,
            "\u{F30A}", // download
            "history.save-to-library",
            BUTTON_COLOR_BLUE,
            HistoryMessage::SaveToLibrary(index).into(),
        ));
    }

    let actions = action_buttons
        .push(action_text_button(
            i18n,
            theme_colors,
            "\u{F341}", // eye
            "history.preview",
            BUTTON_COLOR_BLUE,
            HistoryMessage::PreviewEntry(index).into(),
        ))
        .push(action_text_button(
            i18n,
            theme_colors,
            "\u{F3D8}", // folder2-open
            "history.open-location",
            BUTTON_COLOR_BLUE,
            HistoryMessage::OpenLocation(index).into(),
        ))
        .push(action_text_button(
            i18n,
            theme_colors,
            "\u{F759}", // copy
            "history.copy-path",
            BUTTON_COLOR_BLUE,
            HistoryMessage::CopyPath(index).into(),
        ))
        .push(action_text_button(
            i18n,
            theme_colors,
            "\u{F78B}", // trash3
            "history.remove",
            BUTTON_COLOR_RED,
            HistoryMessage::RemoveEntry(index).into(),
        ));

    // 左侧信息列
    let info = column![
        title_text,
        text(meta_text).size(12).color(theme_colors.light_text_sub),
        actions
    ]
    .spacing(8)
    .width(Length::Fill);

    // 右侧缩略图（点击预览）
    let thumbnail: Element<'a, AppMessage> = match thumb {
        Some(handle) => button(
            image(handle)
                .width(Length::Fixed(HISTORY_THUMB_WIDTH))
                .height(Length::Fixed(HISTORY_THUMB_HEIGHT))
                .content_fit(iced::ContentFit::Cover),
        )
        .on_press(HistoryMessage::PreviewEntry(index).into())
        .padding(0)
        .style(|_theme: &iced::Theme, status| button::Style {
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(8.0),
            },
            ..button::text(_theme, status)
        })
        .into(),
        None => container(
            text(i18n.t("history.loading"))
                .size(12)
                .color(theme_colors.light_text_sub),
        )
        .width(Length::Fixed(HISTORY_THUMB_WIDTH))
        .height(Length::Fixed(HISTORY_THUMB_HEIGHT))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into(),
    };

    container(
        row![info, thumbnail]
            .align_y(Alignment::Center)
            .spacing(16)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(Padding::new(ROW_PADDING))
    .into()
}

/// SnowShot 风格的图标+文字操作按钮（透明底，悬停中性淡染）
fn action_text_button<'a>(
    i18n: &'a I18n,
    theme_colors: ThemeColors,
    icon: &'static str,
    label_key: &str,
    color: Color,
    message: AppMessage,
) -> button::Button<'a, AppMessage> {
    let label = i18n.t(label_key);
    button(
        row![
            text(icon)
                .font(Font::with_name("bootstrap-icons"))
                .size(ACTION_TEXT_SIZE)
                .color(color),
            text(label).size(ACTION_TEXT_SIZE).color(color),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(message)
    .padding(Padding::new(4.0))
    .style(move |_theme: &iced::Theme, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => theme_colors.hover_fill,
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color: color,
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            ..button::text(_theme, status)
        }
    })
}

/// 取路径末段作为显示文件名（超长截断）
fn file_name_of(path: &str) -> String {
    let name = std::path::Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());

    if name.chars().count() > MAX_NAME_CHARS {
        format!("{}…", name.chars().take(MAX_NAME_CHARS).collect::<String>())
    } else {
        name
    }
}
