// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理消息模块
//!
//! 定义下载页面的消息类型

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