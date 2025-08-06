## Release Workflow

- When releasing the software, automatically:
  - Update the documentation in `README.md`, `CLAUDE.md` and `content`
  - Reformat all modified Markdown files using prettier
  - Update version in `Cargo.toml`
  - Check that the software builds successfully
  - Check that the site under `content` builds successfully
  - Run tests using `cargo test`
  - Make sure the cli outputs the correct version
  - Commit changes to git
  - Make sure everything is committed in git
  - Tag the release in git
  - Push to GitHub
  - Release to GitHub using `gh release create`
  - Publish to crates.io using `cargo publish`
