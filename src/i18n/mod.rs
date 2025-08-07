#[derive(Debug, Clone)]
pub struct I18nManager {
}

impl I18nManager {
    pub fn new(_default_language: String) -> Self {
        Self {}
    }

    pub fn get_language_name(&self, lang_code: &str) -> String {
        match lang_code {
            "en" => "English".to_string(),
            "it" => "Italiano".to_string(),
            "es" => "Español".to_string(),
            "fr" => "Français".to_string(),
            "de" => "Deutsch".to_string(),
            "pt" => "Português".to_string(),
            "ja" => "日本語".to_string(),
            "zh" => "中文".to_string(),
            "ru" => "Русский".to_string(),
            "ar" => "العربية".to_string(),
            _ => lang_code.to_uppercase(),
        }
    }
}