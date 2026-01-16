//! UI样式常量定义
//!
//! 所有UI相关的样式常量（尺寸、颜色、间距等）应在此文件中定义，
//! 以确保代码一致性并便于维护。

use iced::Color;

// ============================================================================
// 通用样式常量
// ============================================================================

/// 标准边框宽度
pub const BORDER_WIDTH: f32 = 1.0;

/// 标准边框圆角半径
pub const BORDER_RADIUS: f32 = 5.0;

// ============================================================================
// 颜色常量
// ============================================================================

/// 浅色背景色
pub const COLOR_BG_LIGHT: Color = Color::from_rgb(0.9, 0.9, 0.9);

/// 深色文字颜色
pub const COLOR_TEXT_DARK: Color = Color::from_rgb(0.3, 0.3, 0.3);

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

/// 浅色按钮悬停状态
#[allow(dead_code)]
pub const COLOR_LIGHT_BUTTON_HOVER: Color = Color::from_rgb(0.878, 0.878, 0.878); // #E0E0E0

/// 浅色文字颜色
pub const COLOR_LIGHT_TEXT: Color = Color::from_rgb(0.2, 0.2, 0.2); // #333333

/// 浅色次要文字颜色
pub const COLOR_LIGHT_TEXT_SUB: Color = Color::from_rgb(0.533, 0.533, 0.533); // #888888

/// 选中状态蓝色
pub const COLOR_SELECTED_BLUE: Color = Color::from_rgb(0.098, 0.463, 0.824); // #1976D2

/// 纯净度-安全（绿色）
pub const COLOR_SFW: Color = Color::from_rgb(0.298, 0.686, 0.314); // #4CAF50

/// 纯净度-轻微（黄色）
pub const COLOR_SKETCHY: Color = Color::from_rgb(1.0, 0.757, 0.027); // #FFC107

/// 纯净度-成人（红色）
pub const COLOR_NSFW: Color = Color::from_rgb(0.965, 0.263, 0.212); // #F44336

// ============================================================================
// 按钮颜色常量
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
// Tooltip样式常量
// ============================================================================

/// Tooltip背景颜色
pub const TOOLTIP_BG_COLOR: Color = Color::WHITE;

/// Tooltip边框颜色
pub const TOOLTIP_BORDER_COLOR: Color = Color::BLACK;

/// Tooltip边框圆角半径
pub const TOOLTIP_BORDER_RADIUS: f32 = 4.0;

/// Tooltip边框宽度
pub const TOOLTIP_BORDER_WIDTH: f32 = 1.0;

// ============================================================================
// 对话框样式常量
// ============================================================================

/// 对话框标题文字大小
pub const DIALOG_TITLE_SIZE: f32 = 16.0;

/// 对话框消息文字大小
pub const DIALOG_MESSAGE_SIZE: f32 = 14.0;

/// 对话框按钮文字大小
pub const DIALOG_BUTTON_TEXT_SIZE: f32 = 14.0;

/// 对话框最大宽度
pub const DIALOG_MAX_WIDTH: f32 = 500.0;

/// 对话框边框圆角半径
pub const DIALOG_BORDER_RADIUS: f32 = 8.0;

/// 对话框边框宽度
pub const DIALOG_BORDER_WIDTH: f32 = 1.0;

/// 对话框内边距
pub const DIALOG_PADDING: f32 = 20.0;

/// 对话框元素间距
pub const DIALOG_SPACING: f32 = 15.0;

/// 对话框按钮间距
pub const DIALOG_BUTTON_SPACING: f32 = 10.0;

/// 对话框内部内容边距
pub const DIALOG_INNER_PADDING: f32 = 10.0;

/// 遮罩层透明度
pub const MASK_ALPHA: f32 = 0.5;

/// 开关文字大小
pub const TOGGLE_TEXT_SIZE: f32 = 12.0;

/// 开关间距
pub const TOGGLE_SPACING: f32 = 5.0;

// ============================================================================
// 设置页面样式常量
// ============================================================================

/// 设置区块标题文字大小
pub const SECTION_TITLE_SIZE: f32 = 16.0;

/// 按钮文字大小
pub const BUTTON_TEXT_SIZE: f32 = 14.0;

/// 文本输入框文字大小
pub const TEXT_INPUT_SIZE: f32 = 14.0;

/// 输入框高度
pub const INPUT_HEIGHT: f32 = 30.0;

/// 设置行间距
pub const ROW_SPACING: f32 = 10.0;

/// 设置区块内边距
pub const SECTION_PADDING: f32 = 15.0;

/// 设置区块间距
pub const SECTION_SPACING: f32 = 10.0;

/// 输入框内边距
pub const INPUT_PADDING: u16 = 5;

/// 按钮间距
pub const BUTTON_SPACING: f32 = 2.0;

// ============================================================================
// 图标按钮样式常量
// ============================================================================

/// 图标按钮文字大小
pub const ICON_BUTTON_TEXT_SIZE: f32 = 14.0;

/// 图标按钮内边距
pub const ICON_BUTTON_PADDING: [u16; 2] = [4, 4];

// ============================================================================
// 布局样式常量
// ============================================================================

/// 侧边栏宽度
pub const SIDEBAR_WIDTH: f32 = 180.0;

