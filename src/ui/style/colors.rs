// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 颜色常量定义
//!
//! 所有UI相关的颜色常量应在此文件中定义。

use iced::Color;

// ============================================================================
// 通用颜色
// ============================================================================

/// 浅色背景色（主背景，类似 macOS 风格的浅灰色）
pub const COLOR_BG_LIGHT: Color = Color::from_rgb(0.96, 0.96, 0.96); // #F5F5F7

/// 深色文字颜色
pub const COLOR_TEXT_DARK: Color = Color::from_rgb(0.13, 0.13, 0.13); // #212121

/// 侧边栏背景色（浅灰色）
pub const COLOR_SIDEBAR_BG: Color = Color::from_rgb(0.94, 0.94, 0.94); // #EEEEF0

/// 侧边栏按钮默认背景色（透明）
pub const COLOR_SIDEBAR_BUTTON_DEFAULT: Color = Color::TRANSPARENT;

/// 侧边栏按钮悬停背景色
pub const COLOR_SIDEBAR_BUTTON_HOVER: Color = Color::from_rgb(0.93, 0.93, 0.95); // #EEEEF0

/// 侧边栏按钮选中背景色）
pub const COLOR_SIDEBAR_BUTTON_SELECTED: Color = Color::from_rgb(0.85, 0.85, 0.85); // #D9D9D9

/// 侧边栏选中指示条颜色（蓝色）
pub const COLOR_SIDEBAR_INDICATOR: Color = Color::from_rgb(0.13, 0.59, 0.95); // #2196F3

/// 分隔线颜色（深灰色）
pub const COLOR_SEPARATOR: Color = Color::from_rgb(0.85, 0.85, 0.85); // #D9D9D9

/// 分隔线阴影颜色
pub const COLOR_SEPARATOR_SHADOW: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.15);

/// 模态窗口背景色（半透明黑色）
pub const COLOR_MODAL_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.85);

/// 遮罩层背景色（半透明黑色）
pub const COLOR_OVERLAY_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);

/// 遮罩层文字颜色
pub const COLOR_OVERLAY_TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);

/// 浅色背景（筛选栏等）
pub const COLOR_LIGHT_BG: Color = Color::from_rgb(0.969, 0.969, 0.969); // #F8F8F8

/// 浅色按钮背景
pub const COLOR_LIGHT_BUTTON: Color = Color::from_rgb(0.933, 0.933, 0.933); // #EEEEEE

/// 浅色文字颜色
pub const COLOR_LIGHT_TEXT: Color = Color::from_rgb(0.2, 0.2, 0.2); // #333333

/// 浅色次要文字颜色
pub const COLOR_LIGHT_TEXT_SUB: Color = Color::from_rgb(0.533, 0.533, 0.533); // #888888

/// 选中状态蓝色
pub const COLOR_SELECTED_BLUE: Color = Color::from_rgb(0.098, 0.463, 0.824); // #1976D2

// ============================================================================
// 纯净度颜色
// ============================================================================

/// 纯净度-安全（绿色）
pub const COLOR_SFW: Color = Color::from_rgb(0.298, 0.686, 0.314); // #4CAF50

/// 纯净度-轻微（黄色）
pub const COLOR_SKETCHY: Color = Color::from_rgb(1.0, 0.757, 0.027); // #FFC107

/// 纯净度-成人（红色）
pub const COLOR_NSFW: Color = Color::from_rgb(0.965, 0.263, 0.212); // #F44336

// ============================================================================
// 颜色网格选择器颜色常量（官方接口支持的29种颜色）
// ============================================================================

/// #660000 深红
pub const COLOR_660000: Color = Color::from_rgb(0.4, 0.0, 0.0);

/// #990000 正红
pub const COLOR_990000: Color = Color::from_rgb(0.6, 0.0, 0.0);

/// #CC0000 亮红
pub const COLOR_CC0000: Color = Color::from_rgb(0.8, 0.0, 0.0);

/// #CC3333 浅红
pub const COLOR_CC3333: Color = Color::from_rgb(0.8, 0.2, 0.2);

/// #EA4C88 粉红
pub const COLOR_EA4C88: Color = Color::from_rgb(0.918, 0.298, 0.533);

/// #993399 紫红
pub const COLOR_993399: Color = Color::from_rgb(0.6, 0.2, 0.6);

/// #663399 深紫
pub const COLOR_663399: Color = Color::from_rgb(0.4, 0.2, 0.6);

/// #333399 蓝紫
pub const COLOR_333399: Color = Color::from_rgb(0.2, 0.2, 0.6);

/// #0066CC 宝蓝
pub const COLOR_0066CC: Color = Color::from_rgb(0.0, 0.4, 0.8);

/// #0099CC 天蓝
pub const COLOR_0099CC: Color = Color::from_rgb(0.0, 0.6, 0.8);

/// #66CCCC 青绿
pub const COLOR_66CCCC: Color = Color::from_rgb(0.4, 0.8, 0.8);

/// #77CC33 草绿
pub const COLOR_77CC33: Color = Color::from_rgb(0.467, 0.8, 0.2);

