use std::collections::HashMap;
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = "config.ini";
const DEFAULT_LANGUAGE_KEY: &str = "language";
const WINDOW_WIDTH_KEY: &str = "window_width";
const WINDOW_HEIGHT_KEY: &str = "window_height";
const WINDOW_POSITION_X_KEY: &str = "window_pos_x";
const WINDOW_POSITION_Y_KEY: &str = "window_pos_y";
const AUTO_STARTUP_KEY: &str = "auto_startup";

#[derive(Clone)]
pub struct Config {
    pub language: String,
    pub window_width: u32,
    pub window_height: u32,
    pub window_pos_x: Option<i32>,
    pub window_pos_y: Option<i32>,
    pub auto_startup: bool,
    // 将来可以在这里添加更多配置项
    settings: HashMap<String, String>,
}

impl Config {
    pub fn new(lang: &str) -> Self {
        let mut config = Config {
            language: lang.to_string(), // 默认语言为中文
            window_width: 1200,         // 默认窗口宽度
            window_height: 800,         // 默认窗口高度
            window_pos_x: None,         // 默认窗口位置
            window_pos_y: None,         // 默认窗口位置
            auto_startup: false,        // 默认不随电脑启动
            settings: HashMap::new(),
        };

        // 尝试从配置文件加载设置
        config.load_from_file();
        config
    }

    pub fn load_from_file(&mut self) {
        let config_path = Path::new(CONFIG_FILE);

        if !config_path.exists() {
            // 配置文件不存在，创建默认配置
            self.create_default_config();
            return;
        }

        match fs::read_to_string(config_path) {
            Ok(content) => {
                self.parse_config(&content);
            }
            Err(e) => {
                eprintln!("读取配置文件时出错: {}", e);
                // 如果读取失败，使用默认配置
                self.create_default_config();
            }
        }
    }

    fn parse_config(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue; // 跳过空行和注释
            }

            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();

                self.settings.insert(key.clone(), value.clone());

