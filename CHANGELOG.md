# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.26] - 2025-08-21

### Fixed

- 🧪 **Test Reliability**: Fixed PDF unit tests to work without pandoc/typst
  installed
  - Added `PdfGenerator::new_for_testing()` method for mock instances
  - Tests now use mock generators instead of requiring external tools
  - Improved CI/CD reliability for environments without PDF tools

### Changed

- 🛠️ **PDF Generator Architecture**: Made PDF tools optional in PdfGenerator
  - Changed from required tools to optional with graceful fallback
  - Enhanced error handling with warning messages instead of hard failures
  - Better separation of path utilities from PDF generation functionality

## [0.1.25] - 2025-08-21

### Changed

- ♻️ **Test Organization**: Moved unit tests from source files to dedicated
  tests/ directory
  - Migrated tests from `src/generator/robots.rs` to `tests/robots_test.rs`
  - Migrated tests from `src/generator/pdf.rs` to `tests/pdf_test.rs`
  - Made necessary functions public for test access
  - Improved test isolation and organization following Rust best practices

- 🌍 **I18n Manager Enhancement**: Expanded translation support to all 62
  supported languages
  - Enhanced `translate_string` function to support complete SUPPORTED_LANGUAGES
    set
  - Added translations for "document_information", "document_downloaded_from",
    and "generated_at"
  - Comprehensive language coverage including RTL languages (Arabic, Hebrew,
    Urdu)
  - Support for complex scripts (Chinese, Japanese, Korean, Thai, Hindi, etc.)
  - Improved PDF generation with proper multilingual appendix text
  - Updated documentation to reflect expanded language support (62 languages)

## [0.1.24] - 2025-01-21

### Fixed

- 🔧 **Code Quality**: Fixed all Clippy lint warnings for improved code quality
  - Boxed large error variants to reduce memory footprint and improve
    performance
  - Optimized loop patterns using idiomatic `while let` constructs
  - Fixed function parameter types (`&String` → `&str`) for better API design
  - Eliminated redundant pattern matching and unnecessary allocations
  - Improved error handling patterns with collapsed match expressions
  - Enhanced code readability and maintainability across the entire codebase

- ♻️ **Code Refactoring**: Additional structural improvements for better API
  design
  - Made language utility methods static where appropriate (`I18nManager`
    methods)
  - Removed unused functions to reduce code surface area
  - Improved method signatures and accessibility patterns
  - Enhanced module organization and reduced coupling

- 🔧 **Cross-platform Compilation**: Resolved OpenSSL compilation issues for
  release builds
  - Switched HTTP client to use rustls-tls instead of native OpenSSL for better
    cross-compilation
  - Added Cross.toml configuration for ARM64 cross-compilation support
  - Enhanced build process with verbose logging and error handling

### Added

- 🚀 **Release Infrastructure**: Automated cross-platform binary releases
  - GitHub Actions workflow for building 5 platform binaries (Linux x64/ARM64,
    macOS x64/ARM64, Windows x64)
  - Automated GitHub releases with changelog integration and semantic versioning
  - SHA256 checksums and binary verification for security
  - Deployment workflow optimization using pre-built binaries (~10 minutes → ~30
    seconds)
  - Intelligent fallback to source compilation if binaries unavailable
  - `latest` tag management for easy access to newest releases

- 🎨 **New Manzana Theme**: macOS Tahoe-inspired theme with glassy interface
  - Beautiful glass morphism effects with backdrop blur and saturation
  - Smooth CSS animations and transitions with Apple-like easing curves
  - Custom Inter and JetBrains Mono fonts for modern typography
  - Responsive design with mobile-first approach
  - Touch-friendly interactions optimized for iPad and mobile devices

- 📱 **Enhanced Mobile Navigation**
  - Hamburger menu for page navigation on mobile devices (≤1200px)
  - TOC toggle button for table of contents on mobile (📖 icon)
  - Both menus feature smooth slide-down animations
  - Touch event handling with proper preventDefault for iOS compatibility
  - Smart menu behavior: opening one closes the other automatically

