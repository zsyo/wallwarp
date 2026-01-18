//! 下载管理页面模块
//!
//! 提供下载任务管理界面，支持查看下载进度、管理下载任务等功能。

use super::AppMessage;
use super::common;
use crate::i18n::I18n;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_COLOR_YELLOW, COLOR_LIGHT_TEXT_SUB, EMPTY_STATE_PADDING,
    EMPTY_STATE_TEXT_SIZE, TABLE_SEPARATOR_COLOR, TABLE_SEPARATOR_WIDTH,
};
use crate::utils::helpers::format_file_size;
use iced::widget::{column, container, progress_bar, row, scrollable, text};
use iced::{Alignment, Element, Font, Length};

/// 下载状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    /// 等待中
    Waiting,
    /// 下载中
    Downloading,
    /// 暂停
    Paused,
    /// 已完成
    Completed,
    /// 失败
    Failed(String),
    /// 已取消
    Cancelled,
}

/// 下载任务结构体
#[derive(Debug, Clone)]
pub struct DownloadTask {
    /// 任务ID
    pub id: usize,
    /// 文件名称
    pub file_name: String,
    /// 下载URL
    pub url: String,
    /// 保存路径
    pub save_path: String,
    /// 当前已下载大小（字节）
    pub downloaded_size: u64,
    /// 文件总大小（字节）
    pub total_size: u64,
    /// 下载进度（0.0 - 1.0）
    pub progress: f32,
    /// 下载速度（字节/秒）
    pub speed: u64,
    /// 状态
    pub status: DownloadStatus,
    /// 下载开始时间（用于计算速度）
    pub start_time: Option<std::time::Instant>,
    /// 取消令牌（用于终止下载）
    pub cancel_token: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
}

impl Default for DownloadTask {
    fn default() -> Self {
        Self {
            id: 0,
            file_name: String::new(),
            url: String::new(),
            save_path: String::new(),
            downloaded_size: 0,
            total_size: 0,
            progress: 0.0,
            speed: 0,
            status: DownloadStatus::Waiting,
            start_time: None,
            cancel_token: None,
        }
    }
}

/// 下载页面状态（扩展版，包含完整任务信息）
#[derive(Debug, Default)]
pub struct DownloadStateFull {
    /// 下载任务列表（完整信息）
    pub tasks: Vec<DownloadTaskFull>,
    /// 任务计数器
    pub next_id: usize,
    /// HTTP客户端
    pub client: Option<reqwest::Client>,
    /// 当前正在下载的任务数
    pub downloading_count: usize,
    /// 最大并行下载数
    pub max_concurrent_downloads: usize,
}

