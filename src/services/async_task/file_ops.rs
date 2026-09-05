// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 文件删除/清空相关的异步操作
//!
//! 目录规模可能很大(网络盘/杀毒扫描下单个 remove 也可能耗时)，
//! 统一放在 spawn_blocking 中执行，避免阻塞 runtime 线程造成 UI 卡顿

/// 异步删除单个文件
pub async fn async_delete_file(path: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        std::fs::remove_file(&path).map_err(|e| format!("删除文件失败: {}: {}", path, e))
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

/// 异步清空目录内容（不删除目录本身）
///
/// 返回 Ok(成功删除数)；目录不可访问或存在删除失败项时返回 Err(失败项目数)
pub async fn async_clear_directory(path: String) -> Result<usize, usize> {
    tokio::task::spawn_blocking(move || {
        let entries = match std::fs::read_dir(&path) {
            Ok(entries) => entries,
            Err(_) => return Err(0), // 目录不存在或无法访问
        };

        let mut success_count = 0;
        let mut error_count = 0;

        for entry in entries.flatten() {
            let entry_path = entry.path();
            let result = if entry_path.is_file() {
                std::fs::remove_file(&entry_path)
            } else if entry_path.is_dir() {
                std::fs::remove_dir_all(&entry_path)
            } else {
                Ok(())
            };

            if result.is_ok() {
                success_count += 1;
            } else {
                error_count += 1;
            }
        }

        if error_count == 0 {
            Ok(success_count)
        } else {
            Err(error_count)
        }
    })
    .await
    .unwrap_or(Err(1))
}
