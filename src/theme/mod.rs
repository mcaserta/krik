use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub templates: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Theme {
    pub config: ThemeConfig,
    pub templates: Tera,
    pub theme_path: PathBuf,
}

impl Theme {
    pub fn load_from_path<P: AsRef<Path>>(theme_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let theme_path = theme_path.as_ref().to_path_buf();
        let config_path = theme_path.join("theme.toml");
        
        let config_content = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|_| Self::default_config());
        
        let config: ThemeConfig = toml::from_str(&config_content)?;
        
        let templates_path = theme_path.join("templates");
        let mut templates = if templates_path.exists() {
            Tera::new(&format!("{}/**/*.html", templates_path.display()))?
        } else {
            Self::default_templates()
        };
        
        // Disable auto-escaping for HTML templates
        templates.autoescape_on(vec![]);

        Ok(Theme {
            config,
            templates,
            theme_path,
        })
    }

    fn default_config() -> String {
        r#"
name = "default"
version = "1.0.0"
description = "Default Krik theme"

[templates]
page = "page"
post = "post"
index = "index"
        "#.to_string()
    }

    pub fn default_templates() -> Tera {
        // Try to load from themes/default directory first
        let themes_path = PathBuf::from("themes/default/templates");
        if themes_path.exists() {
            match Tera::new(&format!("{}/**/*.html", themes_path.display())) {
                Ok(mut tera) => {
                    // Disable auto-escaping for HTML templates
                    tera.autoescape_on(vec![]);
                    return tera;
                },
                Err(_) => {
                    // If loading fails, fall back to empty Tera (will use hardcoded fallback)
                }
            }
        }

        // Return empty Tera as fallback (file-based templates will be loaded if available)
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        tera
    }

    pub fn render_page(&self, template_name: &str, context: &Context) -> Result<String, Box<dyn std::error::Error>> {
        // Try with .html extension first
        let template_with_ext = format!("{template_name}.html");
        if let Ok(rendered) = self.templates.render(&template_with_ext, context) {
            return Ok(rendered);
        }
        
        // Fall back to original name
        Ok(self.templates.render(template_name, context)?)
    }
}