impl DownloadStateFull {
    /// 创建新状态
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
            client: None,
            downloading_count: 0,
            max_concurrent_downloads: 3,
        }
    }

    /// 初始化HTTP客户端
    pub fn init_client(&mut self) {
        if self.client.is_none() {
            self.client = Some(reqwest::Client::new());
        }
    }

    /// 获取当前正在下载的任务数
    pub fn get_downloading_count(&self) -> usize {
        self.downloading_count
    }

    /// 检查是否可以开始新下载
    pub fn can_start_download(&self) -> bool {
        self.downloading_count < self.max_concurrent_downloads
    }

    /// 增加正在下载的任务数
    pub fn increment_downloading(&mut self) {
        self.downloading_count = self.downloading_count.saturating_add(1);
    }

    /// 减少正在下载的任务数
    pub fn decrement_downloading(&mut self) {
        if self.downloading_count > 0 {
            self.downloading_count -= 1;
        }
    }

    /// 添加新下载任务（倒序插入到列表开头）
    pub fn add_task(&mut self, url: String, save_path: String, file_name: String, proxy: Option<String>, file_type: String) {
        let task = DownloadTask {
            id: self.next_id,
            file_name: file_name.clone(),
            url: url.clone(),
            save_path: save_path.clone(),
            downloaded_size: 0,
            total_size: 0,
            progress: 0.0,
            speed: 0,
            status: DownloadStatus::Waiting,
            start_time: None,
            cancel_token: Some(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false))),
        };
        // 倒序插入：添加到列表开头
        self.tasks.insert(0, DownloadTaskFull { task, proxy, file_type });
        self.next_id += 1;
    }

    /// 获取下一个等待中的任务（按添加顺序，先添加的先开始）
    pub fn get_next_waiting_task(&mut self) -> Option<&mut DownloadTaskFull> {
        // 查找状态为 Waiting 的任务（因为是倒序，最早添加的在列表末尾）
        self.tasks.iter_mut().find(|t| t.task.status == DownloadStatus::Waiting)
    }

    /// 更新任务进度
    pub fn update_progress(&mut self, id: usize, downloaded: u64, total: u64, speed: u64) {
        if let Some(task_full) = self.tasks.iter_mut().find(|t| t.task.id == id) {
            task_full.task.downloaded_size = downloaded;
            task_full.task.total_size = total;
            task_full.task.speed = speed;
            if total > 0 {
                task_full.task.progress = downloaded as f32 / total as f32;
            }
        }
    }

    /// 更新任务状态
    pub fn update_status(&mut self, id: usize, status: DownloadStatus) {
        if let Some(task_full) = self.tasks.iter_mut().find(|t| t.task.id == id) {
            task_full.task.status = status;
        }
    }

    /// 获取任务（通过索引避免借用冲突）
    pub fn get_task_by_index(&mut self, index: usize) -> Option<&mut DownloadTaskFull> {
        self.tasks.get_mut(index)
    }

    /// 根据ID查找任务索引
    pub fn find_task_index(&self, id: usize) -> Option<usize> {
        self.tasks.iter().position(|t| t.task.id == id)
    }

    /// 获取任务
    pub fn get_task(&mut self, id: usize) -> Option<&mut DownloadTaskFull> {
        self.tasks.iter_mut().find(|t| t.task.id == id)
    }

    /// 移除任务
    pub fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|t| t.task.id != id);
    }

    /// 清空所有已完成的任务
    pub fn clear_completed(&mut self) {
        self.tasks.retain(|t| t.task.status != DownloadStatus::Completed);
    }

    /// 取消任务
    pub fn cancel_task(&mut self, id: usize) {
        if let Some(task_full) = self.tasks.iter_mut().find(|t| t.task.id == id) {
            // 设置取消标志
            if let Some(cancel_token) = &task_full.task.cancel_token {
                cancel_token.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            // 注意：不在这里更新状态，让调用者决定最终状态
        }
    }

    /// 更新下载速度（基于时间计算）
    pub fn update_speed(&mut self) {
        for task_full in self.tasks.iter_mut() {
            if task_full.task.status == DownloadStatus::Downloading {
                if let Some(start_time) = task_full.task.start_time {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    if elapsed > 0.0 && task_full.task.downloaded_size > 0 {
                        task_full.task.speed = (task_full.task.downloaded_size as f64 / elapsed) as u64;
                    }
                }
            }
        }
    }
}

/// 下载任务完整结构体（包含额外信息）
#[derive(Debug, Clone)]
pub struct DownloadTaskFull {
    /// 基础任务信息
    pub task: DownloadTask,
    /// 代理设置
    pub proxy: Option<String>,
    /// 原始文件类型
    pub file_type: String,
}

impl Default for DownloadTaskFull {
    fn default() -> Self {
        Self {
            task: DownloadTask::default(),
            proxy: None,
            file_type: "jpg".to_string(),
        }
    }
}

