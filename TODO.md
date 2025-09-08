# Krik TODO / Roadmap

## ✅ Recently completed (this branch)

- [x] Robust error handling: removed `unwrap/expect` across CLI, server, init,
      lint, templates, markdown, PDF; logging init no longer panics
- [x] Config loading: `SiteConfig::load_from_path` returns `KrikResult`;
      generator logs and falls back to defaults
- [x] i18n: centralized `SUPPORTED_LANGUAGES` into a `HashMap`,
      `I18nManager::get_language_name()` uses it; parser validates via the map
- [x] Templates module split: `templates/` with `context.rs`, `paths.rs`,
      `select.rs`, `render_page.rs`, `render_index.rs`
- [x] TOC/IDs: use AST path for markdown → TOC/heading IDs; kept an HTML-based
      helper for backward-compat tests
- [x] Index bug fix: include non-default language posts (e.g., `foo.it.md`) even
      without a default-language counterpart; choose one per base name
      preferring default language
- [x] Lint performance: precompile regexes with `once_cell::sync::Lazy`
- [x] Parallelization: scan markdown files in parallel; render pages in
      parallel; generate PDFs in parallel with unique temp files
- [x] Template error typing: `render_page` and `render_index` map template
      errors to `KrikError::Template` with template name and context
- [x] Dev server modularization: extracted `server/watcher.rs` and
      `server/net.rs`; cleaned up imports and warnings
- [x] CLI validation module: centralize path normalization/validation and return
      typed errors with user-friendly suggestions
- [x] Adopt `thiserror` for error enums while preserving `KrikError` facade and
      exit-code mapping
- [x] Dependencies: upgraded `warp` to 0.4.1 (enabled `server` + `websocket`
      features), `notify` to 8.2.0, and `serde` to 1.0.219; verified via
      `cargo test` and `cargo run --`

## 🔺 High-priority next steps

- [x] Incremental build cache: maintain a path → `Document` cache in
      `SiteGenerator` and update on change to avoid full rescans

## 🧪 Testing

- [x] Add integration test: index selection when both default and non-default
      language variants exist (ensure default wins)
- [x] Add tests for template render error mapping → `KrikError::Template`
- [x] Expand content fixtures to cover nested posts, TOC edge-cases, and PDF
      toggles

## ⚙️ Performance & concurrency

- [x] avoid redundant stat calls in asset copying

## 🧼 Code quality & API

- [x] Introduce `Theme::builder()` to encapsulate defaults, auto-escape, reload
      behavior, and error handling
- [x] Normalize public functions to return `KrikResult<T>`; removed silent
      fallbacks in `theme`, `assets`, `server/live_reload`
- [x] Remove remaining duplicated logic covered by `i18n::I18nManager` and
      `templates::paths` (parser uses `I18nManager` for language validation)

## 📚 Docs & DX

- [x] Add CONTRIBUTING.md with coding standards, testing strategy, and release
      steps

## 🎨 Themes

- [ ] new theme based on <https://codepen.io/mikemai2awesome/pen/KKvMZVz>
- [ ] new theme based on <https://mikemai.net/mcss/>
- [ ] new theme based on <https://picocss.com/>

## 🚀 Release readiness

- [ ] Update CHANGELOG with parallelization, index fix, error-handling
      improvements
- [x] Add dependency bumps (`warp` 0.4.1, `notify` 8.2.0, `serde` 1.0.219) to
      CHANGELOG
- [ ] Bump version; note performance improvements and safer defaults
