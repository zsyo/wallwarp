// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! KDE Wayland 托盘化的任务栏条目控制（kwinrulesrc 窗口规则）
//!
//! Wayland 协议下客户端无法撤回窗口（winit set_visible 为空操作），
//! "最小化到托盘"只能停在最小化状态，Plasma 任务栏仍显示窗口条目。
//! KWin 窗口规则可补齐：skiptaskbar 为 Force 等级时对已映射窗口即时生效，
//! 写入规则后经 DBus 触发 KWin 重载（KWin 不监听规则文件，必须主动通知），
//! 即可在托盘化瞬间隐藏任务栏条目，恢复时移除规则使条目回归，体验与
//! Windows 一致。仅 KDE Wayland 会话启用（KDE X11 下 set_mode(Hidden)
//! 已完整生效，无需规则）。
//!
//! KWin（Plasma 5.x ~ 6.4 的 rulebooksettings.cpp usrRead）只加载
//! kwinrulesrc 中 [General] 组 rules 键登记的规则组，不扫描文件其余组，
//! 因此写入规则组时必须同步把组名注册进 [General].rules（逗号分隔的组名
//! 列表，count 为其长度）；rules 键缺失而 count>0 的旧格式（数字组名
//! 1..count）注册时一并转换为 rules 列表，避免注册后旧规则失联。
//! 移除时从 rules 列表摘除本应用组名，其余登记项原样保留。
//!
//! 本应用写入的规则组带固定特征（Description=WallWarp + wmclass=wallwarp），
//! 移除时只识别该特征组，不会碰用户手建的规则。

use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

/// 规则组特征行（精确整行匹配，用于幂等判断与安全移除）
const RULE_MARKER_DESC: &str = "Description=WallWarp";
const RULE_MARKER_CLASS: &str = "wmclass=wallwarp";

/// 最小化到托盘：写入"跳过任务栏"规则并登记进 [General].rules，
/// 通知 KWin 即时生效（幂等：组与登记均已存在时不重复写盘）
pub fn on_minimized_to_tray() {
    if !in_kde_wayland() {
        return;
    }
    let path = rules_file_path();
    let content = fs::read_to_string(&path).unwrap_or_default();
    let Some(updated) = ensure_rule_registered(&content) else {
        return;
    };
    if write_atomic(&path, &updated).is_err() {
        warn!("[KWin 规则] 写入跳过任务栏规则失败: {}", path.display());
        return;
    }
    debug!("[KWin 规则] 已写入跳过任务栏规则: {}", path.display());
    notify_kwin_reconfigure();
}

/// 从托盘恢复（及启动清理）：从 [General].rules 摘除并删除本应用规则组，
/// 通知 KWin（幂等：无本应用规则组时不写盘）
///
/// 返回是否实际移除了规则——KWin 的 reconfigure 有 200ms 防抖，调用方
/// 需等重载完成后再创建新窗口（否则新窗口映射时被内存中的旧规则命中，
/// 任务栏条目丢失且不随重载回退）
pub fn on_restored_from_tray() -> bool {
    if !in_kde_wayland() {
        return false;
    }
    let path = rules_file_path();
    let Ok(content) = fs::read_to_string(&path) else {
        return false; // 规则文件不存在即无事可做
    };
    let Some(updated) = unregister_rule(&content) else {
        return false; // 无本应用的规则组，无需改动与通知
    };
    if write_atomic(&path, &updated).is_err() {
        warn!("[KWin 规则] 移除跳过任务栏规则失败: {}", path.display());
        return false;
    }
    debug!("[KWin 规则] 已移除跳过任务栏规则: {}", path.display());
    notify_kwin_reconfigure();
    true
}

fn in_kde_wayland() -> bool {
    super::is_kde_plasma() && super::is_wayland()
}

fn rules_file_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kwinrulesrc")
}