- 🃏 **Glassy Post Cards on Index Page**
  - Redesigned index page with beautiful glass morphism post cards
  - Responsive auto-fitting grid layout (minimum 350px cards)
  - Interactive hover animations with lift and scale effects
  - Gradient top borders and animated tags
  - Calendar emoji with styled date badges
  - Mobile-optimized single column layout

### Improved

- 🎯 **Touch Interface Optimization**
  - Fixed link clicking issues in mobile menus
  - Enhanced touch event handling for better iPad compatibility
  - Improved CSS positioning with fixed viewport positioning
  - Better visual feedback for touch interactions

### Changed

- 🏗️ **Code Architecture Refactoring**: Major refactoring to improve
  maintainability and modularity
  - Broke down complex 238-line `generate_incremental_for_path` function into
    smaller, focused functions
  - Refactored markdown processing pipeline into pure functions with single
    responsibilities
  - Unified `markdown_to_html` and `markdown_to_html_with_toc` into single
    function using AST parser
  - Split template rendering into modular components for better testability
  - Eliminated internal state mutations in favor of pure function patterns
  - All functions now pass state as parameters rather than maintaining internal
    state
  - Improved error handling with standardized error creation functions

## [0.1.23] - 2025-08-12

### Added

- 🔗 **Link Rot Scanning**: New `--check-links` option for lint command
  - Parallel HTTP link validation with up to 10 concurrent requests
  - Smart browser header simulation to avoid bot detection (fixes 403 errors)
  - Comprehensive logging with real-time progress tracking
  - Support for both HTTP and HTTPS links with redirect following
  - Detailed error reporting showing file paths and line numbers

- 📄 **HTML Report Generation**: New `--create-report` option for lint command
  - Professional HTML reports with responsive design
  - ISO 8601 timestamp format in filenames
    (`krik-report-YYYY-MM-DDTHH-MM-SSZ.html`)
  - Beautiful gradient headers and color-coded status indicators
  - Summary dashboard with files scanned, errors, warnings, and broken links
  - Detailed sections for errors, warnings, and broken links with syntax
    highlighting

### Changed

- 🏗️ **Modular Architecture**: Refactored lint module for better maintainability
  - Split 866-line `src/lint.rs` into focused modules:
    - `lint/core.rs` (345 lines) - Core linting functionality
    - `lint/link_checker.rs` (217 lines) - HTTP link validation
    - `lint/report_generator.rs` (347 lines) - HTML report generation
  - Maintained full backward compatibility with existing APIs
  - Improved code organization and separation of concerns

- ⚡ **Performance Improvements**:
  - Link checking now ~8x faster with parallel processing
  - Reduced scan time from ~17 seconds to ~2 seconds for typical content

### Fixed

- 🛡️ **HTTP 403 Prevention**: Added realistic browser headers to avoid bot
  detection
- 📁 **Git Ignore**: Added `krik-report-*.html` to `.gitignore` to prevent
  committing generated reports

## [0.1.22] - 2025-01-11

### Added

- 📖 **CONTRIBUTING.md**: Added comprehensive contribution guidelines
  - Detailed coding standards and Rust style guidelines
  - Complete testing strategy with unit, integration, and smoke tests
  - Step-by-step release process documentation
  - Development environment setup instructions
  - Pull request and code review guidelines
- 🎨 **Onyx Theme**: Enhanced mobile responsiveness and fixed content width
  issues
  - Improved mobile navigation and responsive design
  - Fixed content width constraints for better readability
  - Enhanced theme stability and cross-device compatibility

## [0.1.21] - 2025-01-29

### Added

