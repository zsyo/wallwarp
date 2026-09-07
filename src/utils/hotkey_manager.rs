// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 全局热键管理：注册/注销系统级快捷键，并将触发事件映射回动作。
//!
//! 封装 `global_hotkey::GlobalHotKeyManager`；内部维护"热键 id -> 动作"映射，
//! 供事件转发线程按 id 反查动作。Drop 时自动注销全部已注册热键。

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyManager, HotKeyState};
use std::collections::HashMap;
use tracing::{info, warn};

/// 全局热键触发的动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    /// 切换下一张壁纸
    SwitchNext,
    /// 切换上一张壁纸
    SwitchPrevious,
    /// 显示主窗口（与托盘菜单"显示窗口"同功能）
    ShowWindow,
    /// 保存当前壁纸到库（与托盘菜单"保存当前壁纸到库"同功能）
    SaveCurrent,
}

/// 全局热键触发事件（转发线程 -> 主线程）
#[derive(Debug, Clone, Copy)]
pub struct HotkeyEvent {
    /// 触发的热键 id
    pub id: u32,
    /// 按键状态（按下/释放）
    pub state: HotKeyState,
}

impl HotkeyAction {
    /// 全部动作（按 UI 展示顺序）
    pub const ALL: [HotkeyAction; 4] = [
        HotkeyAction::SwitchNext,
        HotkeyAction::SwitchPrevious,
        HotkeyAction::ShowWindow,
        HotkeyAction::SaveCurrent,
    ];

    /// 配置文件中的字段名
    pub fn config_key(self) -> &'static str {
        match self {
            HotkeyAction::SwitchNext => "hotkey_switch_next",
            HotkeyAction::SwitchPrevious => "hotkey_switch_previous",
            HotkeyAction::ShowWindow => "hotkey_show_window",
            HotkeyAction::SaveCurrent => "hotkey_save_current",
        }
    }

    /// 界面展示的动作名（i18n key）
    pub fn i18n_key(self) -> &'static str {
        match self {
            HotkeyAction::SwitchNext => "settings.hotkey-switch-next",
            HotkeyAction::SwitchPrevious => "settings.hotkey-switch-previous",
            HotkeyAction::ShowWindow => "settings.hotkey-show-window",
            HotkeyAction::SaveCurrent => "settings.hotkey-save-current",
        }
    }
}

/// 全局热键管理器
///
/// 持有原生管理器与已注册热键映射；内部对象不可跨线程使用，
/// 注册/注销均需在主线程调用（iced update 闭包线程满足要求）。
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    /// 已注册热键：原生事件携带的 id -> 动作
    registered: HashMap<u32, (HotKey, HotkeyAction)>,
}

impl HotkeyManager {
    /// 创建原生管理器
    ///
    /// 需在有平台事件循环的线程调用（macOS 为主线程，Windows 为 winit 线程）
    pub fn new() -> global_hotkey::Result<Self> {
        Ok(Self {
            manager: GlobalHotKeyManager::new()?,
            registered: HashMap::new(),
        })
    }

    /// 注册动作的全局热键；已注册的同动作热键先注销
    pub fn register(&mut self, action: HotkeyAction, hotkey: HotKey) -> global_hotkey::Result<()> {
        // 同一动作换键时先注销旧键，避免残留
        self.unregister_action(action);

        self.manager.register(hotkey)?;
        info!(
            "[全局热键] [{:?}] 注册成功: {} (id={})",
            action,
            hotkey.into_string(),
            hotkey.id()
        );
        self.registered.insert(hotkey.id(), (hotkey, action));
        Ok(())
    }

    /// 注销动作当前已注册的热键（未注册时为空操作）
    pub fn unregister_action(&mut self, action: HotkeyAction) {
        let ids: Vec<u32> = self
            .registered
            .iter()
            .filter(|(_, (_, a))| *a == action)
            .map(|(id, _)| *id)
            .collect();
        for id in ids {
            if let Some((hotkey, _)) = self.registered.remove(&id)
                && let Err(e) = self.manager.unregister(hotkey)
            {
                warn!("[全局热键] [{:?}] 注销失败: {}", action, e);
            }
        }
    }

    /// 按原生事件 id 反查动作（仅按键按下事件有效，释放事件忽略）
    pub fn action_of_pressed(&self, id: u32, state: HotKeyState) -> Option<HotkeyAction> {
        if state != HotKeyState::Pressed {
            return None;
        }
        self.registered.get(&id).map(|(_, action)| *action)
    }
}

