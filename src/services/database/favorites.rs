// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹数据库操作模块
//!
//! 收藏项与收藏分组两张表。收藏项主键 `wallhaven_id`：
//! 在线壁纸存 wallhaven id（如 "94x38z"），本地文件存 "file:{规范化绝对路径}"；
//! 收藏时保存完整元数据快照，收藏夹页展示与操作不依赖各页面运行时状态

use super::connection::DatabaseConnection;
use rusqlite::params;

/// 收藏项类型：在线壁纸
pub const KIND_ONLINE: &str = "online";
/// 收藏项类型：本地文件
pub const KIND_LOCAL: &str = "local";

/// 收藏项数据库结构
#[derive(Debug, Clone)]
pub struct FavoriteDB {
    /// 主键：在线项为 wallhaven id，本地项为 "file:{规范化绝对路径}"
    pub wallhaven_id: String,
    /// 类型：KIND_ONLINE / KIND_LOCAL
    pub kind: String,
    /// 展示标题（在线项为 wallhaven-{id}，本地项为文件名）
    pub title: String,
    /// 在线项：原图直链；本地项：空
    pub url: String,
    /// 在线项：页面 URL；本地项：相对 data_path 的路径
    pub path: String,
    /// 在线项：缩略图 URL；本地项：空
    pub thumb_url: String,
    /// 文件类型（image/jpeg 等）
    pub file_type: String,
    /// 文件大小（字节）
    pub file_size: i64,
    pub width: i64,
    pub height: i64,
    /// 纯净度（sfw/sketchy/nsfw）
    pub purity: String,
    /// 分辨率字符串（如 1920x1080）
    pub resolution: String,
    pub ratio: String,
    pub category: String,
    /// 所属分组 id；NULL = 未分组
    pub group_id: Option<i64>,
    /// 收藏时间的 Unix 时间戳（秒）
    pub created_at: i64,
}

/// 收藏分组数据库结构
#[derive(Debug, Clone)]
pub struct FavoriteGroupDB {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
}

/// 收藏夹数据库仓库
pub struct FavoritesRepository {
    db: DatabaseConnection,
}

impl std::fmt::Debug for FavoritesRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FavoritesRepository {{ database }}")
    }
}

impl FavoritesRepository {
    /// 从数据库连接创建收藏夹仓库
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
            "CREATE TABLE IF NOT EXISTS favorite_groups (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("创建表失败: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS favorites (
                wallhaven_id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                title TEXT NOT NULL,
                url TEXT NOT NULL DEFAULT '',
                path TEXT NOT NULL DEFAULT '',
                thumb_url TEXT NOT NULL DEFAULT '',
                file_type TEXT NOT NULL DEFAULT '',
                file_size INTEGER NOT NULL DEFAULT 0,
                width INTEGER NOT NULL DEFAULT 0,
                height INTEGER NOT NULL DEFAULT 0,
                purity TEXT NOT NULL DEFAULT '',
                resolution TEXT NOT NULL DEFAULT '',
                ratio TEXT NOT NULL DEFAULT '',
                category TEXT NOT NULL DEFAULT '',
                group_id INTEGER,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("创建表失败: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_favorites_created_at
             ON favorites(created_at)",
            [],
        )
        .map_err(|e| format!("创建索引失败: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_favorites_group_id
             ON favorites(group_id)",
            [],
        )
        .map_err(|e| format!("创建索引失败: {}", e))?;

        Ok(())
    }

    /// 新增/刷新一条收藏（重复收藏时刷新时间戳与元数据，保留分组）
    #[allow(clippy::too_many_arguments)]
    pub fn upsert_favorite(&self, fav: &FavoriteDB) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "INSERT INTO favorites (
                wallhaven_id, kind, title, url, path, thumb_url, file_type,
                file_size, width, height, purity, resolution, ratio, category,
                group_id, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
            ON CONFLICT(wallhaven_id) DO UPDATE SET
                kind = excluded.kind,
                title = excluded.title,
                url = excluded.url,
                path = excluded.path,
                thumb_url = excluded.thumb_url,
                file_type = excluded.file_type,
                file_size = excluded.file_size,
                width = excluded.width,
                height = excluded.height,
                purity = excluded.purity,
                resolution = excluded.resolution,
                ratio = excluded.ratio,
                category = excluded.category,
                created_at = excluded.created_at",
            params![
                fav.wallhaven_id,
                fav.kind,
                fav.title,
                fav.url,
                fav.path,
                fav.thumb_url,
                fav.file_type,
                fav.file_size,
                fav.width,
                fav.height,
                fav.purity,
                fav.resolution,
                fav.ratio,
                fav.category,
                fav.group_id,
                fav.created_at,
            ],
        )
        .map_err(|e| format!("写入收藏失败: {}", e))?;

