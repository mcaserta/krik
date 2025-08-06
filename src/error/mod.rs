mod recovery;

pub use recovery::{ErrorRecovery, ErrorRecoverable};

use std::fmt;
use std::path::PathBuf;

/// Result type alias for Krik operations  
/// Large error types are intentional for detailed error context
#[allow(clippy::result_large_err)]
pub type KrikResult<T> = Result<T, KrikError>;

/// Main error type for the Krik static site generator
#[derive(Debug)]
pub enum KrikError {
    /// Configuration-related errors
    Config(ConfigError),
    /// File I/O errors
    Io(IoError),
    /// Markdown parsing errors
    Markdown(MarkdownError),
    /// Template processing errors
    Template(TemplateError),
    /// Theme-related errors
    Theme(ThemeError),
    /// Server-related errors
    Server(ServerError),
    /// Content creation errors
    Content(ContentError),
    /// Site generation errors
    Generation(GenerationError),
}

/// Configuration file and parsing errors
#[derive(Debug)]
pub struct ConfigError {
    pub kind: ConfigErrorKind,
    pub path: Option<PathBuf>,
    pub context: String,
}

#[derive(Debug)]
pub enum ConfigErrorKind {
    /// Configuration file not found
    NotFound,
    /// Invalid TOML syntax
    InvalidToml(toml::de::Error),
    /// Invalid YAML syntax  
    InvalidYaml(serde_yaml::Error),
    /// Missing required field
    MissingField(String),
    /// Invalid field value
    InvalidValue { field: String, expected: String, found: String },
    /// File permissions error
    PermissionDenied,
}

/// File I/O related errors
#[derive(Debug)]
pub struct IoError {
    pub kind: IoErrorKind,
    pub path: PathBuf,
    pub context: String,
}

#[derive(Debug)]
pub enum IoErrorKind {
    /// File or directory not found
    NotFound,
    /// Permission denied
    PermissionDenied,
    /// File already exists when it shouldn't
    AlreadyExists,
    /// Invalid file name or path
    InvalidPath,
    /// Disk full or write error
    WriteFailed(std::io::Error),
    /// Read operation failed
    ReadFailed(std::io::Error),
}

/// Markdown processing errors
#[derive(Debug)]
pub struct MarkdownError {
    pub kind: MarkdownErrorKind,
    pub file: PathBuf,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub context: String,
}

#[derive(Debug)]
pub enum MarkdownErrorKind {
    /// Invalid front matter YAML
    InvalidFrontMatter(serde_yaml::Error),
    /// Missing required front matter field
    MissingFrontMatterField(String),
    /// Invalid date format
    InvalidDate(String),
    /// Malformed markdown content
    ParseError(String),
    /// Invalid language code
    InvalidLanguage(String),
    /// Circular reference in content
    CircularReference(PathBuf),
}

/// Template processing errors
#[derive(Debug)]
pub struct TemplateError {
    pub kind: TemplateErrorKind,
    pub template: String,
    pub context: String,
}

#[derive(Debug)]
pub enum TemplateErrorKind {
    /// Template file not found
    NotFound,
    /// Template syntax error
    SyntaxError(tera::Error),
    /// Missing template variable
    MissingVariable(String),
    /// Template rendering failed
    RenderError(tera::Error),
    /// Template compilation failed
    CompileError(tera::Error),
}

/// Theme-related errors
#[derive(Debug)]
pub struct ThemeError {
    pub kind: ThemeErrorKind,
    pub theme_path: PathBuf,
    pub context: String,
}

#[derive(Debug)]
pub enum ThemeErrorKind {
    /// Theme directory not found
    NotFound,
    /// Invalid theme.toml configuration
    InvalidConfig(ConfigError),
    /// Missing required template
    MissingTemplate(String),
    /// Asset processing failed
    AssetError(String),
}

/// Development server errors
#[derive(Debug)]
pub struct ServerError {
    pub kind: ServerErrorKind,
    pub context: String,
}

#[derive(Debug)]
pub enum ServerErrorKind {
    /// Failed to bind to port
    BindError { port: u16, source: std::io::Error },
    /// File watching failed
    WatchError(notify::Error),
    /// WebSocket error
    WebSocketError(String),
    /// Live reload failed
    LiveReloadError(String),
}

/// Content creation and management errors
#[derive(Debug)]
pub struct ContentError {
    pub kind: ContentErrorKind,
    pub path: Option<PathBuf>,
    pub context: String,
}

