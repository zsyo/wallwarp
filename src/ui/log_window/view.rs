// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 日志展示窗口视图：顶栏（等级/最大行数/自动滚动/清空）+ 日志滚动区

use super::SCROLL_ID;
use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::common::{self, styled_text_input};
use crate::ui::log_window::LogWindowMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, COLOR_SKETCHY, INPUT_PADDING, LOG_LEVEL_PICK_LIST_WIDTH, RADIUS_FULL,
    RADIUS_SM, TEXT_INPUT_SIZE, ThemeColors, tint,
};
use crate::ui::{App, AppMessage};
use crate::utils::config::LogLevel;
use crate::utils::log_window_layer::LogLine;
use iced::border::Border;
use iced::widget::{
    Id, Space, button, column, container, opaque, row, scrollable, text, text_input,
};
use iced::{Alignment, Color, Element, Font, Length};
use tracing::Level;

/// 日志等级对应的翻译词条键
fn log_level_option_key(level: LogLevel) -> String {
    format!("log-level-options.{}", level.as_str())
}

/// 日志等级展示样式：等级主色 + 行淡染底（仅 WARN/ERROR 配淡染）
///
/// ERROR/WARN/INFO 分别取通知错误色（主题自适应）/琥珀色/唯一强调色，
/// DEBUG/TRACE 用次级文字色弱化
fn level_style(level: Level, theme_colors: ThemeColors) -> (Color, Option<Color>) {
    match level {
        Level::ERROR => (
            theme_colors.notification_error_bg,
            Some(tint(theme_colors.notification_error_bg, 0.08)),
        ),
        Level::WARN => (COLOR_SKETCHY, Some(tint(COLOR_SKETCHY, 0.08))),
        Level::INFO => (theme_colors.primary, None),
        Level::DEBUG | Level::TRACE => (theme_colors.light_text_sub, None),
    }
}

