// Copyright (C) 2026 zsyo - GNU AGPL v3.0

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

impl DownloadStatus {
    /// 获取状态对应的翻译key
    pub fn get_translation_key(&self) -> &'static str {
        match self {
            DownloadStatus::Waiting => "download-tasks.status-waiting",
            DownloadStatus::Downloading => "download-tasks.status-downloading",
            DownloadStatus::Paused => "download-tasks.status-paused",
            DownloadStatus::Completed => "download-tasks.status-completed",
            DownloadStatus::Failed(_) => "download-tasks.status-failed",
            DownloadStatus::Cancelled => "download-tasks.status-cancelled",
        }
    }

    /// 检查两个状态是否匹配（用于筛选）
    pub fn matches(&self, other: &DownloadStatus) -> bool {
        match (self, other) {
            (DownloadStatus::Failed(_), DownloadStatus::Failed(_)) => true,
            _ => self == other,
        }
    }
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
    /// 任务创建时间
    pub created_at: chrono::DateTime<chrono::Local>,
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
            created_at: chrono::Local::now(),
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
    /// 数据库实例
    pub database: Option<crate::ui::download::database::DownloadDatabase>,
    /// 状态筛选：None表示显示所有状态，Some表示筛选特定状态
    pub status_filter: Option<DownloadStatus>,
    /// 状态筛选下拉框展开状态
    pub status_filter_expanded: bool,
    /// 排序列：None表示未排序，Some表示按指定列排序
    pub sort_column: Option<SortColumn>,
    /// 排序方向：true表示降序，false表示升序
    pub sort_descending: bool,
    /// 是否正在排序
    pub is_sorting: bool,
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
            database: None,
            status_filter: None,
            status_filter_expanded: false,
            sort_column: Some(SortColumn::CreatedAt), // 默认按添加时间排序
            sort_descending: true, // 默认降序
            is_sorting: false,
        }
    }
}

/// 排序列枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    /// 文件名
    FileName,
    /// 大小
    Size,
    /// 状态
    Status,
    /// 添加时间
    CreatedAt,
}

impl SortColumn {
    /// 获取列对应的翻译key
    pub fn get_translation_key(&self) -> &'static str {
        match self {
            SortColumn::FileName => "download-tasks.header-filename",
            SortColumn::Size => "download-tasks.header-size",
            SortColumn::Status => "download-tasks.header-status",
            SortColumn::CreatedAt => "download-tasks.header-created-at",
        }
    }
}
