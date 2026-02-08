// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::ui::download::state::SortColumn;

impl App {
    /// 切换下载任务的排序列
    pub fn toggle_download_sort(&mut self, column: SortColumn) -> iced::Task<AppMessage> {
        // 如果正在排序，不允许切换
        if self.download_state.is_sorting {
            return iced::Task::none();
        }

        // 设置排序中状态
        self.download_state.is_sorting = true;

        // 如果点击的是当前列，切换排序方向
        if self.download_state.sort_column == Some(column) {
            self.download_state.sort_descending = !self.download_state.sort_descending;
        } else {
            // 否则切换到新列，默认降序
            self.download_state.sort_column = Some(column);
            self.download_state.sort_descending = true;
        }

        // 执行排序
        self.sort_download_tasks();

        // 排序完成后解除排序状态
        self.download_state.is_sorting = false;

        iced::Task::none()
    }

    /// 对下载任务列表进行排序
    fn sort_download_tasks(&mut self) {
        let sort_column = match self.download_state.sort_column {
            Some(col) => col,
            None => return,
        };

        let descending = self.download_state.sort_descending;

        self.download_state.tasks.sort_by(|a, b| {
            let comparison = match sort_column {
                SortColumn::FileName => {
                    a.task.file_name.cmp(&b.task.file_name)
                }
                SortColumn::Size => {
                    a.task.total_size.cmp(&b.task.total_size)
                }
                SortColumn::Status => {
                    // 状态排序：Waiting < Downloading < Paused < Completed < Failed < Cancelled
                    status_order(&a.task.status).cmp(&status_order(&b.task.status))
                }
                SortColumn::CreatedAt => {
                    a.task.created_at.cmp(&b.task.created_at)
                }
            };

            if descending {
                comparison.reverse()
            } else {
                comparison
            }
        });
    }
}

/// 获取状态的排序优先级
fn status_order(status: &crate::ui::download::state::DownloadStatus) -> u8 {
    use crate::ui::download::state::DownloadStatus;
    match status {
        DownloadStatus::Waiting => 0,
        DownloadStatus::Downloading => 1,
        DownloadStatus::Paused => 2,
        DownloadStatus::Completed => 3,
        DownloadStatus::Failed(_) => 4,
        DownloadStatus::Cancelled => 5,
    }
}