/// 下载页面消息
#[derive(Debug, Clone)]
pub enum DownloadMessage {
    /// 添加新下载任务 (url, save_path, file_name, proxy, file_type)
    AddTask(String, String, String, Option<String>, String),
    /// 暂停任务
    PauseTask(usize),
    /// 继续任务（断点续传）
    ResumeTask(usize),
    /// 重新下载（清空已下载文件并从头开始）
    RetryTask(usize),
    /// 取消任务
    CancelTask(usize),
    /// 删除任务
    DeleteTask(usize),
    /// 打开文件位置
    OpenFileLocation(usize),
    /// 清空已完成的任务
    ClearCompleted,
    /// 模拟进度更新（测试用）
    SimulateProgress,
    /// 下载完成 (任务ID, 文件大小, 错误信息)
    DownloadCompleted(usize, u64, Option<String>),
    /// 下载进度更新
    DownloadProgress(usize, u64, u64, u64),
    /// 更新下载速度（定时触发）
    UpdateSpeed,
    /// 复制下载链接
    CopyDownloadLink(usize),
    /// 设为壁纸
    SetAsWallpaper(usize),
}

/// 生成下载文件名
pub fn generate_file_name(id: &str, file_type: &str) -> String {
    format!("wallhaven-{}.{}", id, file_type)
}

pub fn download_view<'a>(i18n: &'a I18n, _window_width: u32, download_state: &'a DownloadStateFull) -> Element<'a, AppMessage> {
    let content = if download_state.tasks.is_empty() {
        // 空状态显示
        create_empty_state(i18n)
    } else {
        // 表格布局
        create_table_view(i18n, download_state)
    };

    let scrollable_content = scrollable(content).width(Length::Fill).height(Length::Fill);

    container(scrollable_content).width(Length::Fill).height(Length::Fill).padding(20).into()
}

/// 创建表格视图
fn create_table_view<'a>(i18n: &'a I18n, download_state: &'a DownloadStateFull) -> Element<'a, AppMessage> {
    // 表头
    let header = create_table_header(i18n);

    // 表格内容
    let mut table = column![header].spacing(0).width(Length::Fill);

    // 添加表头下方的水平分隔线
    table = table.push(create_horizontal_separator());

    for task_full in &download_state.tasks {
        // 添加表格行
        table = table.push(create_table_row(i18n, &task_full.task));
        // 添加行下方的水平分隔线
        table = table.push(create_horizontal_separator());
    }

    table.into()
}

/// 创建水平分隔线
fn create_horizontal_separator() -> Element<'static, AppMessage> {
    container(iced::widget::Space::new())
        .width(Length::Fill)
        .height(TABLE_SEPARATOR_WIDTH)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(TABLE_SEPARATOR_COLOR)),
            ..Default::default()
        })
        .into()
}

/// 创建表头
fn create_table_header<'a>(i18n: &'a I18n) -> Element<'a, AppMessage> {
    row![
        // 文件名列
        container(text(i18n.t("download-tasks.header-filename")).size(14))
            .width(Length::FillPortion(3))
            .padding(5),
        // 分隔线
        create_separator(),
        // 大小列
        container(text(i18n.t("download-tasks.header-size")).size(14))
            .width(Length::Fixed(100.0))
            .padding(5),
        // 分隔线
        create_separator(),
        // 状态列
        container(text(i18n.t("download-tasks.header-status")).size(14))
            .width(Length::Fixed(220.0))
            .padding(5),
        // 分隔线
        create_separator(),
        // 下载列
        container(text(i18n.t("download-tasks.header-download")).size(14))
            .width(Length::Fixed(100.0))
            .padding(5),
        // 分隔线
        create_separator(),
        // 操作列（最后一列，不添加分隔线）
        container(text(i18n.t("download-tasks.header-operations")).size(14))
            .width(Length::Fill)
            .padding(5),
    ]
    .width(Length::Fill)
    .padding(5)
    .align_y(Alignment::Center)
    .into()
}

