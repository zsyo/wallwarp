use fluent_bundle::{FluentBundle, FluentResource};
use std::{collections::HashMap, fs, path::PathBuf};
use sys_locale::get_locale;
use unic_langid::LanguageIdentifier;

// --- 内置语言配置常量 ---
const DEFAULT_LANG: &str = r#"
app-name = 幻变
app-title = 幻变 - 壁纸管理器

online-wallpapers = 在线壁纸
    .title = 在线壁纸

local-list = 本地列表
    .title = 本地壁纸列表

download-tasks = 下载任务
    .title = 下载任务管理

settings = 设置
    .title = 设置中心
    .switch-lang = 切换语言
"#;

pub struct I18n {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    pub available_langs: Vec<String>,
    pub current_lang: String,
}

impl I18n {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();
        let mut available_langs = Vec::new();

        // 获取程序运行目录
        let mut base_dir = std::env::current_exe().unwrap_or_default();
        base_dir.pop();

        // 确定 locales 实际路径(优先寻找程序同级目录, 否则寻找 CWD 下的目录)
        let locales_dir = if base_dir.join("locales").exists() {
            base_dir.join("locales")
        } else {
            PathBuf::from("locales")
        };

        // 尝试加载外部文件
        if let Ok(entries) = fs::read_dir(locales_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("ftl") {
                    if let Some(lang_code) = path.file_stem().and_then(|s| s.to_str()) {
                        let lang_code = lang_code.to_lowercase(); // 统一小写
                        if let Ok(content) = fs::read_to_string(&path) {
                            Self::add_bundle(
                                &mut bundles,
                                &mut available_langs,
                                &lang_code,
                                &content,
                            );
                        }
                    }
                }
            }
        }

        // 内置兜底(放置 locales 文件夹缺失)
        if available_langs.is_empty() {
            Self::add_bundle(&mut bundles, &mut available_langs, "zh-cn", DEFAULT_LANG);
        }

        // 根据系统语言选择使用的语言
        // 优先级: 系统语言 -> zh-cn -> 列表第一个
        let sys_lang = get_locale().unwrap_or_default().to_lowercase();
        // 简化系统语言, 例如把 zh-CN-u-h-12 改成 zh-cn
        let short_sys_lang = sys_lang.split('-').take(2).collect::<Vec<_>>().join("-");
        let current_lang = if available_langs.contains(&short_sys_lang) {
            short_sys_lang
        } else if available_langs.contains(&"zh-cn".to_string()) {
            "zh-cn".to_string()
        } else {
            available_langs
                .first()
                .cloned()
                .unwrap_or_else(|| "zh-cn".to_string())
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
            // 查找对应的属性
            msg.get_attribute(name).map(|attr| attr.value())
        } else {
            // 获取主值
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
