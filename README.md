# 🚀 Krik

![Krik Logo](krik.png)

A fast static site generator written in Rust 🦀 that transforms Markdown files into beautiful, responsive websites with internationalization support and modern theming ✨

## ✨ Current Features (All Implemented)

### 🎯 Core Functionality
- **Markdown Processing**: Full GitHub Flavored Markdown support including tables, footnotes, strikethrough, and code blocks
- **YAML Front Matter**: Rich metadata support with custom fields and unified layout system
- **HTML5 Output**: Valid, semantic HTML generation with proper structure
- **Draft Support**: Exclude files from processing with `draft: true` in front matter
- **Directory Structure**: Preserves content organization in the generated site
- **Asset Management**: Automatic copying of images, CSS, and other non-markdown files
- **Site Configuration**: Global settings via `site.toml` (excluded from output)
- **Fast Generation**: Built with Rust for optimal performance on large sites  

### 📝 Content Types & Templates
- **Blog Posts**: Files in `content/posts/` automatically use post template with tags and navigation
- **Static Pages**: Files in `content/pages/` or root use page template
- **Automatic Detection**: Content type determined by directory structure
- **Layout Override**: Manual template selection via `layout` field in front matter
- **Template System**: Tera-based templating with consistent styling across page types  

### 🎨 Theme System
- **File-Based Architecture**: Templates, CSS, and JavaScript stored in separate files under `themes/default/`
- **Asset Separation**: CSS and JS files automatically copied to output with relative path linking
- **Light/Dark Mode**: Automatic OS preference detection with manual toggle
- **Theme Persistence**: User preference saved in localStorage
- **Responsive Design**: Mobile-first approach with modern CSS
- **CSS Custom Properties**: Easy color customization and theming
- **Cross-Platform**: Works on Windows, macOS, iOS, iPadOS, Linux, and mobile devices
- **Smooth Transitions**: Animated theme switching with 0.3s transitions  

### 🌍 Internationalization (i18n)
- **Filename Detection**: Language detection from `file.lang.md` pattern
- **Language Selector**: Dropdown showing available translations in navigation
- **Default Language**: English as fallback with proper language names
- **Translation Links**: Automatic navigation between language versions
- **Supported Languages**: en, it, es, fr, de, pt, ja, zh, ru, ar with full language names  

### 🧭 Navigation & UX
- **Smart Navigation**: Depth-aware relative links that work across directory structures
- **Language Switching**: Seamless transition between translations
- **Theme Toggle**: Fixed-position toggle button with sun/moon icons
- **Back to Home**: Automatic home page links on post templates
- **Scroll to Top**: Smart scroll-to-top button that appears only when needed
- **Table of Contents**: Auto-generated TOC with anchor links for long content
- **Footnote Navigation**: Bidirectional linking with smooth scrolling
- **Sidebar Navigation**: Page links with alphabetical sorting  

### 📡 Advanced Features
- **Atom Feed Generation**: RFC 4287 compliant feeds with xml:base support for proper link resolution
- **Table of Contents**: Auto-generated TOC with `toc: true` in front matter
- **Footnote Enhancement**: Clickable footnote references with smooth return links
- **Scroll-to-Top Button**: Smart visibility based on scroll position with smooth animations
- **Timestamp Handling**: File modification time with YAML override support
- **Responsive Tables**: Properly styled tables with alternating row colors  

## 📦 Installation

### From Source

```bash
git clone <repository-url>
cd krik
cargo build --release
```

The executable will be available at `target/release/kk`.

## 🔧 Usage

### Basic Usage

Generate a site from the current directory:

```bash
kk
```

### Advanced Usage

```bash
kk --input /path/to/content --output /path/to/site --theme /path/to/theme
```

### Command Line Options

- `-i, --input <DIR>`: Input directory containing markdown files (default: current directory)
- `-o, --output <DIR>`: Output directory for generated HTML files (default: `_site`)
- `-t, --theme <DIR>`: Theme directory path (optional)  

## 📁 Content Organization

### Directory Structure

```
content/
├── site.toml        # Site configuration (not copied to output)
├── posts/           # Blog posts (uses 'post' template)
│   ├── sample.md
│   ├── sample.it.md # Italian translation
│   └── time-series.md
├── pages/           # Static pages (uses 'page' template)
│   └── about.md
├── images/          # Static files (copied as-is)
│   └── logo.png
└── any-file.md      # Root level files (uses 'page' template)
```

### Site Configuration

Configure your site in `site.toml`:

```toml
title = "My Blog"
base_url = "https://example.com"  # Optional, enables xml:base in feeds
```

- **title**: Site title displayed in navigation and feeds
- **base_url**: Base URL for proper link resolution in Atom feeds (optional)  

### Front Matter

Add metadata to your markdown files using YAML front matter:

```yaml
---
title: "My Blog Post"
date: 2024-01-15T10:30:00Z
layout: post
tags: ["rust", "static-site", "web"]
toc: true
draft: false  # Set to true to skip processing
---

# Your content here
```

