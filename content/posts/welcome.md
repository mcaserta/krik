---
title: "Welcome to Krik"
date: 2024-01-15T10:00:00Z
tags: ["welcome", "static-site", "rust"]
toc: true
---

# Welcome to Krik

Welcome to the **Krik** static site generator! This post demonstrates many of the features available in this fast, Rust-powered static site generator.

## Table of Contents

This post has a table of contents enabled via `toc: true` in the front matter. You should see a TOC in the sidebar with clickable links to each section.

## Markdown Features

Krik supports full **GitHub Flavored Markdown** with many enhancements:

### Text Formatting

You can use *italic text*, **bold text**, ~~strikethrough~~, and `inline code`.

### Lists

Unordered lists:
- First item
- Second item
  - Nested item
  - Another nested item
- Third item

Ordered lists:
1. First step
2. Second step
3. Third step

### Code Blocks

```rust
fn main() {
    println!("Hello, Krik!");
}
```

```javascript
// Theme toggle functionality
function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
    const newTheme = currentTheme === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
}
```

### Tables

| Feature | Status | Description |
|---------|--------|-------------|
| Markdown | ✅ | Full GFM support |
| Themes | ✅ | Light/dark mode |
| i18n | ✅ | Multi-language |
| Feeds | ✅ | Atom/RSS feeds |
| Dev Server | ✅ | Live reload & file watching |
| Site Init | ✅ | Embedded content & themes |
| Content Creation | ✅ | Quick post & page generation |

### Footnotes

This is a paragraph with a footnote[^1]. You can click on it to jump to the definition, and then click the return arrow to come back.

Here's another footnote[^second] with different content.

## Advanced Features

### Theme System

The site automatically detects your OS theme preference and switches between light and dark modes. Try toggling your system theme or use the theme button in the top navigation!

### Scroll to Top

On longer pages like this one, you'll see a scroll-to-top button appear in the bottom-right corner when you scroll down. It provides smooth scrolling back to the top.

### Navigation

The sidebar shows all pages on your site, and posts like this one include a "Back to Home" link for easy navigation. On mobile devices, the sidebar transforms into a convenient hamburger menu.

### Development Server

Krik includes a powerful development server with live reload functionality:

```bash
kk server                    # Start with live reload
kk server --no-live-reload   # Mobile-safe mode for Safari/iPad
```

The server automatically watches for changes and refreshes your browser, making development fast and efficient.

### Content Creation

Create new content quickly with built-in commands:

```bash
kk post "My New Blog Post"   # Create a new blog post
kk page "About Us"           # Create a new page
```

Both commands generate files with proper front matter and helpful starter content.

---

This is just the beginning! Check out the other posts and pages to see more features in action, or try the new commands to create your own content.

[^1]: This is the first footnote. Click the return arrow (↩) to go back to the text.

[^second]: This is the second footnote with some additional content to show how multiple footnotes work.