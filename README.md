# 🚀 Krik

![Krik Logo](krik.png)

A fast static site generator written in Rust 🦀 that transforms Markdown files
into beautiful, responsive websites with internationalization support and modern
theming ✨

## ✨ Features

### Core

- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with custom fields and layout override
- Draft support via `draft: true` in front matter
- Automatic asset copying and directory structure preservation
- Site configuration via `site.toml`

### Theming & i18n

- Light/dark mode with OS detection and manual toggle
- Language detection from `file.lang.md` pattern (e.g., `post.it.md` → Italian)
- Supported languages (via an internal language map): 62 languages including en,
  it, es, fr, de, pt, ja, zh, ru, ar, and many more
- Index selection rule: for multiple language variants of the same post base
  name, the index shows a single entry preferring the default language; if only
  a non-default language exists (e.g., only `foo.it.md`), it will be included
- Tera-based templating with responsive design

### Advanced

- Development server with live reload and file watching
- RFC 4287 compliant Atom feeds with xml:base support
- XML sitemap generation with multilingual support (`<xhtml:link>` alternate
  language declarations)
- SEO-optimized robots.txt with sitemap reference and bot management
- Table of contents generation with `toc: true`
- Bidirectional footnote navigation with smooth scrolling
- Smart scroll-to-top button and theme-aware syntax highlighting
- **PDF generation** with pandoc and typst engines (`pdf: true` in front matter)
- Language-aware PDF links in HTML templates with responsive design

## 🌐 Demo

See Krik in action! My personal website is built with Krik and showcases
real-world usage:

- **Live Website**: [https://mirkocaserta.com](https://mirkocaserta.com) - A
  complete blog built with Krik
- **Feature Demo**:
  [https://krik.mirkocaserta.com/](https://krik.mirkocaserta.com/) -
  Comprehensive demo showcasing all Krik features including themes, i18n, TOC,
  footnotes, and more
- **Themes Demo**:
  [https://themes.krik.mirkocaserta.com](https://themes.krik.mirkocaserta.com) -
  Gallery showcasing all available Krik themes

The demo site includes example posts demonstrating markdown features, theme
switching, internationalization, and all the advanced capabilities that make
Krik a powerful static site generator.

## 📦 Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform:

```bash
# Linux x64
wget https://github.com/mcaserta/krik/releases/latest/download/kk-linux-x86_64
chmod +x kk-linux-x86_64
sudo mv kk-linux-x86_64 /usr/local/bin/kk

# macOS (Intel)
wget https://github.com/mcaserta/krik/releases/latest/download/kk-macos-x86_64
chmod +x kk-macos-x86_64
sudo mv kk-macos-x86_64 /usr/local/bin/kk

# macOS (Apple Silicon)
wget https://github.com/mcaserta/krik/releases/latest/download/kk-macos-aarch64
chmod +x kk-macos-aarch64
sudo mv kk-macos-aarch64 /usr/local/bin/kk

# Windows (download and add to PATH)
# https://github.com/mcaserta/krik/releases/latest/download/kk-windows-x86_64.exe
```

### From Homebrew (macOS/Linux)

```bash
brew install mcaserta/krik/krik
```

### From Cargo

```bash
cargo install krik
```

### From Source

```bash
git clone https://github.com/mcaserta/krik.git
cd krik
cargo build --release
```

The executable will be available at `target/release/kk`.

## 🔧 Usage

### Initialize New Site

Create a new Krik site with sample content and default theme:

```bash
kk init                     # Initialize in current directory
kk init my-blog             # Initialize in new directory
kk init my-blog --force     # Overwrite existing files
```

### Create Content

Create new blog posts and pages quickly:

```bash
kk post "My Great Post"           # Create new blog post
kk post                           # Create with default title "New post"
kk post "Custom Post" -f my-slug  # Custom filename

kk page "About Us"                # Create new page
kk page                           # Create with default title "New page"
kk page "Contact" -f contact      # Custom filename
```

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

### Lint Content

```bash
kk lint                        # Lint default content directory
kk lint --input ./content      # Lint a specific directory
kk lint --strict               # Treat warnings as errors
kk lint --check-links          # Check for broken HTTP(S) links
kk lint --create-report        # Generate HTML report
kk lint --check-links --create-report --strict  # Full validation with report
```

The linter validates:

- Title: required and non-empty
- Language codes: must match filename suffix (e.g., hello.it.md → it)
- Slugs: filename stem must be slug-like (lowercase, numbers, hyphens)
- Layout: warns on unrecognized values and directory/layout mismatches
- Date: warns if missing for posts; warns if > 1 year in the future
- Tags: array of non-empty strings; warns when tags are not slug-like
- TOC: warns if `toc` is not a boolean
- Duplicate slugs: within the same directory and language
- Duplicate titles: warns within the same directory and language

**Link Rot Scanning** (`--check-links`):

- Parallel HTTP(S) link validation (up to 10 concurrent requests)
- Smart browser header simulation to avoid bot detection
- Real-time progress tracking with comprehensive logging
- Supports redirects and provides detailed error reporting

**HTML Report Generation** (`--create-report`):

- Professional HTML reports with responsive design
- ISO 8601 timestamp filenames (`krik-report-YYYY-MM-DDTHH-MM-SSZ.html`)
- Summary dashboard with visual status indicators
- Detailed sections for errors, warnings, and broken links

Exits non-zero on errors. In `--strict` mode, warnings are also treated as
errors.

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
draft: false # Set to true to skip processing
---
# Your content here
```

Fields: `title`, `date`, `draft`, `layout`, `tags`, `toc`

## Theme System

Tera-based templates with light/dark mode auto-detection and manual toggle.
Templates automatically chosen based on directory (`posts/` → post template,
`pages/` → page template). Override with `layout` field in front matter.

## ⚠️ Error Handling

Krik uses typed errors for clear diagnostics and proper exit codes:

- Central error type: `KrikError` (configuration, I/O, markdown, template,
  theme, server, content, generation)
- Template render errors include the template name and context
- CLI exits with appropriate codes (e.g., config: 2, I/O: 3, markdown: 4,
  template: 5, theme: 6, server: 7, content: 8, generation: 9)

Tips:

- Run with `-v/--verbose` for detailed logs
- Check paths and theme directories; messages include file/template names for
  faster debugging

## 🚀 Deployment

### GitHub Pages

Krik sites can be automatically deployed to GitHub Pages using GitHub Actions.
Create
[.github/workflows/build-and-deploy.yml](.github/workflows/build-and-deploy.yml).

**Setup Steps:**

1. Add the workflow file to your repository
2. Ensure your site content is in the `content/` directory
3. Enable GitHub Pages in repository settings, selecting "Deploy from a branch"
   and choosing `gh-pages`
4. Push to main branch to trigger deployment

The workflow automatically:

- Installs Rust and Krik
- Generates your site with `kk`
- Deploys to the `gh-pages` branch
- Adds `.nojekyll` to prevent Jekyll processing

## Development

```bash
cargo build
cargo test
cargo run -- --input ./content --output ./_site
```

## License

MIT License
