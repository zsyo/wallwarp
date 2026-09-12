// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面状态

use crate::services::database::FavoriteDB;
use crate::ui::favorites::message::{TimeFilter, TypeFilter};
use iced::widget::image::Handle;

/// 一条收藏项（数据库快照 + 运行时展示状态）
#[derive(Debug, Clone)]
pub struct FavoriteEntry {
    /// 数据库快照（kind/url/path/thumb_url/元数据等）
    pub fav: FavoriteDB,
    /// 本地项文件是否仍存在；在线项表示文件是否已下载到壁纸库
    pub in_library: bool,
}

/// 收藏项缩略图状态（区分加载中/已加载/加载失败，供卡片展示占位）
#[derive(Debug, Clone)]
pub enum ThumbState {
    /// 加载中
    Loading,
    /// 已加载
    Loaded(Handle),
    /// 加载失败（本地项源文件失效，或在线项缩略图不可用）
    Failed,
}

/// 收藏夹每批加载数量（滚动到底部追加下一批）
pub const FAVORITES_PAGE_SIZE: usize = 60;

/// 收藏夹页面状态
#[derive(Debug)]
pub struct FavoritesState {
    /// 是否已在本会话加载过
    pub loaded: bool,
    /// 缩略图缓存已失效（缓存目录被清空或路径变更），进页时强制重载缩略图
    pub thumbs_stale: bool,
    /// 收藏项（按筛选与排序后的展示列表，仅已加载的批次）
    pub entries: Vec<FavoriteEntry>,
    /// 全部收藏项（未筛选，用于切换筛选时避免重查数据库）
    pub all_entries: Vec<FavoriteEntry>,
    /// 筛选排序后的完整结果条数（判断是否还有下一批）
    pub filtered_total: usize,
    /// 缩略图状态（与 entries 索引对应）
    pub thumbs: Vec<ThumbState>,
    /// 当前类型筛选
    pub type_filter: TypeFilter,
    /// 类型筛选下拉展开状态
    pub type_filter_expanded: bool,
    /// 当前时间筛选
    pub time_filter: TimeFilter,
    /// 时间筛选下拉展开状态
    pub time_filter_expanded: bool,
    /// 排序方向：true = 收藏时间倒序（最新在前，默认）
    pub sort_descending: bool,
    /// 预览模态可见性
    pub modal_visible: bool,
    /// 预览的条目索引
    pub modal_index: usize,
    /// 预览原图句柄（未解码完成时为 None）
    pub modal_handle: Option<Handle>,
    /// 预览原图下载进度（0.0~1.0，0 = 未开始/缓存命中）
    pub modal_download_progress: f32,
    /// 预览原图已下载字节数
    pub modal_downloaded_bytes: u64,
    /// 预览原图总字节数
    pub modal_total_bytes: u64,
    /// 预览原图下载取消令牌（Some = 下载进行中）
    pub modal_download_cancel_token: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// 待确认移除的条目索引（Some = 显示确认框）
    pub remove_target: Option<usize>,
    /// 待自动设为壁纸的文件名（收藏夹在线项触发下载后，下载完成时匹配）
    pub pending_apply_filename: Option<String>,
}

impl Default for FavoritesState {
    fn default() -> Self {
        Self {
            loaded: false,
            thumbs_stale: false,
            entries: Vec::new(),
            all_entries: Vec::new(),
            filtered_total: 0,
            thumbs: Vec::new(),
            type_filter: TypeFilter::default(),
            type_filter_expanded: false,
            time_filter: TimeFilter::default(),
            time_filter_expanded: false,
            // 默认倒序（最新在前）；bool 的 derive 默认是 false（正序），
            // 与产品预期相反，故手动实现 Default
            sort_descending: true,
            modal_visible: false,
            modal_index: 0,
            modal_handle: None,
            modal_download_progress: 0.0,
            modal_downloaded_bytes: 0,
            modal_total_bytes: 0,
            modal_download_cancel_token: None,
            remove_target: None,
            pending_apply_filename: None,
        }
    }
}

impl FavoritesState {
    /// 复位（数据源变化后重载）
    pub fn invalidate(&mut self) {
        self.loaded = false;
        self.entries.clear();
        self.all_entries.clear();
        self.thumbs.clear();
        self.type_filter_expanded = false;
        self.time_filter_expanded = false;
        self.close_modal();
        self.remove_target = None;
    }

    /// 关闭预览模态并释放原图句柄
    pub fn close_modal(&mut self) {
        self.modal_visible = false;
        self.modal_index = 0;
        self.modal_handle = None;
        self.cancel_modal_download();
    }

    /// 取消预览原图下载并复位进度状态
    pub fn cancel_modal_download(&mut self) {
        use std::sync::atomic::Ordering;
        if let Some(cancel_token) = &self.modal_download_cancel_token {
            cancel_token.store(true, Ordering::Relaxed);
        }
        self.modal_download_cancel_token = None;
        self.modal_download_progress = 0.0;
        self.modal_downloaded_bytes = 0;
        self.modal_total_bytes = 0;
    }

