# Krik - Static Site Generator

Fast Rust static site generator. Builds to `kk` executable.

## Features

- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with layout override
- Light/dark mode with OS detection
- i18n via `file.lang.md` (en, it, es, fr, de, pt, ja, zh, ru, ar)
- Development server with live reload (can be disabled with `--no-live-reload`)
- TOC generation, footnotes, scroll-to-top
- Atom feeds, XML sitemap, and robots.txt generation
- XML sitemap with multilingual support (`<xhtml:link>` alternate language declarations)
- SEO-optimized robots.txt with sitemap reference and bot management
- Mobile-friendly hamburger menu for responsive navigation
- Site initialization with embedded content and themes (`kk init`)
- Content creation commands for posts and pages (`kk post`, `kk page`)

## Usage

```bash
cargo build --release
./target/release/kk init my-blog
cd my blog
./target/release/kk                         # Generate once
./target/release/kk server --port 3000      # Dev server with live reload
./target/release/kk server --no-live-reload # Dev server without live reload (mobile-safe)
```

Structure: `content/posts/*.md` → post template, `content/pages/*.md` → page
template

## Release Workflow

- When releasing the software, automatically:
  - Increment version number in CLI output for `-V` option
  - Update version in `Cargo.toml`
  - Commit changes to git
  - Tag the release in git
  - Push to GitHub
  - Release to GitHub using `gh release create`
  - Publish to crates.io using `cargo publish`
  - **Make sure everything is committed in git before releasing the software**

## UI/UX Behaviors

- **Sidebar Pages**: Pages in the sidebar are alphabetically ordered by title
  for consistent navigation
- **Language Support**: Default language (English) content is shown in index and
  sidebar to avoid duplicates
- **Relative Links**: All generated HTML uses relative paths for better
  portability across different hosting environments
- **Title Deduplication**: H1 titles matching frontmatter titles are removed
  from content to prevent duplication

## Release Process Memories

- Incrementing the version number should be done before checking git for
  uncommitted changes as the version number needs to be incremented before
  pushing to github and crates.io
- Don't forget to also stage and commit the claude.md file when releasing the
  software

## Documentation Maintenance

- When updating the documentation, also update the krik site contents because they also contain documentation and examples