- 🎨 **Aurora Theme**: Added new elegant editorial theme with crisp typography
  - Clean, editorial design focused on readability and content presentation
  - Enhanced typography with improved spacing and font hierarchy
  - Responsive design with mobile-optimized navigation
  - Dark mode support with theme-aware color scheme
  - Sidebar table of contents support
  - Professional, blog-friendly aesthetic

## [0.1.20] - 2025-01-29

### Added

- 🎨 **Solstice Theme**: Added new modern dark theme with gradient backgrounds
  and enhanced typography
  - Beautiful dark-first design with light mode support
  - Enhanced code syntax highlighting with JetBrains Mono font
  - Smooth gradients and modern UI elements
  - Responsive design with mobile-optimized navigation

### Fixed

- 🔧 **Footnote Return Links**: Fixed footnote return links functionality in
  solstice theme
  - Return links (↩) now properly navigate back to footnote references
  - Smooth scrolling behavior matches default theme
  - JavaScript properly creates reference IDs for bidirectional navigation

## [0.1.19] - 2025-01-29

### Fixed

- 🔧 **Language Switching**: Fixed regression in language switching
  functionality
  - Restored proper language switching behavior that was broken in recent
    updates
  - Improved stability and reliability of language navigation

## [0.1.18] - 2025-08-10

### Added

- 🌐 **Language Selector Fix**: Fixed language selector to correctly navigate
  between language variants of the same page
  - Language selector now points to correct same-page variants (e.g., `cv.html`
    ↔ `cv.it.html`)
  - Previously always pointed to `index.*.html` files regardless of current page
  - Works for both posts and pages with language variants
  - Files without language variants correctly hide the language selector

### Fixed

- 🔄 **Development Server Language Variants**: Fixed incremental generation in
  dev mode for language variant files
  - When editing `welcome.it.md`, both `welcome.html` and `welcome.it.html` are
    now properly regenerated
  - Incremental builds now detect and regenerate all related language variants
    automatically
  - Added language variant detection logic to identify files sharing the same
    base name
  - Development server now maintains consistency across all language versions
    during editing

### Technical Improvements

- Enhanced `generate_incremental_for_path()` with language variant awareness
- Added `find_language_variants()` helper function for detecting related
  language files
- Improved template context to pass translation data to JavaScript via
  `window.krikTranslations`
- Updated `switchLanguage()` JavaScript function to use template-provided paths
  instead of manual construction

## [0.1.17] - 2025-08-10

### Added

- 🎨 **Theme Configuration**: Added support for `theme` field in `site.toml`
  - Configure themes directly in your site configuration file
  - Example: `theme = "themes/custom"` in your `site.toml`

### Fixed

- 🔧 **Theme Resolution**: Fixed theme loading priority system with proper
  override logic
  - **Priority 1**: Command line `--theme` option (highest priority)
  - **Priority 2**: `theme` field in `site.toml` configuration
  - **Priority 3**: Default theme (`themes/default`) fallback
  - Previously, themes specified in `site.toml` were completely ignored
- 📝 **Logging**: Enhanced theme selection logging for better debugging
  - Clear messages indicate which theme source is being used
  - Verbose mode (`-v/--verbose`) shows theme resolution decisions

### Changed

- 🏗️ **Site Configuration**: Extended `SiteConfig` struct to include optional
  `theme` field
- 🎯 **CLI Commands**: Updated both `generate` and `server` commands to use
  unified theme resolution
- 🔍 **Error Handling**: Improved theme validation with context-specific error
  messages

## [0.1.16] - 2025-08-09

### Changed

- 📦 **Dependencies**: Updated to latest versions for improved security and
  performance
  - Bumped `warp` to `0.4.1` with `server` and `websocket` features enabled for
    better development server functionality
  - Bumped `notify` to `8.2.0` for enhanced file watching capabilities
  - Bumped `serde` to `1.0.219` for improved serialization performance
- 🏗️ **Templates**: Modularized default theme with base.html template
  - Refactored page/index/post templates to extend base template
  - Improved header/sidebar/TOC reusability across templates
  - Enhanced template organization and maintainability
