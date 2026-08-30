// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 主题类型定义
//!
//! 定义应用内主题枚举与配置。颜色集合见 [`super::theme_colors`]。

use super::theme_colors::ThemeColors;

/// 主题类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// 浅色主题
    #[default]
    Light,
    /// 深色主题
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

    /// 获取当前主题的颜色集合
    pub fn get_theme_colors(&self) -> ThemeColors {
        ThemeColors::from_theme(self.theme)
    }
}
