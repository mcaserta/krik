---
title: "Documentation"
toc: true
---

# Krik Documentation

This page provides comprehensive documentation for using Krik, the static site
generator.

## Quick Start

### Installation

#### From Crates.io (Recommended)

```bash
# Install globally from crates.io
cargo install krik

# No additional setup required - themes and content are embedded!
```

#### From Source

```bash
# Clone the repository
git clone <repository-url>
cd krik

# Build the project
cargo build --release

# The executable will be at target/release/kk
```

### Getting Started

#### Initialize a New Site

Create a new Krik site with sample content and default theme:

```bash
kk init my-blog             # Create new site in 'my-blog' directory
cd my-blog
kk server                   # Start development server
```

Or initialize in the current directory:

```bash
kk init                     # Initialize in current directory
kk init --force             # Overwrite existing files
```

#### Create Content

Create new blog posts and pages quickly:

```bash
kk post "My Great Post"           # Create new blog post
kk post                           # Create with default title "New post"
kk post "Custom Post" -f my-slug  # Custom filename

kk page "About Us"                # Create new page
kk page                           # Create with default title "New page"
kk page "Contact" -f contact      # Custom filename
```

#### Development Server

Start the development server with live reload:

```bash
kk server                    # Start on port 3000 with live reload
kk server --port 8080        # Custom port
kk server --no-live-reload   # Disable live reload (useful for mobile devices)
```

#### Generate Static Site

```bash
# Generate site from current directory
kk

# Generate with custom paths
kk --input ./content --output ./_site --theme ./themes/custom
```

## Content Organization

### Directory Structure

```
content/
├── site.toml           # Site configuration
├── posts/              # Blog posts
│   ├── hello.md
│   └── hello.es.md     # Spanish translation
├── pages/              # Static pages
│   └── about.md
└── images/             # Static assets
    └── logo.png
```

### Site Configuration

Create a `site.toml` file in your content directory:

```toml
title = "My Website"
base_url = "https://example.com"  # Optional, for feed generation
```

**Configuration Options:**

- `title`: Site title (displayed in navigation and feeds)
- `base_url`: Base URL for proper Atom feed link resolution

## Front Matter

All markdown files can include YAML front matter for metadata:

```yaml
---
title: "Page Title"
date: 2024-01-15T10:30:00Z
layout: post
tags: ["rust", "web", "static-site"]
toc: true
draft: false
custom_field: "custom value"
---
# Your content here
```

### Front Matter Fields

| Field         | Type     | Description                                 |
| ------------- | -------- | ------------------------------------------- |
| `title`       | String   | Page/post title                             |
| `date`        | ISO 8601 | Publication date (falls back to file mtime) |
| `layout`      | String   | Template to use (`post`, `page`, or custom) |
| `tags`        | Array    | Tags for categorization                     |
| `toc`         | Boolean  | Enable table of contents generation         |
| `draft`       | Boolean  | Skip file from processing when `true`       |
| Custom fields | Any      | Additional metadata accessible in templates |

## Templates and Layouts

### Automatic Template Selection

- **Posts**: Files in `content/posts/` use the `post` template
- **Pages**: Files in `content/pages/` or root use the `page` template
- **Index**: Homepage uses the `index` template

### Manual Template Override

Use the `layout` field in front matter:

```yaml
---
title: "Special Page"
layout: custom
---
```

### Template Features

- **Post Template**: Tags, "Back to Home" link, language selector, scroll-to-top
- **Page Template**: Clean layout, language selector (if translations
  available), scroll-to-top
- **Index Template**: Post listing, theme toggle, scroll-to-top

## Internationalization (i18n)

### Creating Translations

Add language codes to filenames:

- `hello.md` - Default language (English)
- `hello.it.md` - Italian translation
- `hello.es.md` - Spanish translation
- `hello.fr.md` - French translation

### Supported Languages

| Code | Language   | Native Name |
| ---- | ---------- | ----------- |
| `en` | English    | English     |
| `it` | Italian    | Italiano    |
| `es` | Spanish    | Español     |
| `fr` | French     | Français    |
| `de` | German     | Deutsch     |
| `pt` | Portuguese | Português   |
| `ja` | Japanese   | 日本語      |
| `zh` | Chinese    | 中文        |
| `ru` | Russian    | Русский     |
| `ar` | Arabic     | العربية     |

