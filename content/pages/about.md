---
title: "About Krik"
---

# About Krik

**Krik** is a fast, modern static site generator written in Rust that transforms Markdown files into beautiful, responsive websites.

## Why Krik?

Static site generators have become essential tools for creating fast, secure websites. Krik stands out by combining:

- **Performance**: Built with Rust for maximum speed and efficiency
- **Simplicity**: Intuitive file-based structure with minimal configuration
- **Features**: Comprehensive feature set including i18n, themes, and feeds
- **Modern Web Standards**: HTML5, responsive design, and accessibility

## Key Features

### Core Functionality
- Full GitHub Flavored Markdown support
- YAML front matter for metadata
- Draft support for work-in-progress content
- Automatic asset copying and management

### Internationalization
- Filename-based language detection (`file.lang.md`)
- Language selector dropdown
- Support for 10+ languages with proper names
- Seamless navigation between translations

### Theme System
- Automatic light/dark mode detection
- Manual theme toggle with persistence
- Responsive, mobile-first design
- CSS custom properties for easy customization

### Advanced Navigation
- Auto-generated table of contents
- Bidirectional footnote navigation
- Smart scroll-to-top button
- Depth-aware relative linking

### Content Features
- Atom feed generation (RFC 4287 compliant)
- Tag support for posts
- Directory-based content organization
- Custom template selection

## Technical Details

Krik is built with modern Rust practices and leverages several excellent crates:

- **pulldown-cmark**: Fast CommonMark parser
- **tera**: Powerful templating engine
- **serde**: Serialization framework
- **chrono**: Date and time handling
- **walkdir**: Recursive directory iteration

## Getting Started

1. **Install**: Build from source with `cargo build --release`
2. **Create Content**: Add Markdown files to a `content/` directory
3. **Configure**: Optional `site.toml` for global settings
4. **Generate**: Run `kk` to generate your site
5. **Deploy**: Upload the `_site/` directory to any web server

## Project Status

Krik is actively developed and includes all the features needed for a modern static site:

✅ All core features implemented  
✅ Full theme system with light/dark modes  
✅ Complete internationalization support  
✅ Advanced navigation and UX features  
✅ Standards-compliant feed generation  
✅ Comprehensive documentation  

The project follows semantic versioning and maintains backward compatibility for stable features.

---

Ready to try Krik? Check out the [Welcome post](../posts/welcome.html) and [Markdown showcase](../posts/markdown-showcase.html) to see more features in action!