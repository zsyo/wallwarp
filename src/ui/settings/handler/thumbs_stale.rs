// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;

impl App {
    /// 标记四个壁纸页面的缩略图缓存已失效
    ///
    /// 缓存目录被清空（应用内清空或外部删除）或缓存路径变更后调用：
    /// 进入各页面时强制重载缩略图，不再逐项校验缓存文件存在性
    /// （删除进行中时存在性校验可能误判，故用标志位强制全量重载）
    pub(in crate::ui) fn mark_all_thumbs_stale(&mut self) {
        self.online_state.thumbs_stale = true;
        self.local_state.thumbs_stale = true;
        self.history_state.thumbs_stale = true;
        self.favorites_state.thumbs_stale = true;
    }
}
