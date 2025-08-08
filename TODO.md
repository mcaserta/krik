## Krik TODO / Roadmap

### ✅ Recently completed (this branch)
- [x] Robust error handling: removed `unwrap/expect` across CLI, server, init, lint, templates, markdown, PDF; logging init no longer panics
- [x] Config loading: `SiteConfig::load_from_path` returns `KrikResult`; generator logs and falls back to defaults
- [x] i18n: centralized `SUPPORTED_LANGUAGES` into a `HashMap`, `I18nManager::get_language_name()` uses it; parser validates via the map
- [x] Templates module split: `templates/` with `context.rs`, `paths.rs`, `select.rs`, `render_page.rs`, `render_index.rs`
- [x] TOC/IDs: use AST path for markdown → TOC/heading IDs; kept an HTML-based helper for backward-compat tests
- [x] Index bug fix: include non-default language posts (e.g., `foo.it.md`) even without a default-language counterpart; choose one per base name preferring default language
- [x] Lint performance: precompile regexes with `once_cell::sync::Lazy`
- [x] Parallelization: scan markdown files in parallel; render pages in parallel; generate PDFs in parallel with unique temp files

### 🔺 High-priority next steps
- [ ] Template error typing: replace string `map_err` in `render_page.rs`/`render_index.rs` with `KrikError::Template` carrying template name and context
- [ ] Dev server modularization: extract watcher to `server/watcher.rs` and network discovery to `server/net.rs`; replace remaining stringy errors with `KrikError::Server`
- [ ] Live reload integration: add a `live_reload` flag in page/index context and include snippet in a base template; remove HTML post-processing injection
- [ ] Incremental build cache: maintain a path → `Document` cache in `SiteGenerator` and update on change to avoid full rescans
- [ ] CLI validation module: centralize path normalization/validation and return typed errors with user-friendly suggestions
- [ ] Adopt `thiserror` for error enums to reduce boilerplate while preserving `KrikError` facade and exit-code mapping

### 🧪 Testing
- [ ] Add integration test: index selection when both default and non-default language variants exist (ensure default wins)
- [ ] Add tests for template render error mapping → `KrikError::Template`
- [ ] Add (feature-gated) tests for dev server watcher failure paths and network listing fallbacks
- [ ] Expand content fixtures to cover nested posts, TOC edge-cases, and PDF toggles

### ⚙️ Performance & concurrency
- [ ] Consider bounding rayon parallelism (e.g., via `RAYON_NUM_THREADS`) to avoid oversubscription when running under the tokio server
- [ ] Batch template context creation to minimize allocs for large sites; evaluate small-object reuse where safe
- [ ] Explore streaming copy for large assets; avoid redundant stat calls in asset copying

### 🧼 Code quality & API
- [ ] Introduce `Theme::builder()` to encapsulate defaults, auto-escape, reload behavior, and error handling
- [ ] Normalize and document all public functions returning `KrikResult<T>`; ensure no silent fallbacks except where explicitly intended
- [ ] Remove any remaining duplicated logic now covered by `i18n::I18nManager` and `templates::paths`

### 📚 Docs & DX
- [ ] Document the new parallel build behavior (env vars, expected CPU usage)
- [ ] Update README and `content/pages/documentation.md` for i18n map, index selection rules, and error handling guidance
- [ ] Add CONTRIBUTING.md with coding standards, testing strategy, and release steps

### 🚀 Release readiness
- [ ] Update CHANGELOG with parallelization, index fix, error-handling improvements
- [ ] Bump version; note performance improvements and safer defaults


