// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸历史数据库操作模块
//!
//! 记录应用过的壁纸路径（最多保留 HISTORY_MAX_ENTRIES 条），
//! 用于"历史记录"页浏览与重新应用

use super::connection::DatabaseConnection;
use rusqlite::params;

/// 历史记录保留上限（与内存中的 wallpaper_history 一致）
pub const HISTORY_MAX_ENTRIES: usize = 50;

/// 壁纸历史数据库结构
#[derive(Debug, Clone)]
pub struct WallpaperHistoryDB {
    /// 壁纸文件绝对路径（主键）
    pub path: String,
    /// 最近一次应用的 Unix 时间戳（秒）
    pub applied_at: i64,
}

/// 壁纸历史数据库仓库
pub struct WallpaperHistoryRepository {
    db: DatabaseConnection,
}

impl std::fmt::Debug for WallpaperHistoryRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WallpaperHistoryRepository {{ database }}")
    }
}

impl WallpaperHistoryRepository {
    /// 从数据库连接创建壁纸历史仓库
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建数据库表
    pub fn create_tables(db: &DatabaseConnection) -> Result<(), String> {
        let conn = db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wallpaper_history (
                path TEXT PRIMARY KEY,
                applied_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("创建表失败: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_wallpaper_history_applied_at
             ON wallpaper_history(applied_at)",
            [],
        )
        .map_err(|e| format!("创建索引失败: {}", e))?;

        Ok(())
    }

    /// 记录/刷新一条历史（同一路径再次应用时刷新时间戳）
    pub fn upsert(&self, path: &str, applied_at: i64) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO wallpaper_history (path, applied_at) VALUES (?1, ?2)",
            params![path, applied_at],
        )
        .map_err(|e| format!("写入壁纸历史失败: {}", e))?;

        Ok(())
    }

    /// 删除一条历史
    pub fn delete(&self, path: &str) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute("DELETE FROM wallpaper_history WHERE path = ?1", params![path])
            .map_err(|e| format!("删除壁纸历史失败: {}", e))?;

        Ok(())
    }

    /// 按应用时间倒序加载最多 limit 条历史
    pub fn load_latest(&self, limit: usize) -> Result<Vec<WallpaperHistoryDB>, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT path, applied_at FROM wallpaper_history
                      ORDER BY applied_at DESC LIMIT ?1")
            .map_err(|e| format!("查询壁纸历史失败: {}", e))?;

        let rows = stmt
            .query_map(params![limit as i64], |row| {
                Ok(WallpaperHistoryDB {
                    path: row.get(0)?,
                    applied_at: row.get(1)?,
                })
            })
            .map_err(|e| format!("查询壁纸历史失败: {}", e))?;

        let mut entries = Vec::new();
        for row in rows.flatten() {
            entries.push(row);
        }

        Ok(entries)
    }

    /// 只保留时间最新的 keep 条，删除其余记录
    pub fn prune(&self, keep: usize) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "DELETE FROM wallpaper_history WHERE path NOT IN (
                SELECT path FROM wallpaper_history ORDER BY applied_at DESC LIMIT ?1
            )",
            params![keep as i64],
        )
        .map_err(|e| format!("清理壁纸历史失败: {}", e))?;

        Ok(())
    }

    /// 清空全部历史
    pub fn clear_all(&self) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute("DELETE FROM wallpaper_history", [])
            .map_err(|e| format!("清空壁纸历史失败: {}", e))?;

        Ok(())
    }
}
