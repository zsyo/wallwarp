// Copyright (C) 2026 zsyo - GNU GPL v3.0

use super::OnlineState;
use crate::services::request_context::RequestContext;
use std::sync::atomic::Ordering;

impl OnlineState {
    /// 获取分类API参数字符串
    pub fn get_categories_param(&self) -> String {
        format!("{:03b}", self.categories)
    }

    /// 获取纯净度API参数字符串
    pub fn get_purity_param(&self) -> String {
        format!("{:03b}", self.purities)
    }

    /// 检查是否需要加载下一页
    pub fn should_load_next_page(&self) -> bool {
        !self.last_page && !self.loading_page && self.has_loaded
    }

    /// 根据 API 返回同步分页状态（首次加载与翻页加载的 result handler 共用）
    ///
    /// 空数据且仅一页视为"无数据"：last_page 置 false，
    /// 允许后续尝试不同筛选条件时重新加载
    pub fn update_paging_from_result(
        &mut self,
        result_count: usize,
        last_page: bool,
        total_pages: usize,
        current_page: usize,
    ) {
        self.current_page = current_page;
        self.total_pages = total_pages;
        self.last_page = !(result_count == 0 && current_page == 1 && total_pages == 1) && last_page;
        self.has_loaded = true;
    }

    /// 取消当前正在进行的请求，并创建一个新的请求上下文
    pub fn cancel_and_new_context(&mut self) {
        self.request_context.cancel();
        self.request_context = RequestContext::new();
    }

    /// 取消模态窗口图片下载
    pub fn cancel_modal_download(&mut self) {
        if let Some(cancel_token) = &self.modal_download_cancel_token {
            cancel_token.store(true, Ordering::Relaxed);
        }
        self.modal_download_cancel_token = None;
        self.modal_download_progress = 0.0;
        self.modal_downloaded_bytes = 0;
        self.modal_total_bytes = 0;
    }

    /// 取消所有缩略图加载任务
    pub fn cancel_thumb_loads(&mut self) {
        for cancel_token in &self.thumb_load_cancel_tokens {
            cancel_token.store(true, Ordering::Relaxed);
        }
        self.thumb_load_cancel_tokens.clear();
    }
}
