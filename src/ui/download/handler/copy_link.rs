// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use crate::ui::NotificationType;
use iced::Task;

impl App {
    pub(in crate::ui::download) fn copy_link(&mut self, id: usize) -> Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let url = task.task.url.clone();
            let success_message = self.i18n.t("download-tasks.copy-link-success").to_string();
            let failed_message = self.i18n.t("download-tasks.copy-link-failed").to_string();

            // 异步复制到剪贴板
            return Task::perform(
                async move {
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        let result = Command::new("cmd").args(["/c", "echo", &url, "|", "clip"]).output();
                        match result {
                            Ok(_) => Ok(()),
                            Err(_) => Err("复制失败".to_string()),
                        }
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        use std::process::Command;
                        let result = Command::new("xclip")
                            .args(["-selection", "clipboard"])
                            .write_stdin(url.as_bytes())
                            .output();
                        match result {
                            Ok(_) => Ok(()),
                            Err(_) => Err("复制失败".to_string()),
                        }
                    }
                },
                move |result| match result {
                    Ok(_) => AppMessage::ShowNotification(success_message, NotificationType::Success),
                    Err(e) => {
                        AppMessage::ShowNotification(format!("{}: {}", failed_message, e), NotificationType::Error)
                    }
                },
            );
        }
        Task::none()
    }
}
