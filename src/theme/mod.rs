use crate::error::{
    ConfigError, ConfigErrorKind, KrikError, KrikResult, ThemeError, ThemeErrorKind,
};
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
    /// Builder API for constructing a Theme with sensible defaults
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::new()
    }

    /// Load a theme from a directory using the same behavior as the builder.
    ///
    /// Intentionally falls back to a default config when `theme.toml` is missing.
    pub fn load_from_path<P: AsRef<Path>>(theme_path: P) -> KrikResult<Self> {
        Theme::builder()
            .theme_path(theme_path)
            .autoescape_html(false)
            .enable_reload(false)
            .build()
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
        "#
        .to_string()
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
                }
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

    /// Render a template by name, trying `<name>.html` first, then `<name>`.
    pub fn render_page(&self, template_name: &str, context: &Context) -> KrikResult<String> {
        // Try with .html extension first
        let template_with_ext = format!("{template_name}.html");
        if let Ok(rendered) = self.templates.render(&template_with_ext, context) {
            return Ok(rendered);
        }

        // Fall back to original name
        self.templates.render(template_name, context).map_err(|e| {
            KrikError::Template(crate::error::TemplateError {
                kind: crate::error::TemplateErrorKind::RenderError(e),
                template: template_name.to_string(),
                context: "Rendering template via Theme::render_page".to_string(),
            })
        })
    }

    /// Attempt to reload templates from disk. Safe to call in dev when templates change.
    /// If reload fails, keep existing templates.
    pub fn try_reload_templates(&mut self) {
        let templates_path = self.theme_path.join("templates");
        if templates_path.exists() {
            if let Ok(new_tera) = Tera::new(&format!("{}/**/*.html", templates_path.display())) {
                let mut tera = new_tera;
                tera.autoescape_on(vec![]);
                self.templates = tera;
            }
        }
    }
}

/// Builder for configuring and constructing a Theme instance
#[derive(Debug, Default)]
pub struct ThemeBuilder {
    theme_path: Option<PathBuf>,
    autoescape_html: bool,
    enable_reload: bool,
}

impl ThemeBuilder {
    fn new() -> Self {
        Self {
            theme_path: None,
            autoescape_html: false,
            enable_reload: false,
        }
    }

    /// Set the theme directory path
    pub fn theme_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.theme_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Control HTML auto-escaping (off by default for HTML templates)
    pub fn autoescape_html(mut self, enabled: bool) -> Self {
        self.autoescape_html = enabled;
        self
    }

    /// Enable template reload behavior (no-op unless `try_reload_templates` is called)
    pub fn enable_reload(mut self, enabled: bool) -> Self {
        self.enable_reload = enabled;
        self
    }

    /// Build the Theme with configured options
    pub fn build(self) -> KrikResult<Theme> {
        // Resolve theme path or default to themes/default
        let theme_path = match self.theme_path {
            Some(p) => p,
            None => PathBuf::from("themes/default"),
        };

        let config_path = theme_path.join("theme.toml");
        let config_content = match std::fs::read_to_string(&config_path) {
            Ok(s) => s,
            Err(_) => Theme::default_config(), // Fall back to default config when missing
        };

        // Parse theme configuration; on TOML error, surface a typed Theme error
        let config: ThemeConfig = match toml::from_str(&config_content) {
            Ok(cfg) => cfg,
            Err(e) => {
                return Err(KrikError::Theme(ThemeError {
                    kind: ThemeErrorKind::InvalidConfig(ConfigError {
                        kind: ConfigErrorKind::InvalidToml(e),
                        path: Some(config_path.clone()),
                        context: "Parsing theme configuration".to_string(),
                    }),
                    theme_path: theme_path.clone(),
                    context: format!("Failed to parse {}", config_path.display()),
                }));
            }
        };

        // Load templates from the theme directory if present; otherwise fall back
        let templates_path = theme_path.join("templates");
        let mut templates = if templates_path.exists() {
            match Tera::new(&format!("{}/**/*.html", templates_path.display())) {
                Ok(t) => t,
                Err(e) => {
                    // Surface compile errors as Template errors wrapped by ThemeError
                    return Err(KrikError::Theme(ThemeError {
                        kind: ThemeErrorKind::AssetError(format!(
                            "Template compilation failed: {}",
                            e
                        )),
                        theme_path: theme_path.clone(),
                        context: "Compiling theme templates".to_string(),
                    }));
                }
            }
        } else {
            Theme::default_templates()
        };

        // Auto-escape behavior
        if self.autoescape_html {
            // Tera auto-escapes by default for html/tera; keep defaults (no change)
        } else {
            templates.autoescape_on(vec![]);
        }

        let mut theme = Theme {
            config,
            templates,
            theme_path,
        };

        // Optionally trigger an initial reload to ensure file-based templates are fresh
        if self.enable_reload {
            theme.try_reload_templates();
        }

        Ok(theme)
    }
}
