// Copyright (C) 2026 zsyo - GNU AGPL v3.0
//
// I18n 模块：基于 fluent-bundle 的多语言支持。
// - mod.rs 负责 locales 目录扫描、语言包加载与语言列表管理
// - translate.rs 负责词条查找、参数插值、缺失回退与告警

mod translate;

use fluent_bundle::{FluentBundle, FluentResource};
use std::cell::RefCell;
use std::collections::HashSet;
use std::{collections::HashMap, fs, path::PathBuf};
use sys_locale::get_locale;
use tracing::info;
use unic_langid::LanguageIdentifier;

pub(crate) const DEFAULT_LANG_CODE: &str = "zh-cn";
const LOCALES_DIR_NAME: &str = "locales";
const FTL_EXTENSION: &str = "ftl";
const LANG_NAME_KEY: &str = "lang-name";

const DEFAULT_LANG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/zh-cn.ftl"));

#[derive(Clone, Debug)]
pub struct LangInfo {
    pub code: String,
    pub name: String,
}

pub struct I18n {
    pub(crate) bundles: HashMap<String, FluentBundle<FluentResource>>,
    pub available_langs: Vec<LangInfo>,
    pub current_lang: String,
    /// 已告警过的缺失键，避免每帧渲染重复刷日志
    pub(crate) warned_keys: RefCell<HashSet<String>>,
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

impl I18n {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();
        let mut available_langs = Vec::new();

        if let Ok(entries) = fs::read_dir(Self::resolve_locales_dir()) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some(FTL_EXTENSION)
                    && let Some(lang_code) = path.file_stem().and_then(|s| s.to_str())
                {
                    let lang_code = lang_code.to_lowercase();
                    if let Ok(content) = fs::read_to_string(&path) {
                        Self::add_bundle(&mut bundles, &mut available_langs, &lang_code, &content);
                    }
                }
            }
        }

        if available_langs.is_empty() {
            Self::add_bundle(
                &mut bundles,
                &mut available_langs,
                DEFAULT_LANG_CODE,
                DEFAULT_LANG,
            );
        }

        let sys_lang = get_locale().unwrap_or_default().to_lowercase();
        let short_sys_lang = sys_lang.split('-').take(2).collect::<Vec<_>>().join("-");
        let current_lang = if Self::lang_code_exists(&available_langs, &short_sys_lang) {
            short_sys_lang
        } else if Self::lang_code_exists(&available_langs, DEFAULT_LANG_CODE) {
            DEFAULT_LANG_CODE.to_string()
        } else {
            available_langs
                .first()
                .map(|info| info.code.clone())
                .unwrap_or_else(|| DEFAULT_LANG_CODE.to_string())
        };

        Self {
            bundles,
            available_langs,
            current_lang,
            warned_keys: RefCell::new(HashSet::new()),
        }
    }

    /// 解析 locales 目录：优先使用程序同级目录，其次回退到工作目录
    fn resolve_locales_dir() -> PathBuf {
        let mut base_dir = std::env::current_exe().unwrap_or_default();
        base_dir.pop();

        let dir = base_dir.join(LOCALES_DIR_NAME);
        if dir.exists() {
            dir
        } else {
            PathBuf::from(LOCALES_DIR_NAME)
        }
    }

    /// 重扫 locales 目录，加载运行期间新增的语言文件
    ///
    /// 仅增量添加新语言，已加载语言不受文件删除影响；
    /// 新语言会追加到可选列表末尾，供语言下拉框展示。
    pub fn refresh_languages(&mut self) {
        let existing_count = self.available_langs.len();
        let Ok(entries) = fs::read_dir(Self::resolve_locales_dir()) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(lang_code) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let lang_code = lang_code.to_lowercase();
            if path.extension().and_then(|s| s.to_str()) != Some(FTL_EXTENSION)
                || self.bundles.contains_key(&lang_code)
            {
                continue;
            }
            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };
            Self::add_bundle(
                &mut self.bundles,
                &mut self.available_langs,
                &lang_code,
                &content,
            );
        }

        let added = self.available_langs.len() - existing_count;
        if added > 0 {
            info!("[I18n] [locales] 重扫完成，新增 {} 个语言", added);
        }
    }

    pub(crate) fn lang_code_exists(langs: &[LangInfo], code: &str) -> bool {
        langs.iter().any(|info| info.code == code)
    }

    pub fn lang_names(&self) -> Vec<String> {
        self.available_langs
            .iter()
            .map(|info| info.name.clone())
            .collect()
    }

    pub fn lang_codes(&self) -> Vec<String> {
        self.available_langs
            .iter()
            .map(|info| info.code.clone())
            .collect()
    }

    pub fn lang_codes_and_names(&self) -> Vec<(String, String)> {
        self.available_langs
            .iter()
            .map(|info| (info.code.clone(), info.name.clone()))
            .collect()
    }

    pub fn get_lang_code(&self, name: &str) -> Option<String> {
        self.available_langs
            .iter()
            .find(|info| info.name == name)
            .map(|info| info.code.clone())
    }

    fn add_bundle(
        bundles: &mut HashMap<String, FluentBundle<FluentResource>>,
        langs: &mut Vec<LangInfo>,
        code: &str,
        content: &str,
    ) {
        if let Ok(res) = FluentResource::try_new(content.to_string()) {
            let lang_id: LanguageIdentifier = code.parse().unwrap_or_default();
            let mut bundle = FluentBundle::new(vec![lang_id]);
            if bundle.add_resource(res).is_ok() {
                let lang_name = Self::extract_lang_name(&bundle, code);
                bundles.insert(code.to_string(), bundle);
                if !Self::lang_code_exists(langs, code) {
                    langs.push(LangInfo {
                        code: code.to_string(),
                        name: lang_name,
                    });
                }
            }
        }
    }

    fn extract_lang_name(bundle: &FluentBundle<FluentResource>, code: &str) -> String {
        let mut errors = vec![];
        if let Some(msg) = bundle.get_message(LANG_NAME_KEY)
            && let Some(pattern) = msg.value()
        {
            return bundle
                .format_pattern(pattern, None, &mut errors)
                .to_string();
        }
        code.to_string()
    }

    pub fn set_language(&mut self, lang: String) {
        if self.bundles.contains_key(&lang) {
            self.current_lang = lang;
        }
    }
}
