// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;

impl App {
    /// 若定时切换已开启，按当前配置的间隔重置下次执行时间
    ///
    /// 壁纸历史发生变化(设置/切换壁纸)时调用，保证倒计时不失真
    pub(in crate::ui) fn reset_auto_change_next_execute_time(&mut self) {
        if self.auto_change_state.auto_change_enabled
            && let Some(minutes) = self.config.wallpaper.auto_change_interval.get_minutes()
            && minutes > 0
        {
            self.auto_change_state.next_execute_time =
                Some(chrono::Local::now() + chrono::Duration::minutes(minutes as i64));
        }
    }
}
