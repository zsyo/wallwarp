// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载任务数据库操作模块
//!
//! 提供下载任务的持久化操作

use rusqlite::params;
use serde::{Deserialize, Serialize};
use super::connection::DatabaseConnection;

/// 下载任务数据库结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTaskDB {
    /// 任务ID
    pub id: usize,
    /// 文件名称
    pub file_name: String,
    /// 下载URL
    pub url: String,
    /// 保存路径
    pub save_path: String,
    /// 文件总大小（字节）
    pub total_size: u64,
    /// 状态
    pub status: String,
    /// 任务创建时间（Unix 时间戳）
    pub created_at: i64,
    /// 代理设置
    pub proxy: Option<String>,
    /// 原始文件类型
    pub file_type: String,
}

/// 下载任务数据库仓库
pub struct DownloadTasksRepository {
    db: DatabaseConnection,
}

impl std::fmt::Debug for DownloadTasksRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DownloadTasksRepository {{ database }}")
    }
}

impl DownloadTasksRepository {
    /// 从数据库连接创建下载任务仓库
    ///
    /// # 参数
    /// - `db`: 数据库连接
    ///
    /// # 返回
    /// 返回数据库仓库实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建数据库表
    ///
    /// # 参数
    /// - `db`: 数据库连接
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn create_tables(db: &DatabaseConnection) -> Result<(), String> {
        let conn = db.inner().lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS download_tasks (
                id INTEGER PRIMARY KEY,
                file_name TEXT NOT NULL,
                url TEXT NOT NULL,
                save_path TEXT NOT NULL,
                total_size INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'Waiting',
                created_at INTEGER NOT NULL,
                proxy TEXT,
                file_type TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("创建表失败: {}", e))?;

        // 创建索引以加速查询
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_download_tasks_status ON download_tasks(status)",
            [],
        )
        .map_err(|e| format!("创建索引失败: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_download_tasks_created_at ON download_tasks(created_at)",
            [],
        )
        .map_err(|e| format!("创建索引失败: {}", e))?;

        Ok(())
    }

    /// 保存任务到数据库
    ///
    /// # 参数
    /// - `task`: 要保存的任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn save_task(&self, task: &DownloadTaskDB) -> Result<(), String> {
        let conn = self.db.inner().lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO download_tasks
             (id, file_name, url, save_path, total_size, status, created_at, proxy, file_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                task.id as i64,
                &task.file_name,
                &task.url,
                &task.save_path,
                task.total_size as i64,
                &task.status,
                task.created_at,
                task.proxy.as_deref(),
                &task.file_type,
            ],
        )
        .map_err(|e| format!("保存任务失败: {}", e))?;

        Ok(())
    }

    /// 加载所有任务
    ///
    /// # 返回
    /// 返回所有任务的列表
    pub fn load_all_tasks(&self) -> Result<Vec<DownloadTaskDB>, String> {
        let mut tasks = Vec::new();
        let conn = self.db.inner().lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, file_name, url, save_path, total_size, status, created_at, proxy, file_type
             FROM download_tasks
             ORDER BY id ASC"
        )
        .map_err(|e| format!("查询任务失败: {}", e))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(DownloadTaskDB {
                    id: row.get::<_, i64>(0)? as usize,
                    file_name: row.get(1)?,
                    url: row.get(2)?,
                    save_path: row.get(3)?,
                    total_size: row.get::<_, i64>(4)? as u64,
                    status: row.get(5)?,
                    created_at: row.get(6)?,
                    proxy: row.get(7)?,
                    file_type: row.get(8)?,
                })
            })
            .map_err(|e| format!("查询任务失败: {}", e))?;

        for row in rows {
            if let Ok(task) = row {
                tasks.push(task);
            }
        }

        Ok(tasks)
    }

    /// 删除任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn delete_task(&self, id: usize) -> Result<(), String> {
        let conn = self.db.inner().lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "DELETE FROM download_tasks WHERE id = ?1",
            params![id as i64],
        )
        .map_err(|e| format!("删除任务失败: {}", e))?;

        Ok(())
    }

    /// 清空所有已完成任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn clear_completed(&self) -> Result<(), String> {
        let conn = self.db.inner().lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "DELETE FROM download_tasks WHERE status = 'Completed'",
            [],
        )
        .map_err(|e| format!("清空已完成任务失败: {}", e))?;

        Ok(())
    }

    /// 清空所有任务
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    pub fn clear_all(&self) -> Result<(), String> {
        let conn = self.db.inner().lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "DELETE FROM download_tasks",
            [],
        )
        .map_err(|e| format!("清空所有任务失败: {}", e))?;

        Ok(())
    }
}