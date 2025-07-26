use std::collections::HashMap;
use crate::parser::Document;

#[derive(Debug, Clone)]
pub struct I18nManager {
    default_language: String,
    documents: HashMap<String, Vec<Document>>,
}

impl I18nManager {
    pub fn new(default_language: String) -> Self {
        Self {
            default_language,
            documents: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, document: Document) {
        self.documents
            .entry(document.language.clone())
            .or_default()
            .push(document);
    }

    pub fn get_documents_by_language(&self, language: &str) -> Option<&Vec<Document>> {
        self.documents.get(language)
    }

    pub fn get_default_documents(&self) -> Option<&Vec<Document>> {
        self.documents.get(&self.default_language)
    }

    pub fn get_available_languages(&self) -> Vec<&String> {
        self.documents.keys().collect()
    }

    pub fn find_translation(&self, base_name: &str, target_language: &str) -> Option<&Document> {
        if let Some(docs) = self.documents.get(target_language) {
            docs.iter().find(|doc| doc.base_name == base_name)
        } else {
            None
        }
    }

    pub fn get_available_translations(&self, base_name: &str) -> Vec<(&String, &Document)> {
        let mut translations = Vec::new();
        for (lang, docs) in &self.documents {
            if let Some(doc) = docs.iter().find(|d| d.base_name == base_name) {
                translations.push((lang, doc));
            }
        }
        translations.sort_by_key(|(lang, _)| *lang);
        translations
    }

    pub fn get_default_language(&self) -> &str {
        &self.default_language
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