/// 行间距
pub const ROW_SPACING_LARGE: f32 = 20.0;

/// 外部边距
pub const OUTER_PADDING: f32 = 20.0;

/// Logo尺寸（像素）
pub const LOGO_SIZE: u32 = 128;

/// Logo显示尺寸
pub const LOGO_DISPLAY_SIZE: f32 = 128.0;

/// Logo间距
pub const LOGO_SPACING: f32 = 20.0;

/// 应用名称文字大小
pub const APP_NAME_SIZE: f32 = 24.0;

/// 占位符文字大小
pub const PLACEHOLDER_TEXT_SIZE: f32 = 24.0;

/// 按钮内边距
pub const BUTTON_PADDING: f32 = 10.0;

/// 侧边栏内边距
pub const SIDEBAR_PADDING: f32 = 10.0;

/// 布局边距
pub const LAYOUT_PADDING: f32 = 10.0;

// ============================================================================
// 图片卡片样式常量
// ============================================================================

/// 图片宽度
pub const IMAGE_WIDTH: f32 = 300.0;

/// 图片高度
pub const IMAGE_HEIGHT: f32 = 200.0;

/// 图片间距
pub const IMAGE_SPACING: f32 = 20.0;

/// 空状态边距
pub const EMPTY_STATE_PADDING: u16 = 360;

/// 空状态文字大小
pub const EMPTY_STATE_TEXT_SIZE: f32 = 24.0;

/// 加载状态文字大小
pub const LOADING_TEXT_SIZE: f32 = 24.0;

/// 全部加载文字大小
pub const ALL_LOADED_TEXT_SIZE: f32 = 14.0;

/// 遮罩层高度
pub const OVERLAY_HEIGHT: f32 = 22.0;

/// 遮罩层文字大小
pub const OVERLAY_TEXT_SIZE: f32 = 12.0;

// ============================================================================
// 错误占位符样式常量
// ============================================================================

/// 错误图标大小
pub const ERROR_ICON_SIZE: f32 = 56.0;

/// 错误文字大小
pub const ERROR_TEXT_SIZE: f32 = 18.0;

/// 错误路径文字大小
pub const ERROR_PATH_SIZE: f32 = 10.0;

// ============================================================================
// 模态窗口样式常量
// ============================================================================

/// 模态窗口加载文字大小
pub const MODAL_LOADING_TEXT_SIZE: f32 = 20.0;

/// 边框灰色值
pub const BORDER_COLOR_GRAY: f32 = 0.8;

// ============================================================================
// 筛选栏样式常量
// ============================================================================

/// 筛选栏高度
#[allow(dead_code)]
pub const FILTER_BAR_HEIGHT: f32 = 60.0;

/// 筛选栏内边距
#[allow(dead_code)]
pub const FILTER_BAR_PADDING: f32 = 10.0;

/// 筛选元素间距
#[allow(dead_code)]
pub const FILTER_SPACING: f32 = 10.0;

/// 筛选标签文字大小
#[allow(dead_code)]
pub const FILTER_LABEL_SIZE: f32 = 14.0;

// ============================================================================
// 分页分隔线样式常量
// ============================================================================

/// 分页分隔线高度
pub const PAGE_SEPARATOR_HEIGHT: f32 = 40.0;

/// 分页分隔线文字大小
pub const PAGE_SEPARATOR_TEXT_SIZE: f32 = 18.0;

/// 分页分隔线文字颜色
pub const PAGE_SEPARATOR_TEXT_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);

// ============================================================================
// 设置页面专用样式常量
// ============================================================================

/// 滚动边距
pub const SCROLL_PADDING: f32 = 20.0;

/// 选择列表宽度
pub const PICK_LIST_WIDTH: f32 = 80.0;

/// 端口输入框宽度
pub const PORT_INPUT_WIDTH: f32 = 80.0;

/// 关于信息区域宽度
pub const ABOUT_INFO_WIDTH: f32 = 350.0;

/// 关于区域Logo间距
pub const ABOUT_LOGO_SPACING: f32 = 40.0;

/// 关于区域行高度
pub const ABOUT_ROW_HEIGHT: f32 = 16.0;

// ============================================================================
// 下载页面样式常量
// ============================================================================

/// 下载任务项高度
pub const DOWNLOAD_TASK_HEIGHT: f32 = 60.0;

/// 下载任务项间距
pub const DOWNLOAD_TASK_SPACING: f32 = 10.0;

/// 下载任务项内边距
pub const DOWNLOAD_TASK_PADDING: f32 = 10.0;

/// 下载进度条高度
pub const DOWNLOAD_PROGRESS_HEIGHT: f32 = 6.0;

/// 下载进度条宽度
pub const DOWNLOAD_PROGRESS_WIDTH: f32 = 150.0;

/// 下载大小信息宽度
pub const DOWNLOAD_SIZE_INFO_WIDTH: f32 = 120.0;

/// 下载操作按钮间距
pub const DOWNLOAD_BUTTON_SPACING: f32 = 4.0;

/// 表格分隔线颜色
pub const TABLE_SEPARATOR_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);

/// 表格分隔线宽度
pub const TABLE_SEPARATOR_WIDTH: f32 = 1.0;