### Language Navigation

Pages with translations automatically show a language selector dropdown in the
navigation bar.

## Advanced Features

### Table of Contents

Enable TOC generation with `toc: true` in front matter:

```yaml
---
title: "Long Article"
toc: true
---
```

**Features:**

- Automatic ID generation for headings
- Hierarchical structure preservation
- Clickable navigation links
- Smooth scrolling to sections

### Footnotes

Krik supports enhanced footnotes with bidirectional navigation:

```markdown
This has a footnote[^1].

[^1]: This is the footnote content.
```

**Features:**

- Click footnote numbers to jump to definitions
- Click return arrows (↩) to return to text
- Smooth scrolling for all footnote navigation

### Scroll-to-Top Button

Automatically appears on longer pages with smart visibility:

- Hidden by default until scrolling >300px
- Fixed position in bottom-right corner
- Smooth scrolling animation
- Theme-aware styling
- Mobile-optimized size and positioning

### Atom Feed Generation

Krik automatically generates an RFC 4287 compliant Atom feed at `feed.xml`:

**Features:**

- Only includes posts (content with `post` template)
- Limited to 20 most recent posts
- Full HTML content with proper XML escaping
- xml:base support when `base_url` is configured
- Proper metadata (titles, dates, IDs)

## Theme System

### Light/Dark Mode

**Automatic Detection:**

- Detects OS theme preference via CSS `prefers-color-scheme`
- Supports all major platforms (Windows, macOS, Linux, iOS, Android)
- Real-time updates when OS theme changes

**Manual Toggle:**

- Theme button (🌙/☀️) in top navigation
- Saves preference to localStorage
- Overrides automatic detection
- Smooth 0.3s transitions

### Customization

The theme uses CSS custom properties for easy customization:

```css
:root {
  --bg-color: #ffffff;
  --text-color: #333333;
  --link-color: #0066cc;
  /* ... more variables */
}
```

## Command Line Reference

### Main Commands

```bash
kk [OPTIONS]              # Generate static site
kk init [DIR]             # Initialize new site
kk post [TITLE]           # Create new blog post
kk page [TITLE]           # Create new page
kk server [OPTIONS]       # Start development server
```

### Global Options

| Option               | Description      | Default          |
| -------------------- | ---------------- | ---------------- |
| `-i, --input <DIR>`  | Input directory  | `content`        |
| `-o, --output <DIR>` | Output directory | `_site`          |
| `-t, --theme <DIR>`  | Theme directory  | `themes/default` |
| `-h, --help`         | Show help        |                  |
| `-V, --version`      | Show version     |                  |

### Init Command

```bash
kk init [DIR] [OPTIONS]
```

| Option        | Description                                          |
| ------------- | ---------------------------------------------------- |
| `[DIR]`       | Directory to initialize (default: current directory) |
| `-f, --force` | Overwrite existing files                             |

### Post/Page Commands

```bash
kk post [TITLE] [OPTIONS]
kk page [TITLE] [OPTIONS]
```

| Option                  | Description                   | Default                 |
| ----------------------- | ----------------------------- | ----------------------- |
| `[TITLE]`               | Content title                 | "New post" / "New page" |
| `-f, --filename <NAME>` | Custom filename (without .md) | Generated from title    |
| `--content-dir <DIR>`   | Content directory path        | `content`               |

### Server Command

```bash
kk server [OPTIONS]
```

| Option               | Description         | Default             |
| -------------------- | ------------------- | ------------------- |
| `-i, --input <DIR>`  | Input directory     | `content`           |
| `-o, --output <DIR>` | Output directory    | `_site`             |
| `-t, --theme <DIR>`  | Theme directory     | `themes/default`    |
| `-p, --port <PORT>`  | Server port         | `3000`              |
| `--no-live-reload`   | Disable live reload | Live reload enabled |

## Generated Output

Krik generates a complete static site with:

- **HTML files**: Preserving directory structure
- **Language variants**: `file.lang.html` for translations
- **Static assets**: Images, CSS, etc. copied as-is
- **Theme assets**: CSS and JavaScript from theme directory
- **Atom feed**: `feed.xml` with proper link resolution
- **Navigation**: TOCs, footnote links, scroll-to-top buttons

### Example Output Structure

