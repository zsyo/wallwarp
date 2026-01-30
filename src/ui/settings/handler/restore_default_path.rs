// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::helpers::ensure_directory_exists;
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_restore_default_path(&mut self, path_type: String) -> Task<AppMessage> {
        match path_type.as_str() {
            "data" => {
                let old_path = self.config.data.data_path.clone();
                info!("[设置] [数据路径] 修改: {} -> ./data", old_path);
                // 恢复默认的数据路径 "data"
                self.config.set_data_path("data".to_string());
                // 检查并创建目录
                ensure_directory_exists("data", "数据目录");
            }
            "cache" => {
                let old_path = self.config.data.cache_path.clone();
                info!("[设置] [缓存路径] 修改: {} -> ./cache", old_path);
                // 恢复默认的缓存路径 "cache"
                self.config.set_cache_path("cache".to_string());
                // 检查并创建目录
                ensure_directory_exists("cache", "缓存目录");
            }
            _ => {}
        }
        Task::none()
    }
}