#[derive(Debug)]
pub enum ContentErrorKind {
    /// Invalid content type
    InvalidType(String),
    /// Duplicate slug
    DuplicateSlug(String),
    /// Invalid file name
    InvalidFileName(String),
    /// Content validation failed
    ValidationFailed(Vec<String>),
}

/// Site generation errors
#[derive(Debug)]
pub struct GenerationError {
    pub kind: GenerationErrorKind,
    pub context: String,
}

#[derive(Debug)]
pub enum GenerationErrorKind {
    /// No content found to generate
    NoContent,
    /// Output directory creation failed
    OutputDirError(std::io::Error),
    /// Asset copying failed
    AssetCopyError { source: PathBuf, target: PathBuf, error: std::io::Error },
    /// Feed generation failed
    FeedError(String),
    /// Sitemap generation failed
    SitemapError(String),
}

// Display implementations for user-friendly error messages

impl fmt::Display for KrikError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KrikError::Config(e) => write!(f, "Configuration error: {}", e),
            KrikError::Io(e) => write!(f, "I/O error: {}", e),
            KrikError::Markdown(e) => write!(f, "Markdown error: {}", e),
            KrikError::Template(e) => write!(f, "Template error: {}", e),
            KrikError::Theme(e) => write!(f, "Theme error: {}", e),
            KrikError::Server(e) => write!(f, "Server error: {}", e),
            KrikError::Content(e) => write!(f, "Content error: {}", e),
            KrikError::Generation(e) => write!(f, "Generation error: {}", e),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_str = self.path.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "<unknown>".to_string());

        match &self.kind {
            ConfigErrorKind::NotFound => {
                write!(f, "Configuration file not found: {}", path_str)
            }
            ConfigErrorKind::InvalidToml(e) => {
                write!(f, "Invalid TOML in {}: {}\n  Context: {}", path_str, e, self.context)
            }
            ConfigErrorKind::InvalidYaml(e) => {
                write!(f, "Invalid YAML in {}: {}\n  Context: {}", path_str, e, self.context)
            }
            ConfigErrorKind::MissingField(field) => {
                write!(f, "Missing required field '{}' in {}\n  Context: {}", field, path_str, self.context)
            }
            ConfigErrorKind::InvalidValue { field, expected, found } => {
                write!(f, "Invalid value for field '{}' in {}\n  Expected: {}\n  Found: {}\n  Context: {}", 
                       field, path_str, expected, found, self.context)
            }
            ConfigErrorKind::PermissionDenied => {
                write!(f, "Permission denied accessing configuration file: {}", path_str)
            }
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_str = self.path.to_string_lossy();
        
        match &self.kind {
            IoErrorKind::NotFound => {
                write!(f, "File or directory not found: {}\n  Context: {}", path_str, self.context)
            }
            IoErrorKind::PermissionDenied => {
                write!(f, "Permission denied: {}\n  Context: {}", path_str, self.context)
            }
            IoErrorKind::AlreadyExists => {
                write!(f, "File already exists: {}\n  Context: {}", path_str, self.context)
            }
            IoErrorKind::InvalidPath => {
                write!(f, "Invalid file path: {}\n  Context: {}", path_str, self.context)
            }
            IoErrorKind::WriteFailed(e) => {
                write!(f, "Failed to write file: {}\n  Error: {}\n  Context: {}", path_str, e, self.context)
            }
            IoErrorKind::ReadFailed(e) => {
                write!(f, "Failed to read file: {}\n  Error: {}\n  Context: {}", path_str, e, self.context)
            }
        }
    }
}

impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_str = self.file.to_string_lossy();
        let location = match (self.line, self.column) {
            (Some(line), Some(col)) => format!(" at line {}, column {}", line, col),
            (Some(line), None) => format!(" at line {}", line),
            _ => String::new(),
        };

        match &self.kind {
            MarkdownErrorKind::InvalidFrontMatter(e) => {
                write!(f, "Invalid front matter in {}{}\n  Error: {}\n  Context: {}", 
                       file_str, location, e, self.context)
            }
            MarkdownErrorKind::MissingFrontMatterField(field) => {
                write!(f, "Missing required front matter field '{}' in {}{}\n  Context: {}", 
                       field, file_str, location, self.context)
            }
            MarkdownErrorKind::InvalidDate(date) => {
                write!(f, "Invalid date format '{}' in {}{}\n  Expected ISO 8601 format (e.g., 2024-01-15T10:30:00Z)\n  Context: {}", 
                       date, file_str, location, self.context)
            }
            MarkdownErrorKind::ParseError(msg) => {
                write!(f, "Markdown parsing error in {}{}\n  Error: {}\n  Context: {}", 
                       file_str, location, msg, self.context)
            }
            MarkdownErrorKind::InvalidLanguage(lang) => {
                write!(f, "Invalid language code '{}' in {}{}\n  Supported languages: en, it, es, fr, de, pt, ja, zh, ru, ar\n  Context: {}", 
                       lang, file_str, location, self.context)
            }
            MarkdownErrorKind::CircularReference(ref_path) => {
                write!(f, "Circular reference detected: {} references {}\n  Context: {}", 
                       file_str, ref_path.to_string_lossy(), self.context)
            }
        }
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TemplateErrorKind::NotFound => {
                write!(f, "Template not found: {}\n  Context: {}", self.template, self.context)
            }
            TemplateErrorKind::SyntaxError(e) => {
                write!(f, "Template syntax error in {}\n  Error: {}\n  Context: {}", 
                       self.template, e, self.context)
            }
            TemplateErrorKind::MissingVariable(var) => {
                write!(f, "Missing template variable '{}' in {}\n  Context: {}", 
                       var, self.template, self.context)
            }
            TemplateErrorKind::RenderError(e) => {
                write!(f, "Template rendering failed for {}\n  Error: {}\n  Context: {}", 
                       self.template, e, self.context)
            }
            TemplateErrorKind::CompileError(e) => {
                write!(f, "Template compilation failed for {}\n  Error: {}\n  Context: {}", 
                       self.template, e, self.context)
            }
        }
    }
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let theme_str = self.theme_path.to_string_lossy();
        
        match &self.kind {
            ThemeErrorKind::NotFound => {
                write!(f, "Theme not found: {}\n  Context: {}", theme_str, self.context)
            }
            ThemeErrorKind::InvalidConfig(e) => {
                write!(f, "Invalid theme configuration in {}\n  Error: {}\n  Context: {}", 
                       theme_str, e, self.context)
            }
            ThemeErrorKind::MissingTemplate(template) => {
                write!(f, "Missing required template '{}' in theme {}\n  Context: {}", 
                       template, theme_str, self.context)
            }
            ThemeErrorKind::AssetError(msg) => {
                write!(f, "Asset processing error in theme {}\n  Error: {}\n  Context: {}", 
                       theme_str, msg, self.context)
            }
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ServerErrorKind::BindError { port, source } => {
                write!(f, "Failed to bind to port {}\n  Error: {}\n  Context: {}\n  Suggestion: Try a different port with --port <PORT>", 
                       port, source, self.context)
            }
            ServerErrorKind::WatchError(e) => {
                write!(f, "File watching failed\n  Error: {}\n  Context: {}", e, self.context)
            }
            ServerErrorKind::WebSocketError(msg) => {
                write!(f, "WebSocket error: {}\n  Context: {}", msg, self.context)
            }
            ServerErrorKind::LiveReloadError(msg) => {
                write!(f, "Live reload error: {}\n  Context: {}\n  Suggestion: Try --no-live-reload flag", 
                       msg, self.context)
            }
        }
    }
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_str = self.path.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "<unknown>".to_string());

        match &self.kind {
            ContentErrorKind::InvalidType(content_type) => {
                write!(f, "Invalid content type '{}' for {}\n  Context: {}", 
                       content_type, path_str, self.context)
            }
            ContentErrorKind::DuplicateSlug(slug) => {
                write!(f, "Duplicate slug '{}' found\n  Path: {}\n  Context: {}", 
                       slug, path_str, self.context)
            }
            ContentErrorKind::InvalidFileName(filename) => {
                write!(f, "Invalid file name '{}'\n  Context: {}\n  Suggestion: Use alphanumeric characters, hyphens, and underscores only", 
                       filename, self.context)
            }
            ContentErrorKind::ValidationFailed(errors) => {
                write!(f, "Content validation failed for {}\n  Issues:\n", path_str)?;
                for error in errors {
                    write!(f, "    - {}\n", error)?;
                }
                write!(f, "  Context: {}", self.context)
            }
        }
    }
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            GenerationErrorKind::NoContent => {
                write!(f, "No content found to generate\n  Context: {}\n  Suggestion: Add .md files to your content directory", 
                       self.context)
            }
            GenerationErrorKind::OutputDirError(e) => {
                write!(f, "Failed to create output directory\n  Error: {}\n  Context: {}", 
                       e, self.context)
            }
            GenerationErrorKind::AssetCopyError { source, target, error } => {
                write!(f, "Failed to copy asset\n  From: {}\n  To: {}\n  Error: {}\n  Context: {}", 
                       source.to_string_lossy(), target.to_string_lossy(), error, self.context)
            }
            GenerationErrorKind::FeedError(msg) => {
                write!(f, "Feed generation failed\n  Error: {}\n  Context: {}", msg, self.context)
            }
            GenerationErrorKind::SitemapError(msg) => {
                write!(f, "Sitemap generation failed\n  Error: {}\n  Context: {}", msg, self.context)
            }
        }
    }
}