/// 解析配置中的热键字符串（global-hotkey HotKey 格式，如 "ctrl+alt+ArrowRight"）
///
/// 解析失败按未设置处理并输出告警（配置可能被手工改坏）
pub fn parse_hotkey_string(s: &str, action: HotkeyAction) -> Option<HotKey> {
    if s.trim().is_empty() {
        return None;
    }
    match s.parse::<HotKey>() {
        Ok(hotkey) => Some(hotkey),
        Err(e) => {
            warn!("[全局热键] [{:?}] 配置解析失败，按未设置处理: {} ({})", action, s, e);
            None
        }
    }
}

/// 将 iced 按键事件转换为 HotKey 字符串（热键录制用）
///
/// - `modifiers`: iced 修饰键状态
/// - `key`: iced 按键（Named 或 Character）
///
/// 返回 None 表示该按键不能用作热键主键（纯修饰键或未映射键）。
/// 字符串格式与 `parse_hotkey_string` 对应（修饰键前缀小写 + Code 名）。
pub fn iced_key_to_hotkey_string(
    modifiers: iced::keyboard::Modifiers,
    key: &iced::keyboard::Key,
) -> Option<String> {
    let code = iced_key_to_code(key)?;
    let mut result = String::new();
    if modifiers.shift() {
        result.push_str("shift+");
    }
    if modifiers.control() {
        result.push_str("control+");
    }
    if modifiers.alt() {
        result.push_str("alt+");
    }
    if modifiers.logo() {
        result.push_str("super+");
    }
    result.push_str(&code.to_string());
    Some(result)
}

/// 判断修饰键组合是否允许作为热键（必须含至少一个修饰键；F1-F12 可单独使用）
pub fn hotkey_modifiers_allowed(modifiers: iced::keyboard::Modifiers, key: &iced::keyboard::Key) -> bool {
    if !modifiers.is_empty() {
        return true;
    }
    // F1-F12 无修饰键也允许注册
    matches!(
        key,
        iced::keyboard::Key::Named(
            iced::keyboard::key::Named::F1
                | iced::keyboard::key::Named::F2
                | iced::keyboard::key::Named::F3
                | iced::keyboard::key::Named::F4
                | iced::keyboard::key::Named::F5
                | iced::keyboard::key::Named::F6
                | iced::keyboard::key::Named::F7
                | iced::keyboard::key::Named::F8
                | iced::keyboard::key::Named::F9
                | iced::keyboard::key::Named::F10
                | iced::keyboard::key::Named::F11
                | iced::keyboard::key::Named::F12
        )
    )
}

/// 纯修饰键判断（录制时等待主键，不应立即捕获）
pub fn is_modifier_key(key: &iced::keyboard::Key) -> bool {
    matches!(
        key,
        iced::keyboard::Key::Named(
            iced::keyboard::key::Named::Shift
                | iced::keyboard::key::Named::Control
                | iced::keyboard::key::Named::Alt
                | iced::keyboard::key::Named::Super
                | iced::keyboard::key::Named::Meta
        )
    )
}

