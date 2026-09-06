// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{INPUT_PADDING, ROW_SPACING};
use crate::ui::{App, AppMessage};
use crate::utils::config::{WallpaperAutoChangeInterval, WallpaperAutoChangeMode, WallpaperMode};
use iced::widget::{checkbox, container, row, text, text_input, tooltip};
use iced::{Alignment, Element, Length};

/// 单选选项数据：(标签词条, 选项值, 提示词条)
type RadioOption<'a, V> = (&'static str, V, &'static str);

/// 数据驱动生成一排带提示的单选按钮
///
/// 标签与提示文本按词条键从 i18n 取得，选中回调由调用方整行共用
fn create_radio_row<'a, V>(
    app: &'a App,
    options: &[RadioOption<'a, V>],
    selected: Option<V>,
    on_select: fn(V) -> AppMessage,
) -> Element<'a, AppMessage>
where
    V: Copy + Eq + 'a,
{
    let mut radio_row = row![].spacing(ROW_SPACING);
    for (label_key, value, tooltip_key) in options {
        radio_row = radio_row.push(common::create_radio_with_tooltip(
            app.i18n.t(label_key),
            *value,
            selected,
            on_select,
            app.i18n.t(tooltip_key),
            app.theme_colors,
        ));
    }
    radio_row.into()
}

/// 创建壁纸配置区块
pub fn create_wallpaper_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;

    let rows = vec![
        // 壁纸模式（6 个选项，占满整行）
        super::create_full_width_row(
            app.i18n.t("settings.wallpaper-mode"),
            Some(app.i18n.t("settings.wallpaper-mode-desc")),
            create_radio_row(
                app,
                &[
                    (
                        "wallpaper-mode-options.crop",
                        WallpaperMode::Crop,
                        "wallpaper-mode-options.crop-tooltip",
                    ),
                    (
                        "wallpaper-mode-options.fit",
                        WallpaperMode::Fit,
                        "wallpaper-mode-options.fit-tooltip",
                    ),
                    (
                        "wallpaper-mode-options.stretch",
                        WallpaperMode::Stretch,
                        "wallpaper-mode-options.stretch-tooltip",
                    ),
                    (
                        "wallpaper-mode-options.tile",
                        WallpaperMode::Tile,
                        "wallpaper-mode-options.tile-tooltip",
                    ),
                    (
                        "wallpaper-mode-options.center",
                        WallpaperMode::Center,
                        "wallpaper-mode-options.center-tooltip",
                    ),
                    (
                        "wallpaper-mode-options.span",
                        WallpaperMode::Span,
                        "wallpaper-mode-options.span-tooltip",
                    ),
                ],
                Some(app.settings_state.wallpaper_mode),
                |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
            ),
            theme_colors,
        ),
        // 定时切换模式
        super::create_setting_row(
            app.i18n.t("settings.auto-change-mode"),
            Some(app.i18n.t("settings.auto-change-mode-desc")),
            create_radio_row(
                app,
                &[
                    (
                        "auto-change-mode-options.online",
                        WallpaperAutoChangeMode::Online,
                        "auto-change-mode-options.online-tooltip",
                    ),
                    (
                        "auto-change-mode-options.local",
                        WallpaperAutoChangeMode::Local,
                        "auto-change-mode-options.local-tooltip",
                    ),
                ],
                Some(app.settings_state.auto_change_mode),
                |mode| SettingsMessage::AutoChangeModeSelected(mode).into(),
            ),
            theme_colors,
        ),
        // 定时切换周期（4 个周期 + 自定义 + 启用开关，占满整行）
        super::create_full_width_row(
            app.i18n.t("settings.auto-change-interval"),
            Some(app.i18n.t("settings.auto-change-interval-desc")),
            interval_value_row(app),
            theme_colors,
        ),
        // 定时切换在线配置（排序 + 时间范围 + 关键词，占满整行）
        super::create_full_width_row(
            app.i18n.t("settings.auto-change-online-config"),
            Some(app.i18n.t("settings.auto-change-online-config-desc")),
            online_config_row(app),
            theme_colors,
        ),
        // 预留：多显示器独立壁纸
        super::create_setting_row(
            app.i18n.t("settings.multi-monitor"),
            Some(app.i18n.t("settings.multi-monitor-desc")),
            super::create_coming_soon_badge(app.i18n.t("settings.coming-soon"), theme_colors),
            theme_colors,
        ),
    ];

    super::create_config_section(
        "\u{F42A}", // image
        app.i18n.t("settings.category-wallpaper"),
        rows,
        &app.theme_config,
    )
}

