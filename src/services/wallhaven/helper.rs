// Copyright (C) 2026 zsyo - GNU AGPL v3.0

/// 解析分类位掩码
pub fn parse_category_bitmask(category: &str) -> u32 {
    let mut result = 0u32;
    for (i, c) in category.chars().enumerate() {
        if c == '1' && i < 3 {
            result |= 1 << (2 - i);
        }
    }
    result
}

/// 解析纯净度位掩码
pub fn parse_purity_bitmask(purity: &str) -> u32 {
    match purity {
        "100" | "sfw" => 0b100,
        "010" | "sketchy" => 0b010,
        "001" | "nsfw" => 0b001,
        "110" => 0b110, // sfw + sketchy
        "101" => 0b101, // sfw + nsfw
        "011" => 0b011, // sketchy + nsfw
        "111" => 0b111, // all
        _ => 0b100,
    }
}

/// 对 API key 进行脱敏处理
///
/// 长度达到8个字符时保留前4位与后4位，中间以4个星号代替；
/// 不足8个字符时整体以4个星号代替
pub fn mask_api_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    if chars.len() >= 8 {
        let head: String = chars[..4].iter().collect();
        let tail: String = chars[chars.len() - 4..].iter().collect();
        format!("{}****{}", head, tail)
    } else {
        "****".to_string()
    }
}