/// #669900 翠绿
pub const COLOR_669900: Color = Color::from_rgb(0.4, 0.6, 0.0);

/// #336600 深绿
pub const COLOR_336600: Color = Color::from_rgb(0.2, 0.4, 0.0);

/// #666600 橄榄绿
pub const COLOR_666600: Color = Color::from_rgb(0.4, 0.4, 0.0);

/// #999900 黄绿
pub const COLOR_999900: Color = Color::from_rgb(0.6, 0.6, 0.0);

/// #CCCC33 柠檬黄
pub const COLOR_CCCC33: Color = Color::from_rgb(0.8, 0.8, 0.2);

/// #FFFF00 亮黄
pub const COLOR_FFFF00: Color = Color::from_rgb(1.0, 1.0, 0.0);

/// #FFCC33 金黄
pub const COLOR_FFCC33: Color = Color::from_rgb(1.0, 0.8, 0.2);

/// #FF9900 橙黄
pub const COLOR_FF9900: Color = Color::from_rgb(1.0, 0.6, 0.0);

/// #FF6600 橘红
pub const COLOR_FF6600: Color = Color::from_rgb(1.0, 0.4, 0.0);

/// #CC6633 砖红
pub const COLOR_CC6633: Color = Color::from_rgb(0.8, 0.4, 0.2);

/// #996633 棕褐
pub const COLOR_996633: Color = Color::from_rgb(0.6, 0.4, 0.2);

/// #663300 深棕
pub const COLOR_663300: Color = Color::from_rgb(0.4, 0.2, 0.0);

/// #000000 纯黑
pub const COLOR_000000: Color = Color::from_rgb(0.0, 0.0, 0.0);

/// #999999 深灰
pub const COLOR_999999: Color = Color::from_rgb(0.6, 0.6, 0.6);

/// #CCCCCC 中灰
pub const COLOR_CCCCCC: Color = Color::from_rgb(0.8, 0.8, 0.8);

/// #FFFFFF 纯白
pub const COLOR_FFFFFF: Color = Color::from_rgb(1.0, 1.0, 1.0);

/// #424153 深灰蓝
pub const COLOR_424153: Color = Color::from_rgb(0.259, 0.255, 0.325);

/// 颜色选择器背景色（浅色主题）
pub const COLOR_PICKER_BG: Color = Color::from_rgb(0.969, 0.969, 0.969); // #F8F8F8

/// 颜色选择器激活状态颜色
pub const COLOR_PICKER_ACTIVE: Color = Color::from_rgb(0.0, 0.478, 1.0); // #007AFF

/// 颜色选择器无色斜线颜色
pub const COLOR_NO_COLOR_STROKE: Color = Color::from_rgb(1.0, 0.0, 0.0);

// ============================================================================
// 按钮颜色
// ============================================================================

/// 蓝色按钮颜色
pub const BUTTON_COLOR_BLUE: Color = Color::from_rgb8(0, 123, 255);

/// 绿色按钮颜色
pub const BUTTON_COLOR_GREEN: Color = Color::from_rgb8(40, 167, 69);

/// 红色按钮颜色
pub const BUTTON_COLOR_RED: Color = Color::from_rgb8(220, 53, 69);

/// 灰色按钮颜色
pub const BUTTON_COLOR_GRAY: Color = Color::from_rgb8(108, 117, 125);

/// 黄色按钮颜色
pub const BUTTON_COLOR_YELLOW: Color = Color::from_rgb8(255, 204, 0);

// ============================================================================
// 通知颜色
// ============================================================================

/// 成功通知背景色（绿色）
pub const NOTIFICATION_SUCCESS_BG: Color = Color::from_rgb8(40, 167, 69);

/// 错误通知背景色（红色）
pub const NOTIFICATION_ERROR_BG: Color = Color::from_rgb8(220, 53, 69);

/// 信息通知背景色（蓝色）
pub const NOTIFICATION_INFO_BG: Color = Color::from_rgb8(0, 123, 255);

/// 通知文字颜色（白色）
pub const NOTIFICATION_TEXT_COLOR: Color = Color::WHITE;

// ============================================================================
// 禁用状态颜色
// ============================================================================

/// 禁用状态颜色（灰色）
pub const DISABLED_COLOR: Color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);

/// 禁用按钮背景色（半透明灰色）
pub const DISABLED_BUTTON_BG: Color = Color::from_rgba(0.7, 0.7, 0.7, 0.5);

// ============================================================================
// 文本输入框颜色
// ============================================================================

/// 文本输入框选择颜色
pub const TEXT_INPUT_SELECTION_COLOR: Color = Color::from_rgba(0.098, 0.463, 0.824, 0.3);

// ============================================================================
// Tooltip颜色
// ============================================================================

/// Tooltip背景颜色
pub const TOOLTIP_BG_COLOR: Color = Color::WHITE;

/// Tooltip边框颜色
pub const TOOLTIP_BORDER_COLOR: Color = Color::BLACK;

// ============================================================================
// 其他颜色
// ============================================================================

/// 分页分隔线文字颜色
pub const PAGE_SEPARATOR_TEXT_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);

/// 表格分隔线颜色
pub const TABLE_SEPARATOR_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);