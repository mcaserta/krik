# Krik - Static Site Generator

Krik is a fast static site generator written in Rust that transforms Markdown files into beautiful, responsive websites with internationalization support and modern theming.

## Project Overview

The software is called `krik` and builds to an executable named `kk`.

## Core Architecture

The project is structured using Rust best practices with the following modules:

- `src/main.rs` - CLI entry point and argument parsing
- `src/generator/mod.rs` - Core site generation logic and HTML output
- `src/parser/mod.rs` - Markdown parsing and front matter extraction
- `src/theme/mod.rs` - Theme system and template rendering
- `src/site/mod.rs` - Site configuration management
- `src/i18n/mod.rs` - Internationalization support

## Current Features (Implemented)

### ✅ Core Functionality
- **Markdown Processing**: Full GitHub Flavored Markdown support with tables, footnotes, strikethrough, and code blocks
- **YAML Front Matter**: Rich metadata support with custom fields
- **HTML5 Output**: Valid, semantic HTML generation
- **Draft Support**: Exclude files from processing with `draft: true` in front matter
- **Directory Structure**: Preserves content organization in generated site
- **Non-Markdown Files**: Automatic copying of images, CSS, and other assets
- **Site Configuration**: Global settings via `site.toml` (excluded from output)

### ✅ Content Types & Templates
- **Blog Posts**: Files in `content/posts/` use post template with tags and navigation
- **Pages**: Files in `content/pages/` or root use page template
- **Automatic Detection**: Content type determined by directory structure
- **Layout Override**: Manual template selection via `layout` field in front matter
- **Template System**: Tera-based templating with unified layout system

### ✅ Theme System
- **File-Based Architecture**: Templates, CSS, and JavaScript in separate files
- **Asset Management**: Automatic copying of theme assets to output directory
- **Light/Dark Mode**: OS preference detection with manual toggle
- **Theme Persistence**: User preferences saved in localStorage
- **Responsive Design**: Mobile-first approach with modern CSS
- **CSS Custom Properties**: Easy color customization

### ✅ Internationalization (i18n)
- **Filename Suffix**: Language detection from `file.lang.md` pattern
- **Language Selector**: Dropdown showing available translations
- **Default Language**: English as fallback with proper language names
- **Translation Links**: Automatic navigation between language versions
- **Supported Languages**: en, it, es, fr, de, pt, ja, zh, ru, ar

### ✅ Advanced Features
- **Table of Contents**: Auto-generated TOC with `toc: true` in front matter
- **Footnote Navigation**: Bidirectional linking with smooth scrolling
- **Scroll-to-Top Button**: Smart visibility based on scroll position
- **Atom Feed**: RFC 4287 compliant feed with xml:base support
- **Sidebar Navigation**: Page links with alphabetical sorting
- **Timestamp Handling**: File modification time with YAML override support

### ✅ Navigation & UX
- **Smart Relative Links**: Depth-aware navigation across directory structures
- **Language Switching**: Seamless transition between translations
- **Theme Toggle**: Fixed-position button with sun/moon icons
- **Smooth Animations**: CSS transitions for theme changes and scrolling
- **Keyboard Accessibility**: Proper ARIA labels and focus handling
- **Mobile Optimization**: Touch-friendly interfaces and responsive layouts

## Configuration

### Site Configuration (`site.toml`)
```toml
title = "Site Title"
base_url = "https://example.com"  # Optional, enables xml:base in feeds
```

### Front Matter Options
```yaml
---
title: "Page Title"
date: 2024-01-15T10:30:00Z  # ISO 8601 format
layout: post  # Template override (post, page, or custom)
tags: ["tag1", "tag2"]  # Array of tags
toc: true  # Enable table of contents
draft: false  # Set to true to exclude from site
custom_field: "value"  # Any additional metadata
---
```

## Content Organization

```
content/
├── site.toml           # Site configuration (not copied to output)
├── posts/              # Blog posts (uses 'post' template)
│   ├── sample.md
│   └── sample.it.md    # Italian translation
├── pages/              # Static pages (uses 'page' template)
│   └── about.md
├── images/             # Static files (copied as-is)
│   └── logo.png
└── any-file.md         # Root level files (uses 'page' template)
```

## Generated Output

```
_site/
├── index.html          # Homepage with post listing
├── feed.xml           # Atom feed with xml:base support
├── assets/            # Theme assets
│   ├── css/main.css   # Stylesheet with scroll-to-top styles
│   └── js/main.js     # JavaScript with footnote & scroll functionality
├── posts/
│   ├── sample.html    # Post with tags, navigation, scroll-to-top
│   └── sample.it.html # Italian translation
└── images/
    └── logo.png       # Static assets preserved
```

## Build & Usage

```bash
# Build the project
cargo build --release

# Generate site from current directory
./target/release/kk

# Generate with custom paths
./target/release/kk --input ./content --output ./_site --theme ./themes/custom
```

## Theme Customization

The default theme includes:
- Responsive CSS with light/dark mode support
- JavaScript for theme switching, footnotes, and scroll-to-top
- Tera templates for index, post, and page layouts
- Modern typography and accessibility features

## Release Workflow

- When releasing the software, automatically:
  - Increment version number in CLI output for `-V` option
  - Update version in `Cargo.toml`
  - Commit changes to git
  - Push to GitHub
  - Publish to crates.io using `cargo publish`
  - **Make sure everything is committed in git before releasing the software**

## Release Process Memories

- Incrementing the version number should be done before checking git for uncommitted changes as the version number needs to be incremented before pushing to github and crates.io