/// 规则组文本（skiptaskbarrule=2 即 Force，对已映射窗口即时生效；
/// types=1 仅匹配普通窗口）。组名由调用方生成并负责登记进 [General].rules
fn rule_group_text(id: &str) -> String {
    format!(
        "[{id}]\nDescription=WallWarp\nEnabled=true\ntypes=1\n\
         wmclass=wallwarp\nwmclassmatch=1\nwmclasscomplete=false\n\
         skiptaskbar=true\nskiptaskbarrule=2"
    )
}

/// 随机 UUID（v4 格式，仅作规则组名唯一性用途）
fn random_uuid() -> String {
    use rand::Rng;
    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: Vec<String> = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        hex[0..4].concat(),
        hex[4..6].concat(),
        hex[6..8].concat(),
        hex[8..10].concat(),
        hex[10..16].concat()
    )
}

/// 通知 KWin 重载配置：重读规则书并对所有已映射窗口重新应用规则
/// （reconfigure 有 200ms 防抖；失败仅告警，不影响主流程）
fn notify_kwin_reconfigure() {
    let result = std::process::Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--dest",
            "org.kde.KWin",
            "--object-path",
            "/KWin",
            "--method",
            "org.kde.KWin.reconfigure",
        ])
        .output();
    match result {
        Ok(output) if output.status.success() => {}
        Ok(output) => warn!(
            "[KWin 规则] reconfigure 调用失败: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ),
        Err(e) => warn!("[KWin 规则] 无法调用 gdbus 通知 KWin: {e}"),
    }
}

/// 确保 WallWarp 规则组存在且已登记进 [General].rules
///
/// 返回更新后的完整文件内容；组与登记均已存在时返回 None（免写盘）。
/// 兼容处理：旧版本曾写入"只追加组、未登记"的文件，此时补登记；
/// [General] 缺 rules 键的旧 KConfig 格式（数字组名 1..count）先转换为
/// 等价登记表再追加，避免注册后用户旧规则失联
fn ensure_rule_registered(content: &str) -> Option<String> {
    let mut groups: Vec<String> = split_groups(content).into_iter().map(str::to_string).collect();

    // 复用已存在的 WallWarp 特征组（如旧版本写入的未登记组），否则新建
    let mut appended = false;
    let group_id = match groups
        .iter()
        .find(|group| is_wallwarp_group(group))
        .and_then(|group| group_name(group))
    {
        Some(id) => id,
        None => {
            let id = random_uuid();
            groups.push(rule_group_text(&id));
            appended = true;
            id
        }
    };

    let mut already_registered = false;
    match groups
        .iter_mut()
        .find(|group| group_name(group).as_deref() == Some("General"))
    {
        Some(general) => {
            let mut rules =
                general_rules(general).unwrap_or_else(|| legacy_group_ids(general));
            if rules.iter().any(|id| id == &group_id) {
                already_registered = true;
            } else {
                rules.push(group_id.clone());
                set_general_keys(general, &rules);
            }
        }
        None => groups.push(format!("[General]\ncount=1\nrules={group_id}")),
    }
    if !appended && already_registered {
        return None;
    }
    Some(format!("{}\n", groups.join("\n\n")))
}

/// 从 [General].rules 摘除本应用组名并删除对应规则组
///
/// 返回更新后的完整文件内容；文件中无本应用规则组时返回 None。
/// 未登记过的组（旧版本缺陷产物）只删组，不动 [General]
fn unregister_rule(content: &str) -> Option<String> {
    let mut groups: Vec<String> = split_groups(content).into_iter().map(str::to_string).collect();
    let removed_ids: Vec<String> = groups
        .iter()
        .filter(|group| is_wallwarp_group(group))
        .filter_map(|group| group_name(group))
        .collect();
    if removed_ids.is_empty() {
        return None;
    }
    groups.retain(|group| !is_wallwarp_group(group));
    if let Some(general) = groups
        .iter_mut()
        .find(|group| group_name(group).as_deref() == Some("General"))
        && let Some(rules) = general_rules(general)
    {
        let kept: Vec<String> = rules
            .iter()
            .filter(|id| !removed_ids.contains(id))
            .cloned()
            .collect();
        if kept.len() < rules.len() {
            set_general_keys(general, &kept);
        }
    }
    Some(format!("{}\n", groups.join("\n\n")))
}

