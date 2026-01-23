use super::App;
use super::AppMessage;
use super::download::DownloadMessage;
use std::path::PathBuf;
use tracing::error;
use tracing::info;

impl App {
    /// 辅助方法：开始下载壁纸（支持并行限制和进度更新）
    pub fn start_download(&mut self, url: String, id: &str, file_type: &str) -> iced::Task<AppMessage> {
        let file_name = super::download::generate_file_name(id, file_type.split('/').last().unwrap_or("jpg"));
        let data_path = self.config.data.data_path.clone();
        let cache_path = self.config.data.cache_path.clone();
        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };
        let file_type = file_type.split('/').last().unwrap_or("jpg").to_string();

        // 生成完整保存路径
        let full_save_path = std::path::PathBuf::from(&data_path).join(&file_name);

        // 添加任务（倒序排列）
        self.download_state.add_task(
            url.clone(),
            full_save_path.to_string_lossy().to_string(),
            file_name.clone(),
            proxy.clone(),
            file_type.clone(),
        );

        // 获取任务ID
        let task_id = self.download_state.next_id.saturating_sub(1);

        if self.download_state.can_start_download() {
            // 可以开始下载 - 使用索引查找任务
            let task_index = self.download_state.find_task_index(task_id);
            if let Some(index) = task_index {
                let task_full = self.download_state.get_task_by_index(index);
                if let Some(task_full) = task_full {
                    // 先保存所有需要的数据，再修改状态
                    let url = task_full.task.url.clone();
                    let save_path = std::path::PathBuf::from(&task_full.task.save_path);
                    let proxy = task_full.proxy.clone();
                    let task_id = task_full.task.id;
                    let cancel_token = task_full.task.cancel_token.clone().unwrap();
                    let downloaded_size = task_full.task.downloaded_size;
                    let total_size = task_full.task.total_size;
                    let cache_path = cache_path.clone();

                    // 更新状态
                    task_full.task.status = super::download::DownloadStatus::Downloading;
                    task_full.task.start_time = Some(std::time::Instant::now());
                    self.download_state.increment_downloading();

                    // 启动异步下载任务（带进度更新）
                    return iced::Task::perform(
                        super::async_tasks::async_download_wallpaper_task_with_progress(
                            url,
                            save_path,
                            proxy,
                            task_id,
                            cancel_token,
                            downloaded_size,
                            total_size,
                            cache_path,
                        ),
                        move |result| match result {
                            Ok(size) => {
                                info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                            }
                            Err(e) => {
                                error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                            }
                        },
                    );
                }
            }
        }

