// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 桌面悬浮球：置顶透明小窗口，点击弹出操作菜单，可拖动并记忆位置

mod view;

pub use view::floating_ball_view;

use crate::i18n::I18n;
use crate::utils::config::GlobalConfig;
use std::collections::HashMap;
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

/// 悬浮球贴边的屏幕边缘（仅支持左右贴边）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapEdge {
    Left,
    Right,
}

/// 悬浮球的贴边吸附上下文
///
/// 记录贴边方向与所用显示器的工作区（物理像素）。
/// 多屏场景下球在接缝处会横跨两屏，MonitorFromWindow 判定有歧义，
/// 因此展开时必须使用吸附时保存的工作区，而不是重新查询。
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SnapState {
    /// 当前贴边方向（None = 自由位置）
    pub edge: Option<SnapEdge>,
    /// 贴边所用显示器的工作区（物理像素，排除任务栏）
    pub work: iced::Rectangle,
}

/// 悬浮球窗口边长（逻辑像素）
pub const BALL_SIZE: f32 = 48.0;
/// 悬浮球图标边长（逻辑像素）
pub const BALL_ICON_SIZE: f32 = 32.0;
/// 触发拖动的最小移动距离（逻辑像素）
pub const DRAG_THRESHOLD: f32 = 5.0;

/// 悬浮球交互状态（用于区分点击与拖动）
#[derive(Debug, Default)]
pub struct FloatingBallState {
    /// 鼠标是否按在悬浮球上
    pressed: bool,
    /// 按下时光标位置（窗口内坐标，首次 on_move 时记录）
    press_pos: Option<iced::Point>,
    /// 是否已进入系统级拖动
    dragging: bool,
    /// 鼠标是否悬停在悬浮球上
    hovered: bool,
    /// 悬浮球菜单是否打开（打开期间不贴边隐藏）
    menu_open: bool,
    /// 贴边吸附上下文（None 表示处于自由位置）。
    /// 贴边时视图将球向边缘偏移一半，仅绘制半圆（窗口外部分被裁剪）
    snap: SnapState,
}

impl FloatingBallState {
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    /// 记录按下（按下时还没有光标位置）
    pub fn press(&mut self) {
        self.pressed = true;
        self.press_pos = None;
        self.dragging = false;
    }

    /// 光标移动：返回是否达到拖动阈值
    pub fn cursor_moved(&mut self, pos: iced::Point) -> bool {
        if !self.pressed {
            return false;
        }
        match self.press_pos {
            None => {
                self.press_pos = Some(pos);
                false
            }
            Some(start) => {
                let dx = (pos.x - start.x).abs();
                let dy = (pos.y - start.y).abs();
                !self.dragging && (dx > DRAG_THRESHOLD || dy > DRAG_THRESHOLD)
            }
        }
    }

    /// 标记进入拖动
    pub fn set_dragging(&mut self) {
        self.dragging = true;
    }

