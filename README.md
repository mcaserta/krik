# 🚀 Krik

![Krik Logo](krik.png)

A fast static site generator written in Rust 🦀 that transforms Markdown files into beautiful, responsive websites with internationalization support and modern theming ✨

## ✨ Features

### Core
- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with custom fields and layout override
- Draft support via `draft: true` in front matter
- Automatic asset copying and directory structure preservation
- Site configuration via `site.toml`

### Theming & i18n
- Light/dark mode with OS detection and manual toggle
- Language detection from `file.lang.md` pattern
- Supported languages: en, it, es, fr, de, pt, ja, zh, ru, ar
- Tera-based templating with responsive design

### Advanced
- Development server with live reload and file watching
- RFC 4287 compliant Atom feeds with xml:base support
- Table of contents generation with `toc: true`
- Bidirectional footnote navigation with smooth scrolling
- Smart scroll-to-top button and theme-aware syntax highlighting  

## 📦 Installation

### From Source

```bash
git clone <repository-url>
cd krik
cargo build --release
```

The executable will be available at `target/release/kk`.

### Global Install

```bash
cargo install krik
```

Copy themes from the repository to your site root:
```bash
cp -r themes/ /path/to/your/site/
```

## 🔧 Usage

### Basic Usage

Generate a site from the current directory:

```bash
kk
```

### Development Server

```bash
kk server                    # Start on port 3000 with live reload
kk server --port 8080        # Custom port
kk server --no-live-reload   # Disable live reload (useful for mobile devices)
```

Features: Live reload, file watching, multi-interface binding, network discovery

### Production Build

```bash
kk --input /path/to/content --output /path/to/site --theme /path/to/theme
```

### Options

- `-i, --input <DIR>`: Input directory (default: `content`)
- `-o, --output <DIR>`: Output directory (default: `_site`)
- `-t, --theme <DIR>`: Theme directory (default: `themes/default`)
- `-p, --port <PORT>`: Server port (default: `3000`)
- `--no-live-reload`: Disable live reload functionality (server subcommand only) 

## 📁 Content Organization

### Directory Structure

```
content/
├── site.toml        # Site configuration (not copied to output)
├── posts/           # Blog posts (uses 'post' template)
│   ├── sample.md
│   ├── sample.it.md # Italian translation
│   └── time-series.md
├── pages/           # Static pages (uses 'page' template)
│   └── about.md
├── images/          # Static files (copied as-is)
│   └── logo.png
└── any-file.md      # Root level files (uses 'page' template)
```

### Site Configuration

```toml
title = "My Blog"
base_url = "https://example.com"  # Optional, for feeds
```  

### Front Matter

Add metadata to your markdown files using YAML front matter:

```yaml
---
title: "My Blog Post"
date: 2024-01-15T10:30:00Z
layout: post
tags: ["rust", "static-site", "web"]
toc: true
draft: false  # Set to true to skip processing
---

# Your content here
```

Fields: `title`, `date`, `draft`, `layout`, `tags`, `toc`  



## Theme System

Tera-based templates with light/dark mode auto-detection and manual toggle. Templates automatically chosen based on directory (`posts/` → post template, `pages/` → page template). Override with `layout` field in front matter.

## Development

```bash
cargo build
cargo test
cargo run -- --input ./content --output ./_site
```

## License

MIT License