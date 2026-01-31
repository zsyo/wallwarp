// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::{App, AppMessage};
use chrono::{DateTime, Local};
use iced::futures::{Stream, future, stream};
use tracing::info;

impl App {
    /// 创建定时切换壁纸任务
    pub fn create_timer_stream(next_ts: i64) -> impl Stream<Item = AppMessage> {
        let nt = DateTime::from_timestamp_secs(next_ts)
            .map(|dt| dt.with_timezone(&Local))
            .expect("Invalid timestamp");

        stream::once(async move {
            info!(
                "[定时切换] [启动] 更新任务，下次执行时间: {}",
                nt.format("%Y-%m-%d %H:%M:%S")
            );

            if next_ts > 0 {
                let now = chrono::Local::now().timestamp();
                if next_ts > now {
                    let delay = next_ts - now;
                    tokio::time::sleep(std::time::Duration::from_secs(delay as u64)).await
                }
                AutoChangeMessage::AutoChangeTick.into()
            } else {
                future::pending().await
            }
        })
    }
}
