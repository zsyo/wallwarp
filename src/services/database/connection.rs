// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 数据库连接管理模块
//!
//! 提供数据库连接的基础操作

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// 数据库连接管理器
///
/// 使用 Arc<Mutex<Connection>> 实现线程安全的共享连接
#[derive(Clone)]
pub struct DatabaseConnection {
    conn: Arc<Mutex<Connection>>,
}

impl DatabaseConnection {
    /// 打开或创建数据库连接
    ///
    /// # 参数
    /// - `db_path`: 数据库文件路径
    ///
    /// # 返回
    /// 返回数据库连接管理器实例
    pub fn open(db_path: &str) -> Result<Self, String> {
        let path = PathBuf::from(db_path);
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建数据库目录失败: {}", e))?;
        }

        // 打开数据库
        let conn = Connection::open(db_path)
            .map_err(|e| format!("打开数据库失败: {}", e))?;

        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    /// 获取底层的 rusqlite 连接
    pub fn inner(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    /// 获取可变的底层 rusqlite 连接
    pub fn inner_mut(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }
}