/// 渲染单行日志：等级色点徽章 + 时间戳（弱化）+ 等级词（语义色）+ 消息体
fn log_line_row<'a>(line: &'a LogLine, theme_colors: ThemeColors) -> Element<'a, AppMessage> {
    let (level_color, row_tint) = level_style(line.level, theme_colors);

    container(
        row![
            // 等级徽章（8px 圆角色点）
            container(Space::new())
                .width(Length::Fixed(8.0))
                .height(Length::Fixed(8.0))
                .style(move |_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(level_color)),
                    border: Border {
                        radius: RADIUS_FULL.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                }),
            text(&line.time)
                .size(12)
                .font(Font::MONOSPACE)
                .color(theme_colors.light_text_sub),
            text(format!("{:5}", line.level.as_str()))
                .size(12)
                .font(Font::MONOSPACE)
                .color(level_color),
            // 消息体含中文，不用等宽字体：等宽字体无中文字形，回退字体
            // 在小字号下渲染发虚且行高与等宽不一致，导致同行中英文高低不齐
            text(&line.message).size(12).color(theme_colors.text),
            Space::new().width(Length::Fill),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([2, 6])
    .style(move |_theme: &iced::Theme| container::Style {
        background: row_tint.map(iced::Background::Color),
        border: Border {
            radius: RADIUS_SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    })
    .into()
}

/// 创建日志展示窗口视图
pub fn log_window_view(app: &App) -> Element<'_, AppMessage> {
    let theme_colors = app.theme_colors;
    let state = &app.log_window_state;
    let level = app.config.global.log_window_level;

    // 等级下拉框（独立于设置页的日志等级，仅控制本窗口显示）
    let level_trigger = dropdown_trigger_button(
        app.i18n.t(&log_level_option_key(level)),
        LOG_LEVEL_PICK_LIST_WIDTH,
        theme_colors,
        LogWindowMessage::LevelPickerExpanded.into(),
    );
    let options_content = column(LogLevel::all().iter().map(|level| {
        let is_selected = level == &app.config.global.log_window_level;
        button(text(app.i18n.t(&log_level_option_key(*level))).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(LogWindowMessage::LevelSelected(*level).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);
    let picker_content = container(options_content)
        .padding(8)
        .width(Length::Fixed(LOG_LEVEL_PICK_LIST_WIDTH))
        .style(dropdown_panel_style(theme_colors));
    let level_picker = DropDown::new(
        level_trigger,
        opaque(picker_content),
        state.level_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(LogWindowMessage::LevelPickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom);

    // 最大保留行数输入：仅本地缓冲，保存按钮/回车提交时校验区间
    let max_lines_input = text_input("1000", &state.max_lines_input)
        .width(Length::Fixed(72.0))
        .size(TEXT_INPUT_SIZE)
        .padding(INPUT_PADDING)
        .style(styled_text_input(theme_colors))
        .on_input(|input| LogWindowMessage::MaxLinesChanged(input).into())
        .on_submit(LogWindowMessage::MaxLinesSubmitted.into());

    // 第一行：等级 + 保留行数（输入/保存）+ 自动滚动 + 清空
    let toolbar_row = row![
        text(app.i18n.t("log-window.level"))
            .size(14)
            .color(theme_colors.text),
        level_picker,
        Space::new().width(12),
        text(app.i18n.t("log-window.max-lines"))
            .size(14)
            .color(theme_colors.text),
        max_lines_input,
        common::create_colored_button(
            app.i18n.t("settings.save"),
            BUTTON_COLOR_BLUE,
            LogWindowMessage::MaxLinesSubmitted.into(),
        ),
        Space::new().width(12),
        text(app.i18n.t("log-window.auto-scroll"))
            .size(14)
            .color(theme_colors.text),
        iced::widget::toggler(state.auto_scroll)
            .on_toggle(|enabled| LogWindowMessage::AutoScrollToggled(enabled).into()),
        Space::new().width(12),
        text(app.i18n.t("log-window.always-on-top"))
            .size(14)
            .color(theme_colors.text),
        iced::widget::toggler(state.always_on_top)
            .on_toggle(|enabled| LogWindowMessage::AlwaysOnTopToggled(enabled).into()),
        Space::new().width(Length::Fill),
        common::create_colored_button(
            app.i18n.t("log-window.clear"),
            BUTTON_COLOR_BLUE,
            LogWindowMessage::ClearBuffer.into(),
        ),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    // 第二行：筛选关键字（占满宽度；只影响显示，后台照常全量记录）
    let filter_input = text_input(
        &app.i18n.t("log-window.filter-placeholder"),
        &state.filter_text,
    )
    .width(Length::Fill)
    .size(TEXT_INPUT_SIZE)
    .padding(INPUT_PADDING)
    .style(styled_text_input(theme_colors))
    .on_input(|keyword| LogWindowMessage::FilterChanged(keyword).into());

    let filter_row = row![
        text(app.i18n.t("log-window.filter"))
            .size(14)
            .color(theme_colors.text),
        filter_input,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let toolbar_container = container(column![toolbar_row, filter_row].spacing(6))
        .width(Length::Fill)
        .padding([8, 12])
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.light_bg)),
            ..Default::default()
        });

    // 日志内容区：筛选关键字非空时只显示匹配行（大小写不敏感，后台仍全量记录）；
    // 无数据或无匹配时分别给不同空态提示。等宽渲染各行，允许横向滚动避免长行截断
    let filter = state.filter_text.trim().to_lowercase();
    let visible_lines: Vec<&LogLine> = if filter.is_empty() {
        state.lines.iter().collect()
    } else {
        state
            .lines
            .iter()
            .filter(|line| line.message.to_lowercase().contains(&filter))
            .collect()
    };

    let body: Element<'_, AppMessage> = if visible_lines.is_empty() {
        let empty_key = if filter.is_empty() {
            "log-window.empty"
        } else {
            "log-window.no-match"
        };
        container(
            text(app.i18n.t(empty_key))
                .size(14)
                .color(theme_colors.light_text_sub),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        let log_column = column(
            visible_lines
                .into_iter()
                .map(|line| log_line_row(line, theme_colors)),
        )
        .spacing(2);
        scrollable(log_column)
            .id(Id::new(SCROLL_ID))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    };

    container(column![toolbar_container, body])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
