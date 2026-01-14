//! 请求上下文模块，提供类似 Golang 中 context 的取消机制

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// 请求上下文，用于控制异步请求的生命周期
#[derive(Clone, Debug)]
pub struct RequestContext {
    /// 是否已取消
    cancelled: Arc<AtomicBool>,
}

impl RequestContext {
    /// 创建一个新的请求上下文
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 检查请求是否已被取消
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// 取消请求
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// 检查并返回取消状态，如果已取消则返回 Some(())
    #[inline]
    pub fn check_cancelled(&self) -> Option<()> {
        self.is_cancelled().then(|| ())
    }
}

impl Default for RequestContext {
    fn default() -> Self {
        Self::new()
    }
}