#### Draft Support

Mark files as drafts to exclude them from the generated site:

```yaml
---
title: "Work in Progress"
draft: true
---

This content won't appear in the generated site.
```

#### Supported Fields

- **title**: Page/post title (used in HTML title and headers)
- **date**: Publication date in ISO 8601 format (falls back to file modification time)
- **draft**: Skip file from processing when set to `true` (boolean)
- **layout**: Template to use (`post`, `page`, or custom template name)
- **tags**: Array of tags for categorization (displayed on post templates)
- **toc**: Enable table of contents generation (boolean)
- **Any custom field**: Additional metadata accessible in templates  

### 📡 Atom Feed Generation

Krik automatically generates an Atom feed (`feed.xml`) with:

- **RFC 4287 Compliance**: Standards-compliant Atom 1.0 feeds
- **xml:base Support**: Proper base URL handling for relative links in feed content
- **Post Filtering**: Only includes content with `post` template (from `posts/` directory)
- **Recent Posts**: Limited to 20 most recent posts by date
- **Rich Content**: Full HTML content with proper XML escaping
- **Metadata Inclusion**: Post titles, dates, and unique IDs  

The `base_url` field in `site.toml` enables `xml:base` attribute in the feed, ensuring all relative URLs resolve correctly when viewed in feed readers.

### 🌍 Internationalization

Create translations by adding language codes to filenames:

- `sample.md` - Default language (English)
- `sample.it.md` - Italian translation
- `sample.es.md` - Spanish translation  

**Supported language codes**: `en`, `it`, `es`, `fr`, `de`, `pt`, `ja`, `zh`, `ru`, `ar` with full language names

## 🎨 Theme System

### File-Based Architecture

Krik uses a file-based theme system that separates templates, CSS, and JavaScript into individual files for easy customization:

```
themes/default/
├── theme.toml           # Theme configuration
├── templates/           # HTML templates using Tera syntax
│   ├── index.html       # Homepage template
│   ├── page.html        # Page template
│   └── post.html        # Blog post template
└── assets/              # Static assets
    ├── css/
    │   └── main.css     # Main stylesheet with scroll-to-top styles
    └── js/
        └── main.js      # JavaScript with theme, footnote & scroll functionality
```

#### Theme Configuration

The `theme.toml` file defines template mappings:

```toml
name = "default"
version = "1.0.0"
description = "Default Krik theme"

[templates]
page = "page"
post = "post"
index = "index"
```

#### Template System

Templates use the Tera templating engine with access to:

- **Page metadata**: `title`, `date`, `tags`, `toc`, etc.
- **Content variables**: `content`, `lang`, `base_name`
- **Site data**: `posts`, `page_links`, `available_translations`
- **Navigation helpers**: `home_path`, `assets_path`, `feed_path`

#### Asset Management

- CSS and JavaScript files are automatically copied to the output directory
- Templates reference assets using relative paths (`assets/css/main.css`)
- Asset copying preserves directory structure in the generated site  

### Light/Dark Mode

Krik automatically detects your operating system's theme preference and applies the appropriate light or dark theme. The theme system includes:

- **Automatic Detection**: Uses CSS `prefers-color-scheme` media query for OS-independent detection
- **Manual Toggle**: Fixed-position theme toggle button (🌙/☀️) in the top-right corner
- **Persistent Preference**: User's manual theme choice is saved to localStorage
- **Real-time Updates**: Listens for OS theme changes and updates automatically (unless manually overridden)
- **Smooth Transitions**: All color changes animate smoothly over 0.3 seconds  

### Theme Customization

The theme system uses CSS custom properties for easy customization:

```css
:root {
    --bg-color: #ffffff;
    --text-color: #333333;
    --link-color: #0066cc;
    --surface-color: #f5f5f5;
    /* ... more variables */
}

@media (prefers-color-scheme: dark) {
    :root {
        --bg-color: #1a1a1a;
        --text-color: #e0e0e0;
        --link-color: #66b3ff;
        --surface-color: #2a2a2a;
        /* ... dark theme overrides */
    }
}
```

### Cross-Platform Support

The theme detection works across all major platforms:

- **Desktop**: Windows, macOS, Linux
- **Mobile**: iOS, Android
- **Tablets**: iPadOS, Android tablets
- **Browsers**: Chrome, Firefox, Safari, Edge  

## 📄 Templates

### Content Types & Templates

Krik uses a unified layout system with automatic template detection:

#### Automatic Template Selection
- **Posts**: Files in `content/posts/` directory automatically use the `post` template
- **Pages**: Files in `content/pages/` directory automatically use the `page` template
- **Root Content**: Files in content root default to the `page` template

#### Template Features
- **Post Template**: Includes tags display, "Back to Home" navigation, language switcher, and scroll-to-top
- **Page Template**: Clean layout with language switcher (when translations available) and scroll-to-top
- **Index Template**: Homepage with post listing, theme toggle, and scroll-to-top  

#### Manual Layout Override

