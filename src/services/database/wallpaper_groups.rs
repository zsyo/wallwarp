// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸组数据库操作模块
//!
//! 壁纸组 = 一套命名的壁纸配置快照（图源 + 在线筛选 + 铺满模式），
//! 支持快速切换。筛选快照以 JSON 存储于 `filters` 列，结构见 [`GroupFilters`]；
//! 接入新图源新增筛选字段时在此扩展（旧记录缺字段按 serde 默认值回退）

use super::connection::DatabaseConnection;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tracing::{error, trace};

/// 壁纸组在线筛选快照（覆盖 Wallhaven 筛选字段与 Basic 档图源共用字段，
/// 取值与 config.wallhaven 一致；旧记录缺字段按 serde 默认值回退）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupFilters {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub purity: String,
    #[serde(default)]
    pub sorting: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub source_color: String,
    #[serde(default)]
    pub pixabay_category: String,
    #[serde(default)]
    pub editors_choice: bool,
    #[serde(default)]
    pub top_range: String,
    #[serde(default)]
    pub resolution_mode: String,
    #[serde(default)]
    pub atleast_resolution: String,
    #[serde(default)]
    pub resolutions: String,
    #[serde(default)]
    pub ratios: String,
}

/// 壁纸组数据库结构
#[derive(Debug, Clone)]
pub struct WallpaperGroupDB {
    pub id: i64,
    /// 组名（唯一）
    pub name: String,
    /// 来源图源 id（services::source::SourceKind::id()）
    pub source: String,
    /// 在线筛选快照（JSON 文本，结构 [`GroupFilters`]）
    pub filters: String,
    /// 铺满模式（utils::config::WallpaperMode 的配置取值）
    pub mode: String,
    pub created_at: i64,
}

impl WallpaperGroupDB {
    /// 解析筛选快照 JSON；解析失败回退为全默认快照（不阻断组的应用）
    ///
    /// 解析结果落 trace 档（含各字段实际取值，便于排障降级映射的来源）；
    /// 解析失败改 error 记录后回退，避免静默吞掉损坏的快照数据
    pub fn parse_filters(&self) -> GroupFilters {
        match serde_json::from_str::<GroupFilters>(&self.filters) {
            Ok(filters) => {
                trace!(
                    "[壁纸组] [{}] 筛选快照解析成功: sorting={} color={} top_range={} resolution_mode={} atleast={} resolutions={} ratios={}",
                    self.name,
                    filters.sorting,
                    filters.color,
                    filters.top_range,
                    filters.resolution_mode,
                    filters.atleast_resolution,
                    filters.resolutions,
                    filters.ratios,
                );
                filters
            }
            Err(e) => {
                error!(
                    "[壁纸组] [{}] 筛选快照解析失败，回退默认快照: {}",
                    self.name, e
                );
                GroupFilters::default()
            }
        }
    }
}

/// 壁纸组仓库
pub struct WallpaperGroupsRepository {
    db: DatabaseConnection,
}

impl std::fmt::Debug for WallpaperGroupsRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WallpaperGroupsRepository {{ database }}")
    }
}

impl WallpaperGroupsRepository {
    /// 从数据库连接创建壁纸组仓库
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
            "CREATE TABLE IF NOT EXISTS wallpaper_groups (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                source TEXT NOT NULL DEFAULT 'wallhaven',
                filters TEXT NOT NULL DEFAULT '{}',
                mode TEXT NOT NULL DEFAULT 'crop',
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("创建表失败: {}", e))?;

        Ok(())
    }

    /// 加载全部壁纸组（按创建时间正序）
    pub fn load_all(&self) -> Result<Vec<WallpaperGroupDB>, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT id, name, source, filters, mode, created_at FROM wallpaper_groups ORDER BY created_at ASC")
            .map_err(|e| format!("查询壁纸组失败: {}", e))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(WallpaperGroupDB {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    source: row.get(2)?,
                    filters: row.get(3)?,
                    mode: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
            .map_err(|e| format!("查询壁纸组失败: {}", e))?;

        let mut entries = Vec::new();
        for row in rows.flatten() {
            entries.push(row);
        }

        Ok(entries)
    }

    /// 新建/覆盖一组（同名组覆盖筛选快照），返回组 id
    pub fn upsert(
        &self,
        name: &str,
        source: &str,
        filters_json: &str,
        mode: &str,
        created_at: i64,
    ) -> Result<i64, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "INSERT INTO wallpaper_groups (name, source, filters, mode, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(name) DO UPDATE SET
                source = excluded.source,
                filters = excluded.filters,
                mode = excluded.mode,
                created_at = excluded.created_at",
            params![name, source, filters_json, mode, created_at],
        )
        .map_err(|e| format!("写入壁纸组失败: {}", e))?;

        let id: i64 = conn
            .query_row(
                "SELECT id FROM wallpaper_groups WHERE name = ?1",
                params![name],
                |row| row.get(0),
            )
            .map_err(|e| format!("查询壁纸组 id 失败: {}", e))?;
        Ok(id)
    }

    /// 删除壁纸组
    pub fn delete(&self, id: i64) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute("DELETE FROM wallpaper_groups WHERE id = ?1", params![id])
            .map_err(|e| format!("删除壁纸组失败: {}", e))?;

        Ok(())
    }
}