        // 显示通知
        iced::Task::done(AppMessage::ShowNotification(
            format!("已添加到下载队列 (等待中)"),
            super::NotificationType::Success,
        ))
    }

    /// 处理下载相关消息
    pub fn handle_download_message(&mut self, msg: DownloadMessage) -> iced::Task<AppMessage> {
        match msg {
            DownloadMessage::AddTask(url, save_path, file_name, _proxy, file_type) => {
                self.handle_add_download_task_with_save_path(url, save_path, file_name, file_type)
            }
            DownloadMessage::PauseTask(id) => self.handle_pause_download_task(id),
            DownloadMessage::ResumeTask(id) => self.handle_resume_download_task(id),
            DownloadMessage::RetryTask(id) => self.handle_retry_download_task(id),
            DownloadMessage::CancelTask(id) => self.handle_cancel_download_task(id),
            DownloadMessage::DeleteTask(id) => self.handle_delete_download_task(id),
            DownloadMessage::OpenFileLocation(id) => self.handle_open_file_location(id),
            DownloadMessage::ClearCompleted => self.handle_clear_completed_downloads(),
            DownloadMessage::DownloadCompleted(id, size, error) => self.handle_download_completed(id, size, error),
            DownloadMessage::DownloadProgress(id, downloaded, total, speed) => self.handle_download_progress(id, downloaded, total, speed),
            DownloadMessage::SimulateProgress => self.handle_simulate_progress(),
            DownloadMessage::UpdateSpeed => self.handle_update_speed(),
            DownloadMessage::CopyDownloadLink(id) => self.handle_copy_download_link(id),
            DownloadMessage::SetAsWallpaper(id) => self.handle_set_as_wallpaper(id),
        }
    }

    fn handle_add_download_task_with_save_path(&mut self, url: String, save_path: String, file_name: String, file_type: String) -> iced::Task<AppMessage> {
        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };

        // 合并目录和文件名生成完整路径
        let full_save_path = PathBuf::from(&save_path).join(&file_name);

        // 添加任务（使用完整路径）
        let full_path_str = full_save_path.to_string_lossy().to_string();
        self.download_state
            .add_task(url.clone(), full_path_str.clone(), file_name.clone(), proxy.clone(), file_type.clone());

        // 获取新添加的任务ID
        let task_id = self.download_state.next_id.saturating_sub(1);

        // 更新状态为下载中并启动下载
        match self.download_state.get_task(task_id) {
            Some(task_full) => {
                task_full.task.status = super::download::DownloadStatus::Downloading;
                task_full.task.start_time = Some(std::time::Instant::now());

                let url = task_full.task.url.clone();
                let save_path = PathBuf::from(&task_full.task.save_path);
                let proxy = task_full.proxy.clone();
                let task_id = task_full.task.id;

                return iced::Task::perform(
                    super::async_tasks::async_download_wallpaper_task(url, save_path, proxy, task_id),
                    move |result| match result {
                        Ok(size) => {
                            info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                            AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                        }
                        Err(e) => {
                            error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                            AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                        }
                    },
                );
            }
            None => {}
        }
        iced::Task::none()
    }

    fn handle_pause_download_task(&mut self, id: usize) -> iced::Task<AppMessage> {
        // 记录暂停时的下载进度
        if let Some(index) = self.download_state.find_task_index(id) {
            if let Some(task) = self.download_state.get_task_by_index(index) {
                info!(
                    "[下载任务] [ID:{}] 暂停：total_size = {} bytes, downloaded_size = {} bytes",
                    id, task.task.total_size, task.task.downloaded_size
                );

                // 注意：不读取缓存文件大小，因为异步写入可能还没刷新到磁盘
                // 直接使用任务中记录的 downloaded_size 即可
            }
        }

        // 然后设置状态为暂停
        self.download_state.update_status(id, super::download::DownloadStatus::Paused);

        // 最后设置取消标志，终止下载
        self.download_state.cancel_task(id);

        // 注意：不删除已下载的缓存文件，保留以便断点续传
        iced::Task::none()
    }

    fn handle_resume_download_task(&mut self, id: usize) -> iced::Task<AppMessage> {
        // 使用索引查找任务
        // 先检查是否可以开始下载并保存所有需要的数据
        let can_start = self.download_state.can_start_download();
        let current_status = self.download_state.tasks.iter().find(|t| t.task.id == id).map(|t| t.task.status.clone());

        let task_data = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| (t.task.url.clone(), PathBuf::from(&t.task.save_path), t.proxy.clone(), t.task.id));

        if let Some((url, save_path, proxy, task_id)) = task_data {
            if current_status == Some(super::download::DownloadStatus::Waiting)
                || current_status == Some(super::download::DownloadStatus::Paused)
                || current_status == Some(super::download::DownloadStatus::Cancelled)
                || matches!(current_status, Some(super::download::DownloadStatus::Failed(_)))
            {
                if can_start {
                    // 更新状态为下载中
                    let should_reset = current_status == Some(super::download::DownloadStatus::Cancelled)
                        || matches!(current_status, Some(super::download::DownloadStatus::Failed(_)));
                    if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
                        task_full.task.status = super::download::DownloadStatus::Downloading;
                        task_full.task.start_time = Some(std::time::Instant::now());

                        // 重置取消令牌
                        if let Some(cancel_token) = &task_full.task.cancel_token {
                            cancel_token.store(false, std::sync::atomic::Ordering::Relaxed);
                        }

                        // 如果任务已取消或失败，重置已下载大小和进度
                        if should_reset {
                            task_full.task.downloaded_size = 0;
                            task_full.task.progress = 0.0;
                            task_full.task.speed = 0;

                            // 清空已下载的文件
                            let _ = std::fs::remove_file(&task_full.task.save_path);
                        }
                    }

                    // 获取取消令牌、文件总大小
                    let (cancel_token, total_size) = if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == task_id) {
                        (task.task.cancel_token.clone().unwrap(), task.task.total_size)
                    } else {
                        (std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), 0)
                    };

                    // 读取实际文件大小作为下载偏移量
                    let cache_path = self.config.data.cache_path.clone();
                    let actual_file_size = if total_size > 0 {
                        if let Ok(cache_file_path) = crate::services::download::DownloadService::get_online_image_cache_path(&cache_path, &url, total_size) {
                            if let Ok(metadata) = std::fs::metadata(&cache_file_path) {
                                let size = metadata.len();
                                // 减去 1KB 作为安全边界
                                let safe_size = if size > 1024 { size - 1024 } else { 0 };
                                info!(
                                    "[下载任务] [ID:{}] 恢复：实际文件大小 = {} bytes, 安全偏移量 = {} bytes",
                                    task_id, size, safe_size
                                );
                                safe_size
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    // 更新任务的 downloaded_size
                    if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
                        task_full.task.downloaded_size = actual_file_size;
                        // 更新进度
                        if total_size > 0 {
                            task_full.task.progress = actual_file_size as f32 / total_size as f32;
                        }
                    }
                    info!(
                        "[下载任务] [ID:{}] 恢复：使用偏移量 = {} bytes, total_size = {} bytes",
                        task_id, actual_file_size, total_size
                    );

                    self.download_state.increment_downloading();
                    return iced::Task::perform(
                        super::async_tasks::async_download_wallpaper_task_with_progress(
                            url,
                            save_path,
                            proxy,
                            task_id,
                            cancel_token,
                            actual_file_size,
                            total_size,
                            cache_path,
                        ),
                        move |result| match result {
                            Ok(size) => {
                                info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                            }

                            Err(e) => {
                                error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                            }
                        },
                    );
                }
            }
        }
        iced::Task::none()
    }

    fn handle_retry_download_task(&mut self, id: usize) -> iced::Task<AppMessage> {
        // 重新下载：清空已下载文件，从头开始下载
        // 先检查是否可以开始下载并保存所有需要的数据
        let can_start = self.download_state.can_start_download();
        let task_data = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| (t.task.url.clone(), PathBuf::from(&t.task.save_path), t.proxy.clone(), t.task.id));

        if let Some((url, save_path, proxy, task_id)) = task_data {
            if can_start {
                if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
                    // 重置任务状态和进度
                    task_full.task.status = super::download::DownloadStatus::Downloading;
                    task_full.task.start_time = Some(std::time::Instant::now());
                    task_full.task.downloaded_size = 0;
                    task_full.task.progress = 0.0;
                    task_full.task.speed = 0;

                    // 重置取消令牌
                    if let Some(cancel_token) = &task_full.task.cancel_token {
                        cancel_token.store(false, std::sync::atomic::Ordering::Relaxed);
                    }

                    // 清空已下载的文件（data_path中的文件）
                    let _ = std::fs::remove_file(&task_full.task.save_path);
                    info!("[下载任务] [ID:{}] 重新下载：已清空文件: {}", task_id, task_full.task.save_path);

                    // 清空缓存文件（cache_path/online中的文件）
                    let cache_path = self.config.data.cache_path.clone();
                    if let Ok(cache_file_path) = crate::services::download::DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                        let _ = std::fs::remove_file(&cache_file_path);
                        info!("[下载任务] [ID:{}] 重新下载：已清空缓存文件: {}", task_id, cache_file_path);
                    }

                    // 清空最终缓存文件（不带.download后缀的文件）
                    if let Ok(final_cache_path) = crate::services::download::DownloadService::get_online_image_cache_final_path(&cache_path, &url, 0) {
                        let _ = std::fs::remove_file(&final_cache_path);
                        info!("[下载任务] [ID:{}] 重新下载：已清空最终缓存文件: {}", task_id, final_cache_path);
                    }
                }

                self.download_state.increment_downloading();

                // 获取取消令牌和文件总大小（已下载大小为0，因为要重新下载）
                let (cancel_token, total_size) = if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == task_id) {
                    (task.task.cancel_token.clone().unwrap(), task.task.total_size)
                } else {
                    (std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), 0)
                };

                let cache_path = self.config.data.cache_path.clone();
                return iced::Task::perform(
                    super::async_tasks::async_download_wallpaper_task_with_progress(
                        url,
                        save_path,
                        proxy,
                        task_id,
                        cancel_token,
                        0,          // 重新下载，从0开始
                        total_size, // 保留文件总大小，用于缓存路径计算
                        cache_path,
                    ),
                    move |result| match result {
                        Ok(size) => {
                            info!("[下载任务] [ID:{}] 重新下载成功, 文件大小: {} bytes", task_id, size);
                            AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                        }
                        Err(e) => {
                            error!("[下载任务] [ID:{}] 重新下载失败: {}", task_id, e);
                            AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                        }
                    },
                );
            }
        }

        iced::Task::none()
    }

    fn handle_cancel_download_task(&mut self, id: usize) -> iced::Task<AppMessage> {
        // 先保存任务信息，因为取消后可能无法访问
        let task_info = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| (t.task.url.clone(), t.task.save_path.clone(), t.task.file_name.clone(), t.task.status.clone()));

        // 取消任务
        self.download_state.cancel_task(id);
        // 将任务状态设置为已取消
        self.download_state.update_status(id, crate::ui::download::DownloadStatus::Cancelled);

        // 清除未完成的下载文件
        if let Some((url, save_path, _file_name, status)) = task_info {
            // 只有在下载中、等待中或暂停时才清除文件
            if status == crate::ui::download::DownloadStatus::Downloading
                || status == crate::ui::download::DownloadStatus::Waiting
                || status == crate::ui::download::DownloadStatus::Paused
            {
                // 1. 删除目标文件（data_path中的文件）
                if let Ok(_metadata) = std::fs::metadata(&save_path) {
                    // 只有文件未完全下载时才删除（通过检查文件大小是否与完整大小匹配）
                    // 这里我们简单处理：只要状态不是Completed就删除
                    let _ = std::fs::remove_file(&save_path);
                    info!("[下载任务] [ID:{}] 已删除未完成的目标文件: {}", id, save_path);
                }

                // 2. 删除缓存文件（cache_path/online中的文件）
                // 需要计算缓存文件路径
                let cache_path = self.config.data.cache_path.clone();
                if let Ok(cache_file_path) = crate::services::download::DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                    if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                        let _ = std::fs::remove_file(&cache_file_path);
                        info!("[下载任务] [ID:{}] 已删除未完成的缓存文件: {}", id, cache_file_path);
                    }
                }

                // 3. 删除缓存文件（不带.download后缀的最终文件）
                if let Ok(final_cache_path) = crate::services::download::DownloadService::get_online_image_cache_final_path(&cache_path, &url, 0) {
                    if let Ok(_metadata) = std::fs::metadata(&final_cache_path) {
                        let _ = std::fs::remove_file(&final_cache_path);
                        info!("[下载任务] [ID:{}] 已删除未完成的最终缓存文件: {}", id, final_cache_path);
                    }
                }
            }
        }

        iced::Task::none()
    }

    fn handle_delete_download_task(&mut self, id: usize) -> iced::Task<AppMessage> {
        // 仅删除任务记录，不删除文件
        // 因为文件已经下载完成，用户可能需要保留文件
        self.download_state.remove_task(id);
        iced::Task::none()
    }

    fn handle_open_file_location(&mut self, id: usize) -> iced::Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let full_path = super::common::get_absolute_path(&task.task.save_path);

            // Windows: 使用 explorer /select,路径
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("explorer").args(["/select,", &full_path]).spawn();
            }
            // macOS: 使用 open -R 路径
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("open").args(["-R", &full_path]).spawn();
            }
            // Linux: 使用 xdg-open 路径
            #[cfg(target_os = "linux")]
            {
                let _ = std::process::Command::new("xdg-open").arg(&full_path).spawn();
            }
        }

        iced::Task::none()
    }

    fn handle_clear_completed_downloads(&mut self) -> iced::Task<AppMessage> {
        self.download_state.clear_completed();
        iced::Task::none()
    }

    fn handle_download_completed(&mut self, id: usize, size: u64, error: Option<String>) -> iced::Task<AppMessage> {
        let task_index = self.download_state.find_task_index(id);
        if let Some(index) = task_index {
            if let Some(task) = self.download_state.get_task_by_index(index) {
                // 检查当前状态
                let current_status = task.task.status.clone();

                if current_status == super::download::DownloadStatus::Paused {
                    // 任务已暂停，保持暂停状态
                } else if error.is_some() {
                    // 下载失败
                    let error_msg = error.unwrap();
                    // 检查是否是用户取消
                    if error_msg == "下载已取消" {
                        // 检查任务是否在暂停状态被取消
                        // 如果任务原本是暂停状态，则保持暂停，否则设置为已取消
                        // 如果不是暂停状态，设置为已取消
                        if current_status != super::download::DownloadStatus::Paused {
                            task.task.status = super::download::DownloadStatus::Cancelled;
                        }
                    } else {
                        task.task.status = super::download::DownloadStatus::Failed(error_msg.clone());

                        // 清除未完成的下载文件
                        let url = task.task.url.clone();
                        let save_path = task.task.save_path.clone();

                        // 1. 删除目标文件（data_path中的文件）
                        if let Ok(_metadata) = std::fs::metadata(&save_path) {
                            let _ = std::fs::remove_file(&save_path);
                            info!("[下载任务] [ID:{}] 已删除未完成的目标文件: {}", id, save_path);
                        }

                        // 2. 删除缓存文件（cache_path/online中的.download文件）
                        let cache_path = self.config.data.cache_path.clone();
                        if let Ok(cache_file_path) = crate::services::download::DownloadService::get_online_image_cache_path(&cache_path, &url, size) {
                            if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                                let _ = std::fs::remove_file(&cache_file_path);
                                info!("[下载任务] [ID:{}] 已删除未完成的缓存文件: {}", id, cache_file_path);
                            }
                        }

                        // 3. 删除最终缓存文件（不带.download后缀的文件）
                        if let Ok(final_cache_path) = crate::services::download::DownloadService::get_online_image_cache_final_path(&cache_path, &url, size) {
                            if let Ok(_metadata) = std::fs::metadata(&final_cache_path) {
                                let _ = std::fs::remove_file(&final_cache_path);
                                info!("[下载任务] [ID:{}] 已删除未完成的最终缓存文件: {}", id, final_cache_path);
                            }
                        }
                    }
                } else {
                    // 下载成功
                    // 验证实际文件大小
                    let actual_size = if let Ok(metadata) = std::fs::metadata(&task.task.save_path) {
                        metadata.len()
                    } else {
                        size
                    };

                    task.task.status = super::download::DownloadStatus::Completed;
                    task.task.progress = 1.0;
                    task.task.total_size = actual_size;
                    task.task.downloaded_size = actual_size;

                    // 检查是否需要自动设置壁纸
                    let file_name = std::path::Path::new(&task.task.save_path).file_name().and_then(|n| n.to_str()).unwrap_or("");

                    if let Some(pending_filename) = self.online_state.pending_set_wallpaper_filename.as_ref() {
                        if pending_filename == file_name {
                            // 当前下载的文件是待设置壁纸的文件，自动设置壁纸
                            let full_path = super::common::get_absolute_path(&task.task.save_path);
                            let wallpaper_mode = self.config.wallpaper.mode;
                            let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                            // 清除待设置壁纸的文件名
                            self.online_state.pending_set_wallpaper_filename = None;

                            // 异步设置壁纸
                            return iced::Task::perform(
                                super::async_tasks::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                                move |result| match result {
                                    Ok(_) => AppMessage::AddToWallpaperHistory(full_path),
                                    Err(e) => AppMessage::ShowNotification(format!("{}: {}", failed_message, e), super::NotificationType::Error),
                                },
                            );
                        }
                    }
                }
            }
        }

        // 减少正在下载的任务计数
        self.download_state.decrement_downloading();

        // 检查是否有等待中的任务需要开始
        if let Some(next_task) = self.download_state.get_next_waiting_task() {
            let _next_id = next_task.task.id;
            let next_url = next_task.task.url.clone();
            let next_save_path = PathBuf::from(&next_task.task.save_path);
            let next_proxy = next_task.proxy.clone();
            let next_task_id = next_task.task.id;
            let next_cancel_token = next_task.task.cancel_token.clone().unwrap();
            let next_downloaded_size = next_task.task.downloaded_size;
            let next_total_size = next_task.task.total_size;

            next_task.task.status = super::download::DownloadStatus::Downloading;
            next_task.task.start_time = Some(std::time::Instant::now());
            self.download_state.increment_downloading();

            let cache_path = self.config.data.cache_path.clone();

            // 启动下一个下载任务
            return iced::Task::perform(
                super::async_tasks::async_download_wallpaper_task_with_progress(
                    next_url,
                    next_save_path,
                    next_proxy,
                    next_task_id,
                    next_cancel_token,
                    next_downloaded_size,
                    next_total_size,
                    cache_path,
                ),
                move |result| match result {
                    Ok(s) => {
                        info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", next_task_id, s);
                        AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(next_task_id, s, None))
                    }
                    Err(e) => {
                        error!("[下载任务] [ID:{}] 下载失败: {}", next_task_id, e);
                        AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(next_task_id, 0, Some(e)))
                    }
                },
            );
        }

        iced::Task::none()
    }

    fn handle_download_progress(&mut self, id: usize, downloaded: u64, total: u64, speed: u64) -> iced::Task<AppMessage> {
        self.download_state.update_progress(id, downloaded, total, speed);
        iced::Task::none()
    }

    fn handle_simulate_progress(&mut self) -> iced::Task<AppMessage> {
        // 模拟进度更新（测试用）
        for task in self.download_state.tasks.iter_mut() {
            if task.task.status == super::download::DownloadStatus::Downloading {
                let increment = (task.task.total_size as f32 * 0.01).max(1024.0) as u64;
                task.task.downloaded_size = (task.task.downloaded_size + increment).min(task.task.total_size);
                if task.task.total_size > 0 {
                    task.task.progress = task.task.downloaded_size as f32 / task.task.total_size as f32;
                }
                if task.task.downloaded_size >= task.task.total_size {
                    task.task.status = super::download::DownloadStatus::Completed;
                }
            }
        }
        iced::Task::none()
    }

    fn handle_update_speed(&mut self) -> iced::Task<AppMessage> {
        self.download_state.update_speed();
        iced::Task::none()
    }

    fn handle_copy_download_link(&mut self, id: usize) -> iced::Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let url = task.task.url.clone();
            let success_message = self.i18n.t("download-tasks.copy-link-success").to_string();
            let failed_message = self.i18n.t("download-tasks.copy-link-failed").to_string();

            // 异步复制到剪贴板
            return iced::Task::perform(
                async move {
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        let result = Command::new("cmd").args(["/c", "echo", &url, "|", "clip"]).output();
                        match result {
                            Ok(_) => Ok(()),
                            Err(_) => Err("复制失败".to_string()),
                        }
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        use std::process::Command;
                        let result = Command::new("xclip").args(["-selection", "clipboard"]).write_stdin(url.as_bytes()).output();
                        match result {
                            Ok(_) => Ok(()),
                            Err(_) => Err("复制失败".to_string()),
                        }
                    }
                },
                move |result| match result {
                    Ok(_) => AppMessage::ShowNotification(success_message, super::NotificationType::Success),
                    Err(e) => AppMessage::ShowNotification(format!("{}: {}", failed_message, e), super::NotificationType::Error),
                },
            );
        }
        iced::Task::none()
    }

    fn handle_set_as_wallpaper(&mut self, id: usize) -> iced::Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let path = task.task.save_path.clone();
            let full_path = super::common::get_absolute_path(&path);
            let wallpaper_mode = self.config.wallpaper.mode;

            // 检查文件是否存在
            if std::path::Path::new(&full_path).exists() {
                // 提前获取翻译文本，避免线程安全问题
                let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                // 异步设置壁纸
                return iced::Task::perform(
                    super::async_tasks::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                    move |result| match result {
                        Ok(_) => AppMessage::AddToWallpaperHistory(full_path),
                        Err(e) => AppMessage::ShowNotification(format!("{}: {}", failed_message, e), super::NotificationType::Error),
                    },
                );
            } else {
                let error_message = self.i18n.t("download-tasks.set-wallpaper-file-not-found").to_string();
                return iced::Task::done(AppMessage::ShowNotification(error_message, super::NotificationType::Error));
            }
        }
        iced::Task::none()
    }
}