/// 定时切换周期行的值区域：周期单选（不含关闭，启停由独立开关控制）+ 自定义分钟输入 + 启用开关
fn interval_value_row<'a>(app: &'a App) -> Element<'a, AppMessage> {
    row![
        create_radio_row(
            app,
            &[
                (
                    "auto-change-interval-options.ten-min",
                    WallpaperAutoChangeInterval::Minutes(10),
                    "auto-change-interval-options.ten-min-tooltip",
                ),
                (
                    "auto-change-interval-options.thirty-min",
                    WallpaperAutoChangeInterval::Minutes(30),
                    "auto-change-interval-options.thirty-min-tooltip",
                ),
                (
                    "auto-change-interval-options.one-hour",
                    WallpaperAutoChangeInterval::Minutes(60),
                    "auto-change-interval-options.one-hour-tooltip",
                ),
                (
                    "auto-change-interval-options.two-hour",
                    WallpaperAutoChangeInterval::Minutes(120),
                    "auto-change-interval-options.two-hour-tooltip",
                ),
            ],
            Some(app.settings_state.auto_change_interval),
            |interval| SettingsMessage::AutoChangeIntervalSelected(interval).into(),
        ),
        create_custom_interval_tooltip(app),
        create_auto_change_toggle(app),
    ]
    .spacing(ROW_SPACING)
    .align_y(Alignment::Center)
    .into()
}

/// 定时切换在线配置行：排序选择 + 时间范围选择 + 关键词输入 + 保存
fn online_config_row<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    row![
        super::create_sorting_picker(app, theme_colors),
        super::create_time_range_picker(app, theme_colors),
        text_input(
            &app.i18n.t("settings.auto-change-query-placeholder"),
            &app.settings_state.auto_change_query
        )
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(INPUT_PADDING)
        .on_input(|query| SettingsMessage::AutoChangeQueryChanged(query).into())
        .style(common::styled_text_input(theme_colors)),
        common::create_colored_button(
            app.i18n.t("settings.save"),
            crate::ui::style::BUTTON_COLOR_BLUE,
            SettingsMessage::SaveAutoChangeQuery.into()
        ),
    ]
    .spacing(ROW_SPACING)
    .align_y(Alignment::Center)
    .into()
}

/// 创建"启用定时切换"开关（启停状态持久化到配置，配置的周期不受影响）
///
/// 与行内单选项同规格：16px 框、8px 间距、默认字号文字
fn create_auto_change_toggle<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let is_enabled = app.auto_change_state.auto_change_enabled;

    row![
        checkbox(is_enabled)
            .size(16)
            .on_toggle(move |_state| SettingsMessage::AutoChangeToggled(!is_enabled).into())
            .style(common::checkbox_style(theme_colors, is_enabled)),
        text(app.i18n.t("settings.auto-change-toggle")).color(theme_colors.text),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

/// 创建自定义分钟数单选项（带数字输入框的复合控件）
fn create_custom_interval_tooltip<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    tooltip(
        row![
            iced::widget::radio(
                app.i18n.t("auto-change-interval-options.custom"),
                WallpaperAutoChangeInterval::Custom(app.settings_state.custom_interval_minutes),
                Some(app.settings_state.auto_change_interval),
                |interval| {
                    if let WallpaperAutoChangeInterval::Custom(minutes) = interval {
                        SettingsMessage::AutoChangeIntervalSelected(
                            WallpaperAutoChangeInterval::Custom(minutes),
                        )
                        .into()
                    } else {
                        SettingsMessage::AutoChangeIntervalSelected(interval).into()
                    }
                }
            )
            .style(common::radio_transparent_style(theme_colors.text)),
            container(
                row![
                    iced_aw::NumberInput::new(
                        &app.settings_state.custom_interval_minutes,
                        1..=1440,
                        |minutes| SettingsMessage::CustomIntervalMinutesChanged(minutes).into()
                    )
                    .width(Length::Fill)
                    .padding(INPUT_PADDING)
                    .input_style(common::styled_text_input(theme_colors))
                    .style(move |_theme: &iced::Theme, _status| {
                        iced_aw::number_input::Style {
                            button_background: Some(iced::Background::Color(
                                theme_colors.text_input_background,
                            )),
                            icon_color: theme_colors.light_text_sub,
                        }
                    }),
                    text(app.i18n.t("settings.minutes"))
                        .size(14)
                        .color(theme_colors.light_text),
                ]
                .spacing(4)
                .align_y(Alignment::Center)
            )
            .width(Length::Fixed(120.0)),
        ]
        .spacing(ROW_SPACING)
        .align_y(Alignment::Center),
        text(app.i18n.t("auto-change-interval-options.custom-tooltip")).style(
            move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            },
        ),
        tooltip::Position::Top,
    )
    .style(common::create_tooltip_style(theme_colors))
    .into()
}