- 🚀 **Development**: Improved server binding and incremental build system
  - Fixed `warp::serve().run()` to ensure server binds correctly
  - Persistent generator cache for incremental builds
  - Incremental PDF generation with toggle add/remove functionality
  - Removed live-reload JS from templates, now relies on Rust-side injection

### Fixed

- ✅ **Build System**: Verified all dependency updates with comprehensive
  testing
- 🔧 **Server**: Fixed development server binding issues for more reliable local
  development
- 📄 **Templates**: Improved template inheritance and code reusability

### Performance

- ⚡ **Incremental Builds**: Added persistent cache system for faster rebuild
  cycles
- 📊 **Asset Handling**: Reduced redundant stat calls during asset processing
- 🔄 **PDF Generation**: Optimized PDF generation with incremental updates

## [0.1.15] - 2025-01-27

### Added

- Tag chips now display on post cards in the index page for better content
  categorization
- Tags are rendered as small rounded pills below the post title and date

### Improved

- Enhanced visual hierarchy on the index page with better tag placement
- Improved post card layout aesthetics and usability

### Removed

- Unused `.post-cta` CSS class that was no longer referenced

## [0.1.14] - 2025-08-08

### Fixed

- Addressed various clippy warnings and minor code quality issues
- Polished README and documentation for clarity

### Changed

- Improved logging defaults and CLI help text
- Minor performance tweaks in generation pipeline

## [0.1.13] - 2025-08-07

### Added

- 📝 Comprehensive logging system with structured output
  - `--verbose` flag for debug-level logging across all commands
  - Module-specific logging with spans for better organization
  - Rich context including file paths, line numbers, and timestamps
  - Color support with automatic terminal detection
  - Multiple log levels: INFO, DEBUG, WARN, ERROR
- 🔧 Enhanced CLI experience with better feedback
  - Detailed progress reporting during site generation
  - File processing statistics and error tracking
  - Improved debugging capabilities for troubleshooting

### Changed

- Replaced all `println!` and `eprintln!` calls with proper logging
- Enhanced error reporting with structured logging format
- Better user experience with configurable verbosity levels

### Fixed

- Consistent logging across all modules and commands
- Improved error context and debugging information

## [0.1.12] - 2025-08-07

### Added

- 📄 PDF generation support using pandoc and typst engines
  - Documents with `pdf: true` in front matter generate PDF versions
  - Cross-platform executable detection for pandoc/typst
  - Proper image path resolution for PDF generation
  - Conditional PDF appendix with download URL and generation timestamp
  - Multi-language appendix translations (9 languages supported)
- 🔗 PDF download links in HTML templates
  - PDF download button (📄) positioned next to theme switcher
  - Language-aware PDF filenames for translated documents
  - Only appears for documents with `pdf: true` in front matter
  - Responsive design matching existing theme system

### Changed

- Language detection now uses filename-based logic consistently across PDF and
  HTML generators
- Template context now includes `pdf` field from front matter
- Enhanced error handling and path resolution for PDF generation

## [0.1.11] - 2025-08-05

### Added

- Comprehensive error handling system with detailed error contexts
- Enhanced error recovery mechanisms across all modules
- Improved diagnostics for troubleshooting issues

### Changed

- Refactored description rendering from templates to native Rust implementation
- Improved performance and reliability of content processing
- Better error messages for debugging

### Fixed

- Various stability improvements in content generation
- Enhanced error reporting for better user experience

## [0.1.10] - 2025-08-05

### Added

- Enhanced SEO meta tags support
- Improved Open Graph integration
- Better social media sharing capabilities

### Changed

- Updated documentation for robots.txt and SEO features
- Refined meta tag generation logic

### Fixed

- SEO meta tag consistency across templates
- Open Graph property handling

## [0.1.9] - 2025-07-27

### Added