    /// 鼠标释放：返回是否为有效点击（未拖动）
    pub fn release(&mut self) -> bool {
        let clicked = self.pressed && !self.dragging;
        self.pressed = false;
        self.press_pos = None;
        clicked
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    /// 当前贴边吸附上下文
    pub fn snap(&self) -> SnapState {
        self.snap
    }

    pub fn set_snap(&mut self, snap: SnapState) {
        self.snap = snap;
    }

    /// 清除贴边状态（进入自由拖动时）
    pub fn clear_snap(&mut self) {
        self.snap = SnapState::default();
    }

    pub fn is_menu_open(&self) -> bool {
        self.menu_open
    }

    pub fn set_menu_open(&mut self, open: bool) {
        self.menu_open = open;
    }
}

/// 悬浮球菜单管理器
///
/// 菜单结构同托盘菜单，最后一项为“关闭悬浮球”（而非退出程序）；
/// 菜单事件经由 muda 全局 channel 流入现有的 TrayMenuEvent 订阅。
pub struct FloatingBallManager {
    menu: Menu,
    kv: HashMap<String, String>,
    items: HashMap<String, MenuItem>,
}

impl FloatingBallManager {
    pub fn new(i18n: &I18n) -> Self {
        let mut kv = HashMap::new();
        let mut items = HashMap::new();

        let add_item = |items: &mut HashMap<String, MenuItem>,
                        kv: &mut HashMap<String, String>,
                        id: &str,
                        i18n_key: &str,
                        enabled: bool| {
            let item = MenuItem::with_id(id, i18n.t(i18n_key), enabled, None);
            kv.insert(id.to_string(), i18n_key.to_string());
            items.insert(id.to_string(), item);
        };

        add_item(&mut items, &mut kv, "ball_show", "menu.tray-show", true);
        add_item(
            &mut items,
            &mut kv,
            "ball_switch_previous",
            "menu.tray-switch-previous",
            false,
        );
        add_item(
            &mut items,
            &mut kv,
            "ball_switch_next",
            "menu.tray-switch-next",
            true,
        );
        add_item(
            &mut items,
            &mut kv,
            "ball_save_current",
            "menu.tray-save-current",
            true,
        );
        add_item(
            &mut items,
            &mut kv,
            "ball_settings",
            "menu.tray-settings",
            true,
        );
        add_item(&mut items, &mut kv, "ball_close", "menu.ball-close", true);

        let menu = Menu::with_items(&[
            items.get("ball_show").unwrap(),
            &PredefinedMenuItem::separator(),
            items.get("ball_switch_previous").unwrap(),
            items.get("ball_switch_next").unwrap(),
            items.get("ball_save_current").unwrap(),
            &PredefinedMenuItem::separator(),
            items.get("ball_settings").unwrap(),
            items.get("ball_close").unwrap(),
        ])
        .unwrap();

        Self { menu, kv, items }
    }

    /// 在指定窗口句柄处弹出悬浮球菜单（须在拥有窗口的主线程调用）
    pub fn show_popup_at(&self, hwnd: isize) -> bool {
        use tray_icon::menu::ContextMenu;
        unsafe { self.menu.show_context_menu_for_hwnd(hwnd, None) }
    }

    pub fn update_switch_previous_item(&mut self, history_count: usize) {
        self.items
            .get("ball_switch_previous")
            .unwrap()
            .set_enabled(history_count >= 2);
    }

    pub fn update_save_current_item(&mut self, can_save: bool) {
        self.items
            .get("ball_save_current")
            .unwrap()
            .set_enabled(can_save);
    }

    pub fn update_i18n(&mut self, i18n: &I18n) {
        for (id, lang_key) in self.kv.iter() {
            if let Some(item) = self.items.get_mut(id) {
                item.set_text(i18n.t(lang_key));
            }
        }
    }
}

/// 根据配置构建悬浮球窗口参数
///
/// 位置：配置有效（非 i32::MIN）时恢复上次位置，否则窗口居中
pub fn window_settings(global: &GlobalConfig) -> iced::window::Settings {
    use iced::window::{self, Level};

    let position = if global.floating_ball_x != i32::MIN && global.floating_ball_y != i32::MIN {
        window::Position::Specific(iced::Point::new(
            global.floating_ball_x as f32,
            global.floating_ball_y as f32,
        ))
    } else {
        window::Position::Centered
    };

    window::Settings {
        position,
        size: iced::Size::new(BALL_SIZE, BALL_SIZE),
        min_size: Some(iced::Size::new(BALL_SIZE, BALL_SIZE)),
        max_size: Some(iced::Size::new(BALL_SIZE, BALL_SIZE)),
        resizable: false,
        closeable: false,
        decorations: false,
        transparent: true,
        level: Level::AlwaysOnTop,
        exit_on_close_request: false,
        #[cfg(windows)]
        platform_specific: window::settings::PlatformSpecific {
            skip_taskbar: true,
            corner_preference: window::settings::platform::CornerPreference::DoNotRound,
            ..Default::default()
        },
        ..window::Settings::default()
    }
}