Specify a custom template in front matter:

```yaml
---
title: "Special Page"
layout: custom
---
```

The `layout` field accepts any template name and overrides directory-based detection.

## 📤 Generated Output

The generator creates:

- **HTML files** with preserved directory structure and proper template application
- **Language-specific files** (e.g., `sample.it.html`) with automatic language detection
- **Static assets** copied with original directory structure maintained
- **Theme assets** (CSS/JS) copied from `themes/default/assets/` to output directory
- **Atom feed** (`feed.xml`) with RFC 4287 compliance and xml:base support
- **Responsive index page** with post listing and theme toggle functionality
- **Table of contents** with anchor links for pages that enable TOC
- **Scroll-to-top buttons** with smart visibility and smooth animations
- **Language switchers** on pages with available translations
- **Theme system** with light/dark mode support and OS preference detection
- **Navigation elements** including smart "Back to Home" links with correct relative paths  

## 🔍 Example

### Input Structure
```
content/
├── site.toml            # Site configuration
├── posts/
│   ├── hello.md
│   └── hello.es.md
└── images/
    └── banner.jpg
```

### Generated Output
```
_site/
├── index.html                 # Homepage with post listing and scroll-to-top
├── feed.xml                   # Atom feed with xml:base support
├── assets/                    # Theme assets
│   ├── css/
│   │   └── main.css          # Stylesheet with scroll-to-top and theme styles
│   └── js/
│       └── main.js           # JavaScript with theme, footnote & scroll functionality
├── posts/
│   ├── hello.html            # Post with tags, navigation, scroll-to-top
│   └── hello.es.html         # Spanish translation with language selector
└── images/
    └── banner.jpg            # Static assets preserved
```

## ⚡ Advanced Features

### 🧭 Smart Navigation
- **Relative Path Calculation**: Links automatically adjust based on directory depth
- **Cross-Directory Navigation**: "Back to Home" links work from any subdirectory level
- **Language-Aware URLs**: Translation links point to correct language variants

### 📝 Markdown Enhancements
- **Tables**: Full support with responsive styling and alternating row colors
- **Code Blocks**: Syntax highlighting ready with proper `<code>` structure
- **Footnotes**: Clickable footnote references with smooth scroll return links
- **Strikethrough**: ~~Text~~ with proper `<del>` tags
- **Task Lists**: Checkbox support for todo items
- **Table of Contents**: Auto-generated TOC from headings with anchor links

#### Table of Contents Generation

Enable TOC in any page or post using front matter:

```yaml
---
title: "Long Article"
toc: true
---

# Introduction
## Section 1
### Subsection 1.1
## Section 2
```

Features:
- **Automatic ID Generation**: Headings get unique IDs for anchor linking
- **Hierarchical Structure**: Preserves heading levels in generated TOC
- **Click Navigation**: TOC links scroll smoothly to target sections
- **Duplicate Handling**: Multiple headings with same text get unique IDs
- **Responsive Design**: TOC adapts to content structure

#### Footnote Enhancement

Footnotes include enhanced navigation:

```markdown
This is a footnote reference[^1].

[^1]: This is the footnote content.
```

Features:
- **Bidirectional Navigation**: Click footnote number to jump to definition
- **Return Links**: Click ↩ symbol to return to original position
- **Smooth Scrolling**: All footnote navigation uses smooth scrolling
- **Proper IDs**: Generates semantic IDs for footnote references and definitions

#### Scroll-to-Top Button

Smart navigation enhancement for long pages:

Features:
- **Smart Visibility**: Only appears when user scrolls >300px from top
- **Fixed Positioning**: Bottom-right corner with responsive placement
- **Smooth Animation**: Fade in/out transitions with scale effects on hover
- **Theme Integration**: Adapts colors to current light/dark theme
- **Keyboard Accessible**: Proper ARIA labels and focus handling
- **Mobile Optimized**: Smaller size and touch-friendly positioning on mobile

Behavior:
- **Hidden by Default**: Button is invisible until scrolling is needed
- **Smooth Scrolling**: Uses `behavior: 'smooth'` for animated return to top
- **Hover Effects**: Scale transformation and enhanced shadow on hover
- **Cross-Platform**: Works on all devices and browsers

### 🚀 Performance Features
- **Fast Rust Engine**: Optimized for large sites with hundreds of pages
- **Parallel Processing**: Multi-threaded file processing where possible
- **Efficient Asset Copying**: Only copies changed files (planned)
- **Minimal CSS**: Lightweight, modern CSS without framework bloat

### ♿ Accessibility
- **Semantic HTML**: Proper heading hierarchy and landmark elements
- **ARIA Labels**: Screen reader friendly navigation and controls
- **Keyboard Navigation**: Full keyboard accessibility for theme toggle
- **Color Contrast**: WCAG compliant color schemes in both light and dark modes

## 🛠️ Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Development Mode
```bash
cargo run -- --input ./content --output ./_site
```

## 📄 License

This project is licensed under the MIT License.