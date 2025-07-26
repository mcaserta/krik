# Krik - Static Site Generator

Fast Rust static site generator. Builds to `kk` executable.

## Features
- GitHub Flavored Markdown with tables, footnotes, code blocks
- YAML front matter with layout override
- Light/dark mode with OS detection
- i18n via `file.lang.md` (en, it, es, fr, de, pt, ja, zh, ru, ar)
- Development server with live reload (can be disabled with `--no-live-reload`)
- TOC generation, footnotes, scroll-to-top
- Atom feeds, syntax highlighting
- Mobile-friendly hamburger menu for responsive navigation

## Usage
```bash
cargo build --release
./target/release/kk                         # Generate once
./target/release/kk server --port 3000      # Dev server with live reload
./target/release/kk server --no-live-reload # Dev server without live reload (mobile-safe)
```

Structure: `content/posts/*.md` → post template, `content/pages/*.md` → page template

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

## Release Process Memories

- Incrementing the version number should be done before checking git for uncommitted changes as the version number needs to be incremented before pushing to github and crates.io
- Don't forget to also stage and commit the claude.md file when releasing the software