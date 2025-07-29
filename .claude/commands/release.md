## Release Workflow

- When releasing the software, automatically:
  - Increment version number in CLI output for `-V` option
  - Update version in `Cargo.toml`
  - Commit changes to git
  - **Make sure everything is committed in git before releasing the software**
  - Tag the release in git
  - Push to GitHub
  - Release to GitHub using `gh release create`
  - Publish to crates.io using `cargo publish`
