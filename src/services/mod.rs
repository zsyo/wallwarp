// Copyright (C) 2026 zsyo - GNU AGPL v3.0

pub mod async_task;
pub mod database;
pub mod download;
pub mod local;
pub mod proxy;
pub mod request_context;
pub mod wallhaven;

use std::sync::Arc;
use tokio::sync::Semaphore;

/// 全局网络请求并发控制器
/// 使用Arc<Semaphore>实现线程安全的并发限制
pub struct ConcurrencyController {
    semaphore: Arc<Semaphore>,
}

impl ConcurrencyController {
    /// 创建新的并发控制器
    /// max_concurrent: 最大并发数
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// 获取信号量许可
    /// 返回一个Permit，当Permit被drop时会自动释放许可
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore.acquire().await.unwrap()
    }

    /// 获取Arc引用，用于在多处共享同一个控制器
    pub fn arc(&self) -> Arc<Semaphore> {
        Arc::clone(&self.semaphore)
    }
}

impl Default for ConcurrencyController {
    fn default() -> Self {
        Self::new(5) // 默认最大并发数为5
    }
}

/// 全局并发控制器实例
/// 使用LazyLock实现线程安全的延迟初始化
pub static GLOBAL_CONCURRENCY_CONTROLLER: std::sync::LazyLock<ConcurrencyController> =
    std::sync::LazyLock::new(|| ConcurrencyController::new(5));

/// 下载进度更新消息
#[derive(Debug, Clone)]
pub struct DownloadProgressUpdate {
    pub task_id: usize,
    pub downloaded: u64,
    pub total: u64,
    pub speed: u64,
}

/// 全局下载进度channel发送器
pub static DOWNLOAD_PROGRESS_TX: std::sync::OnceLock<tokio::sync::broadcast::Sender<DownloadProgressUpdate>> =
    std::sync::OnceLock::new();

/// 初始化全局下载进度channel
pub fn init_download_progress_channel() {
    DOWNLOAD_PROGRESS_TX.get_or_init(|| {
        let (tx, _rx) = tokio::sync::broadcast::channel(100);
        tx
    });
}

/// 发送下载进度更新
pub fn send_download_progress(task_id: usize, downloaded: u64, total: u64, speed: u64) {
    if let Some(tx) = DOWNLOAD_PROGRESS_TX.get() {
        let update = DownloadProgressUpdate {
            task_id,
            downloaded,
            total,
            speed,
        };
        let _ = tx.send(update);
    }
}