    /// 文件落库后按文件名原地刷新在线收藏项的 in_library
    ///
    /// 同步更新 all_entries 与 entries（不重跑 apply_filter，避免缩略图重置）
    pub fn mark_online_in_library_by_file_name(&mut self, file_name: &str) {
        for entry in self
            .all_entries
            .iter_mut()
            .chain(self.entries.iter_mut())
        {
            if entry.fav.kind == crate::services::database::KIND_ONLINE
                && !entry.in_library
                && entry.fav.title == file_name
            {
                entry.in_library = true;
            }
        }
    }

    /// 时间筛选下界（Unix 秒）；None = 不限
    fn time_filter_lower_bound(&self) -> Option<i64> {
        use chrono::{Datelike, Local, TimeZone};
        let now = Local::now();
        match self.time_filter {
            TimeFilter::All => None,
            TimeFilter::Today => Some(
                now.date_naive()
                    .and_hms_opt(0, 0, 0)
                    .and_then(|t| Local.from_local_datetime(&t).single())
                    .map(|t| t.timestamp())?,
            ),
            TimeFilter::ThreeDays => Some(now.timestamp() - 72 * 3600),
            TimeFilter::ThisWeek => {
                // 本周一 0 点（chrono: weekday() Sun=0..Sat=6，周一前移 6 天折算）
                let days_since_monday = (now.weekday().num_days_from_sunday() + 6) % 7;
                Some(
                    (now.date_naive() - chrono::Duration::days(days_since_monday as i64))
                        .and_hms_opt(0, 0, 0)
                        .and_then(|t| Local.from_local_datetime(&t).single())
                        .map(|t| t.timestamp())?,
                )
            }
            TimeFilter::ThisMonth => Some(
                now.date_naive()
                    .with_day(1)
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .and_then(|t| Local.from_local_datetime(&t).single())
                    .map(|t| t.timestamp())?,
            ),
        }
    }

    /// 按当前类型/时间筛选与排序方向，从 all_entries 生成完整结果并截取首批
    pub fn apply_filter(&mut self) {
        let lower_bound = self.time_filter_lower_bound();
        let mut filtered: Vec<FavoriteEntry> = self
            .all_entries
            .iter()
            .filter(|e| {
                let type_match = match self.type_filter {
                    TypeFilter::All => true,
                    TypeFilter::Online => e.fav.kind == crate::services::database::KIND_ONLINE,
                    TypeFilter::Local => e.fav.kind == crate::services::database::KIND_LOCAL,
                };
                let time_match = lower_bound.is_none_or(|lb| e.fav.created_at >= lb);
                type_match && time_match
            })
            .cloned()
            .collect();

        // 按收藏时间排序（stable：同秒保持原相对顺序）
        if self.sort_descending {
            filtered.sort_by_key(|e| std::cmp::Reverse(e.fav.created_at));
        } else {
            filtered.sort_by_key(|e| e.fav.created_at);
        }

        self.filtered_total = filtered.len();
        filtered.truncate(FAVORITES_PAGE_SIZE);
        self.entries = filtered;
        self.thumbs = vec![ThumbState::Loading; self.entries.len()];
    }

    /// 是否还有未加载的批次
    pub fn has_more(&self) -> bool {
        self.entries.len() < self.filtered_total
    }

    /// 追加下一批筛选结果（返回是否有新增）
    pub fn load_more(&mut self) -> bool {
        if !self.has_more() {
            return false;
        }
        let lower_bound = self.time_filter_lower_bound();
        let mut filtered: Vec<FavoriteEntry> = self
            .all_entries
            .iter()
            .filter(|e| {
                let type_match = match self.type_filter {
                    TypeFilter::All => true,
                    TypeFilter::Online => e.fav.kind == crate::services::database::KIND_ONLINE,
                    TypeFilter::Local => e.fav.kind == crate::services::database::KIND_LOCAL,
                };
                let time_match = lower_bound.is_none_or(|lb| e.fav.created_at >= lb);
                type_match && time_match
            })
            .cloned()
            .collect();
        if self.sort_descending {
            filtered.sort_by_key(|e| std::cmp::Reverse(e.fav.created_at));
        } else {
            filtered.sort_by_key(|e| e.fav.created_at);
        }

        // entries 已含前 start 条（同一排序结果），只追加后续批次
        let start = self.entries.len();
        let end = (start + FAVORITES_PAGE_SIZE).min(filtered.len());
        self.entries
            .extend(filtered.drain(start..end).collect::<Vec<_>>());
        self.thumbs.extend(std::iter::repeat_n(
            ThumbState::Loading,
            self.entries.len() - self.thumbs.len(),
        ));
        true
    }
}
