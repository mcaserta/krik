# Krik Project: Gemini AI Assistant Context

This document provides context for the Gemini AI assistant to understand the
Krik static site generator project.

## Project Overview

Krik is a fast and feature-rich static site generator written in Rust. It
transforms Markdown files into modern, responsive websites with built-in support
for internationalization (i18n), theming, and various advanced features.

### Core Technologies

- **Language:** Rust
- **Framework:** The project is a command-line application and does not use a
  specific web framework for its core logic.
- **Templating:** [Tera](https://keats.github.io/tera/) is used for templating,
  allowing for flexible and powerful theme creation.
- **Markdown Processing:**
  [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) is used for
  parsing GitHub Flavored Markdown.
- **Command-Line Interface:** [clap](https://github.com/clap-rs/clap) is used to
  create a user-friendly and powerful command-line interface.
- **Development Server:** [Tokio](https://tokio.rs/) and
  [Warp](https://github.com/seanmonstar/warp) are used to provide a lightweight
  development server with live reload capabilities.

### Architecture

The project is structured into several modules, each responsible for a specific
part of the static site generation process:

- **`cli`:** Defines the command-line interface and its subcommands.
- **`content`:** Manages the parsing and representation of content from Markdown
  files.
- **`error`:** Defines custom error types for robust error handling.
- **`generator`:** Contains the core logic for the static site generation
  pipeline, including scanning, transforming, rendering, and emitting files.
- **`i18n`:** Handles internationalization by detecting language codes in
  filenames.
- **`parser`:** Responsible for parsing Markdown files and their front matter.
- **`server`:** Implements the development server with live reload
  functionality.
- **`site`:** Manages the site configuration from the `site.toml` file.
- **`theme`:** Handles the loading and management of themes.

## Building and Running

### Building the Project

To build the project, use the following command:

```bash
cargo build --release
```

The executable will be available at `target/release/kk`.

### Running the Project

The main executable is `kk`. It provides several subcommands to manage the
static site generation process.

**Generate a site:**

```bash
kk
```

**Start the development server:**

```bash
kk server
```

**Create a new post:**

```bash
kk post "My New Post"
```

**Create a new page:**

```bash
kk page "About Me"
```

**Lint the content:**

```bash
kk lint
```

### Running Tests

To run the test suite, use the following command:

```bash
cargo test
```

## Development Conventions

### Coding Style

The project follows the standard Rust coding style, which is enforced by
`rustfmt`.

### Testing

The project has a comprehensive test suite in the `tests` directory. The tests
cover various aspects of the application, including Markdown parsing, content
generation, and template rendering.

### Contribution Guidelines

The `README.md` file provides detailed instructions on how to contribute to the
project. It also includes information on how to set up the development
environment and run the tests.
