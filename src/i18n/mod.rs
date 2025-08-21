use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const DEFAULT_LANGUAGE: &str = "en";

pub static SUPPORTED_LANGUAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("af", "Afrikaans"),
        ("am", "አማርኛ"),
        ("ar", "العربية"),
        ("az", "Azərbaycanca"),
        ("bg", "Български"),
        ("bn", "বাংলা"),
        ("bs", "Bosanski"),
        ("ca", "Català"),
        ("cs", "Čeština"),
        ("cy", "Cymraeg"),
        ("da", "Dansk"),
        ("de", "Deutsch"),
        ("el", "Ελληνικά"),
        ("en", "English"),
        ("es", "Español"),
        ("et", "Eesti"),
        ("eu", "Euskara"),
        ("fa", "فارسی"),
        ("fi", "Suomi"),
        ("fr", "Français"),
        ("gl", "Galego"),
        ("gu", "ગુજરાતી"),
        ("he", "עברית"),
        ("hi", "हिन्दी"),
        ("hr", "Hrvatski"),
        ("hu", "Magyar"),
        ("id", "Bahasa Indonesia"),
        ("is", "Íslenska"),
        ("it", "Italiano"),
        ("ja", "日本語"),
        ("kn", "ಕನ್ನಡ"),
        ("ko", "한국어"),
        ("lt", "Lietuvių"),
        ("lv", "Latviešu"),
        ("mk", "Македонски"),
        ("ml", "മലയാളം"),
        ("mr", "मराठी"),
        ("ms", "Bahasa Melayu"),
        ("nl", "Nederlands"),
        ("no", "Norsk"),
        ("pa", "ਪੰਜਾਬੀ"),
        ("pl", "Polski"),
        ("pt", "Português"),
        ("ro", "Română"),
        ("ru", "Русский"),
        ("si", "සිංහල"),
        ("sk", "Slovenčina"),
        ("sl", "Slovenščina"),
        ("sq", "Shqip"),
        ("sr", "Српски"),
        ("sv", "Svenska"),
        ("sw", "Kiswahili"),
        ("ta", "தமிழ்"),
        ("te", "తెలుగు"),
        ("th", "ไทย"),
        ("tl", "Tagalog"),
        ("tr", "Türkçe"),
        ("uk", "Українська"),
        ("ur", "اردو"),
        ("vi", "Tiếng Việt"),
        ("xh", "isiXhosa"),
        ("yo", "Yorùbá"),
        ("zh", "中文"),
        ("zu", "isiZulu")
    ])
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
        Self {
            default_language: normalized,
        }
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
