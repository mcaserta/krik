use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::error::{KrikResult, KrikError, ConfigError, ConfigErrorKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SiteConfig {
    pub title: Option<String>,
    pub base_url: Option<String>,
    pub theme: Option<String>,
}

impl SiteConfig {
    pub fn load_from_path<P: AsRef<Path>>(site_dir: P) -> KrikResult<Self> {
        let site_dir = site_dir.as_ref();
        
        // Try loading from root directory first
        let config_path = site_dir.join("site.toml");
        if let Some(config) = Self::try_load_config(&config_path)? {
            return Ok(config);
        }
        
        // Try loading from content directory
        let content_config_path = site_dir.join("content").join("site.toml");
        if let Some(config) = Self::try_load_config(&content_config_path)? {
            return Ok(config);
        }
        
        // Return default configuration
        Ok(Self::default())
    }
    
    fn try_load_config(config_path: &Path) -> KrikResult<Option<Self>> {
        if config_path.exists() {
            match std::fs::read_to_string(config_path) {
                Ok(content) => {
                    match toml::from_str::<SiteConfig>(&content) {
                        Ok(config) => return Ok(Some(config)),
                        Err(e) => {
                            return Err(KrikError::Config(ConfigError {
                                kind: ConfigErrorKind::InvalidToml(e),
                                path: Some(PathBuf::from(config_path)),
                                context: "Parsing site configuration".to_string(),
                            }));
                        }
                    }
                }
                Err(e) => {
                    return Err(KrikError::Config(ConfigError {
                        kind: match e.kind() {
                            std::io::ErrorKind::NotFound => ConfigErrorKind::NotFound,
                            std::io::ErrorKind::PermissionDenied => ConfigErrorKind::PermissionDenied,
                            _ => ConfigErrorKind::InvalidValue { field: "file".to_string(), expected: "readable file".to_string(), found: format!("{e}") },
                        },
                        path: Some(PathBuf::from(config_path)),
                        context: "Reading site configuration".to_string(),
                    }));
                }
            }
        }
        Ok(None)
    }
    
    pub fn get_site_title(&self) -> String {
        self.title.clone().unwrap_or_else(|| "Krik Site".to_string())
    }
    
    pub fn get_base_url(&self) -> Option<String> {
        self.base_url.clone()
    }
}

