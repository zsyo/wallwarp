use fluent_bundle::{FluentBundle, FluentResource};
use std::{collections::HashMap, fs, path::PathBuf};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;

const DEFAULT_LANG_CODE: &str = "zh-cn";
const LOCALES_DIR_NAME: &str = "locales";
const FTL_EXTENSION: &str = "ftl";

const DEFAULT_LANG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/zh-cn.ftl"));

pub struct I18n {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    pub available_langs: Vec<String>,
    pub current_lang: String,
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
                if path.extension().and_then(|s| s.to_str()) == Some(FTL_EXTENSION) {
                    if let Some(lang_code) = path.file_stem().and_then(|s| s.to_str()) {
                        let lang_code = lang_code.to_lowercase();
                        if let Ok(content) = fs::read_to_string(&path) {
                            Self::add_bundle(&mut bundles, &mut available_langs, &lang_code, &content);
                        }
                    }
                }
            }
        }

        if available_langs.is_empty() {
            Self::add_bundle(&mut bundles, &mut available_langs, DEFAULT_LANG_CODE, DEFAULT_LANG);
        }

        let sys_lang = get_locale().unwrap_or_default().to_lowercase();
        let short_sys_lang = sys_lang.split('-').take(2).collect::<Vec<_>>().join("-");
        let current_lang = if available_langs.contains(&short_sys_lang) {
            short_sys_lang
        } else if available_langs.contains(&DEFAULT_LANG_CODE.to_string()) {
            DEFAULT_LANG_CODE.to_string()
        } else {
            available_langs
                .first()
                .cloned()
                .unwrap_or_else(|| DEFAULT_LANG_CODE.to_string())
        };

        Self {
            bundles,
            available_langs,
            current_lang,
        }
    }

    fn add_bundle(
        bundles: &mut HashMap<String, FluentBundle<FluentResource>>,
        langs: &mut Vec<String>,
        code: &str,
        content: &str,
    ) {
        if let Ok(res) = FluentResource::try_new(content.to_string()) {
            let lang_id: LanguageIdentifier = code.parse().unwrap_or_default();
            let mut bundle = FluentBundle::new(vec![lang_id]);
            if bundle.add_resource(res).is_ok() {
                bundles.insert(code.to_string(), bundle);
                if !langs.contains(&code.to_string()) {
                    langs.push(code.to_string());
                }
            }
        }
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
