// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 临时网络冒烟测试：验证代理 + TLS 请求链路（复现在线壁纸页的请求方式）
//!
//! 用法：`cargo run --example net_smoke`（需在项目根目录，存在 config.toml）

use wallwarp::services::proxy;

async fn try_request(label: &str, client: reqwest::Client, url: &str) -> bool {
    println!("--- {label} ---");
    match client.get(url).send().await {
        Ok(resp) => {
            println!(
                "  状态: {} {}",
                resp.status().as_u16(),
                resp.status().canonical_reason().unwrap_or("")
            );
            let len = resp.content_length();
            println!("  内容长度: {:?}", len);
            true
        }
        Err(e) => {
            println!("  请求失败: {e}");
            let mut src = std::error::Error::source(&e);
            while let Some(s) = src {
                println!("    原因: {s}");
                src = s.source();
            }
            false
        }
    }
}

#[tokio::main]
async fn main() {
    let url = "https://wallhaven.cc/api/v1/search?q=test&page_size=1";

    // 读取 config.toml 中的代理设置（与 App 相同来源）
    let proxy = std::fs::read_to_string("config.toml").ok().and_then(|raw| {
        raw.lines().find_map(|l| {
            let l = l.trim();
            l.strip_prefix("proxy = \"")
                .map(|v| v.trim_end_matches('"').to_string())
        })
    });

    println!("配置代理: {proxy:?}");

    if let Some(p) = &proxy {
        match proxy::create_optimized_client_with_proxy(p) {
            Ok(client) => {
                try_request("走配置代理（与 App 一致）", client, url).await;
            }
            Err(e) => println!("代理客户端创建失败: {e}"),
        }

        // SOCKS5 代理附带测试 socks5h 变体（DNS 由代理解析）
        if let Some(remote_dns) = p
            .strip_prefix("socks5://")
            .map(|rest| format!("socks5h://{rest}"))
        {
            match proxy::create_optimized_client_with_proxy(&remote_dns) {
                Ok(client) => {
                    try_request("socks5h 变体（远端 DNS）", client, url).await;
                }
                Err(e) => println!("socks5h 客户端创建失败: {e}"),
            }
        }
    }

    try_request("直连（对照）", proxy::create_optimized_client(), url).await;
}