                // 特定配置项处理
                match key.as_str() {
                    DEFAULT_LANGUAGE_KEY => self.language = value,
                    WINDOW_WIDTH_KEY => {
                        if let Ok(width) = value.parse::<u32>() {
                            self.window_width = width;
                        }
                    }
                    WINDOW_HEIGHT_KEY => {
                        if let Ok(height) = value.parse::<u32>() {
                            self.window_height = height;
                        }
                    }
                    WINDOW_POSITION_X_KEY => {
                        if let Ok(pos_x) = value.parse::<i32>() {
                            self.window_pos_x = Some(pos_x);
                        }
                    }
                    WINDOW_POSITION_Y_KEY => {
                        if let Ok(pos_y) = value.parse::<i32>() {
                            self.window_pos_y = Some(pos_y);
                        }
                    }
                    AUTO_STARTUP_KEY => {
                        self.auto_startup = value.parse::<bool>().unwrap_or(false);
                    }
                    _ => {} // 其他配置项
                }
            }
        }
    }

    pub fn save_to_file(&self) {
        let content = self.generate_config_content();

        match fs::write(CONFIG_FILE, content) {
            Ok(_) => {
                // 仅在调试时输出，避免在正常运行时输出
                // println!("配置已保存到文件: {}", CONFIG_FILE);
            }
            Err(e) => {
                eprintln!("保存配置文件时出错: {}", e);
            }
        }
    }

    fn generate_config_content(&self) -> String {
        let mut content = String::new();
        content.push_str("# WallWarp 配置文件\n");
        content.push_str(&format!("{}={}\n", DEFAULT_LANGUAGE_KEY, self.language));
        content.push_str(&format!("{}={}\n", WINDOW_WIDTH_KEY, self.window_width));
        content.push_str(&format!("{}={}\n", WINDOW_HEIGHT_KEY, self.window_height));

        if let Some(pos_x) = self.window_pos_x {
            content.push_str(&format!("{}={}\n", WINDOW_POSITION_X_KEY, pos_x));
        }

        if let Some(pos_y) = self.window_pos_y {
            content.push_str(&format!("{}={}\n", WINDOW_POSITION_Y_KEY, pos_y));
        }

        // 添加随电脑启动配置
        content.push_str(&format!("{}={}\n", AUTO_STARTUP_KEY, self.auto_startup));

        // 如果有其他设置也添加进来
        for (key, value) in &self.settings {
            if key != DEFAULT_LANGUAGE_KEY
                && key != WINDOW_WIDTH_KEY
                && key != WINDOW_HEIGHT_KEY
                && key != WINDOW_POSITION_X_KEY
                && key != WINDOW_POSITION_Y_KEY
                && key != AUTO_STARTUP_KEY
            {
                content.push_str(&format!("{}={}\n", key, value));
            }
        }

        content
    }

    fn create_default_config(&self) {
        let mut content = String::new();
        content.push_str("# WallWarp 配置文件\n");
        content.push_str(&format!("{}=zh-cn\n", DEFAULT_LANGUAGE_KEY));
        content.push_str(&format!("{}={}\n", WINDOW_WIDTH_KEY, self.window_width));
        content.push_str(&format!("{}={}\n", WINDOW_HEIGHT_KEY, self.window_height));

        match fs::write(CONFIG_FILE, content) {
            Ok(_) => {
                println!("已创建默认配置文件: {}", CONFIG_FILE);
            }
            Err(e) => {
                eprintln!("创建默认配置文件时出错: {}", e);
            }
        }
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language.clone();
        self.settings
            .insert(DEFAULT_LANGUAGE_KEY.to_string(), language);
        self.save_to_file();
    }

    pub fn update_window_size(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        self.settings
            .insert(WINDOW_WIDTH_KEY.to_string(), width.to_string());
        self.settings
            .insert(WINDOW_HEIGHT_KEY.to_string(), height.to_string());
        self.save_to_file();
    }

    pub fn update_window_position(&mut self, x: i32, y: i32) {
        self.window_pos_x = Some(x);
        self.window_pos_y = Some(y);
        self.settings
            .insert(WINDOW_POSITION_X_KEY.to_string(), x.to_string());
        self.settings
            .insert(WINDOW_POSITION_Y_KEY.to_string(), y.to_string());
        self.save_to_file();
    }

    pub fn update_window_config(
        &mut self,
        width: u32,
        height: u32,
        x: Option<i32>,
        y: Option<i32>,
    ) {
        self.window_width = width;
        self.window_height = height;

        if let Some(pos_x) = x {
            self.window_pos_x = Some(pos_x);
            self.settings
                .insert(WINDOW_POSITION_X_KEY.to_string(), pos_x.to_string());
        }

        if let Some(pos_y) = y {
            self.window_pos_y = Some(pos_y);
            self.settings
                .insert(WINDOW_POSITION_Y_KEY.to_string(), pos_y.to_string());
        }

        self.settings
            .insert(WINDOW_WIDTH_KEY.to_string(), width.to_string());
        self.settings
            .insert(WINDOW_HEIGHT_KEY.to_string(), height.to_string());
        self.save_to_file();
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        self.settings
            .insert(WINDOW_WIDTH_KEY.to_string(), width.to_string());
        self.settings
            .insert(WINDOW_HEIGHT_KEY.to_string(), height.to_string());
    }

    pub fn set_window_position(&mut self, x: i32, y: i32) {
        self.window_pos_x = Some(x);
        self.window_pos_y = Some(y);
        self.settings
            .insert(WINDOW_POSITION_X_KEY.to_string(), x.to_string());
        self.settings
            .insert(WINDOW_POSITION_Y_KEY.to_string(), y.to_string());
    }

    pub fn set_auto_startup(&mut self, enable: bool) {
        self.auto_startup = enable;
        self.settings
            .insert(AUTO_STARTUP_KEY.to_string(), enable.to_string());
        self.save_to_file();
    }
}