/// 创建表格行
fn create_table_row<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    row![
        // 文件名列
        container(text(&task.file_name).size(13)).width(Length::FillPortion(3)).padding(5),
        // 分隔线
        create_separator(),
        // 大小列
        container(text(format_file_size(task.total_size)).size(12))
            .width(Length::Fixed(100.0))
            .padding(5),
        // 分隔线
        create_separator(),
        // 状态列
        container(create_status_display(i18n, task)).width(Length::Fixed(220.0)).padding(5),
        // 分隔线
        create_separator(),
        // 下载列
        container(create_download_display(i18n, task)).width(Length::Fixed(100.0)).padding(5),
        // 分隔线
        create_separator(),
        // 操作列（最后一列，不添加分隔线）
        container(create_operation_buttons(i18n, task)).width(Length::Fill).padding(5),
    ]
    .width(Length::Fill)
    .padding(5)
    .align_y(Alignment::Center)
    .into()
}
/// 创建状态显示
fn create_status_display<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    match &task.status {
        DownloadStatus::Downloading => {
            // 下载中：显示进度条和百分比
            let progress_bar = container(progress_bar(0.0..=1.0, task.progress))
                .width(Length::Fixed(160.0))
                .height(Length::Fixed(12.0));
            let progress_text = text(format!("{:.0}%", task.progress * 100.0)).size(11).style(|_| text::Style {
                color: Some(BUTTON_COLOR_BLUE),
            });
            row![progress_bar, progress_text].spacing(6).align_y(Alignment::Center).into()
        }
        DownloadStatus::Waiting => text(i18n.t("download-tasks.status-waiting"))
            .size(12)
            .style(|_| text::Style {
                color: Some(COLOR_LIGHT_TEXT_SUB),
            })
            .into(),
        DownloadStatus::Paused => text(i18n.t("download-tasks.status-paused"))
            .size(12)
            .style(|_| text::Style {
                color: Some(BUTTON_COLOR_GRAY),
            })
            .into(),
        DownloadStatus::Completed => text(i18n.t("download-tasks.status-completed"))
            .size(12)
            .style(|_| text::Style {
                color: Some(BUTTON_COLOR_GREEN),
            })
            .into(),
        DownloadStatus::Failed(_msg) => text(i18n.t("download-tasks.status-failed-error"))
            .size(12)
            .style(|_| text::Style { color: Some(BUTTON_COLOR_RED) })
            .into(),
        DownloadStatus::Cancelled => text(i18n.t("download-tasks.status-cancelled"))
            .size(12)
            .style(|_| text::Style {
                color: Some(BUTTON_COLOR_YELLOW),
            })
            .into(),
    }
}

/// 创建下载显示
fn create_download_display<'a>(_i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    let speed_text = match &task.status {
        DownloadStatus::Downloading => format_speed(task.speed),
        _ => "0 B/s".to_string(),
    };

    text(speed_text)
        .size(12)
        .style(|_| text::Style {
            color: Some(COLOR_LIGHT_TEXT_SUB),
        })
        .into()
}

