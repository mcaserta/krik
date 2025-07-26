use crate::parser::{Document, extract_language_from_filename, parse_markdown_with_frontmatter};
use crate::theme::Theme;
use crate::i18n::I18nManager;
use crate::site::SiteConfig;
use chrono::{DateTime, Utc};
use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tera::Context;
use walkdir::WalkDir;

/// The main site generator that processes Markdown files and creates a static website.
///
/// `SiteGenerator` handles the entire process of creating a static site:
/// - Scanning and parsing Markdown files with front matter
/// - Processing internationalization and translations
/// - Applying themes and templates
/// - Generating HTML output with navigation features
/// - Creating Atom feeds
///
/// # Example
///
/// ```rust,no_run
/// use krik::generator::SiteGenerator;
/// use std::path::PathBuf;
///
/// let mut generator = SiteGenerator::new(
///     "content",           // Source directory
///     "_site",            // Output directory  
///     Some("themes/custom") // Optional theme directory
/// )?;
///
/// generator.scan_files()?;
/// generator.generate_site()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct SiteGenerator {
    /// Source directory containing Markdown files and assets
    pub source_dir: PathBuf,
    /// Output directory where the generated site will be written
    pub output_dir: PathBuf,
    /// Theme configuration and templates
    pub theme: Theme,
    /// Internationalization manager for multi-language support
    pub i18n: I18nManager,
    /// Site-wide configuration loaded from site.toml
    pub site_config: SiteConfig,
    /// Parsed documents ready for processing
    pub documents: Vec<Document>,
}

impl SiteGenerator {
    /// Creates a new `SiteGenerator` instance.
    ///
    /// # Arguments
    ///
    /// * `source_dir` - Directory containing Markdown files and content
    /// * `output_dir` - Directory where generated HTML will be written  
    /// * `theme_dir` - Optional custom theme directory (defaults to `themes/default`)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `SiteGenerator` or an error if initialization fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use krik::generator::SiteGenerator;
    ///
    /// // Using default theme
    /// let generator = SiteGenerator::new("content", "_site", None::<&str>)?;
    ///
    /// // Using custom theme
    /// let generator = SiteGenerator::new("content", "_site", Some("my-theme"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<P: AsRef<Path>>(
        source_dir: P,
        output_dir: P,
        theme_dir: Option<P>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let source_dir = source_dir.as_ref().to_path_buf();
        let output_dir = output_dir.as_ref().to_path_buf();
        
        let theme = if let Some(theme_path) = theme_dir {
            Theme::load_from_path(theme_path)?
        } else {
            Theme::load_from_path("themes/default").unwrap_or_else(|_| {
                Theme {
                    config: crate::theme::ThemeConfig {
                        name: "default".to_string(),
                        version: "1.0.0".to_string(),
                        author: None,
                        description: None,
                        templates: HashMap::new(),
                    },
                    templates: crate::theme::Theme::default_templates(),
                    theme_path: PathBuf::from("themes/default"),
                }
            })
        };

        let i18n = I18nManager::new("en".to_string());
        let site_config = SiteConfig::load_from_path(&source_dir);

