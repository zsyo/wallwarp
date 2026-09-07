// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹顶部工具条：统计 + 类型/时间/分组筛选 + 排序 + 新建/删除分组 + 刷新

use crate::i18n::I18n;
use crate::ui::common::drop_down::{
    self, Displayable, dropdown_picker, flat_dropdown_trigger_button,
};
use crate::ui::favorites::message::{GroupFilter, TimeFilter, TypeFilter};
use crate::ui::favorites::FavoritesMessage;
use crate::ui::favorites::FavoritesState;
use crate::ui::style::{BUTTON_COLOR_RED, FILTER_CONTROL_HEIGHT, ThemeConfig};
use crate::ui::{AppMessage, common};
use iced::widget::{container, row, text, tooltip};
use iced::{Alignment, Element, Length};

/// 创建收藏夹顶部工具条
pub fn create_favorites_toolbar<'a>(
    i18n: &'a I18n,
    favorites_state: &'a FavoritesState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let count_text = text(format!(
        "{}: {}",
        i18n.t("favorites.count-label"),
        favorites_state.entries.len()
    ))
    .size(15)
    .color(theme_colors.text);

    // 类型筛选下拉（全部/在线/本地）
    let type_options = vec![
        Displayable {
            value: TypeFilter::All,
            display: i18n.t("favorites.filter-type-all"),
        },
        Displayable {
            value: TypeFilter::Online,
            display: i18n.t("favorites.type-online"),
        },
        Displayable {
            value: TypeFilter::Local,
            display: i18n.t("favorites.type-local"),
        },
    ];
    let type_current = match favorites_state.type_filter {
        TypeFilter::All => i18n.t("favorites.filter-type-all"),
        TypeFilter::Online => i18n.t("favorites.type-online"),
        TypeFilter::Local => i18n.t("favorites.type-local"),
    };
    let type_trigger = flat_dropdown_trigger_button(
        type_current,
        80.0,
        theme_colors,
        FavoritesMessage::TypeFilterExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));
    let type_drop_down = dropdown_picker(
        type_trigger.into(),
        type_options,
        |v| v == favorites_state.type_filter,
        |v| FavoritesMessage::TypeFilterChanged(v).into(),
        FavoritesMessage::TypeFilterDismiss.into(),
        favorites_state.type_filter_expanded,
        100.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    );

    // 时间筛选下拉（全部/今日/三天内/本周/本月）
    let time_options = vec![
        Displayable {
            value: TimeFilter::All,
            display: i18n.t("favorites.filter-time-all"),
        },
        Displayable {
            value: TimeFilter::Today,
            display: i18n.t("favorites.filter-time-today"),
        },
        Displayable {
            value: TimeFilter::ThreeDays,
            display: i18n.t("favorites.filter-time-3days"),
        },
        Displayable {
            value: TimeFilter::ThisWeek,
            display: i18n.t("favorites.filter-time-week"),
        },
        Displayable {
            value: TimeFilter::ThisMonth,
            display: i18n.t("favorites.filter-time-month"),
        },
    ];
    let time_current = match favorites_state.time_filter {
        TimeFilter::All => i18n.t("favorites.filter-time-all"),
        TimeFilter::Today => i18n.t("favorites.filter-time-today"),
        TimeFilter::ThreeDays => i18n.t("favorites.filter-time-3days"),
        TimeFilter::ThisWeek => i18n.t("favorites.filter-time-week"),
        TimeFilter::ThisMonth => i18n.t("favorites.filter-time-month"),
    };
    let time_trigger = flat_dropdown_trigger_button(
        time_current,
        80.0,
        theme_colors,
        FavoritesMessage::TimeFilterExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));
    let time_drop_down = dropdown_picker(
        time_trigger.into(),
        time_options,
        |v| v == favorites_state.time_filter,
        |v| FavoritesMessage::TimeFilterChanged(v).into(),
        FavoritesMessage::TimeFilterDismiss.into(),
        favorites_state.time_filter_expanded,
        100.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    );

    // 分组筛选下拉（全部/未分组/各分组）
    let mut group_options: Vec<Displayable<GroupFilter>> = vec![Displayable {
        value: GroupFilter::All,
        display: i18n.t("favorites.group-all"),
    }];
    if favorites_state
        .all_entries
        .iter()
        .any(|e| e.fav.group_id.is_none())
    {
        group_options.push(Displayable {
            value: GroupFilter::Ungrouped,
            display: i18n.t("favorites.group-ungrouped"),
        });
    }
    for group in &favorites_state.groups {
        group_options.push(Displayable {
            value: GroupFilter::Group(group.id),
            display: group.name.clone(),
        });
    }

    let group_current = match &favorites_state.group_filter {
        GroupFilter::All => i18n.t("favorites.group-all"),
        GroupFilter::Ungrouped => i18n.t("favorites.group-ungrouped"),
        GroupFilter::Group(id) => favorites_state
            .groups
            .iter()
            .find(|g| g.id == *id)
            .map(|g| g.name.clone())
            .unwrap_or_else(|| i18n.t("favorites.group-all")),
    };

    let group_trigger = flat_dropdown_trigger_button(
        group_current,
        120.0,
        theme_colors,
        FavoritesMessage::GroupFilterExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    let group_filter_drop_down = dropdown_picker(
        group_trigger.into(),
        group_options,
        |v| v == favorites_state.group_filter,
        |v| FavoritesMessage::GroupFilterChanged(v).into(),
        FavoritesMessage::GroupFilterDismiss.into(),
        favorites_state.group_filter_expanded,
        140.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    );

    // 排序方向切换按钮（收藏时间倒序=chevron-down 最新在前，正序=chevron-up）
    let (sort_icon, sort_tooltip_key) = if favorites_state.sort_descending {
        ("\u{F282}", "favorites.sort-descending") // chevron-down
    } else {
        ("\u{F286}", "favorites.sort-ascending") // chevron-up
    };
    let sort_button = common::create_button_with_tooltip(
        common::create_icon_button_with_size(
            sort_icon,
            theme_colors.light_text,
            16,
            FavoritesMessage::SortOrderToggled.into(),
        ),
        i18n.t(sort_tooltip_key),
        tooltip::Position::Top,
        theme_config,
    );

    // 新建分组按钮
    let create_group_button = common::create_button_with_tooltip(
        common::create_icon_button_with_size(
            "\u{F2E7}", // plus-lg
            theme_colors.light_text,
            16,
            FavoritesMessage::CreateGroupRequested.into(),
        ),
        i18n.t("favorites.create-group"),
        tooltip::Position::Top,
        theme_config,
    );

    // 删除分组按钮（仅选中具体分组时可用）
    let delete_group_button: Element<'a, AppMessage> =
        if matches!(favorites_state.group_filter, GroupFilter::Group(_)) {
            common::create_button_with_tooltip(
                common::create_icon_button_with_size(
                    "\u{F78B}", // trash3
                    BUTTON_COLOR_RED,
                    16,
                    FavoritesMessage::DeleteGroupRequested.into(),
                ),
                i18n.t("favorites.delete-group"),
                tooltip::Position::Top,
                theme_config,
            )
        } else {
            common::create_icon_button_disabled("\u{F78B}", theme_colors.disabled_color).into()
        };

    let refresh_button = common::create_button_with_tooltip(
        common::create_icon_button_with_size(
            "\u{F130}", // arrow-repeat
            theme_colors.light_text,
            16,
            FavoritesMessage::Refresh.into(),
        ),
        i18n.t("favorites.refresh"),
        tooltip::Position::Left,
        theme_config,
    );

    row![
        count_text,
        container(type_drop_down).padding(iced::Padding::new(4.0).left(10.0)),
        container(time_drop_down).padding(iced::Padding::new(4.0).left(4.0)),
        container(group_filter_drop_down).padding(iced::Padding::new(4.0).left(4.0)),
        sort_button,
        create_group_button,
        delete_group_button,
        refresh_button,
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}