// Standard Error trait implementations

impl std::error::Error for KrikError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            KrikError::Config(e) => Some(e),
            KrikError::Io(e) => Some(e),
            KrikError::Markdown(e) => Some(e),
            KrikError::Template(e) => Some(e),
            KrikError::Theme(e) => Some(e),
            KrikError::Server(e) => Some(e),
            KrikError::Content(e) => Some(e),
            KrikError::Generation(e) => Some(e),
        }
    }
}

impl std::error::Error for ConfigError {}
impl std::error::Error for IoError {}
impl std::error::Error for MarkdownError {}
impl std::error::Error for TemplateError {}
impl std::error::Error for ThemeError {}
impl std::error::Error for ServerError {}
impl std::error::Error for ContentError {}
impl std::error::Error for GenerationError {}

// Conversion implementations from external error types

impl From<std::io::Error> for KrikError {
    fn from(e: std::io::Error) -> Self {
        KrikError::Io(IoError {
            kind: match e.kind() {
                std::io::ErrorKind::NotFound => IoErrorKind::NotFound,
                std::io::ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
                std::io::ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
                _ => IoErrorKind::ReadFailed(e),
            },
            path: PathBuf::new(), // Will be set by context
            context: "I/O operation".to_string(),
        })
    }
}

impl From<toml::de::Error> for KrikError {
    fn from(e: toml::de::Error) -> Self {
        KrikError::Config(ConfigError {
            kind: ConfigErrorKind::InvalidToml(e),
            path: None,
            context: "TOML parsing".to_string(),
        })
    }
}

impl From<serde_yaml::Error> for KrikError {
    fn from(e: serde_yaml::Error) -> Self {
        KrikError::Config(ConfigError {
            kind: ConfigErrorKind::InvalidYaml(e),
            path: None,
            context: "YAML parsing".to_string(),
        })
    }
}

impl From<tera::Error> for KrikError {
    fn from(e: tera::Error) -> Self {
        KrikError::Template(TemplateError {
            kind: TemplateErrorKind::RenderError(e),
            template: "<unknown>".to_string(),
            context: "Template processing".to_string(),
        })
    }
}

// Helper macros for creating contextual errors

/// Create a context-aware I/O error
#[macro_export]
macro_rules! io_error {
    ($kind:expr, $path:expr, $context:expr) => {
        $crate::error::KrikError::Io($crate::error::IoError {
            kind: $kind,
            path: $path.into(),
            context: $context.to_string(),
        })
    };
}

/// Create a context-aware markdown error
#[macro_export]
macro_rules! markdown_error {
    ($kind:expr, $file:expr, $context:expr) => {
        $crate::error::KrikError::Markdown($crate::error::MarkdownError {
            kind: $kind,
            file: $file.into(),
            line: None,
            column: None,
            context: $context.to_string(),
        })
    };
    ($kind:expr, $file:expr, $line:expr, $context:expr) => {
        $crate::error::KrikError::Markdown($crate::error::MarkdownError {
            kind: $kind,
            file: $file.into(),
            line: Some($line),
            column: None,
            context: $context.to_string(),
        })
    };
}

/// Create a context-aware template error
#[macro_export]
macro_rules! template_error {
    ($kind:expr, $template:expr, $context:expr) => {
        $crate::error::KrikError::Template($crate::error::TemplateError {
            kind: $kind,
            template: $template.to_string(),
            context: $context.to_string(),
        })
    };
}

/// Create a context-aware config error
#[macro_export]
macro_rules! config_error {
    ($kind:expr, $path:expr, $context:expr) => {
        $crate::error::KrikError::Config($crate::error::ConfigError {
            kind: $kind,
            path: Some($path.into()),
            context: $context.to_string(),
        })
    };
}