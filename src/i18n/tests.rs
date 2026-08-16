// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! I18n 模块单元测试
//!
//! 注意：测试通过 `I18n::new()` 加载真实 locales/ 目录
//! （cargo test 的工作目录为 crate 根目录，可正确解析）。

use super::I18n;

/// 验证带参数词条的占位符被正确插值，而不是原样输出
///
/// 回归背景：FTL 中 `{name}` 是消息引用（会被原样输出并报 ResolverError），
/// 变量插值必须写成 `{$name}`，见 `online-wallpapers.page-separator`。
#[test]
fn t_with_args_interpolates_placeholders() {
    let i18n = I18n::new();
    let text = i18n.t_with_args(
        "online-wallpapers.page-separator",
        &[("current", 3.to_string()), ("total", 5.to_string())],
    );

    assert!(
        !text.contains('{') && !text.contains('$'),
        "占位符未被插值: {}",
        text
    );
    assert!(text.contains('3') && text.contains('5') && text.contains('/'), "插值结果异常: {}", text);
}

/// 验证缺失键回退：返回键名本身且不 panic
#[test]
fn t_missing_key_returns_key() {
    let i18n = I18n::new();
    assert_eq!(i18n.t("definitely-not-exists.key"), "definitely-not-exists.key");
}