- Alphabetical sidebar ordering for consistent navigation
- Improved page organization in sidebar

### Changed

- Pages in sidebar now display in alphabetical order by title
- Better navigation user experience

## [0.1.8] - 2025-07-27

### Added

- Full relative links support for better portability
- Enhanced cross-platform compatibility

### Fixed

- Path resolution issues across different hosting environments
- Link generation for various deployment scenarios

## [0.1.7] - 2025-07-27

### Added

- Comprehensive module refactoring and organization
- Improved CLI structure and commands
- Better separation of concerns across modules

### Changed

- Complete reorganization of CLI and generator modules
- Enhanced code maintainability and extensibility
- Improved project structure

### Fixed

- Various bugs related to module organization
- CLI command handling improvements

## [0.1.6] - 2025-07-26

### Added

- 🚀 Site initialization with embedded content and themes (`kk init`)
- 📝 Content creation commands (`kk post`, `kk page`)
- Embedded default theme and sample content
- Quick project setup capabilities

### Changed

- Enhanced project bootstrapping experience
- Improved getting started workflow

## [0.1.5] - 2025-07-26

### Added

- `--no-live-reload` option for mobile-safe development
- 🍔 Mobile hamburger menu for responsive navigation
- Better mobile device compatibility

### Changed

- Improved mobile user experience
- Enhanced responsive design

## [0.1.4] - 2025-07-25

### Added

- 🎨 Theme-aware syntax highlighting with Prism.js
- Dynamic code block highlighting based on light/dark theme
- Support for multiple programming languages

### Changed

- Enhanced code presentation in blog posts
- Better developer experience for technical content

## [0.1.3] - 2025-07-25

### Fixed

- Atom feed timestamp generation now uses actual post dates
- Improved feed validity and compatibility

### Changed

- Better RSS/Atom feed standards compliance

## [0.1.2] - 2025-07-25

### Added

- Cargo install instructions in README
- Improved installation documentation

### Fixed

- CLI input directory default now matches documentation
- Better consistency between docs and behavior

## [0.1.1] - 2025-07-25

### Fixed

- HTML escaping issues in URLs resolved
- XML escaping for URLs in feeds and sitemaps
- Template auto-escaping configuration improvements

### Changed

- Better URL handling across all generated content
- Enhanced XML/HTML output validation

## [0.1.0] - 2025-07-25

### Added

- 🚀 Initial release of Krik static site generator
- 📝 GitHub Flavored Markdown support with tables, footnotes, code blocks
- 🎨 Light/dark mode with OS detection and manual toggle
- 🌍 Internationalization via `file.lang.md` filename pattern
- 🔄 Development server with live reload and file watching
- 📑 Table of contents generation with `toc: true`
- 📰 Atom feed generation for posts
- 🗺️ XML sitemap with multilingual support
- 🤖 robots.txt generation
- 🎨 Responsive theme with mobile hamburger menu
- 📱 Mobile-friendly design and navigation
- ⬆️ Scroll-to-top functionality
- 🏗️ Template system with Tera templating engine
- 📊 Front matter support with custom fields
- 🚀 Fast Rust-powered generation
- 🔧 Configurable via `site.toml`

### Core Features

- Static site generation from Markdown files
- Multi-language content support (en, it, es, fr, de, pt, ja, zh, ru, ar)
- Responsive design with automatic theme switching
- SEO-optimized with meta tags and Open Graph support
- Development server for local testing
- Cross-platform compatibility

---

## Contributing

When adding entries to this changelog:

1. **Add unreleased changes** to the `[Unreleased]` section
2. **Use conventional commit prefixes**: feat:, fix:, docs:, chore:, etc.
3. **Include emojis** for better visual organization
4. **Group by type**: Added, Changed, Deprecated, Removed, Fixed, Security
5. **Reference issues/PRs** where applicable

For more details on releases, see the
[GitHub Releases](https://github.com/mcaserta/krik/releases) page.
