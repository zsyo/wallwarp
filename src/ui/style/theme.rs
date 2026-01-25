// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 主题相关定义
//!
//! 此模块定义了主题相关的类型和配置，为未来支持深色/浅色主题切换做准备。

use iced::Color;

/// 主题类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// 浅色主题
    #[default]
    Light,
    /// 深色主题（预留）
    Dark,
}

impl Theme {
    /// 获取主题名称
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }

    /// 从名称解析主题
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }
}

/// 主题配置
#[derive(Debug, Clone, Copy)]
pub struct ThemeConfig {
    /// 当前主题
    pub theme: Theme,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig { theme: Theme::Light }
    }
}

impl ThemeConfig {
    /// 创建新的主题配置
    pub fn new(theme: Theme) -> Self {
        ThemeConfig { theme }
    }

    /// 切换主题
    pub fn toggle(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }

    /// 设置主题
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// 获取当前主题
    pub fn get_theme(&self) -> Theme {
        self.theme
    }

    /// 判断是否为深色主题
    pub fn is_dark(&self) -> bool {
        self.theme == Theme::Dark
    }

    /// 判断是否为浅色主题
    pub fn is_light(&self) -> bool {
        self.theme == Theme::Light
    }
}

/// 主题颜色集合
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    // 基础颜色
    /// 背景色
    pub background: Color,
    /// 文字颜色
    pub text: Color,
    /// 主要颜色
    pub primary: Color,
    /// 次要颜色
    pub secondary: Color,
    /// 边框颜色
    pub border: Color,

    // 侧边栏颜色
    /// 侧边栏背景色
    pub sidebar_bg: Color,
    /// 侧边栏按钮默认背景色
    pub sidebar_button_default: Color,
    /// 侧边栏按钮悬停背景色
    pub sidebar_button_hover: Color,
    /// 侧边栏按钮选中背景色
    pub sidebar_button_selected: Color,
    /// 侧边栏选中指示条颜色
    pub sidebar_indicator: Color,

    // 分隔线颜色
    /// 分隔线颜色
    pub separator: Color,
    /// 分隔线阴影颜色
    pub separator_shadow: Color,

    // 遮罩层和模态窗口颜色
    /// 模态窗口背景色
    pub modal_bg: Color,
    /// 遮罩层背景色
    pub overlay_bg: Color,
    /// 遮罩层文字颜色
    pub overlay_text: Color,

    // 浅色背景和文字颜色
    /// 浅色背景（筛选栏等）
    pub light_bg: Color,
    /// 浅色按钮背景
    pub light_button: Color,
    /// 浅色文字颜色
    pub light_text: Color,
    /// 浅色次要文字颜色
    pub light_text_sub: Color,
    /// 设置页下拉框背景色（比 light_button 更深，以便在配置组背景上可见）
    pub settings_dropdown_bg: Color,

    // 选中状态颜色
    /// 选中状态蓝色
    pub selected_blue: Color,

    // 通知颜色
    /// 成功通知背景色
    pub notification_success_bg: Color,
    /// 错误通知背景色
    pub notification_error_bg: Color,
    /// 信息通知背景色
    pub notification_info_bg: Color,
    /// 通知文字颜色
    pub notification_text_color: Color,

    // 禁用状态颜色
    /// 禁用状态颜色
    pub disabled_color: Color,
    /// 禁用按钮背景色
    pub disabled_button_bg: Color,

    // 文本输入框颜色
    /// 文本输入框选择颜色
    pub text_input_selection_color: Color,
    /// 文本输入框背景色
    pub text_input_background: Color,

    // Tooltip颜色
    /// Tooltip背景颜色
    pub tooltip_bg_color: Color,
    /// Tooltip边框颜色
    pub tooltip_border_color: Color,

    // 其他颜色
    /// 分页分隔线文字颜色
    pub page_separator_text_color: Color,
    /// 表格分隔线颜色
    pub table_separator_color: Color,
}

impl ThemeColors {
    /// 创建浅色主题颜色
    pub fn light() -> Self {
        ThemeColors {
            // 基础颜色
            background: Color::from_rgb(0.96, 0.96, 0.96),
            text: Color::from_rgb(0.13, 0.13, 0.13),
            primary: Color::from_rgb(0.13, 0.59, 0.95),
            secondary: Color::from_rgb(0.5, 0.5, 0.5),
            border: Color::from_rgb(0.85, 0.85, 0.85),

            // 侧边栏颜色
            sidebar_bg: Color::from_rgb(0.94, 0.94, 0.94),
            sidebar_button_default: Color::TRANSPARENT,
            sidebar_button_hover: Color::from_rgb(0.93, 0.93, 0.95),
            sidebar_button_selected: Color::from_rgb(0.85, 0.85, 0.85),
            sidebar_indicator: Color::from_rgb(0.13, 0.59, 0.95),

            // 分隔线颜色
            separator: Color::from_rgb(0.85, 0.85, 0.85),
            separator_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.15),

            // 遮罩层和模态窗口颜色
            modal_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.85),
            overlay_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
            overlay_text: Color::from_rgb(1.0, 1.0, 1.0),

