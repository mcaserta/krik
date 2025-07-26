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
- Theme-aware syntax highlighting with Prism.js (100+ languages)

## Usage
```bash
cargo build --release
./target/release/kk  # Uses content/ → _site/
```

## Structure
```
content/posts/*.md → post template
content/pages/*.md → page template  
site.toml → config (title, base_url)
file.lang.md → translations
```

Front matter: title, date, layout, tags, toc, draft

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