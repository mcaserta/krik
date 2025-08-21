---
title: "About Krik"
pdf: true
---

# About Krik

![Krik logo](../images/krik.png)

**Krik** is a fast, modern static site generator written in Rust that transforms
Markdown files into beautiful, responsive websites with built-in development
tools and content management features.

## Why Krik?

Static site generators have become essential tools for creating fast, secure
websites. Krik stands out by combining:

- **Performance**: Built with Rust for maximum speed and efficiency
- **Developer Experience**: Embedded content, live reload server, and quick
  content creation
- **Zero Dependencies**: Complete sites with embedded themes - no git repo
  required
- **Simplicity**: Intuitive file-based structure with minimal configuration
- **Features**: Comprehensive feature set including i18n, themes, feeds, and
  development tools
- **Modern Web Standards**: HTML5, responsive design, and accessibility

## Key Features

### Development Tools

- **Site initialization**: `kk init` creates complete sites with embedded
  content and themes
- **Content creation**: `kk post` and `kk page` commands for instant content
  generation
- **Development server**: Live reload with file watching and mobile-safe mode
- **One-command setup**: No external dependencies or git repositories needed

### Core Functionality

- Full GitHub Flavored Markdown support with tables, footnotes, and code blocks
- YAML front matter for rich metadata and layout control
- Draft support for work-in-progress content (`draft: true`)
- Automatic asset copying and directory structure preservation
- Site configuration via `site.toml`

### Internationalization

- Filename-based language detection (`file.lang.md`)
- Automatic language selector dropdown
- Support for 10+ languages with proper native names
- Seamless navigation between translations
- Multi-language content organization

### Theme System

- Automatic light/dark mode detection via OS preferences
- Manual theme toggle with localStorage persistence
- Responsive, mobile-first design with hamburger menu
- CSS custom properties for easy customization
- Theme-aware syntax highlighting

#### Theme Gallery

Explore all available themes in the live gallery:

- **Themes Demo**:
  [https://themes.krik.mirkocaserta.com](https://themes.krik.mirkocaserta.com)

### Advanced Navigation & UX

- Auto-generated table of contents with `toc: true`
- Bidirectional footnote navigation with smooth scrolling
- Smart scroll-to-top button with visibility controls
- Depth-aware relative linking across directory structures
- Mobile-optimized touch interfaces

### Content Features

- RFC 4287 compliant Atom feed generation with xml:base support
- Tag support for post categorization
- Directory-based content organization (posts vs pages)
- Custom template selection via front matter
- Timestamp handling with file modification time fallback

## Installation & Usage

### Quick Start (Recommended)

```bash
# Install from crates.io
cargo install krik

# Create a new site
kk init my-blog
cd my-blog

# Start development server
kk server

# Create content
kk post "My First Post"
kk page "About Me"
```

### Project Links

- **Source Code**: [GitHub Repository](https://github.com/mcaserta/krik)
- **Package**: [crates.io](https://crates.io/crates/krik)
- **Issues & Feature Requests**:
  [GitHub Issues](https://github.com/mcaserta/krik/issues)
- **Releases**: [GitHub Releases](https://github.com/mcaserta/krik/releases)

## Technical Details

Krik is built with modern Rust practices and leverages several excellent crates:

- **pulldown-cmark**: Fast CommonMark parser with GitHub extensions
- **tera**: Powerful Jinja2-inspired templating engine
- **serde**: Serialization framework for YAML and JSON handling
- **chrono**: Date and time handling with timezone support
- **walkdir**: Recursive directory iteration for file processing
- **include_dir**: Compile-time file embedding for zero-dependency deployment
- **clap**: Modern CLI argument parsing with subcommands
- **warp**: Fast web server for development mode with live reload

## Project Status

Krik v0.1.5+ is feature-complete and includes all the tools needed for modern
static site development:

✅ **Complete Development Workflow**: Init → Create → Develop → Deploy  
✅ **Embedded Content & Themes**: No external dependencies required  
✅ **Content Creation Tools**: Quick post and page generation  
✅ **Live Development Server**: File watching with mobile-safe options  
✅ **Full Theme System**: Light/dark modes with responsive design  
✅ **Complete Internationalization**: Multi-language support with navigation  
✅ **Advanced Navigation**: TOC, footnotes, scroll-to-top, mobile menu  
✅ **Standards Compliance**: Valid HTML5, RFC 4287 feeds, accessibility  
✅ **Comprehensive Documentation**: Getting started guides and references

The project follows semantic versioning and maintains backward compatibility for
stable features. Active development continues with focus on performance,
usability, and developer experience.

## Contributing

Krik is open source and welcomes contributions! Whether you're reporting bugs,
suggesting features, or submitting code:

1. Check the [GitHub Issues](https://github.com/mcaserta/krik/issues) for
   existing discussions
2. Fork the repository and create a feature branch
3. Test your changes thoroughly
4. Submit a pull request with clear description

## License

Krik is released under the MIT License. See the
[LICENSE file](https://github.com/mcaserta/krik/blob/main/LICENSE) for details.

---

Ready to try Krik? Check out the
[Getting Started Guide](../posts/getting-started-guide.html) for a complete
tutorial, or explore the [Documentation](documentation.html) for detailed
feature references!
