# Krik - Static Site Generator

Fast Rust static site generator with Markdown, i18n, themes, and modern web features. Builds to executable `kk`.

## Architecture

- `src/main.rs` - CLI entry point
- `src/generator/mod.rs` - Site generation and HTML output
- `src/parser/mod.rs` - Markdown and front matter parsing
- `src/theme/mod.rs` - Theme system and templates
- `src/site/mod.rs` - Site configuration
- `src/i18n/mod.rs` - Internationalization

## Features

### Core
- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with custom fields
- Draft support via `draft: true`
- Automatic asset copying
- Site config via `site.toml`

### Content & Templates
- Posts (`content/posts/`) and pages (`content/pages/`)
- Tera templating with layout override
- Automatic content type detection

### Theme & i18n
- Light/dark mode with OS detection
- Language detection from `file.lang.md`
- Supported: en, it, es, fr, de, pt, ja, zh, ru, ar
- Responsive design with accessibility

### Advanced
- Table of contents with `toc: true`
- Bidirectional footnote navigation
- Scroll-to-top button
- RFC 4287 Atom feeds with xml:base
- Smart relative links across directory depths

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
- Don't forget to also stage and commit the claude.md file when releasing the software