            // 浅色背景和文字颜色
            light_bg: Color::from_rgb(0.92, 0.92, 0.92),
            light_button: Color::from_rgb(1.0, 1.0, 1.0),
            light_text: Color::from_rgb(0.2, 0.2, 0.2),
            light_text_sub: Color::from_rgb(0.533, 0.533, 0.533),
            settings_dropdown_bg: Color::WHITE,

            // 选中状态颜色
            selected_blue: Color::from_rgb(0.098, 0.463, 0.824),

            // 通知颜色
            notification_success_bg: Color::from_rgb8(40, 167, 69),
            notification_error_bg: Color::from_rgb8(220, 53, 69),
            notification_info_bg: Color::from_rgb8(0, 123, 255),
            notification_text_color: Color::WHITE,

            // 禁用状态颜色
            disabled_color: Color::from_rgba(0.5, 0.5, 0.5, 1.0),
            disabled_button_bg: Color::from_rgba(0.7, 0.7, 0.7, 0.5),

            // 文本输入框颜色
            text_input_selection_color: Color::from_rgba(0.098, 0.463, 0.824, 0.3),
            text_input_background: Color::WHITE,

            // Tooltip颜色
            tooltip_bg_color: Color::WHITE,
            tooltip_border_color: Color::BLACK,

            // 其他颜色
            page_separator_text_color: Color::from_rgb(0.5, 0.5, 0.5),
            table_separator_color: Color::from_rgb(0.9, 0.9, 0.9),
        }
    }

    /// 创建深色主题颜色
    pub fn dark() -> Self {
        ThemeColors {
            // 基础颜色
            background: Color::from_rgb(0.13, 0.13, 0.13),
            text: Color::from_rgb(0.96, 0.96, 0.96),
            primary: Color::from_rgb(0.25, 0.7, 1.0),
            secondary: Color::from_rgb(0.7, 0.7, 0.7),
            border: Color::from_rgb(0.3, 0.3, 0.3),

            // 侧边栏颜色
            sidebar_bg: Color::from_rgb(0.18, 0.18, 0.18),
            sidebar_button_default: Color::TRANSPARENT,
            sidebar_button_hover: Color::from_rgb(0.25, 0.25, 0.25),
            sidebar_button_selected: Color::from_rgb(0.3, 0.3, 0.3),
            sidebar_indicator: Color::from_rgb(0.25, 0.7, 1.0),

            // 分隔线颜色
            separator: Color::from_rgb(0.3, 0.3, 0.3),
            separator_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3),

            // 遮罩层和模态窗口颜色
            modal_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            overlay_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            overlay_text: Color::from_rgb(1.0, 1.0, 1.0),

            // 浅色背景和文字颜色
            light_bg: Color::from_rgb(0.2, 0.2, 0.2),
            light_button: Color::from_rgb(0.25, 0.25, 0.25),
            light_text: Color::from_rgb(0.9, 0.9, 0.9),
            light_text_sub: Color::from_rgb(0.6, 0.6, 0.6),
            settings_dropdown_bg: Color::from_rgb(0.25, 0.25, 0.25),

            // 选中状态颜色
            selected_blue: Color::from_rgb(0.25, 0.7, 1.0),

            // 通知颜色
            notification_success_bg: Color::from_rgb8(46, 125, 50),
            notification_error_bg: Color::from_rgb8(198, 40, 40),
            notification_info_bg: Color::from_rgb8(2, 119, 189),
            notification_text_color: Color::WHITE,

            // 禁用状态颜色
            disabled_color: Color::from_rgba(0.6, 0.6, 0.6, 1.0),
            disabled_button_bg: Color::from_rgba(0.5, 0.5, 0.5, 0.5),

            // 文本输入框颜色
            text_input_selection_color: Color::from_rgba(0.25, 0.7, 1.0, 0.4),
            text_input_background: Color::from_rgb(0.25, 0.25, 0.25),

            // Tooltip颜色
            tooltip_bg_color: Color::from_rgb(0.25, 0.25, 0.25),
            tooltip_border_color: Color::from_rgb(0.5, 0.5, 0.5),

            // 其他颜色
            page_separator_text_color: Color::from_rgb(0.7, 0.7, 0.7),
            table_separator_color: Color::from_rgb(0.25, 0.25, 0.25),
        }
    }

    /// 根据主题获取颜色
    pub fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => ThemeColors::light(),
            Theme::Dark => ThemeColors::dark(),
        }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors::light()
    }
}
