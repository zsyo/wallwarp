// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn download_completed(
        &mut self,
        id: usize,
        size: u64,
        error: Option<String>,
        generation: u64,
    ) -> Task<AppMessage> {
        // 下载完成后需要自动设为壁纸的目标路径（状态收尾后再执行）
        let mut pending_wallpaper_path: Option<String> = None;
        // 收藏夹页发起的"下载后自动设壁纸"是否命中（文件名匹配）
        let mut favorite_pending_hit = false;
        // 当前完成事件是否负责释放并发槽位（仅当任务仍处于"下载中"时）
        let release_slot;
        let task_index = self.download_state.find_task_index(id);
        if let Some(index) = task_index {
            if let Some(task) = self.download_state.get_task_by_index(index) {
                // 过期完成事件（旧一轮下载循环）：直接忽略，避免覆盖新一轮下载的状态
                if task.task.generation != generation {
                    tracing::debug!(
                        "[下载任务] [ID:{}] 忽略过期的完成事件：事件代数 {}，任务当前代数 {}",
                        id,
                        generation,
                        task.task.generation
                    );
                    return Task::none();
                }

                // 检查当前状态
                let current_status = task.task.status.clone();
                // 仅当完成事件本身把任务从"下载中"切走时才释放并发槽位；
                // 暂停/取消/删除路径已提前释放，避免重复扣减
                release_slot = current_status == DownloadStatus::Downloading;

                if current_status == DownloadStatus::Paused {
                    // 任务已暂停，保持暂停状态
                } else if error.is_some() {
                    // 下载失败
                    let error_msg = error.unwrap();
                    // 检查是否是用户取消
                    if error_msg == crate::services::download::DOWNLOAD_CANCELLED {
                        // 检查任务是否在暂停状态被取消
                        // 如果任务原本是暂停状态，则保持暂停，否则设置为已取消
                        // 如果不是暂停状态，设置为已取消
                        if current_status != DownloadStatus::Paused {
                            task.task.status = DownloadStatus::Cancelled;
                        }
                    } else {
                        // 失败状态的临时文件清理在下载任务错误路径中完成
                        // (那里持有真实临时路径,此处无法可靠定位)
                        task.task.status = DownloadStatus::Failed(error_msg.clone());
                    }
                } else {
                    // 下载成功
                    // 验证实际文件大小
                    let actual_size = if let Ok(metadata) = std::fs::metadata(&task.task.save_path)
                    {
                        metadata.len()
                    } else {
                        size
                    };

                    task.task.status = DownloadStatus::Completed;
                    task.task.progress = 1.0;
                    task.task.total_size = actual_size;
                    task.task.downloaded_size = actual_size;
                    // 壁纸库新增了文件，本地页列表缓存失效
                    self.local_state.loaded_data_path = None;

                    // 检查是否需要自动设置壁纸
                    let file_name = std::path::Path::new(&task.task.save_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");

                    if let Some(pending_filename) =
                        self.online_state.pending_set_wallpaper_filename.as_ref()
                        && pending_filename == file_name
                    {
                        // 当前下载的文件是待设置壁纸的文件，自动设置壁纸
                        // 清除待设置壁纸的文件名
                        self.online_state.pending_set_wallpaper_filename = None;
                        pending_wallpaper_path = Some(crate::utils::helpers::get_absolute_path(
                            &task.task.save_path,
                        ));
                    }

                    // 收藏夹页"下载后自动设壁纸"匹配（同一文件只会命中一处 pending）
                    if pending_wallpaper_path.is_none()
                        && let Some(pending_filename) =
                            self.favorites_state.pending_apply_filename.as_ref()
                        && pending_filename == file_name
                    {
                        self.favorites_state.pending_apply_filename = None;
                        pending_wallpaper_path = Some(crate::utils::helpers::get_absolute_path(
                            &task.task.save_path,
                        ));
                        favorite_pending_hit = true;
                    }

                    // 收藏夹在线项按文件名匹配：文件已落库，原地刷新 in_library
                    // （卡片/模态按钮即时切换，不重跑筛选避免缩略图重置）
                    self.favorites_state
                        .mark_online_in_library_by_file_name(file_name);
                }

                // 保存状态到数据库（在状态修改完成后）
                if let Some(task_full) = self.download_state.tasks.get(index) {
                    let _ = self.download_state.save_to_database(task_full);
                }
            } else {
                // 任务已被删除，槽位由删除路径负责释放
                return Task::none();
            }
        } else {
            // 任务不存在（已被删除），槽位由删除路径负责释放
            return Task::none();
        }

        // 释放正在下载的任务计数
        if release_slot {
            self.download_state.decrement_downloading();
        }

        // 下载完成的文件被标记为自动设为壁纸：跳过自动启动下一个任务
        if let Some(full_path) = pending_wallpaper_path {
            let apply_task = self.apply_wallpaper(full_path);
            if favorite_pending_hit {
                // 收藏夹来源：顺带刷新收藏项的 in_library 标记（下次进入页面时重载）
                self.favorites_state.loaded = false;
            }
            return apply_task;
        }

        // 检查是否有等待中的任务需要开始
        let next_waiting = self
            .download_state
            .get_next_waiting_task()
            .map(|next| (next.task.id, next.task.downloaded_size));
        if let Some((next_task_id, next_downloaded_size)) = next_waiting {
            return self.spawn_download(next_task_id, next_downloaded_size, Task::none());
        }
        Task::none()
    }
}
