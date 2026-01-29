// Copyright (C) 2026 zsyo - GNU AGPL v3.0

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
