// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::DownloadStateFull;
use super::DownloadStatus;
use super::DownloadTask;
use super::DownloadTaskFull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

impl DownloadStateFull {
    /// 初始化HTTP客户端
    pub fn init_client(&mut self) {
        if self.client.is_none() {
            self.client = Some(reqwest::Client::new());
        }
    }

    /// 初始化数据库
    ///
    /// # 参数
    /// - `db_path`: 数据库文件路径
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn init_database(&mut self, db_path: &str) -> Result<(), String> {
        let db = crate::ui::download::database::DownloadDatabase::open(db_path)?;
        self.database = Some(db);
        Ok(())
    }

    /// 从数据库加载所有任务
    ///
    /// # 返回
    /// 成功返回加载的任务数量，失败返回错误信息
    pub fn load_from_database(&mut self) -> Result<usize, String> {
        if let Some(db) = &self.database {
            let tasks_db = db.load_all_tasks()?;

            // 转换为内存中的任务格式
            self.tasks.clear();
            self.next_id = 0;

            for task_db in tasks_db {
                // 解析状态
                let mut status = Self::parse_status(&task_db.status);

                // 程序启动时，所有未完成（非 Completed、Cancelled和Failed）的任务都应该处于暂停状态
                let is_completed = matches!(
                    status,
                    DownloadStatus::Completed | DownloadStatus::Cancelled | DownloadStatus::Failed(_)
                );

                if !is_completed {
                    // 其他所有状态（Downloading、Waiting）都设置为 Paused
                    status = DownloadStatus::Paused;
                }

                // 所有任务都使用默认的已下载大小和进度（为0）
                // 这些值会在用户恢复下载时通过 resume_download_task 方法正确更新
                let downloaded_size = 0;
                let progress = 0.0;

                let task: DownloadTask = DownloadTask {
                    id: task_db.id,
                    file_name: task_db.file_name,
                    url: task_db.url,
                    save_path: task_db.save_path,
                    downloaded_size,
                    total_size: task_db.total_size,
                    progress,
                    speed: 0, // 启动时速度重置为0
                    status,
                    start_time: None,
                    cancel_token: Some(Arc::new(AtomicBool::new(false))),
                    created_at: chrono::DateTime::from_timestamp(task_db.created_at, 0)
                        .map(|dt| dt.with_timezone(&chrono::Local))
                        .unwrap_or_else(chrono::Local::now),
                };

                self.tasks.push(DownloadTaskFull {
                    task,
                    proxy: task_db.proxy,
                    file_type: task_db.file_type,
                });

                // 更新 next_id 为最大 ID + 1
                if task_db.id >= self.next_id {
                    self.next_id = task_db.id + 1;
                }
            }

            // 重新计算正在下载的任务数
            self.downloading_count = self
                .tasks
                .iter()
                .filter(|t| matches!(t.task.status, DownloadStatus::Downloading))
                .count();

            // 按ID倒序排序（ID越大表示越新添加的，应该在前面）
            self.tasks.sort_by(|a, b| b.task.id.cmp(&a.task.id));

            Ok(self.tasks.len())
        } else {
            Err("数据库未初始化".to_string())
        }
    }

