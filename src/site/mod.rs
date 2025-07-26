use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: Option<String>,
    pub base_url: Option<String>,
}

impl SiteConfig {
    pub fn load_from_path<P: AsRef<Path>>(site_dir: P) -> Self {
        let site_dir = site_dir.as_ref();
        
        // Try loading from root directory first
        let config_path = site_dir.join("site.toml");
        if let Some(config) = Self::try_load_config(&config_path) {
            return config;
        }
        
        // Try loading from content directory
        let content_config_path = site_dir.join("content").join("site.toml");
        if let Some(config) = Self::try_load_config(&content_config_path) {
            return config;
        }
        
        // Return default configuration
        Self::default()
    }
    
    fn try_load_config(config_path: &Path) -> Option<Self> {
        if config_path.exists() {
            match std::fs::read_to_string(config_path) {
                Ok(content) => {
                    match toml::from_str::<SiteConfig>(&content) {
                        Ok(config) => return Some(config),
                        Err(_) => {
                            // If parsing fails, continue to next location
                        }
                    }
                }
                Err(_) => {
                    // If reading fails, continue to next location
                }
            }
        }
        None
    }
    
    pub fn get_site_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| "Krik Site".to_string())
    }
    
    pub fn get_base_url(&self) -> Option<String> {
        self.base_url.clone()
    }
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: None,
            base_url: None,
        }
    }
}