# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