    /// 保存任务到数据库
    ///
    /// # 参数
    /// - `task_full`: 要保存的完整任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn save_to_database(&self, task_full: &DownloadTaskFull) -> Result<(), String> {
        if let Some(db) = &self.database {
            let task_db = crate::ui::download::database::DownloadTaskDB {
                id: task_full.task.id,
                file_name: task_full.task.file_name.clone(),
                url: task_full.task.url.clone(),
                save_path: task_full.task.save_path.clone(),
                total_size: task_full.task.total_size,
                status: Self::status_to_string(&task_full.task.status),
                created_at: task_full.task.created_at.timestamp(),
                proxy: task_full.proxy.clone(),
                file_type: task_full.file_type.clone(),
            };
            db.save_task(&task_db)
        } else {
            Err("数据库未初始化".to_string())
        }
    }

    /// 从数据库删除任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn delete_from_database(&self, id: usize) -> Result<(), String> {
        if let Some(db) = &self.database {
            db.delete_task(id)
        } else {
            Err("数据库未初始化".to_string())
        }
    }

    /// 从数据库清空所有已完成任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn clear_completed_from_database(&self) -> Result<(), String> {
        if let Some(db) = &self.database {
            db.clear_completed()
        } else {
            Err("数据库未初始化".to_string())
        }
    }

    /// 将状态字符串解析为 DownloadStatus 枚举
    fn parse_status(status_str: &str) -> DownloadStatus {
        match status_str {
            "Waiting" => DownloadStatus::Waiting,
            "Downloading" => DownloadStatus::Downloading,
            "Paused" => DownloadStatus::Paused,
            "Completed" => DownloadStatus::Completed,
            "Cancelled" => DownloadStatus::Cancelled,
            _ => DownloadStatus::Failed(status_str.to_string()),
        }
    }

    /// 将 DownloadStatus 枚举转换为字符串
    fn status_to_string(status: &DownloadStatus) -> String {
        match status {
            DownloadStatus::Waiting => "Waiting".to_string(),
            DownloadStatus::Downloading => "Downloading".to_string(),
            DownloadStatus::Paused => "Paused".to_string(),
            DownloadStatus::Completed => "Completed".to_string(),
            DownloadStatus::Cancelled => "Cancelled".to_string(),
            DownloadStatus::Failed(msg) => format!("Failed({})", msg),
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
            cancel_token: Some(Arc::new(AtomicBool::new(false))),
            created_at: chrono::Local::now(),
        };

        let task_full = DownloadTaskFull { task, proxy, file_type };

        // 倒序插入：添加到列表开头
        self.tasks.insert(0, task_full.clone());

        // 保存到数据库
        let _ = self.save_to_database(&task_full);

        self.next_id += 1;
    }

    /// 获取下一个等待中的任务（按添加顺序，先添加的先开始）
    pub fn get_next_waiting_task(&mut self) -> Option<&mut DownloadTaskFull> {
        // 查找状态为 Waiting 的任务（因为是倒序，最早添加的在列表末尾）
        self.tasks.iter_mut().find(|t| t.task.status == DownloadStatus::Waiting)
    }

    /// 更新任务进度
    pub fn update_progress(&mut self, id: usize, downloaded: u64, total: u64, speed: u64) {
        if let Some(index) = self.tasks.iter().position(|t| t.task.id == id) {
            self.tasks[index].task.downloaded_size = downloaded;
            self.tasks[index].task.total_size = total;
            self.tasks[index].task.speed = speed;
            if total > 0 {
                self.tasks[index].task.progress = downloaded as f32 / total as f32;
            }

            // 保存到数据库
            let _ = self.save_to_database(&self.tasks[index]);
        }
    }

    /// 更新任务状态
    pub fn update_status(&mut self, id: usize, status: DownloadStatus) {
        if let Some(index) = self.tasks.iter().position(|t| t.task.id == id) {
            self.tasks[index].task.status = status.clone();

            // 保存到数据库
            let _ = self.save_to_database(&self.tasks[index]);
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
        // 从数据库删除
        let _ = self.delete_from_database(id);

        // 从内存删除
        self.tasks.retain(|t| t.task.id != id);
    }

    /// 清空所有已完成的任务
    pub fn clear_completed(&mut self) {
        // 从数据库删除已完成任务
        let _ = self.clear_completed_from_database();

        // 从内存删除已完成任务
        self.tasks.retain(|t| t.task.status != DownloadStatus::Completed);
    }

    /// 取消任务
    pub fn cancel_task(&mut self, id: usize) {
        if let Some(task_full) = self.tasks.iter_mut().find(|t| t.task.id == id) {
            // 设置取消标志
            if let Some(cancel_token) = &task_full.task.cancel_token {
                cancel_token.store(true, Ordering::Relaxed);
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
