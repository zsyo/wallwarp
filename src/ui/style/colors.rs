// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 颜色常量定义
//!
//! 仅存放**与主题无关**的固定颜色：Wallhaven 官方色板、纯净度语义色、
//! 按钮语义色、通知色等。随明暗主题变化的颜色一律定义在
//! [`super::theme_colors::ThemeColors`] 中，禁止在此新增主题相关色值。

use iced::Color;

// ============================================================================
// 强调色（全应用统一为现代蓝，与 theme_colors 的 primary 保持一致）
// ============================================================================

/// 选中状态蓝色（强调色，浅色基准值；主题感知场景请用 ThemeColors::primary）
pub const COLOR_SELECTED_BLUE: Color = Color::from_rgb8(59, 130, 246); // #3B82F6

/// 颜色选择器激活状态颜色
pub const COLOR_PICKER_ACTIVE: Color = Color::from_rgb8(59, 130, 246); // #3B82F6

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

/// 颜色选择器 Any 选项底色（浅灰，与主题无关）
pub const COLOR_LIGHT_BUTTON: Color = Color::from_rgb8(239, 241, 244); // #EFF1F4

/// 颜色选择器无色斜线颜色
pub const COLOR_NO_COLOR_STROKE: Color = Color::from_rgb(1.0, 0.0, 0.0);

// ============================================================================
// 按钮语义色（固定值，不随主题变化；白色文字均有足够对比度）
// ============================================================================

/// 蓝色按钮颜色（主要操作）
pub const BUTTON_COLOR_BLUE: Color = Color::from_rgb8(59, 130, 246); // #3B82F6

/// 绿色按钮颜色（成功/确认）
pub const BUTTON_COLOR_GREEN: Color = Color::from_rgb8(22, 163, 74); // #16A34A

/// 红色按钮颜色（危险/删除）
pub const BUTTON_COLOR_RED: Color = Color::from_rgb8(220, 38, 38); // #DC2626

/// 灰色按钮颜色（次要操作）
pub const BUTTON_COLOR_GRAY: Color = Color::from_rgb8(107, 114, 128); // #6B7280

/// 黄色按钮颜色（警告/取消）
pub const BUTTON_COLOR_YELLOW: Color = Color::from_rgb8(245, 158, 11); // #F59E0B

// ============================================================================
// 通知颜色
// ============================================================================

/// 成功通知背景色（绿色）
pub const NOTIFICATION_SUCCESS_BG: Color = Color::from_rgb8(22, 163, 74); // #16A34A

/// 错误通知背景色（红色）
pub const NOTIFICATION_ERROR_BG: Color = Color::from_rgb8(220, 38, 38); // #DC2626

/// 信息通知背景色（蓝色）
pub const NOTIFICATION_INFO_BG: Color = Color::from_rgb8(59, 130, 246); // #3B82F6

/// 通知文字颜色（白色）
pub const NOTIFICATION_TEXT_COLOR: Color = Color::WHITE;

// ============================================================================
// 禁用状态颜色
// ============================================================================

/// 禁用状态颜色（灰色）
pub const DISABLED_COLOR: Color = Color::from_rgb8(154, 160, 168); // #9AA0A8

/// 禁用按钮背景色（半透明灰色）
pub const DISABLED_BUTTON_BG: Color = Color::from_rgba8(107, 114, 128, 0.25);

// ============================================================================
// 其他颜色
// ============================================================================

/// 模态窗口背景色（半透明黑色）
pub const COLOR_MODAL_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.85);

/// 遮罩层背景色（半透明黑色）
pub const COLOR_OVERLAY_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);

/// 遮罩层文字颜色
pub const COLOR_OVERLAY_TEXT: Color = Color::WHITE;

/// 分页分隔线文字颜色
pub const PAGE_SEPARATOR_TEXT_COLOR: Color = Color::from_rgb8(107, 114, 128); // #6B7280
