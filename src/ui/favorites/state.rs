// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面状态

use crate::services::database::FavoriteDB;
use crate::ui::favorites::message::{GroupFilter, TimeFilter, TypeFilter};
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

/// 收藏夹页面状态
#[derive(Debug, Default)]
pub struct FavoritesState {
    /// 是否已在本会话加载过
    pub loaded: bool,
    /// 收藏项（按筛选与排序后的展示列表）
    pub entries: Vec<FavoriteEntry>,
    /// 全部收藏项（未筛选，用于切换筛选时避免重查数据库）
    pub all_entries: Vec<FavoriteEntry>,
    /// 缩略图状态（与 entries 索引对应）
    pub thumbs: Vec<ThumbState>,
    /// 全部分组
    pub groups: Vec<crate::services::database::FavoriteGroupDB>,
    /// 当前分组筛选
    pub group_filter: GroupFilter,
    /// 分组筛选下拉展开状态
    pub group_filter_expanded: bool,
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
    /// 待确认移除的条目索引（Some = 显示确认框）
    pub remove_target: Option<usize>,
    /// 新建分组对话框可见性
    pub create_group_visible: bool,
    /// 新建分组输入内容
    pub create_group_name: String,
    /// 删除分组确认框可见性
    pub delete_group_confirm_visible: bool,
    /// 待自动设为壁纸的文件名（收藏夹在线项触发下载后，下载完成时匹配）
    pub pending_apply_filename: Option<String>,
}

impl FavoritesState {
    /// 复位（数据源变化后重载）
    pub fn invalidate(&mut self) {
        self.loaded = false;
        self.entries.clear();
        self.all_entries.clear();
        self.thumbs.clear();
        self.groups.clear();
        self.group_filter_expanded = false;
        self.type_filter_expanded = false;
        self.time_filter_expanded = false;
        self.close_modal();
        self.remove_target = None;
        self.create_group_visible = false;
        self.create_group_name.clear();
        self.delete_group_confirm_visible = false;
    }

    /// 关闭预览模态并释放原图句柄
    pub fn close_modal(&mut self) {
        self.modal_visible = false;
        self.modal_index = 0;
        self.modal_handle = None;
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

    /// 按当前分组/类型/时间筛选与排序方向，从 all_entries 生成 entries 视图
    pub fn apply_filter(&mut self) {
        let lower_bound = self.time_filter_lower_bound();
        let mut filtered: Vec<FavoriteEntry> = self
            .all_entries
            .iter()
            .filter(|e| {
                let group_match = match &self.group_filter {
                    GroupFilter::All => true,
                    GroupFilter::Ungrouped => e.fav.group_id.is_none(),
                    GroupFilter::Group(id) => e.fav.group_id == Some(*id),
                };
                let type_match = match self.type_filter {
                    TypeFilter::All => true,
                    TypeFilter::Online => e.fav.kind == crate::services::database::KIND_ONLINE,
                    TypeFilter::Local => e.fav.kind == crate::services::database::KIND_LOCAL,
                };
                let time_match = lower_bound.is_none_or(|lb| e.fav.created_at >= lb);
                group_match && type_match && time_match
            })
            .cloned()
            .collect();

        // 按收藏时间排序（stable：同秒保持原相对顺序）
        if self.sort_descending {
            filtered.sort_by_key(|e| std::cmp::Reverse(e.fav.created_at));
        } else {
            filtered.sort_by_key(|e| e.fav.created_at);
        }

        self.entries = filtered;
        self.thumbs = vec![ThumbState::Loading; self.entries.len()];
    }
}
