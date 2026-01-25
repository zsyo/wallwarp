// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 尺寸常量定义
//!
//! 所有UI相关的尺寸常量（宽度、高度、间距、字体大小等）应在此文件中定义。

// ============================================================================
// 通用尺寸
// ============================================================================

/// 标准边框宽度
pub const BORDER_WIDTH: f32 = 1.0;

/// 标准边框圆角半径
pub const BORDER_RADIUS: f32 = 5.0;

/// 侧边栏选中指示条宽度
pub const SIDEBAR_INDICATOR_WIDTH: f32 = 6.0;

/// 分隔线宽度
pub const SEPARATOR_WIDTH: f32 = 1.0;

/// 分隔线阴影偏移
pub const SEPARATOR_SHADOW_OFFSET: f32 = 3.0;

/// 分隔线阴影模糊
pub const SEPARATOR_SHADOW_BLUR: f32 = 6.0;

/// 遮罩层透明度
pub const MASK_ALPHA: f32 = 0.5;

/// 边框灰色值
pub const BORDER_COLOR_GRAY: f32 = 0.8;

// ============================================================================
// Tooltip尺寸
// ============================================================================

/// Tooltip边框圆角半径
pub const TOOLTIP_BORDER_RADIUS: f32 = 4.0;

/// Tooltip边框宽度
pub const TOOLTIP_BORDER_WIDTH: f32 = 1.0;

// ============================================================================
// 对话框尺寸
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

/// 开关文字大小
pub const TOGGLE_TEXT_SIZE: f32 = 12.0;

/// 开关间距
pub const TOGGLE_SPACING: f32 = 5.0;

// ============================================================================
// 设置页面尺寸
// ============================================================================

/// 设置区块标题文字大小
pub const SECTION_TITLE_SIZE: f32 = 20.0;

/// 设置区块内容间距
pub const SECTION_CONTENT_SPACING: f32 = 5.0;

/// 按钮文字大小
pub const BUTTON_TEXT_SIZE: f32 = 14.0;

/// 文本输入框文字大小
pub const TEXT_INPUT_SIZE: f32 = 14.0;

/// 输入框高度
pub const INPUT_HEIGHT: f32 = 30.0;

/// 设置行间距
pub const ROW_SPACING: f32 = 10.0;

/// 设置行间距
pub const SETTINGS_ROW_SPACING: f32 = 20.0;

/// 设置区块内边距
pub const SECTION_PADDING: f32 = 15.0;

/// 设置区块间距
pub const SECTION_SPACING: f32 = 10.0;

/// 输入框内边距
pub const INPUT_PADDING: u16 = 5;

/// 按钮间距
pub const BUTTON_SPACING: f32 = 6.0;

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
// 图标按钮尺寸
// ============================================================================

/// 图标按钮文字大小
pub const ICON_BUTTON_TEXT_SIZE: f32 = 14.0;

/// 图标按钮内边距
pub const ICON_BUTTON_PADDING: [u16; 2] = [4, 4];

// ============================================================================
// 布局尺寸
// ============================================================================

/// 侧边栏宽度
pub const SIDEBAR_WIDTH: f32 = 200.0;

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
// 图片卡片尺寸
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
// 错误占位符尺寸
// ============================================================================

/// 错误图标大小
pub const ERROR_ICON_SIZE: f32 = 56.0;

/// 错误文字大小
pub const ERROR_TEXT_SIZE: f32 = 18.0;

/// 错误路径文字大小
pub const ERROR_PATH_SIZE: f32 = 10.0;

// ============================================================================
// 模态窗口尺寸
// ============================================================================

/// 模态窗口加载文字大小
pub const MODAL_LOADING_TEXT_SIZE: f32 = 20.0;

// ============================================================================
// 分页分隔线尺寸
// ============================================================================

/// 分页分隔线高度
pub const PAGE_SEPARATOR_HEIGHT: f32 = 40.0;

/// 分页分隔线文字大小
pub const PAGE_SEPARATOR_TEXT_SIZE: f32 = 18.0;

// ============================================================================
// 下载页面尺寸
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

/// 表格分隔线宽度
pub const TABLE_SEPARATOR_WIDTH: f32 = 1.0;
