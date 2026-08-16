// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use fluent_bundle::{FluentBundle, FluentResource};
use std::{collections::HashMap, fs, path::PathBuf};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;

const DEFAULT_LANG_CODE: &str = "zh-cn";
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
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    pub available_langs: Vec<LangInfo>,
    pub current_lang: String,
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

        let mut base_dir = std::env::current_exe().unwrap_or_default();
        base_dir.pop();

        let locales_dir = if base_dir.join(LOCALES_DIR_NAME).exists() {
            base_dir.join(LOCALES_DIR_NAME)
        } else {
            PathBuf::from(LOCALES_DIR_NAME)
        };

        if let Ok(entries) = fs::read_dir(locales_dir) {
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
        }
    }

    fn lang_code_exists(langs: &[LangInfo], code: &str) -> bool {
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

    pub fn t(&self, key: &str) -> String {
        let mut parts = key.splitn(2, '.');
        let id_name = parts.next().unwrap_or_default();
        let attr_name = parts.next();

        let bundle = match self.bundles.get(&self.current_lang) {
            Some(bundle) => bundle,
            None => return key.to_string(),
        };

        let msg = match bundle.get_message(id_name) {
            Some(msg) => msg,
            None => return key.to_string(),
        };

        self.value_or_attr(bundle, msg, id_name, attr_name)
    }

    fn value_or_attr(
        &self,
        bundle: &FluentBundle<FluentResource>,
        msg: fluent_bundle::FluentMessage,
        id_name: &str,
        attr_name: Option<&str>,
    ) -> String {
        let mut errors = vec![];

        let pattern = if let Some(name) = attr_name {
            msg.get_attribute(name).map(|attr| attr.value())
        } else {
            msg.value()
        };

        match pattern {
            Some(p) => bundle.format_pattern(p, None, &mut errors).to_string(),
            None => match attr_name {
                Some(name) => name.to_string(),
                None => id_name.to_string(),
            },
        }
    }
}
