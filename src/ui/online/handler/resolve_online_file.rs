// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线壁纸文件落库来源的统一解析
//!
//! "设为壁纸/下载"在库(data_path)与缓存(cache_path/online)之间的
//! 判定逻辑原先在四个 handler 中各写一份且行为不一致，统一收敛到此处

use crate::services::download::DownloadService;
use crate::services::wallhaven;
use crate::ui::App;
use std::path::PathBuf;

/// 在线壁纸文件的命中来源
pub(in crate::ui::online) enum OnlineFileHit {
    /// 文件已在壁纸库中(即 target_path 本身)
    InData,
    /// 文件在下载缓存中(已完成下载，附缓存文件路径)
    InCache(String),
}

/// 在线壁纸文件的落库位置
pub(in crate::ui::online) struct OnlineFileLocation {
    /// data_path 中的目标路径
    pub target_path: PathBuf,
    /// 命中的来源(None = 库与缓存中都没有，需要走下载流程)
    pub source: Option<OnlineFileHit>,
}

impl App {
    /// 解析在线壁纸文件的落库来源
    ///
    /// 判定标准统一为"存在且大小匹配"(file_size 来自 Wallhaven API)
    pub(in crate::ui::online) fn resolve_online_file(
        &self,
        url: &str,
        id: &str,
        file_type: &str,
        file_size: u64,
    ) -> OnlineFileLocation {
        // 生成 data_path 中的目标文件路径
        let file_name =
            wallhaven::generate_file_name(id, file_type.split('/').next_back().unwrap_or("jpg"));
        let data_path = self.config.data.data_path.clone();
        let target_path = PathBuf::from(&data_path).join(&file_name);

        // 1. 目标文件已存在且大小匹配
        if let Ok(metadata) = std::fs::metadata(&target_path)
            && metadata.len() == file_size
        {
            return OnlineFileLocation {
                target_path,
                source: Some(OnlineFileHit::InData),
            };
        }

        // 2. 缓存文件已存在且大小匹配
        let cache_path = self.config.data.cache_path.clone();
        if let Ok(cache_file_path) =
            DownloadService::get_online_image_cache_final_path(&cache_path, url, file_size)
            && let Ok(metadata) = std::fs::metadata(&cache_file_path)
            && metadata.len() == file_size
        {
            return OnlineFileLocation {
                target_path,
                source: Some(OnlineFileHit::InCache(cache_file_path)),
            };
        }

        OnlineFileLocation {
            target_path,
            source: None,
        }
    }

    /// 检查下载任务列表中是否已有相同 URL 的进行中任务
    pub(in crate::ui::online) fn has_active_download_for(&self, url: &str) -> bool {
        self.download_state.tasks.iter().any(|task| {
            task.task.url == url
                && task.task.status != crate::ui::download::DownloadStatus::Completed
                && task.task.status != crate::ui::download::DownloadStatus::Cancelled
                && !matches!(
                    task.task.status,
                    crate::ui::download::DownloadStatus::Failed(_)
                )
        })
    }
}