```
_site/
├── index.html          # Homepage
├── feed.xml           # Atom feed
├── assets/            # Theme assets
│   ├── css/main.css
│   └── js/main.js
├── posts/
│   ├── hello.html
│   └── hello.es.html  # Translation
├── pages/
│   └── about.html
└── images/
    └── logo.png       # Static assets
```

## Best Practices

### Content Organization

- Use `posts/` for blog entries and time-sensitive content
- Use `pages/` for static pages like About, Contact, etc.
- Keep assets organized in subdirectories
- Use consistent naming conventions for translations

### Front Matter

- Always include a `title` for better navigation
- Use `date` for posts to ensure proper chronological ordering
- Add `tags` to posts for better categorization
- Enable `toc` for longer articles with multiple sections

### Performance

- Optimize images before adding to content
- Use appropriate image formats (WebP when possible)
- Keep individual posts/pages to reasonable lengths
- Use drafts (`draft: true`) for work-in-progress content

### Accessibility

- Use proper heading hierarchy (H1 → H2 → H3)
- Include alt text for images
- Ensure good color contrast in custom themes
- Test with keyboard navigation

## Deployment

### GitHub Pages

You can automatically deploy your Krik site to GitHub Pages using GitHub
Actions. This workflow will build and deploy your site whenever you push to the
main branch.

#### Setup Steps

1. **Create the workflow file**: Add `.github/workflows/build-and-deploy.yml` to
   your repository:

```yaml
name: Build and Deploy Site with Krik

on:
  push:
    branches: [main]
  workflow_dispatch: # Allow manual trigger

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Full clone needed for branch operations
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          default: true
          override: true

      - name: Install Krik
        run: cargo install krik

      - name: Configure Git
        run: |
          git config --global user.name "GitHub Actions"
          git config --global user.email "actions@github.com"

      - name: Run Krik to generate site
        run: kk

      - name: Deploy to gh-pages
        run: |
          mkdir -p /tmp/generated-site
          cp -r _site/* /tmp/generated-site/

          if git show-ref --verify --quiet refs/remotes/origin/gh-pages; then
            git checkout gh-pages
          else
            git checkout --orphan gh-pages
            git rm -rf .
          fi

          find . -maxdepth 1 ! -name '.git' ! -name '.' -exec rm -rf {} \;
          cp -r /tmp/generated-site/* .
          touch .nojekyll

          git add .
          if ! git diff --staged --quiet; then
            git commit -m "Deploy site generated on $(date)"
            git push origin gh-pages
          fi
```

2. **Enable GitHub Pages**:
   - Go to your repository settings
   - Navigate to "Pages" section
   - Under "Source", select "Deploy from a branch"
   - Choose the `gh-pages` branch
   - Select "/ (root)" as the folder
   - Click "Save"

3. **Configure your site**: Ensure your content is in the `content/` directory
   with proper structure

4. **Deploy**: Push to your main branch to trigger the deployment

#### What the Workflow Does

The GitHub Actions workflow automatically:

- **Installs dependencies**: Sets up Rust toolchain and installs Krik from
  crates.io
- **Generates the site**: Runs `kk` to build your static site
- **Creates gh-pages branch**: Sets up the deployment branch if it doesn't exist
- **Deploys files**: Copies generated files to the gh-pages branch
- **Adds .nojekyll**: Prevents GitHub from processing files with Jekyll
- **Pushes changes**: Commits and pushes the generated site

#### Workflow Features

- **Automatic deployment**: Triggers on every push to main branch
- **Manual trigger**: Can be run manually via GitHub Actions interface
- **Branch management**: Handles both new and existing gh-pages branches
- **Clean deployment**: Removes old files before deploying new ones
- **Skip empty deployments**: Only commits when there are actual changes

#### Repository Structure

Your repository should look like this:

```
your-repository/
├── .github/
│   └── workflows/
│       └── build-and-deploy.yml    # Deployment workflow
├── content/                        # Your Krik content
│   ├── site.toml
│   ├── posts/
│   │   └── *.md
│   └── pages/
│       └── *.md
└── README.md
```

After the first successful deployment, your site will be available at:
`https://yourusername.github.io/your-repository-name/`

---

This documentation covers all major features of Krik. For more examples, check
out the other posts and pages in this demo site!
