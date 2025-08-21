# Krik - Static Site Generator

Fast Rust static site generator. Builds to `kk` executable.

## Features

- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with layout override
- Light/dark mode with OS detection
- i18n via `file.lang.md` (62 languages including en, it, es, fr, de, pt, ja,
  zh, ru, ar)
- Development server with live reload (can be disabled with `--no-live-reload`)
- TOC generation, footnotes, scroll-to-top
- Atom feeds, XML sitemap, robots.txt generation
- Mobile-friendly hamburger menu for responsive navigation
- Site initialization with embedded content and themes (`kk init`)
- Content creation commands for posts and pages (`kk post`, `kk page`)
- Link rot scanning with parallel HTTP validation (`kk lint --check-links`)
- HTML report generation for lint results (`kk lint --create-report`)

## Usage

```bash
cargo build --release
./target/release/kk init my-blog
cd my-blog
./target/release/kk                         # Generate once
./target/release/kk server --port 3000      # Dev server with live reload
./target/release/kk server --no-live-reload # Dev server without live reload (mobile-safe)
```

Structure: `content/posts/*.md` → post template, `content/pages/*.md` → page
template

## UI/UX Behaviors

- **Sidebar Pages**: Pages in the sidebar are alphabetically ordered by title
  for consistent navigation
- **Language Support**: Default language (English) content is shown in index and
  sidebar to avoid duplicates
- **Relative Links**: All generated HTML uses relative paths for better
  portability across different hosting environments
- **Title Deduplication**: H1 titles matching frontmatter titles are removed
  from content to prevent duplication