/// 创建操作按钮
fn create_operation_buttons<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    let pause_button = common::create_icon_button_with_tooltip(
        "\u{F4C3}", // pause-fill
        BUTTON_COLOR_YELLOW,
        AppMessage::Download(DownloadMessage::PauseTask(task.id)),
        i18n.t("download-tasks.tooltip-pause"),
    );
    let resume_button = common::create_icon_button_with_tooltip(
        "\u{F4F4}", // play-fill
        BUTTON_COLOR_GREEN,
        AppMessage::Download(DownloadMessage::ResumeTask(task.id)),
        i18n.t("download-tasks.tooltip-resume"),
    );
    let retry_button = common::create_icon_button_with_tooltip(
        "\u{F130}", // play-fill (重新下载)
        BUTTON_COLOR_BLUE,
        AppMessage::Download(DownloadMessage::RetryTask(task.id)),
        i18n.t("download-tasks.tooltip-retry"),
    );
    let copy_button = common::create_icon_button_with_tooltip(
        "\u{F759}", // link-45deg
        BUTTON_COLOR_BLUE,
        AppMessage::Download(DownloadMessage::CopyDownloadLink(task.id)),
        i18n.t("download-tasks.tooltip-copy-url"),
    );
    let cancel_button = common::create_icon_button_with_tooltip(
        "\u{F117}", // x-circle-fill
        BUTTON_COLOR_RED,
        AppMessage::Download(DownloadMessage::CancelTask(task.id)),
        i18n.t("download-tasks.tooltip-cancel"),
    );
    let delete_button = common::create_icon_button_with_tooltip(
        "\u{F78B}", // trash-fill (删除任务)
        BUTTON_COLOR_RED,
        AppMessage::Download(DownloadMessage::DeleteTask(task.id)),
        i18n.t("download-tasks.tooltip-delete"),
    );
    let view_button = common::create_icon_button_with_tooltip(
        "\u{F341}", // folder-fill (查看文件)
        BUTTON_COLOR_YELLOW,
        AppMessage::Download(DownloadMessage::OpenFileLocation(task.id)),
        i18n.t("download-tasks.tooltip-open"),
    );
    let set_wallpaper_button = common::create_icon_button_with_tooltip(
        "\u{F429}", // image-fill (设为壁纸)
        BUTTON_COLOR_GREEN,
        AppMessage::Download(DownloadMessage::SetAsWallpaper(task.id)),
        i18n.t("local-list.tooltip-set-wallpaper"),
    );

    match &task.status {
        DownloadStatus::Downloading => {
            // 下载中：暂停/复制下载链接/取消
            row![pause_button, copy_button, cancel_button].spacing(6).into()
        }
        DownloadStatus::Paused => {
            // 暂停中：继续/复制下载链接/取消
            row![resume_button, copy_button, cancel_button].spacing(6).into()
        }
        DownloadStatus::Failed(_) => {
            // 下载失败：重新下载/复制下载链接/删除
            row![retry_button, copy_button, delete_button].spacing(6).into()
        }
        DownloadStatus::Cancelled => {
            // 已取消：重新下载/复制下载链接/删除
            row![retry_button, copy_button, delete_button].spacing(6).into()
        }
        DownloadStatus::Completed => {
            // 下载完成：查看文件/设为壁纸/复制下载链接/删除(仅删除任务)
            row![view_button, set_wallpaper_button, copy_button, delete_button].spacing(6).into()
        }
        DownloadStatus::Waiting => {
            // 等待中：复制下载链接/取消
            row![copy_button, cancel_button].spacing(6).into()
        }
    }
}

/// 创建空状态界面
fn create_empty_state<'a>(i18n: &'a I18n) -> Element<'a, AppMessage> {
    let icon = text("\u{F30A}")
        .font(Font::with_name("bootstrap-icons"))
        .size(48.0)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_LIGHT_TEXT_SUB),
        });

    let empty_text = text(i18n.t("download-tasks.no-tasks")).size(EMPTY_STATE_TEXT_SIZE);

    let hint_text = text(i18n.t("download-tasks.no-tasks-hint")).size(14).style(|_theme: &iced::Theme| text::Style {
        color: Some(COLOR_LIGHT_TEXT_SUB),
    });

    column![icon, empty_text, hint_text]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(EMPTY_STATE_PADDING)
        .spacing(10)
        .into()
}

/// 格式化下载速度
fn format_speed(speed: u64) -> String {
    format!("{}/s", format_file_size(speed))
}

impl From<DownloadMessage> for AppMessage {
    fn from(download_message: DownloadMessage) -> AppMessage {
        AppMessage::Download(download_message)
    }
}

/// 创建表格列分隔线
fn create_separator() -> Element<'static, AppMessage> {
    container(iced::widget::Space::new())
        .width(TABLE_SEPARATOR_WIDTH)
        .height(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(TABLE_SEPARATOR_COLOR)),
            ..Default::default()
        })
        .into()
}
