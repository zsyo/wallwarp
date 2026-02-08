// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载任务数据库模块
//!
//! 使用 SQLite 持久化下载任务记录
//!
//! 本模块提供对下载任务数据库的访问接口，内部使用 services/database 模块实现
//!
//! 注意：现在使用全局单例数据库管理器，避免多处打开同一数据库文件

use crate::services::database::{DatabaseManager, DownloadTasksRepository};

// 重新导出类型以便其他模块使用
pub use crate::services::database::DownloadTaskDB;

/// 下载任务数据库管理器
pub struct DownloadDatabase {
    repository: DownloadTasksRepository,
}

impl std::fmt::Debug for DownloadDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DownloadDatabase {{ repository }}")
    }
}

impl DownloadDatabase {
    /// 打开或创建数据库
    ///
    /// # 参数
    /// - `db_path`: 数据库文件路径
    ///
    /// # 返回
    /// 返回数据库管理器实例
    ///
    /// # 注意
    /// 此方法会初始化全局数据库管理器，应该在应用启动时调用一次
    pub fn open(db_path: &str) -> Result<Self, String> {
        // 初始化全局数据库管理器
        DatabaseManager::init(db_path)?;
        
        // 创建下载任务仓库实例
        let db_manager = DatabaseManager::get();
        let repository = DownloadTasksRepository::new(
            db_manager.connection().clone()
        );
        
        Ok(Self { repository })
    }

    /// 获取下载任务数据库管理器实例（使用全局连接）
    ///
    /// # 返回
    /// 返回数据库管理器实例
    ///
    /// # Panics
    /// 如果在调用 open() 之前调用此方法，会 panic
    pub fn get() -> Self {
        let db_manager = DatabaseManager::get();
        let repository = DownloadTasksRepository::new(
            db_manager.connection().clone()
        );
        Self { repository }
    }

    /// 保存任务到数据库
    ///
    /// # 参数
    /// - `task`: 要保存的任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn save_task(&self, task: &DownloadTaskDB) -> Result<(), String> {
        self.repository.save_task(task)
    }

    /// 加载所有任务
    ///
    /// # 返回
    /// 返回所有任务的列表
    pub fn load_all_tasks(&self) -> Result<Vec<DownloadTaskDB>, String> {
        self.repository.load_all_tasks()
    }

    /// 删除任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn delete_task(&self, id: usize) -> Result<(), String> {
        self.repository.delete_task(id)
    }

    /// 清空所有已完成任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn clear_completed(&self) -> Result<(), String> {
        self.repository.clear_completed()
    }

    /// 清空所有任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn clear_all(&self) -> Result<(), String> {
        self.repository.clear_all()
    }
}