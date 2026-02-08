// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 数据库服务模块
//!
//! 提供统一的数据库操作接口，支持多表管理
//!
//! 使用单例模式管理数据库连接，避免多地方重复打开同一个文件

pub mod connection;
pub mod download_tasks;

pub use connection::DatabaseConnection;
pub use download_tasks::{DownloadTaskDB, DownloadTasksRepository};

use std::sync::OnceLock;

/// 全局数据库管理器
///
/// 使用单例模式管理数据库连接，确保整个应用只打开一次数据库文件
pub struct DatabaseManager {
    connection: DatabaseConnection,
}

impl DatabaseManager {
    /// 初始化全局数据库管理器
    ///
    /// # 参数
    /// - `db_path`: 数据库文件路径
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误信息
    ///
    /// # 注意
    /// 此方法应该在应用启动时调用一次，后续通过 get() 获取实例
    pub fn init(db_path: &str) -> Result<(), String> {
        let connection = DatabaseConnection::open(db_path)?;
        
        // 创建所有需要的表
        DownloadTasksRepository::create_tables(&connection)?;
        
        GLOBAL_DATABASE.get_or_init(|| DatabaseManager { connection });
        
        Ok(())
    }
    
    /// 获取全局数据库管理器实例
    ///
    /// # 返回
    /// 返回全局数据库管理器的引用
    ///
    /// # Panics
    /// 如果在调用 init() 之前调用此方法，会 panic
    pub fn get() -> &'static DatabaseManager {
        GLOBAL_DATABASE.get().expect("DatabaseManager 未初始化，请先调用 init()")
    }
    
    /// 获取数据库连接
    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

/// 全局数据库管理器实例
static GLOBAL_DATABASE: OnceLock<DatabaseManager> = OnceLock::new();