/// iced 按键映射到 keyboard_types Code（global-hotkey 使用后者）
fn iced_key_to_code(key: &iced::keyboard::Key) -> Option<Code> {
    use iced::keyboard::key::Named as N;
    match key {
        iced::keyboard::Key::Named(name) => Some(match name {
            N::Enter => Code::Enter,
            N::Tab => Code::Tab,
            N::Space => Code::Space,
            N::ArrowDown => Code::ArrowDown,
            N::ArrowLeft => Code::ArrowLeft,
            N::ArrowRight => Code::ArrowRight,
            N::ArrowUp => Code::ArrowUp,
            N::Escape => Code::Escape,
            N::End => Code::End,
            N::Home => Code::Home,
            N::PageDown => Code::PageDown,
            N::PageUp => Code::PageUp,
            N::Backspace => Code::Backspace,
            N::Delete => Code::Delete,
            N::Insert => Code::Insert,
            N::F1 => Code::F1,
            N::F2 => Code::F2,
            N::F3 => Code::F3,
            N::F4 => Code::F4,
            N::F5 => Code::F5,
            N::F6 => Code::F6,
            N::F7 => Code::F7,
            N::F8 => Code::F8,
            N::F9 => Code::F9,
            N::F10 => Code::F10,
            N::F11 => Code::F11,
            N::F12 => Code::F12,
            N::F13 => Code::F13,
            N::F14 => Code::F14,
            N::F15 => Code::F15,
            N::F16 => Code::F16,
            N::F17 => Code::F17,
            N::F18 => Code::F18,
            N::F19 => Code::F19,
            N::F20 => Code::F20,
            N::CapsLock => Code::CapsLock,
            N::ScrollLock => Code::ScrollLock,
            N::Pause => Code::Pause,
            N::PrintScreen => Code::PrintScreen,
            N::AudioVolumeDown => Code::AudioVolumeDown,
            N::AudioVolumeMute => Code::AudioVolumeMute,
            N::AudioVolumeUp => Code::AudioVolumeUp,
            N::MediaTrackNext => Code::MediaTrackNext,
            N::MediaTrackPrevious => Code::MediaTrackPrevious,
            N::MediaStop => Code::MediaStop,
            N::MediaPlayPause => Code::MediaPlayPause,
            _ => return None,
        }),
        // 字符按键（字母/数字等）按 latin 字符映射：'a' -> KeyA、'1' -> Digit1
        iced::keyboard::Key::Character(c) => {
            let ch = c.chars().next()?;
            let name = match ch.to_ascii_lowercase() {
                'a'..='z' => format!("Key{}", ch.to_ascii_uppercase()),
                '0'..='9' => format!("Digit{}", ch),
                _ => return None,
            };
            name.parse::<Code>().ok()
        }
        iced::keyboard::Key::Unidentified => None,
    }
}

/// 将 HotKey 字符串格式化为界面展示文本（如 "Ctrl + Alt + →"）
pub fn format_hotkey_display(hotkey_str: &str) -> String {
    let Some(hotkey) = parse_hotkey_string(hotkey_str, HotkeyAction::SwitchNext) else {
        return hotkey_str.to_string();
    };

    let mut parts: Vec<String> = Vec::new();
    // 展示顺序与平台习惯一致：Ctrl -> Alt -> Shift -> 平台主修饰键
    if hotkey.mods.contains(Modifiers::CONTROL) {
        parts.push("Ctrl".to_string());
    }
    if hotkey.mods.contains(Modifiers::ALT) {
        parts.push("Alt".to_string());
    }
    if hotkey.mods.contains(Modifiers::SHIFT) {
        parts.push("Shift".to_string());
    }
    if hotkey.mods.contains(Modifiers::SUPER) {
        // 平台习惯名称：Windows 为 Win、macOS 为 Cmd、Linux 为 Super
        parts.push(platform_super_name().to_string());
    }
    parts.push(format_code_display(&hotkey.key));
    parts.join(" + ")
}

/// 平台上 Super 修饰键的习惯名称（Windows 为 Win、macOS 为 Cmd、Linux 为 Super）
fn platform_super_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "Win"
    } else if cfg!(target_os = "macos") {
        "Cmd"
    } else {
        "Super"
    }
}

/// Code 的界面展示名（方向键用箭头符号，字母大写，其余保留 Code 名主体）
fn format_code_display(code: &Code) -> String {
    match code {
        Code::ArrowLeft => "←".to_string(),
        Code::ArrowRight => "→".to_string(),
        Code::ArrowUp => "↑".to_string(),
        Code::ArrowDown => "↓".to_string(),
        Code::Escape => "Esc".to_string(),
        Code::Enter => "Enter".to_string(),
        Code::Space => "Space".to_string(),
        Code::Backquote => "`".to_string(),
        Code::Minus => "-".to_string(),
        Code::Equal => "=".to_string(),
        Code::BracketLeft => "[".to_string(),
        Code::BracketRight => "]".to_string(),
        Code::Semicolon => ";".to_string(),
        Code::Quote => "'".to_string(),
        Code::Backslash => "\\".to_string(),
        Code::Comma => ",".to_string(),
        Code::Period => ".".to_string(),
        Code::Slash => "/".to_string(),
        _ => {
            let name = code.to_string();
            // KeyA -> A、Digit1 -> 1、F5 -> F5、PageDown -> PageDown
            if let Some(rest) = name.strip_prefix("Key") {
                rest.to_string()
            } else if let Some(rest) = name.strip_prefix("Digit") {
                rest.to_string()
            } else {
                name
            }
        }
    }
}
