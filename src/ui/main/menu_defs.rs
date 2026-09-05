// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 托盘菜单与悬浮球菜单的共享定义
//!
//! 两个菜单的 5 个动作项完全同源(仅菜单事件 id 前缀不同)，
//! 末项分别为"退出程序"(托盘)与"关闭悬浮球"(悬浮球)。
//! 菜单事件统一经 [`menu_action_from_id`] 解析后分发。

use crate::i18n::I18n;
use crate::platform::menu::MenuItemDef;
use std::collections::HashMap;

/// 托盘菜单事件 id 前缀
pub(in crate::ui::main) const TRAY_ID_PREFIX: &str = "tray_";
/// 悬浮球菜单事件 id 前缀
pub(in crate::ui::main) const BALL_ID_PREFIX: &str = "ball_";

/// 分隔线位置：两个菜单结构一致(显示主窗口之后、保存当前之后)
pub(in crate::ui::main) const MENU_SEPARATOR_AFTER: &[usize] = &[0, 3];

/// 托盘与悬浮球共有的菜单动作
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(in crate::ui::main) enum MenuAction {
    ShowWindow,
    SwitchPrevious,
    SwitchNext,
    SaveCurrent,
    Settings,
}

impl MenuAction {
    /// 菜单事件 id 中去掉前缀的部分
    pub(in crate::ui::main) fn id_suffix(self) -> &'static str {
        match self {
            MenuAction::ShowWindow => "show",
            MenuAction::SwitchPrevious => "switch_previous",
            MenuAction::SwitchNext => "switch_next",
            MenuAction::SaveCurrent => "save_current",
            MenuAction::Settings => "settings",
        }
    }

    /// 显示文本的 i18n key
    pub(in crate::ui::main) fn i18n_key(self) -> &'static str {
        match self {
            MenuAction::ShowWindow => "menu.tray-show",
            MenuAction::SwitchPrevious => "menu.tray-switch-previous",
            MenuAction::SwitchNext => "menu.tray-switch-next",
            MenuAction::SaveCurrent => "menu.tray-save-current",
            MenuAction::Settings => "menu.tray-settings",
        }
    }

    /// 初始可用状态(上一张需要历史满 2 条，初始禁用)
    pub(in crate::ui::main) fn initially_enabled(self) -> bool {
        self != MenuAction::SwitchPrevious
    }
}

/// 公共动作项定义表(顺序即菜单顺序)
pub(in crate::ui::main) const MENU_ACTIONS: [MenuAction; 5] = [
    MenuAction::ShowWindow,
    MenuAction::SwitchPrevious,
    MenuAction::SwitchNext,
    MenuAction::SaveCurrent,
    MenuAction::Settings,
];

/// 菜单项的完整事件 id
pub(in crate::ui::main) fn menu_item_id(prefix: &str, action: MenuAction) -> String {
    format!("{prefix}{}", action.id_suffix())
}

/// 构建"5 个公共动作项 + 1 个专属末项"的菜单定义
///
/// 返回 (菜单项定义列表, 事件 id → i18n key 映射，用于语言切换时刷新文本)
pub(in crate::ui::main) fn build_menu_defs(
    prefix: &str,
    last_item: (&str, &str),
    i18n: &I18n,
) -> (Vec<MenuItemDef>, HashMap<String, String>) {
    let mut defs: Vec<MenuItemDef> = MENU_ACTIONS
        .iter()
        .map(|&action| MenuItemDef {
            id: menu_item_id(prefix, action),
            text: i18n.t(action.i18n_key()),
            enabled: action.initially_enabled(),
        })
        .collect();
    let mut kv: HashMap<String, String> = MENU_ACTIONS
        .iter()
        .map(|&action| (menu_item_id(prefix, action), action.i18n_key().to_string()))
        .collect();
    kv.insert(last_item.0.to_string(), last_item.1.to_string());

    defs.push(MenuItemDef {
        id: last_item.0.to_string(),
        text: i18n.t(last_item.1),
        enabled: true,
    });

    (defs, kv)
}

/// 从菜单事件 id 解析公共动作(末项 id 不在此列，由调用方先行处理)
pub(in crate::ui::main) fn menu_action_from_id(id: &str) -> Option<MenuAction> {
    let suffix = id
        .strip_prefix(TRAY_ID_PREFIX)
        .or_else(|| id.strip_prefix(BALL_ID_PREFIX))?;
    MENU_ACTIONS
        .iter()
        .copied()
        .find(|action| action.id_suffix() == suffix)
}
