// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载页手动添加任务：输入 URL 后直接创建下载任务
//!
//! 下载服务本身支持任意 URL，此处补上手动入口

use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;

impl App {
    /// 提交手动添加的任务
    pub(in crate::ui::download) fn manual_url_submitted(&mut self) -> Task<AppMessage> {
        let url = self.download_state.manual_url.trim().to_string();
        if url.is_empty() {
            return Task::none();
        }

        // 仅支持 http/https 链接
        if !url.starts_with("http://") && !url.starts_with("https://") {
            let error_message = self.i18n.t("download-tasks.invalid-url").to_string();
            return self.show_notification(error_message, NotificationType::Error);
        }

        // 从 URL 提取文件名与类型，保存到壁纸库目录
        let (file_name, file_type) = derive_file_name_from_url(&url);
        let save_path = self.config.data.data_path.clone();

        // 壁纸库将新增文件，本地页列表缓存失效
        self.local_state.loaded_data_path = None;

        // 创建任务（未满并发上限时立即开始，否则进入排队）并清空输入框
        let add_task = self.add_download_task(url, save_path, file_name, file_type);
        self.download_state.manual_url.clear();

        let queued_message = self
            .i18n
            .t("download-tasks.added-to-download-queue")
            .to_string();
        Task::batch([
            add_task,
            self.show_notification(queued_message, NotificationType::Info),
        ])
    }
}

/// 从 URL 推导文件名与 MIME 类型
///
/// 取 URL 路径的最后一段（去掉查询参数与锚点）作为文件名；
/// 无可用扩展名时用时间戳生成，扩展名默认 jpg
fn derive_file_name_from_url(url: &str) -> (String, String) {
    let path = url.split(['?', '#']).next().unwrap_or(url);
    let last_segment = path.rsplit('/').find(|s| !s.is_empty()).unwrap_or("");

    let extension = last_segment
        .rsplit_once('.')
        .map(|(_, ext)| ext.to_lowercase())
        .filter(|ext| (2..=5).contains(&ext.len()))
        .unwrap_or_else(|| "jpg".to_string());

    let stem = match last_segment.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && (2..=5).contains(&ext.len()) => stem.to_string(),
        _ => chrono::Local::now().format("wallpaper_%Y%m%d_%H%M%S%3f").to_string(),
    };

    let file_type = match extension.as_str() {
        "png" => "image/png",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        _ => "image/jpeg",
    }
    .to_string();

    (format!("{stem}.{extension}"), file_type)
}
