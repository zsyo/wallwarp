// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProxyProtocol {
    Http,
    Socks5,
    /// SOCKS5 + 远端 DNS：域名交给代理解析，绕开本地 DNS 污染
    Socks5h,
}

impl std::fmt::Display for ProxyProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyProtocol::Http => write!(f, "http"),
            ProxyProtocol::Socks5 => write!(f, "socks5"),
            ProxyProtocol::Socks5h => write!(f, "socks5h"),
        }
    }
}

impl ProxyProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProxyProtocol::Http => "http",
            ProxyProtocol::Socks5 => "socks5",
            ProxyProtocol::Socks5h => "socks5h",
        }
    }
}

impl FromStr for ProxyProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "socks5" => Ok(ProxyProtocol::Socks5),
            "socks5h" => Ok(ProxyProtocol::Socks5h),
            _ => Ok(ProxyProtocol::Http),
        }
    }
}
