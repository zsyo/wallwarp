//! 下载管理页面模块
//!
//! 提供下载任务管理界面，支持查看下载进度、管理下载任务等功能。

use super::AppMessage;
use super::common;
use crate::i18n::I18n;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, COLOR_LIGHT_TEXT, COLOR_LIGHT_TEXT_SUB,
    EMPTY_STATE_PADDING, EMPTY_STATE_TEXT_SIZE,
};
use iced::widget::{column, container, progress_bar, row, scrollable, text};
use iced::{Alignment, Color, Element, Font, Length};

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
    pub fn add_task(
        &mut self,
        url: String,
        save_path: String,
        file_name: String,
        proxy: Option<String>,
        file_type: String,
    ) {
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
        println!("[下载状态] 添加任务，列表长度: {}, 正在下载: {}/{}", self.tasks.len(), self.downloading_count, self.max_concurrent_downloads);
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
            println!("[下载进度] ID:{}, 已下载:{}, 总大小:{}, 进度:{:.1}%, 速度: {}/s",
                id, downloaded, total, task_full.task.progress * 100.0, format_file_size(speed));
        }
    }

    /// 更新任务速度（基于已下载量和时间）
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

    /// 更新任务状态
    pub fn update_status(&mut self, id: usize, status: DownloadStatus) {
        if let Some(task_full) = self.tasks.iter_mut().find(|t| t.task.id == id) {
            println!("[下载状态] ID:{}, 状态变化: {:?} -> {:?}", id, task_full.task.status, status);
            task_full.task.status = status;
        }
    }

    /// 取消任务
    pub fn cancel_task(&mut self, id: usize) {
        if let Some(task_full) = self.tasks.iter_mut().find(|t| t.task.id == id) {
            println!("[下载状态] ID:{}, 设置取消标志", id);
            // 设置取消标志
            if let Some(cancel_token) = &task_full.task.cancel_token {
                cancel_token.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            // 注意：不在这里更新状态，让调用者决定最终状态
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
        let count_before = self.tasks.len();
        self.tasks.retain(|t| t.task.id != id);
        if self.tasks.len() < count_before {
            println!("[下载状态] 移除任务 ID:{}", id);
        }
    }

    /// 清空所有已完成的任务
    pub fn clear_completed(&mut self) {
        let count_before = self.tasks.len();
        self.tasks.retain(|t| t.task.status != DownloadStatus::Completed);
        println!("[下载状态] 清空已完成任务: {} -> {}", count_before, self.tasks.len());
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
    /// 继续任务
    ResumeTask(usize),
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
    /// 下载进度更新 (任务ID, 已下载大小, 总大小, 速度)
    DownloadProgress(usize, u64, u64, u64),
    /// 更新下载速度（定时触发）
    UpdateSpeed,
}

/// 生成下载文件名
pub fn generate_file_name(id: &str, file_type: &str) -> String {
    format!("wallhaven-{}.{}", id, file_type)
}

pub fn download_view<'a>(
    i18n: &'a I18n,
    _window_width: u32,
    download_state: &'a DownloadStateFull,
) -> Element<'a, AppMessage> {
    // println!("[下载页面] 渲染视图，任务数量: {}", download_state.tasks.len());

    let content = if download_state.tasks.is_empty() {
        // 空状态显示
        // println!("[下载页面] 任务列表为空");
        create_empty_state(i18n)
    } else {
        // 任务列表
        // println!("[下载页面] 渲染 {} 个任务", download_state.tasks.len());
        create_task_list(i18n, download_state)
    };

    let scrollable_content = scrollable(content).width(Length::Fill).height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
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

    let hint_text = text(i18n.t("download-tasks.no-tasks-hint"))
        .size(14)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_LIGHT_TEXT_SUB),
        });

    column![icon, empty_text, hint_text]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(EMPTY_STATE_PADDING)
        .spacing(10)
        .into()
}

/// 创建任务列表界面
fn create_task_list<'a>(i18n: &'a I18n, download_state: &'a DownloadStateFull) -> Element<'a, AppMessage> {
    let mut content = column![].spacing(10).width(Length::Fill);

    for task_full in &download_state.tasks {
        content = content.push(create_task_item(i18n, &task_full.task));
    }

    content.into()
}

