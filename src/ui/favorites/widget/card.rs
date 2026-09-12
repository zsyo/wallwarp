// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏卡片：类型徽章 + 缩略图 + 底部信息遮罩 + 操作按钮

use crate::i18n::I18n;
use crate::services::database::FavoriteDB;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::favorites::state::{FavoriteEntry, ThumbState};
use crate::ui::favorites::FavoritesMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, ERROR_ICON_SIZE, ERROR_TEXT_SIZE,
    IMAGE_HEIGHT, IMAGE_WIDTH, RADIUS_FULL, ThemeColors, ThemeConfig, with_alpha,
};
use crate::utils::helpers::format_file_size;
use iced::widget::{column, container, image, stack, text, tooltip};
use iced::{Alignment, Element, Font, Length, Padding};

/// 类型徽章内边距
const BADGE_PADDING: [f32; 2] = [2.0, 8.0];
/// 类型徽章与卡片边缘的距离
const BADGE_MARGIN: f32 = 6.0;
/// 类型徽章文字大小
const BADGE_TEXT_SIZE: f32 = 11.0;

/// 创建一条收藏卡片
pub fn create_favorite_card<'a>(
    i18n: &'a I18n,
    index: usize,
    entry: &'a FavoriteEntry,
    thumb: &'a ThumbState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();
    let fav = &entry.fav;

    // 本地项源文件已失效：失效占位 + 仅保留移除操作
    let is_local_missing =
        fav.kind == crate::services::database::KIND_LOCAL && !entry.in_library;

    // 缩略图或占位
    let thumbnail: Element<'a, AppMessage> = if is_local_missing {
        error_placeholder(
            i18n,
            "\u{F33A}", // exclamation-triangle-fill
            "favorites.file-missing",
            theme_colors,
        )
    } else {
        match thumb {
            ThumbState::Loaded(handle) => image(handle)
                .width(Length::Fixed(IMAGE_WIDTH))
                .height(Length::Fixed(IMAGE_HEIGHT))
                .content_fit(iced::ContentFit::Cover)
                .into(),
            ThumbState::Loading => container(
                text(i18n.t("favorites.loading"))
                    .size(12)
                    .color(theme_colors.light_text_sub),
            )
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
            ThumbState::Failed => error_placeholder(
                i18n,
                "\u{F428}", // image-alt
                "favorites.load-failed",
                theme_colors,
            ),
        }
    };

    // 内容区 = 缩略图 + 左上角类型徽章
    let content = stack(vec![
        thumbnail,
        container(type_badge(i18n, &fav.kind, theme_colors))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::Start)
            .padding(Padding::new(BADGE_MARGIN))
            .into(),
    ]);

    // 底部遮罩信息
    let file_size_text = if fav.file_size > 0 {
        format_file_size(fav.file_size as u64)
    } else {
        String::new()
    };
    let resolution = if fav.resolution.is_empty() {
        None
    } else {
        Some(fav.resolution.clone())
    };

    // 操作按钮：设为壁纸 + 下载(在线未入库) / 在文件夹查看(本地正常项与已入库在线项) + 移除
    // 本地项源文件失效时，设壁纸/查看禁用（仅保留移除）
    let mut actions = vec![if is_local_missing {
        common::create_icon_button_disabled("\u{F429}", theme_colors.disabled_color).into() // image-fill
    } else {
        favorite_action_button(
            "\u{F429}", // image-fill
            BUTTON_COLOR_GREEN,
            i18n.t("favorites.tooltip-set-wallpaper"),
            FavoritesMessage::ApplyEntry(index).into(),
            theme_config,
        )
    }];

    if fav.kind == crate::services::database::KIND_ONLINE && !entry.in_library {
        actions.push(favorite_action_button(
            "\u{F30A}", // download
            BUTTON_COLOR_BLUE,
            i18n.t("favorites.tooltip-download"),
            FavoritesMessage::DownloadEntry(index).into(),
            theme_config,
        ));
    } else if is_local_missing {
        actions.push(
            common::create_icon_button_disabled("\u{F3D8}", theme_colors.disabled_color).into(), // folder2-open
        );
    } else {
        // 本地正常项与已入库在线项：在文件夹中查看
        actions.push(favorite_action_button(
            "\u{F3D8}", // folder2-open
            BUTTON_COLOR_GREEN,
            i18n.t("favorites.tooltip-open-location"),
            FavoritesMessage::OpenLocation(index).into(),
            theme_config,
        ));
    }

    actions.push(favorite_action_button(
        "\u{F78B}", // trash3
        BUTTON_COLOR_RED,
        i18n.t("favorites.remove"),
        FavoritesMessage::RemoveEntry(index).into(),
        theme_config,
    ));

    // 本地项源文件失效时禁止点击预览（避免对不存在文件构造 Handle）
    let on_press = if is_local_missing {
        None
    } else {
        Some(FavoritesMessage::PreviewEntry(index).into())
    };

    common::wallpaper_card(
        content.into(),
        file_size_text,
        resolution,
        actions,
        4.0,
        on_press,
        theme_colors,
    )
}