        Ok(())
    }

    /// 删除一条收藏
    pub fn remove_favorite(&self, wallhaven_id: &str) -> Result<(), String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "DELETE FROM favorites WHERE wallhaven_id = ?1",
            params![wallhaven_id],
        )
        .map_err(|e| format!("删除收藏失败: {}", e))?;

        Ok(())
    }

    /// 判断某项是否已收藏
    pub fn is_favorite(&self, wallhaven_id: &str) -> Result<bool, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT 1 FROM favorites WHERE wallhaven_id = ?1")
            .map_err(|e| format!("查询收藏状态失败: {}", e))?;

        let exists = stmt
            .exists(params![wallhaven_id])
            .map_err(|e| format!("查询收藏状态失败: {}", e))?;

        Ok(exists)
    }

    /// 加载全部收藏项（按收藏时间倒序）
    pub fn load_all_favorites(&self) -> Result<Vec<FavoriteDB>, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let mut stmt = conn
            .prepare(&format!(
                "SELECT {} FROM favorites ORDER BY created_at DESC",
                Self::FAVORITE_COLUMNS
            ))
            .map_err(|e| format!("查询收藏失败: {}", e))?;

        let rows = stmt
            .query_map([], Self::map_row)
            .map_err(|e| format!("查询收藏失败: {}", e))?;

        let mut entries = Vec::new();
        for row in rows.flatten() {
            entries.push(row);
        }

        Ok(entries)
    }

    /// 新建收藏分组，返回新分组 id
    pub fn insert_group(&self, name: &str, created_at: i64) -> Result<i64, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        conn.execute(
            "INSERT INTO favorite_groups (name, created_at) VALUES (?1, ?2)",
            params![name, created_at],
        )
        .map_err(|e| format!("新建收藏分组失败: {}", e))?;

        Ok(conn.last_insert_rowid())
    }

    /// 删除收藏分组（组内收藏项回落到未分组）
    pub fn delete_group(&self, group_id: i64) -> Result<(), String> {
        let mut conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let tx = conn
            .transaction()
            .map_err(|e| format!("开启事务失败: {}", e))?;

        tx.execute(
            "UPDATE favorites SET group_id = NULL WHERE group_id = ?1",
            params![group_id],
        )
        .map_err(|e| format!("回落分组内收藏项失败: {}", e))?;

        tx.execute(
            "DELETE FROM favorite_groups WHERE id = ?1",
            params![group_id],
        )
        .map_err(|e| format!("删除收藏分组失败: {}", e))?;

        tx.commit()
            .map_err(|e| format!("提交事务失败: {}", e))?;

        Ok(())
    }

    /// 加载全部分组（按创建时间正序）
    pub fn load_groups(&self) -> Result<Vec<FavoriteGroupDB>, String> {
        let conn = self
            .db
            .inner()
            .lock()
            .map_err(|e| format!("获取数据库锁失败: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT id, name, created_at FROM favorite_groups ORDER BY created_at ASC")
            .map_err(|e| format!("查询收藏分组失败: {}", e))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(FavoriteGroupDB {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .map_err(|e| format!("查询收藏分组失败: {}", e))?;

        let mut entries = Vec::new();
        for row in rows.flatten() {
            entries.push(row);
        }

        Ok(entries)
    }

    /// SELECT 列清单（与 map_row 的读取顺序一致）
    const FAVORITE_COLUMNS: &'static str = "wallhaven_id, kind, title, url, path, thumb_url, \
        file_type, file_size, width, height, purity, resolution, ratio, category, \
        group_id, created_at";

    /// 行映射（列顺序与 FAVORITE_COLUMNS 一致）
    fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<FavoriteDB> {
        Ok(FavoriteDB {
            wallhaven_id: row.get(0)?,
            kind: row.get(1)?,
            title: row.get(2)?,
            url: row.get(3)?,
            path: row.get(4)?,
            thumb_url: row.get(5)?,
            file_type: row.get(6)?,
            file_size: row.get(7)?,
            width: row.get(8)?,
            height: row.get(9)?,
            purity: row.get(10)?,
            resolution: row.get(11)?,
            ratio: row.get(12)?,
            category: row.get(13)?,
            group_id: row.get(14)?,
            created_at: row.get(15)?,
        })
    }
}