/// 创建单个任务项
fn create_task_item<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    // 文件名和状态图标
    let status_icon = match &task.status {
        DownloadStatus::Waiting => text("\u{F252}")
            .font(Font::with_name("bootstrap-icons"))
            .size(16)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(COLOR_LIGHT_TEXT_SUB),
            }),
        DownloadStatus::Downloading => text("\u{F252}")
            .font(Font::with_name("bootstrap-icons"))
            .size(16)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(BUTTON_COLOR_BLUE),
            }),
        DownloadStatus::Paused => text("\u{F8C9}")
            .font(Font::with_name("bootstrap-icons"))
            .size(16)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(BUTTON_COLOR_GRAY),
            }),
        DownloadStatus::Completed => text("\u{F26C}")
            .font(Font::with_name("bootstrap-icons"))
            .size(16)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(BUTTON_COLOR_GREEN),
            }),
        DownloadStatus::Failed(_) => text("\u{F659}")
            .font(Font::with_name("bootstrap-icons"))
            .size(16)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(BUTTON_COLOR_RED),
            }),
    };

    let file_info = column![
        text(&task.file_name)
            .size(14)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(COLOR_LIGHT_TEXT),
            }),
        create_status_text(i18n, task),
    ]
    .spacing(2);

    let left_content = row![status_icon, container(file_info).padding(8)].align_y(Alignment::Center);

    // 进度条
    let progress = container(progress_bar(0.0..=1.0, task.progress))
        .width(Length::Fixed(150.0))
        .height(Length::Fixed(6.0));

    let progress_text = text(format!("{:.0}%", task.progress * 100.0))
        .size(12)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_LIGHT_TEXT_SUB),
        });

    let progress_section = column![progress, progress_text].width(Length::Fixed(180.0)).spacing(2);

    // 大小信息
    let size_info = text(format!(
        "{} / {}",
        format_file_size(task.downloaded_size),
        format_file_size(task.total_size)
    ))
    .size(12)
    .style(|_theme: &iced::Theme| text::Style {
        color: Some(COLOR_LIGHT_TEXT_SUB),
    });

    // 操作按钮
    let action_buttons = create_action_buttons(i18n, task);

    // 整体布局
    let item_content = row![
        left_content,
        container(iced::widget::Space::new()).width(Length::Fill),
        progress_section,
        container(size_info).width(Length::Fixed(120.0)),
        action_buttons,
    ]
    .align_y(Alignment::Center)
    .padding(10);

    container(item_content)
        .width(Length::Fill)
        .style(create_task_item_style)
        .into()
}

/// 创建状态文本
fn create_status_text<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    match &task.status {
        DownloadStatus::Waiting => container(text(i18n.t("download-tasks.status-waiting")).size(12).style(
            |_theme: &iced::Theme| text::Style {
                color: Some(COLOR_LIGHT_TEXT_SUB),
            },
        ))
        .into(),
        DownloadStatus::Downloading => {
            let speed_text = format_speed(task.speed);
            container(
                text(format!(
                    "{} - {}",
                    i18n.t("download-tasks.status-downloading"),
                    speed_text
                ))
                .size(12)
                .style(|_theme: &iced::Theme| text::Style {
                    color: Some(BUTTON_COLOR_BLUE),
                }),
            )
            .into()
        }
        DownloadStatus::Paused => container(text(i18n.t("download-tasks.status-paused")).size(12).style(
            |_theme: &iced::Theme| text::Style {
                color: Some(BUTTON_COLOR_GRAY),
            },
        ))
        .into(),
        DownloadStatus::Completed => container(text(i18n.t("download-tasks.status-completed")).size(12).style(
            |_theme: &iced::Theme| text::Style {
                color: Some(BUTTON_COLOR_GREEN),
            },
        ))
        .into(),
        DownloadStatus::Failed(msg) => container(
            text(format!("{}: {}", i18n.t("download-tasks.status-failed"), msg))
                .size(12)
                .style(|_theme: &iced::Theme| text::Style {
                    color: Some(BUTTON_COLOR_RED),
                }),
        )
        .into(),
    }
}

/// 创建操作按钮组
fn create_action_buttons<'a>(i18n: &'a I18n, task: &'a DownloadTask) -> Element<'a, AppMessage> {
    match &task.status {
        DownloadStatus::Waiting | DownloadStatus::Paused => {
            let resume_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F144}",
                    BUTTON_COLOR_GREEN,
                    AppMessage::Download(DownloadMessage::ResumeTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-resume"),
            );
            let cancel_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F8FB}",
                    BUTTON_COLOR_RED,
                    AppMessage::Download(DownloadMessage::CancelTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-cancel"),
            );
            row![resume_button, cancel_button].spacing(4).into()
        }
        DownloadStatus::Downloading => {
            let pause_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F8C9}",
                    BUTTON_COLOR_GRAY,
                    AppMessage::Download(DownloadMessage::PauseTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-pause"),
            );
            let cancel_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F8FB}",
                    BUTTON_COLOR_RED,
                    AppMessage::Download(DownloadMessage::CancelTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-cancel"),
            );
            row![pause_button, cancel_button].spacing(4).into()
        }
        DownloadStatus::Completed => {
            let open_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F341}",
                    BUTTON_COLOR_BLUE,
                    AppMessage::Download(DownloadMessage::OpenFileLocation(task.id)),
                ),
                i18n.t("download-tasks.tooltip-open"),
            );
            let delete_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F78B}",
                    BUTTON_COLOR_RED,
                    AppMessage::Download(DownloadMessage::DeleteTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-delete"),
            );
            row![open_button, delete_button].spacing(4).into()
        }
        DownloadStatus::Failed(_) => {
            let retry_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F4E4}",
                    BUTTON_COLOR_BLUE,
                    AppMessage::Download(DownloadMessage::ResumeTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-retry"),
            );
            let delete_button = common::create_button_with_tooltip(
                common::create_icon_button(
                    "\u{F78B}",
                    BUTTON_COLOR_RED,
                    AppMessage::Download(DownloadMessage::DeleteTask(task.id)),
                ),
                i18n.t("download-tasks.tooltip-delete"),
            );
            row![retry_button, delete_button].spacing(4).into()
        }
    }
}

/// 任务项样式
fn create_task_item_style(theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
        border: iced::border::Border {
            color: theme.extended_palette().primary.weak.color,
            width: 1.0,
            radius: iced::border::Radius::from(6.0),
        },
        ..Default::default()
    }
}

/// 格式化文件大小
fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
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
