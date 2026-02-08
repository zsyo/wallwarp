// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理消息模块
//!
//! 定义下载页面的消息类型

use crate::ui::{App, AppMessage};

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
    /// 展开/收起状态筛选下拉框
    ToggleStatusFilter,
    /// 设置状态筛选（None表示显示所有状态）
    SetStatusFilter(Option<crate::ui::download::state::DownloadStatus>),
}

impl From<DownloadMessage> for AppMessage {
    fn from(msg: DownloadMessage) -> AppMessage {
        AppMessage::Download(msg)
    }
}

impl App {
    /// 处理下载相关消息
    pub fn handle_download_message(&mut self, msg: DownloadMessage) -> iced::Task<AppMessage> {
        match msg {
            DownloadMessage::AddTask(url, save_path, file_name, _proxy, file_type) => {
                self.add_download_task(url, save_path, file_name, file_type)
            }
            DownloadMessage::PauseTask(id) => self.pause_download_task(id),
            DownloadMessage::ResumeTask(id) => self.resume_download_task(id),
            DownloadMessage::RetryTask(id) => self.retry_download_task(id),
            DownloadMessage::CancelTask(id) => self.cancel_download_task(id),
            DownloadMessage::DeleteTask(id) => self.delete_download_task(id),
            DownloadMessage::OpenFileLocation(id) => self.view_downloaded_file(id),
            DownloadMessage::ClearCompleted => self.clear_download_completed_tasks(),
            DownloadMessage::DownloadCompleted(id, size, error) => self.download_completed(id, size, error),
            DownloadMessage::DownloadProgress(id, downloaded, total, speed) => {
                self.update_download_progress(id, downloaded, total, speed)
            }
            DownloadMessage::UpdateSpeed => self.update_download_speed(),
            DownloadMessage::CopyDownloadLink(id) => self.copy_download_link(id),
            DownloadMessage::SetAsWallpaper(id) => self.set_downloaded_as_wallpaper(id),
            DownloadMessage::ToggleStatusFilter => {
                self.download_state.status_filter_expanded = !self.download_state.status_filter_expanded;
                iced::Task::none()
            }
            DownloadMessage::SetStatusFilter(filter) => {
                self.download_state.status_filter = filter;
                self.download_state.status_filter_expanded = false;
                iced::Task::none()
            }
        }
    }
}
