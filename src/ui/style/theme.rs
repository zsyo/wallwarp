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
        ThemeConfig {
            theme: Theme::Light,
        }
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

/// 主题颜色集合（预留）
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
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
}

impl ThemeColors {
    /// 创建浅色主题颜色
    pub fn light() -> Self {
        ThemeColors {
            background: Color::from_rgb(0.96, 0.96, 0.96),
            text: Color::from_rgb(0.13, 0.13, 0.13),
            primary: Color::from_rgb(0.13, 0.59, 0.95),
            secondary: Color::from_rgb(0.5, 0.5, 0.5),
            border: Color::from_rgb(0.85, 0.85, 0.85),
        }
    }

    /// 创建深色主题颜色（预留）
    pub fn dark() -> Self {
        ThemeColors {
            background: Color::from_rgb(0.13, 0.13, 0.13),
            text: Color::from_rgb(0.96, 0.96, 0.96),
            primary: Color::from_rgb(0.25, 0.7, 1.0),
            secondary: Color::from_rgb(0.7, 0.7, 0.7),
            border: Color::from_rgb(0.3, 0.3, 0.3),
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