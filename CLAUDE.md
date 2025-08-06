# Krik - Static Site Generator

Fast Rust static site generator. Builds to `kk` executable.

## Features

- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with layout override
- Light/dark mode with OS detection
- i18n via `file.lang.md` (en, it, es, fr, de, pt, ja, zh, ru, ar)
- Development server with live reload (can be disabled with `--no-live-reload`)
- TOC generation, footnotes, scroll-to-top
- Atom feeds, XML sitemap, robots.txt generation
- Mobile-friendly hamburger menu for responsive navigation
- Site initialization with embedded content and themes (`kk init`)
- Content creation commands for posts and pages (`kk post`, `kk page`)

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

## Release Workflow

- When releasing the software, automatically:
  - Update the documentation in `README.md`, `CLAUDE.md` and `content`
  - Reformat all modified Markdown files using prettier
  - Update version in `Cargo.toml`
  - Check that the software builds successfully
  - Run tests using `cargo test`
  - Make sure the cli outputs the correct version
  - Commit changes to git
  - Make sure everything is committed in git
  - Tag the release in git
  - Push to GitHub
  - Release to GitHub using `gh release create`
  - Publish to crates.io using `cargo publish`

## UI/UX Behaviors

- **Sidebar Pages**: Pages in the sidebar are alphabetically ordered by title
  for consistent navigation
- **Language Support**: Default language (English) content is shown in index and
  sidebar to avoid duplicates
- **Relative Links**: All generated HTML uses relative paths for better
  portability across different hosting environments
- **Title Deduplication**: H1 titles matching frontmatter titles are removed
  from content to prevent duplication
