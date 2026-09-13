// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 检查更新服务（GitHub Releases）
//!
//! 经 GitHub Releases API 查询最新正式版本，与当前显示版本（build.rs 注入的
//! `WALLWARP_VERSION`）比较判断是否有更新。网络层复用 [`crate::services::proxy`]
//! 的客户端构建（配置代理 > 环境变量代理 > 直连），与 Wallhaven 请求同源。

use serde::Deserialize;
use tracing::{error, info};

/// GitHub 仓库（owner/name）
const GITHUB_REPO: &str = "zsyo/wallwarp";

/// GitHub Releases API：最新正式版本（不含 prerelease/draft）
fn releases_api_url() -> String {
    format!(
        "https://api.github.com/repos/{}/releases/latest",
        GITHUB_REPO
    )
}

/// GitHub 最新正式 Release 信息
#[derive(Debug, Clone, Deserialize)]
pub struct LatestRelease {
    /// 发布 tag（如 v1.6.0）
    #[allow(dead_code)]
    pub tag_name: String,
    /// 发布页链接
    #[allow(dead_code)]
    pub html_url: String,
}

/// 更新检查结果
#[derive(Debug, Clone)]
pub struct UpdateCheckResult {
    /// 当前版本
    pub current_version: String,
    /// 最新版本（tag 形式，如 v1.6.0）
    pub latest_version: String,
    /// 是否有更新
    pub has_update: bool,
    /// 发布页链接
    pub release_url: String,
}

/// 查询 GitHub 最新正式版本并判断是否有更新
///
/// # 参数
/// - `proxy`: 配置文件中的代理 URL
/// - `proxy_enabled`: 代理是否启用
///
/// # 返回
/// 返回更新检查结果；请求或解析失败时返回错误信息
pub async fn check_for_update(
    proxy: Option<String>,
    proxy_enabled: bool,
) -> Result<UpdateCheckResult, String> {
    let release = fetch_latest_release(proxy, proxy_enabled).await?;
    let current_version = env!("WALLWARP_VERSION").to_string();
    let latest_version = release.tag_name;
    let has_update = is_newer_version(&latest_version, &current_version);
    info!(
        "[检查更新] [current:{}] [latest:{}] 是否有更新: {}",
        current_version, latest_version, has_update
    );
    Ok(UpdateCheckResult {
        current_version,
        latest_version,
        has_update,
        release_url: release.html_url,
    })
}

/// 拉取 GitHub 最新正式 Release
async fn fetch_latest_release(
    proxy: Option<String>,
    proxy_enabled: bool,
) -> Result<LatestRelease, String> {
    let url = releases_api_url();
    let identifier = format!("repo:{}", GITHUB_REPO);

    info!("[检查更新] 请求URL: {}", url);
    let response = crate::services::proxy::create_proxy_client(proxy, proxy_enabled, true)
        .get(&url)
        .header(
            reqwest::header::USER_AGENT,
            concat!("WallWarp/", env!("WALLWARP_VERSION")),
        )
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| {
            error!("[检查更新] [{}] 请求失败: {}", identifier, e);
            e.to_string()
        })?;

    let status = response.status();
    if !status.is_success() {
        let message = format!("HTTP {}", status.as_u16());
        error!("[检查更新] [{}] 响应状态异常: {}", identifier, message);
        return Err(message);
    }

    let release: LatestRelease = response.json().await.map_err(|e| {
        error!("[检查更新] [{}] JSON解析失败: {}", identifier, e);
        e.to_string()
    })?;

    Ok(release)
}

/// 判断 `candidate`（tag，可带 v 前缀）是否比 `current` 更新
///
/// 支持预发布后缀（tag 分隔符为 `_`，如 `1.6.0_beta.1`/`1.6.0_rc2`）：
/// 正式版 > 同版本号的 rc > 同版本号的 beta，正式版之间按语义化版本比较
pub fn is_newer_version(candidate: &str, current: &str) -> bool {
    parse_version(candidate) > parse_version(current)
}

/// 解析版本为可比较元组 (major, minor, patch, 预发布序, 预发布号)
///
/// 预发布序：正式版 2 > rc 1 > beta 0；无法识别的形态视为最旧（全 0）
fn parse_version(version: &str) -> (u64, u64, u64, u8, u64) {
    let version = version.trim().trim_start_matches('v');

    // 拆出主版本与预发布段（形如 "1.6.0_beta.1" / "1.6.0_rc2"）
    let (core, pre_rank, pre_num) = match version.split_once('_') {
        Some((core, pre)) => {
            let (kind, num_str) = match pre.split_once('.') {
                Some((k, n)) => (k, n),
                None => match pre.find(|c: char| c.is_ascii_digit()) {
                    Some(idx) => (&pre[..idx], &pre[idx..]),
                    None => (pre, ""),
                },
            };
            let rank = match kind {
                "rc" => 1u8,
                "beta" => 0u8,
                _ => 0u8,
            };
            (core, rank, num_str.parse().unwrap_or(0))
        }
        None => (version, 2u8, 0u64), // 无预发布段 = 正式版
    };

    let mut nums = [0u64; 3];
    for (i, part) in core.split('.').take(3).enumerate() {
        nums[i] = part.parse().unwrap_or(0);
    }
    (nums[0], nums[1], nums[2], pre_rank, pre_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer_version() {
        // 正式版本比较
        assert!(is_newer_version("v1.6.0", "1.5.1"));
        assert!(!is_newer_version("v1.5.1", "1.5.1"));
        assert!(is_newer_version("1.5.2", "1.5.1"));
        // 预发布版本（tag 分隔符为 _）
        assert!(is_newer_version("v1.6.0", "1.6.0_beta.1"));
        assert!(is_newer_version("v1.6.0_rc2", "1.6.0_beta.3"));
        assert!(!is_newer_version("v1.6.0_beta.2", "1.6.0"));
        assert!(is_newer_version("v1.6.0_beta.2", "1.6.0_beta.1"));
    }
}