/// 错误占位（图标 + 文案，样式与在线页失败占位一致）
fn error_placeholder<'a>(
    i18n: &'a I18n,
    icon: &'static str,
    label_key: &str,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    container(
        column![
            text(icon)
                .font(Font::with_name("bootstrap-icons"))
                .color(theme_colors.disabled_color)
                .size(ERROR_ICON_SIZE),
            text(i18n.t(label_key))
                .size(ERROR_TEXT_SIZE)
                .color(theme_colors.text),
        ]
        .spacing(8)
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(IMAGE_WIDTH))
    .height(Length::Fixed(IMAGE_HEIGHT))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .into()
}

/// 类型徽章：在线=蓝色底、本地=绿色底（叠在缩略图上，主题无关）
fn type_badge<'a>(
    i18n: &'a I18n,
    kind: &str,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let (label, bg) = if kind == crate::services::database::KIND_ONLINE {
        (
            i18n.t("favorites.type-online"),
            with_alpha(BUTTON_COLOR_BLUE, 0.75),
        )
    } else {
        (
            i18n.t("favorites.type-local"),
            with_alpha(BUTTON_COLOR_GREEN, 0.75),
        )
    };

    container(
        text(label)
            .size(BADGE_TEXT_SIZE)
            .color(theme_colors.overlay_text),
    )
    .padding(BADGE_PADDING)
    .style(move |_theme| container::Style {
        background: Some(iced::Background::Color(bg)),
        border: iced::border::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(RADIUS_FULL),
        },
        ..Default::default()
    })
    .into()
}

/// 卡片操作按钮（图标 + tooltip，与在线页卡片按钮同款）
fn favorite_action_button<'a>(
    icon: &'static str,
    color: iced::Color,
    tooltip_text: String,
    message: AppMessage,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    common::create_button_with_tooltip(
        common::create_icon_button(icon, color, message),
        tooltip_text,
        tooltip::Position::Top,
        theme_config,
    )
}

/// 预览弹窗信息浮层（类型/分辨率/纯净度/收藏数/文件大小）
pub fn create_favorite_modal_info<'a>(
    i18n: &'a I18n,
    fav: &'a FavoriteDB,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let _ = theme_config;
    let type_label = if fav.kind == crate::services::database::KIND_ONLINE {
        i18n.t("favorites.type-online")
    } else {
        i18n.t("favorites.type-local")
    };

    let mut info_column = iced::widget::column![
        common::modal_info_row(
            i18n.t("favorites.info-type").as_str(),
            type_label,
        ),
        common::modal_info_row(
            i18n.t("wallpaper-info.resolution").as_str(),
            if fav.resolution.is_empty() {
                format!("{} x {}", fav.width, fav.height)
            } else {
                fav.resolution.clone()
            },
        ),
    ]
    .spacing(4)
    .align_x(Alignment::Start);

    if !fav.purity.is_empty() {
        info_column = info_column.push(common::modal_info_row(
            i18n.t("wallpaper-info.purity").as_str(),
            fav.purity.to_uppercase(),
        ));
    }
    if fav.file_size > 0 {
        info_column = info_column.push(common::modal_info_row(
            i18n.t("wallpaper-info.file-size").as_str(),
            format_file_size(fav.file_size as u64),
        ));
    }

    common::modal_info_pill(info_column.into())
}
