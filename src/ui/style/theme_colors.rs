// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 主题颜色集合
//!
//! 定义浅色/深色两套语义色板。全应用唯一强调色为现代蓝：
//! 浅色 #3B82F6，深色 #5C9DFF。所有需要强调色的地方必须使用
//! `primary`/`primary_hover`/`primary_active`，禁止再散落新的蓝色值。

use iced::Color;

use super::theme::Theme;

/// 主题颜色集合
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    // 基础颜色
    /// 背景色
    pub background: Color,
    /// 文字颜色
    pub text: Color,
    /// 强调色（全应用统一，按钮/选中/指示条/焦点态）
    pub primary: Color,
    /// 强调色悬停态
    pub primary_hover: Color,
    /// 强调色按下态
    pub primary_active: Color,
    /// 次要颜色
    pub secondary: Color,
    /// 边框颜色
    pub border: Color,
    /// 通用悬停填充（列表行/菜单项/图标按钮等中性悬停反馈）
    pub hover_fill: Color,

    // 侧边栏颜色
    /// 侧边栏背景色
    pub sidebar_bg: Color,
    /// 侧边栏按钮默认背景色
    pub sidebar_button_default: Color,
    /// 侧边栏按钮悬停背景色
    pub sidebar_button_hover: Color,
    /// 侧边栏按钮选中背景色（强调色淡染）
    pub sidebar_button_selected: Color,
    /// 侧边栏选中指示条颜色
    pub sidebar_indicator: Color,

    // 标题栏颜色
    /// 标题栏背景色
    pub title_bar_bg: Color,

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
    /// 对话框背景色
    pub dialog_bg: Color,

    // 浅色背景和文字颜色
    /// 浅色背景（筛选栏等）
    pub light_bg: Color,
    /// 浅色按钮背景
    pub light_button: Color,
    /// 浅色文字颜色
    pub light_text: Color,
    /// 浅色次要文字颜色
    pub light_text_sub: Color,
    /// 设置页下拉框背景色
    pub settings_dropdown_bg: Color,

    // 选中状态颜色（统一并入 primary）
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
            background: Color::from_rgb8(246, 247, 249), // #F6F7F9
            text: Color::from_rgb8(31, 35, 40),          // #1F2328
            primary: Color::from_rgb8(59, 130, 246),     // #3B82F6
            primary_hover: Color::from_rgb8(37, 99, 235), // #2563EB
            primary_active: Color::from_rgb8(29, 78, 216), // #1D4ED8
            secondary: Color::from_rgb8(107, 114, 128),  // #6B7280
            border: Color::from_rgb8(226, 228, 233),     // #E2E4E9
            hover_fill: Color::from_rgba8(15, 23, 42, 0.05), // #0F172A @ 5%

            // 侧边栏颜色
            sidebar_bg: Color::from_rgb8(238, 240, 243), // #EEF0F3
            sidebar_button_default: Color::TRANSPARENT,
            sidebar_button_hover: Color::from_rgba8(15, 23, 42, 0.04), // #0F172A @ 4%
            sidebar_button_selected: Color::from_rgba8(59, 130, 246, 0.12), // #3B82F6 @ 12%
            sidebar_indicator: Color::from_rgb8(59, 130, 246),         // #3B82F6

            // 标题栏颜色
            title_bar_bg: Color::from_rgb8(241, 242, 245), // #F1F2F5

            // 分隔线颜色
            separator: Color::from_rgb8(229, 231, 235), // #E5E7EB
            separator_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.12),

            // 遮罩层和模态窗口颜色
            modal_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.85),
            overlay_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
            overlay_text: Color::WHITE,
            dialog_bg: Color::WHITE,

            // 浅色背景和文字颜色
            light_bg: Color::from_rgb8(239, 241, 244), // #EFF1F4
            light_button: Color::WHITE,
            light_text: Color::from_rgb8(31, 35, 40), // #1F2328
            light_text_sub: Color::from_rgb8(107, 114, 128), // #6B7280
            settings_dropdown_bg: Color::WHITE,

            // 选中状态颜色（统一强调色）
            selected_blue: Color::from_rgb8(59, 130, 246), // #3B82F6

            // 通知颜色
            notification_success_bg: Color::from_rgb8(22, 163, 74), // #16A34A
            notification_error_bg: Color::from_rgb8(220, 38, 38),   // #DC2626
            notification_info_bg: Color::from_rgb8(59, 130, 246),   // #3B82F6
            notification_text_color: Color::WHITE,

            // 禁用状态颜色
            disabled_color: Color::from_rgb8(154, 160, 168), // #9AA0A8
            disabled_button_bg: Color::from_rgba8(107, 114, 128, 0.25), // #6B7280 @ 25%

            // 文本输入框颜色
            text_input_selection_color: Color::from_rgba8(59, 130, 246, 0.25), // #3B82F6 @ 25%
            text_input_background: Color::WHITE,

            // Tooltip颜色
            tooltip_bg_color: Color::WHITE,
            tooltip_border_color: Color::from_rgb8(208, 211, 216), // #D0D3D8

            // 其他颜色
            page_separator_text_color: Color::from_rgb8(107, 114, 128), // #6B7280
            table_separator_color: Color::from_rgb8(235, 237, 240),     // #EBEDF0
        }
    }

    /// 创建深色主题颜色
    pub fn dark() -> Self {
        ThemeColors {
            // 基础颜色
            background: Color::from_rgb8(23, 25, 30), // #17191E
            text: Color::from_rgb8(232, 234, 237),    // #E8EAED
            primary: Color::from_rgb8(92, 157, 255),  // #5C9DFF
            primary_hover: Color::from_rgb8(122, 178, 255), // #7AB2FF
            primary_active: Color::from_rgb8(62, 134, 245), // #3E86F5
            secondary: Color::from_rgb8(154, 160, 168), // #9AA0A8
            border: Color::from_rgb8(46, 50, 58),     // #2E323A
            hover_fill: Color::from_rgba8(255, 255, 255, 0.07), // #FFFFFF @ 7%

            // 侧边栏颜色
            sidebar_bg: Color::from_rgb8(30, 33, 39), // #1E2127
            sidebar_button_default: Color::TRANSPARENT,
            sidebar_button_hover: Color::from_rgba8(255, 255, 255, 0.06), // #FFFFFF @ 6%
            sidebar_button_selected: Color::from_rgba8(92, 157, 255, 0.18), // #5C9DFF @ 18%
            sidebar_indicator: Color::from_rgb8(92, 157, 255),            // #5C9DFF

            // 标题栏颜色
            title_bar_bg: Color::from_rgb8(26, 28, 33), // #1A1C21

            // 分隔线颜色
            separator: Color::from_rgb8(42, 45, 52), // #2A2D34
            separator_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3),

            // 遮罩层和模态窗口颜色
            modal_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.9),
            overlay_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            overlay_text: Color::WHITE,
            dialog_bg: Color::from_rgb8(35, 38, 45), // #23262D

            // 浅色背景和文字颜色
            light_bg: Color::from_rgb8(32, 35, 42), // #20232A
            light_button: Color::from_rgb8(42, 46, 54), // #2A2E36
            light_text: Color::from_rgb8(232, 234, 237), // #E8EAED
            light_text_sub: Color::from_rgb8(154, 160, 168), // #9AA0A8
            settings_dropdown_bg: Color::from_rgb8(35, 38, 45), // #23262D

            // 选中状态颜色（统一强调色）
            selected_blue: Color::from_rgb8(92, 157, 255), // #5C9DFF

            // 通知颜色
            notification_success_bg: Color::from_rgb8(34, 165, 91), // #22A55B
            notification_error_bg: Color::from_rgb8(229, 72, 77),   // #E5484D
            notification_info_bg: Color::from_rgb8(92, 157, 255),   // #5C9DFF
            notification_text_color: Color::WHITE,

            // 禁用状态颜色
            disabled_color: Color::from_rgb8(107, 114, 128), // #6B7280
            disabled_button_bg: Color::from_rgba8(154, 160, 168, 0.18), // #9AA0A8 @ 18%

            // 文本输入框颜色
            text_input_selection_color: Color::from_rgba8(92, 157, 255, 0.4), // #5C9DFF @ 40%
            text_input_background: Color::from_rgb8(35, 38, 45),              // #23262D

            // Tooltip颜色
            tooltip_bg_color: Color::from_rgb8(38, 41, 47), // #26292F
            tooltip_border_color: Color::from_rgb8(74, 79, 88), // #4A4F58

            // 其他颜色
            page_separator_text_color: Color::from_rgb8(154, 160, 168), // #9AA0A8
            table_separator_color: Color::from_rgb8(38, 41, 48),        // #262930
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