/// 提取 KConfig 组的组名（块首行 `[name]`）
fn group_name(group: &str) -> Option<String> {
    let first = group.lines().next()?;
    let name = first.trim().strip_prefix('[')?.strip_suffix(']')?;
    Some(name.to_string())
}

/// 读取 [General] 组的 rules 登记表（逗号分隔组名列表）
fn general_rules(general: &str) -> Option<Vec<String>> {
    general.lines().find_map(|line| {
        line.strip_prefix("rules=").map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|id| !id.is_empty())
                .map(str::to_string)
                .collect()
        })
    })
}

/// 旧格式（rules 键缺失）的等价登记表：KWin 按 1..count 数字组名读取
fn legacy_group_ids(general: &str) -> Vec<String> {
    let count = general
        .lines()
        .find_map(|line| line.strip_prefix("count="))
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0);
    (1..=count).map(|i| i.to_string()).collect()
}

/// 重写 [General] 组的 count 与 rules 两个键（组内其余行原样保留）
fn set_general_keys(general: &mut String, rules: &[String]) {
    let count_line = format!("count={}", rules.len());
    let rules_line = format!("rules={}", rules.join(","));
    let mut updated: Vec<String> = Vec::new();
    let mut has_count = false;
    let mut has_rules = false;
    for line in general.lines() {
        if line.starts_with("count=") {
            updated.push(count_line.clone());
            has_count = true;
        } else if line.starts_with("rules=") {
            updated.push(rules_line.clone());
            has_rules = true;
        } else {
            updated.push(line.to_string());
        }
    }
    if !has_count {
        updated.push(count_line);
    }
    if !has_rules {
        updated.push(rules_line);
    }
    *general = updated.join("\n");
}

/// 判断组是否为本应用写入的规则（特征行精确整行匹配）
fn is_wallwarp_group(group: &str) -> bool {
    let mut has_desc = false;
    let mut has_class = false;
    for line in group.lines() {
        match line.trim_end_matches('\r') {
            RULE_MARKER_DESC => has_desc = true,
            RULE_MARKER_CLASS => has_class = true,
            _ => {}
        }
    }
    has_desc && has_class
}

/// 按 KConfig 组（`[xxx]` 行起到下一组头之前）切分内容；首个组之前的
/// 散键内容作为独立块保留，保证读写往返不丢数据。
/// 按 \n 逐段推进偏移（str::lines 会把 \r\n 整体剥掉，CRLF 下若按
/// lines 的行长累计会漏计 \r，块边界漂移截断上一块末行）
fn split_groups(content: &str) -> Vec<&str> {
    let mut blocks: Vec<&str> = Vec::new();
    let mut block_start: Option<usize> = None;
    let mut offset = 0usize;

    while offset < content.len() {
        let line_end = content[offset..]
            .find('\n')
            .map_or(content.len(), |pos| offset + pos + 1);
        let trimmed = content[offset..line_end].trim_end_matches(['\n', '\r']);
        if trimmed.starts_with('[') {
            if let Some(start) = block_start.take() {
                blocks.push(content[start..offset].trim_end());
            }
            block_start = Some(offset);
        } else if block_start.is_none() && !trimmed.trim().is_empty() {
            block_start = Some(offset);
        }
        offset = line_end;
    }
    if let Some(start) = block_start {
        blocks.push(content[start..].trim_end());
    }
    blocks.retain(|block| !block.is_empty());
    blocks
}

/// 临时文件 + rename 原子写，避免写盘中断损坏 KWin 配置
fn write_atomic(path: &Path, content: &str) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, content)?;
    fs::rename(&tmp, path)
}
