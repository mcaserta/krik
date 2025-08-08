use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const DEFAULT_LANGUAGE: &str = "en";

pub static SUPPORTED_LANGUAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("en", "English");
    m.insert("it", "Italiano");
    m.insert("es", "Español");
    m.insert("fr", "Français");
    m.insert("de", "Deutsch");
    m.insert("pt", "Português");
    m.insert("ja", "日本語");
    m.insert("zh", "中文");
    m.insert("ru", "Русский");
    m.insert("ar", "العربية");
    m
});

#[derive(Debug, Clone)]
pub struct I18nManager {
    default_language: String,
}

impl I18nManager {
    pub fn new(default_language: String) -> Self {
        let normalized = if SUPPORTED_LANGUAGES.contains_key(default_language.as_str()) {
            default_language
        } else {
            DEFAULT_LANGUAGE.to_string()
        };
        Self { default_language: normalized }
    }

    pub fn default_language(&self) -> &str {
        &self.default_language
    }

    pub fn is_supported_language(&self, code: &str) -> bool {
        SUPPORTED_LANGUAGES.contains_key(code)
    }

    pub fn get_language_name(&self, lang_code: &str) -> String {
        SUPPORTED_LANGUAGES
            .get(lang_code)
            .map(|s| s.to_string())
            .unwrap_or_else(|| lang_code.to_uppercase())
    }
}

pub fn normalize_language_or_default(code: &str) -> String {
    if SUPPORTED_LANGUAGES.contains_key(code) {
        code.to_string()
    } else {
        DEFAULT_LANGUAGE.to_string()
    }
}