        Ok(SiteGenerator {
            source_dir,
            output_dir,
            theme,
            i18n,
            site_config,
            documents: Vec::new(),
        })
    }

    /// Scans the source directory for Markdown files and parses them.
    ///
    /// This method recursively walks the source directory, finds all `.md` files,
    /// parses their front matter and content, and stores them for later processing.
    /// Files marked as `draft: true` in their front matter are skipped.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if file reading or parsing fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use krik::generator::SiteGenerator;
    ///
    /// let mut generator = SiteGenerator::new("content", "_site", None::<&str>)?;
    /// generator.scan_files()?; // Scan for .md files
    /// println!("Found {} documents", generator.documents.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn scan_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content_dir = self.source_dir.join("content");
        let scan_dir = if content_dir.exists() { content_dir } else { self.source_dir.clone() };
        
        for entry in WalkDir::new(&scan_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let file_path = entry.path();
            let content = fs::read_to_string(file_path)?;
            let filename = file_path.file_stem().unwrap().to_str().unwrap();
            
            let (base_name, language) = extract_language_from_filename(filename);
            let (mut front_matter, markdown_content) = parse_markdown_with_frontmatter(&content)?;
            
            // Skip files marked as draft
            if front_matter.draft.unwrap_or(false) {
                continue;
            }
            
            if front_matter.date.is_none() {
                if let Ok(metadata) = file_path.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        front_matter.date = Some(DateTime::<Utc>::from(modified));
                    }
                }
            }

            let document = Document {
                front_matter,
                content: markdown_content,
                file_path: file_path.to_string_lossy().to_string(),
                language,
                base_name,
            };

            self.documents.push(document.clone());
            self.i18n.add_document(document);
        }

        self.documents.sort_by(|a, b| {
            let date_a = a.front_matter.date.unwrap_or_else(|| DateTime::<Utc>::from(std::time::UNIX_EPOCH));
            let date_b = b.front_matter.date.unwrap_or_else(|| DateTime::<Utc>::from(std::time::UNIX_EPOCH));
            date_b.cmp(&date_a)
        });

        Ok(())
    }

    /// Generates the complete static site from parsed documents.
    ///
    /// This method creates the output directory and generates:
    /// - Individual HTML pages for each document
    /// - An index page listing all posts
    /// - An Atom feed (feed.xml)
    /// - Copied theme assets (CSS, JavaScript)
    /// - Copied content assets (images, etc.)
    ///
    /// The generated site includes all navigation features like table of contents,
    /// footnote links, scroll-to-top buttons, and language selectors.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if generation fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use krik::generator::SiteGenerator;
    ///
    /// let mut generator = SiteGenerator::new("content", "_site", None::<&str>)?;
    /// generator.scan_files()?;
    /// generator.generate_site()?; // Generate complete site
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn generate_site(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;

        // Copy all non-markdown files
        self.copy_non_markdown_files()?;

        // Copy theme assets
        self.copy_theme_assets()?;

        for document in &self.documents {
            self.generate_page(document)?;
        }

        self.generate_index()?;
        self.generate_feed()?;

        Ok(())
    }

    fn copy_non_markdown_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content_dir = self.source_dir.join("content");
        let scan_dir = if content_dir.exists() { content_dir } else { self.source_dir.clone() };
        
        for entry in WalkDir::new(&scan_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| !e.path().extension().is_some_and(|ext| ext == "md"))
            .filter(|e| e.path().file_name().map_or(true, |name| name != "site.toml"))
        {
            let source_path = entry.path();
            let relative_path = source_path.strip_prefix(&scan_dir)?;
            let target_path = self.output_dir.join(relative_path);

            // Create parent directories if they don't exist
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy the file
            fs::copy(source_path, target_path)?;
        }
        Ok(())
    }

    fn copy_theme_assets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let assets_dir = self.theme.theme_path.join("assets");
        
        if !assets_dir.exists() {
            return Ok(()); // No assets to copy
        }
        
        let output_assets_dir = self.output_dir.join("assets");
        
        for entry in WalkDir::new(&assets_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let source_path = entry.path();
            let relative_path = source_path.strip_prefix(&assets_dir)?;
            let target_path = output_assets_dir.join(relative_path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Copy the file
            fs::copy(source_path, target_path)?;
        }
        
        Ok(())
    }

    fn generate_toc_and_content(&self, content: &str, title: Option<&str>) -> (String, String) {
        // First, generate normal HTML content
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        let parser = Parser::new_ext(content, options);
        let mut html_content = String::new();
        html::push_html(&mut html_content, parser);
        
        // Use regex to find headings and generate TOC
        let heading_regex = Regex::new(r"<h([1-6])>([^<]+)</h[1-6]>").unwrap();
        let mut toc_items = Vec::new();
        let mut heading_id_counter = HashMap::new();
        let mut modified_html = html_content.clone();
        
        // Find all headings and collect TOC data
        for cap in heading_regex.captures_iter(&html_content) {
            let level = cap[1].parse::<u8>().unwrap();
            let text = cap[2].trim().to_string();
            
            // Generate a unique ID for the heading
            let base_id = text
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
                .collect::<String>()
                .trim_matches('-')
                .to_string();
            
            let count = heading_id_counter.entry(base_id.clone()).or_insert(0);
            let id = if *count == 0 {
                base_id.clone()
            } else {
                format!("{base_id}-{count}")
            };
            *count += 1;
            
            toc_items.push((level, text.clone(), id.clone()));
            
            // Replace the heading in HTML to add the ID
            let old_heading = &cap[0];
            let new_heading = format!(r#"<h{level} id="{id}">{text}</h{level}>"#);
            modified_html = modified_html.replace(old_heading, &new_heading);
        }
        
        // Remove duplicate H1 heading if it matches the page title
        if let Some(title) = title {
            let h1_regex = Regex::new(r"<h1[^>]*>([^<]+)</h1>").unwrap();
            if let Some(cap) = h1_regex.captures(&modified_html) {
                let h1_text = cap[1].trim();
                if h1_text == title {
                    // Remove the first H1 heading that matches the title
                    modified_html = h1_regex.replace(&modified_html, "").to_string();
                }
            }
        }

        // Generate TOC HTML
        let toc_html = if toc_items.is_empty() {
            String::new()
        } else {
            let mut toc = String::from("<ul>");
            for (level, text, id) in toc_items {
                let level_class = match level {
                    1 => "toc-h1",
                    2 => "toc-h2", 
                    3 => "toc-h3",
                    4 => "toc-h4",
                    5 => "toc-h5",
                    6 => "toc-h6",
                    _ => "toc-h1",
                };
                toc.push_str(&format!(
                    "<li class=\"{}\"><a href=\"#{}\">{}</a></li>",
                    level_class, id, text
                ));
            }
            toc.push_str("</ul>");
            toc
        };
        
        (toc_html, modified_html)
    }

    fn generate_page(&self, document: &Document) -> Result<(), Box<dyn std::error::Error>> {
        // Check if TOC is enabled in front matter
        let toc_enabled = document.front_matter.extra
            .get("toc")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let (toc_html, mut html_content) = if toc_enabled {
            self.generate_toc_and_content(&document.content, document.front_matter.title.as_deref())
        } else {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_FOOTNOTES);
            let parser = Parser::new_ext(&document.content, options);
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);
            (String::new(), html_content)
        };

        // Remove duplicate H1 heading if it matches the page title
        if let Some(title) = &document.front_matter.title {
            let h1_regex = Regex::new(r"<h1[^>]*>([^<]+)</h1>").unwrap();
            if let Some(cap) = h1_regex.captures(&html_content) {
                let h1_text = cap[1].trim();
                if h1_text == title {
                    // Remove the first H1 heading that matches the title
                    html_content = h1_regex.replace(&html_content, "").to_string();
                }
            }
        }

        let mut context = Context::new();
        context.insert("title", &document.front_matter.title);
        context.insert("content", &html_content);
        context.insert("date", &document.front_matter.date);
        context.insert("tags", &document.front_matter.tags);
        context.insert("lang", &document.language);
        context.insert("base_name", &document.base_name);
        context.insert("site_title", &self.site_config.get_site_title());
        
        // Add TOC to context if generated
        if !toc_html.is_empty() {
            context.insert("toc", &toc_html);
        }
        
        // Get all page documents for left sidebar
        let page_links: Vec<HashMap<String, String>> = self.documents
            .iter()
            .filter(|doc| doc.language == self.i18n.get_default_language())
            .filter(|doc| doc.file_path.contains("/pages/") || (!doc.file_path.contains("/posts/") && !doc.file_path.contains("index")))
            .map(|doc| {
                let mut page = HashMap::new();
                page.insert("title".to_string(), doc.front_matter.title.as_ref().unwrap_or(&"Untitled".to_string()).clone());
                
                // Generate URL with directory structure
                let source_path = Path::new(&doc.file_path);
                let content_dir = self.source_dir.join("content");
                let scan_dir = if content_dir.exists() { &content_dir } else { &self.source_dir };
                let relative_path = source_path.strip_prefix(scan_dir).unwrap_or(Path::new(""));
                let relative_dir = relative_path.parent().unwrap_or(Path::new(""));
                
                // Calculate relative URL from current document's directory
                let current_source_path = Path::new(&document.file_path);
                let current_relative_path = current_source_path.strip_prefix(scan_dir).unwrap_or(Path::new(""));
                let current_relative_dir = current_relative_path.parent().unwrap_or(Path::new(""));
                
                let url = if current_relative_dir == relative_dir {
                    format!("{}.html", doc.base_name)
                } else if relative_dir == Path::new("") {
                    let depth = current_relative_dir.components().count();
                    let mut path = String::new();
                    for _ in 0..depth {
                        path.push_str("../");
                    }
                    path.push_str(&format!("{}.html", doc.base_name));
                    path
                } else {
                    let depth = current_relative_dir.components().count();
                    let mut path = String::new();
                    for _ in 0..depth {
                        path.push_str("../");
                    }
                    path.push_str(&format!("{}/{}.html", relative_dir.display(), doc.base_name));
                    path
                };
                
                page.insert("url".to_string(), url);
                page
            })
            .collect();
        
        // Sort pages alphabetically by title
        let mut sorted_page_links = page_links;
        sorted_page_links.sort_by(|a, b| a.get("title").unwrap().cmp(b.get("title").unwrap()));
        
        context.insert("page_links", &sorted_page_links);
        
        // Get available translations for this page
        let translations = self.i18n.get_available_translations(&document.base_name);
        let available_translations: Vec<HashMap<String, String>> = translations
            .iter()
            .map(|(lang, _doc)| {
                let mut translation = HashMap::new();
                translation.insert("lang".to_string(), (*lang).clone());
                translation.insert("lang_name".to_string(), self.i18n.get_language_name(lang));
                translation
            })
            .collect();
        
        context.insert("available_translations", &available_translations);
        
        // Get the original file path and convert .md to .html while preserving directory structure
        let source_path = Path::new(&document.file_path);
        let content_dir = self.source_dir.join("content");
        let scan_dir = if content_dir.exists() { &content_dir } else { &self.source_dir };
        let relative_path = source_path.strip_prefix(scan_dir)?;
        let relative_dir = relative_path.parent().unwrap_or(Path::new(""));
        
        // Calculate relative path to home page based on directory depth
        let home_path = if relative_dir == Path::new("") {
            "index.html".to_string()
        } else {
            let depth = relative_dir.components().count();
            let mut path = String::new();
            for _ in 0..depth {
                path.push_str("../");
            }
            path.push_str("index.html");
            path
        };
        context.insert("home_path", &home_path);
        
        // Calculate relative path to assets directory based on directory depth
        let assets_path = if relative_dir == Path::new("") {
            "assets".to_string()
        } else {
            let depth = relative_dir.components().count();
            let mut path = String::new();
            for _ in 0..depth {
                path.push_str("../");
            }
            path.push_str("assets");
            path
        };
        context.insert("assets_path", &assets_path);
        
        // Calculate relative path to feed.xml based on directory depth
        let feed_path = if relative_dir == Path::new("") {
            "feed.xml".to_string()
        } else {
            let depth = relative_dir.components().count();
            let mut path = String::new();
            for _ in 0..depth {
                path.push_str("../");
            }
            path.push_str("feed.xml");
            path
        };
        context.insert("feed_path", &feed_path);
        
        for (key, value) in &document.front_matter.extra {
            // Skip the 'toc' key as we handle it specially
            if key != "toc" {
                context.insert(key, value);
            }
        }

        // Determine content type based on directory structure
        let default_template = if document.file_path.contains("/posts/") {
            "post"
        } else if document.file_path.contains("/pages/") {
            "page"
        } else {
            "page"
        };

        let template_name = if document.front_matter.extra.contains_key("layout") {
            document.front_matter.extra["layout"].as_str().unwrap_or(default_template)
        } else {
            default_template
        };

        let rendered = self.theme.render_page(template_name, &context)?;
        
        let output_filename = if document.language == self.i18n.get_default_language() {
            format!("{}.html", document.base_name)
        } else {
            format!("{}.{}.html", document.base_name, document.language)
        };

        let output_path = self.output_dir.join(relative_dir).join(output_filename);
        
        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(output_path)?;
        file.write_all(rendered.as_bytes())?;

        Ok(())
    }

    fn generate_index(&self) -> Result<(), Box<dyn std::error::Error>> {
        let posts: Vec<_> = self.documents
            .iter()
            .filter(|doc| doc.language == self.i18n.get_default_language())
            .filter(|doc| {
                // Determine content type using same logic as generate_page
                let default_template = if doc.file_path.contains("/posts/") {
                    "post"
                } else if doc.file_path.contains("/pages/") {
                    "page"
                } else {
                    "page"
                };
                
                let template_name = if doc.front_matter.extra.contains_key("layout") {
                    doc.front_matter.extra["layout"].as_str().unwrap_or(default_template)
                } else {
                    default_template
                };
                
                template_name == "post"
            })
            .map(|doc| {
                let mut post = HashMap::new();
                post.insert("title", doc.front_matter.title.as_ref().unwrap_or(&"Untitled".to_string()).clone());
                
                // Generate URL with directory structure
                let source_path = Path::new(&doc.file_path);
                let content_dir = self.source_dir.join("content");
                let scan_dir = if content_dir.exists() { &content_dir } else { &self.source_dir };
                let relative_path = source_path.strip_prefix(scan_dir).unwrap_or(Path::new(""));
                let relative_dir = relative_path.parent().unwrap_or(Path::new(""));
                
                let url = if relative_dir == Path::new("") {
                    format!("{}.html", doc.base_name)
                } else {
                    format!("{}/{}.html", relative_dir.display(), doc.base_name)
                };
                
                post.insert("url", url);
                if let Some(date) = &doc.front_matter.date {
                    post.insert("date", date.to_rfc3339());
                }
                post
            })
            .collect();

        // Get all page documents for left sidebar
        let page_links: Vec<HashMap<String, String>> = self.documents
            .iter()
            .filter(|doc| doc.language == self.i18n.get_default_language())
            .filter(|doc| doc.file_path.contains("/pages/") || (!doc.file_path.contains("/posts/") && !doc.file_path.contains("index")))
            .map(|doc| {
                let mut page = HashMap::new();
                page.insert("title".to_string(), doc.front_matter.title.as_ref().unwrap_or(&"Untitled".to_string()).clone());
                
                // Generate URL with directory structure
                let source_path = Path::new(&doc.file_path);
                let content_dir = self.source_dir.join("content");
                let scan_dir = if content_dir.exists() { &content_dir } else { &self.source_dir };
                let relative_path = source_path.strip_prefix(scan_dir).unwrap_or(Path::new(""));
                let relative_dir = relative_path.parent().unwrap_or(Path::new(""));
                
                let url = if relative_dir == Path::new("") {
                    format!("{}.html", doc.base_name)
                } else {
                    format!("{}/{}.html", relative_dir.display(), doc.base_name)
                };
                
                page.insert("url".to_string(), url);
                page
            })
            .collect();

        // Sort pages alphabetically by title
        let mut sorted_page_links = page_links;
        sorted_page_links.sort_by(|a, b| a.get("title").unwrap().cmp(b.get("title").unwrap()));

        let mut context = Context::new();
        context.insert("posts", &posts);
        context.insert("site_title", &self.site_config.get_site_title());
        context.insert("page_links", &sorted_page_links);
        context.insert("assets_path", "assets"); // Index is at root level
        context.insert("feed_path", "feed.xml"); // Index is at root level

        let rendered = self.theme.render_page("index", &context)?;
        
        let index_path = self.output_dir.join("index.html");
        let mut file = File::create(index_path)?;
        file.write_all(rendered.as_bytes())?;

        Ok(())
    }

    fn generate_feed(&self) -> Result<(), Box<dyn std::error::Error>> {
        let posts: Vec<_> = self.documents
            .iter()
            .filter(|doc| doc.language == self.i18n.get_default_language())
            .filter(|doc| {
                // Determine content type using same logic as generate_page
                let default_template = if doc.file_path.contains("/posts/") {
                    "post"
                } else if doc.file_path.contains("/pages/") {
                    "page"
                } else {
                    "page"
                };
                
                let template_name = if doc.front_matter.extra.contains_key("layout") {
                    doc.front_matter.extra["layout"].as_str().unwrap_or(default_template)
                } else {
                    default_template
                };
                
                template_name == "post"
            })
            .take(20) // Limit to 20 most recent posts
            .collect();

        let site_title = self.site_config.get_site_title();
        let now = chrono::Utc::now();
        
        // Get the most recent post date for the feed's updated timestamp
        let feed_updated = posts.first()
            .and_then(|post| post.front_matter.date)
            .unwrap_or(now);
        
        let mut feed_content = String::new();
        feed_content.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        
        // Add xml:base attribute if base_url is configured
        if let Some(base_url) = self.site_config.get_base_url() {
            feed_content.push_str(&format!("<feed xmlns=\"http://www.w3.org/2005/Atom\" xml:base=\"{}\">\n", escape_xml_url(&base_url)));
        } else {
            feed_content.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
        }
        
        feed_content.push_str(&format!("  <title>{}</title>\n", escape_xml(&site_title)));
        feed_content.push_str("  <link href=\"feed.xml\" rel=\"self\"/>\n");
        feed_content.push_str("  <link href=\"index.html\"/>\n");
        feed_content.push_str(&format!("  <updated>{}</updated>\n", feed_updated.to_rfc3339()));
        feed_content.push_str(&format!("  <id>feed.xml</id>\n"));
        
        for post in posts {
            let title = post.front_matter.title.as_ref().map(String::as_str).unwrap_or("Untitled");
            let date = post.front_matter.date.unwrap_or(now);
            
            // Generate URL with directory structure
            let source_path = Path::new(&post.file_path);
            let content_dir = self.source_dir.join("content");
            let scan_dir = if content_dir.exists() { &content_dir } else { &self.source_dir };
            let relative_path = source_path.strip_prefix(scan_dir).unwrap_or(Path::new(""));
            let relative_dir = relative_path.parent().unwrap_or(Path::new(""));
            
            let url = if relative_dir == Path::new("") {
                format!("{}.html", post.base_name)
            } else {
                format!("{}/{}.html", relative_dir.display(), post.base_name)
            };
            
            // Convert markdown to HTML for content
            let mut options = pulldown_cmark::Options::empty();
            options.insert(pulldown_cmark::Options::ENABLE_TABLES);
            options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
            options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
            let parser = pulldown_cmark::Parser::new_ext(&post.content, options);
            let mut html_content = String::new();
            pulldown_cmark::html::push_html(&mut html_content, parser);
            
            feed_content.push_str("  <entry>\n");
            feed_content.push_str(&format!("    <title>{}</title>\n", escape_xml(title)));
            feed_content.push_str(&format!("    <link href=\"{}\"/>\n", escape_xml_url(&url)));
            feed_content.push_str(&format!("    <id>{}</id>\n", escape_xml_url(&url)));
            feed_content.push_str(&format!("    <updated>{}</updated>\n", date.to_rfc3339()));
            feed_content.push_str(&format!("    <content type=\"html\">{}</content>\n", escape_xml(&html_content)));
            feed_content.push_str("  </entry>\n");
        }
        
        feed_content.push_str("</feed>\n");
        
        let feed_path = self.output_dir.join("feed.xml");
        let mut file = File::create(feed_path)?;
        file.write_all(feed_content.as_bytes())?;

        Ok(())
    }
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_xml_url(text: &str) -> String {
    // For URLs in XML attributes, we only need to escape &, <, >, and quotes
    // Forward slashes are valid and should not be escaped
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

