// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理状态模块
//!
//! 定义下载任务的状态结构和状态管理方法

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
        self.tasks.insert(
            0,
            DownloadTaskFull {
                task,
                proxy,
                file_type,
            },
        );
        self.next_id += 1;
    }

    /// 获取下一个等待中的任务（按添加顺序，先添加的先开始）
    pub fn get_next_waiting_task(&mut self) -> Option<&mut DownloadTaskFull> {
        // 查找状态为 Waiting 的任务（因为是倒序，最早添加的在列表末尾）
        self.tasks
            .iter_mut()
            .find(|t| t.task.status == DownloadStatus::Waiting)
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
                        task_full.task.speed =
                            (task_full.task.downloaded_size as f64 / elapsed) as u64;
                    }
                }
            }
        }
    }
}