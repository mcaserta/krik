---
title: "Getting Started with Krik: From Installation to Your First Post"
date: 2025-01-15T12:00:00Z
tags: ["tutorial", "getting-started", "guide"]
toc: true
---

# Getting Started with Krik: From Installation to Your First Post

Welcome to Krik! This comprehensive guide will take you from zero to publishing
your first blog post in just a few minutes. Krik makes it incredibly easy to
create beautiful, fast static websites with minimal setup.

## Installation

### Quick Install (Recommended)

The fastest way to get started is installing Krik from crates.io:

```bash
cargo install krik
```

That's it! No additional setup required - themes and sample content are embedded
directly in the executable.

### Verify Installation

Check that Krik is installed correctly:

```bash
kk --version
kk --help
```

## Creating Your First Site

### Initialize a New Site

Create a new blog in just one command:

```bash
# Create a new blog directory
kk init my-awesome-blog
cd my-awesome-blog
```

This creates a complete site structure with:

- Sample blog posts and pages
- Default theme with light/dark mode
- Site configuration
- All necessary assets

### Start the Development Server

Launch the development server with live reload:

```bash
kk server
```

Open your browser to `http://localhost:3000` and you'll see your new site! The
server automatically watches for changes and refreshes your browser when you
edit files.

## Creating Content

### Your First Blog Post

Create a new blog post with a simple command:

```bash
kk post "My First Blog Post"
```

This creates `content/posts/my-first-blog-post.md` with:

- Proper YAML front matter
- Current timestamp
- Helpful starter content
- Tips for writing

### Custom Filenames

Want a specific filename? Use the `--filename` option:

```bash
kk post "How to Build Amazing Websites" --filename amazing-websites
```

This creates `amazing-websites.md` instead of the auto-generated filename.

### Creating Pages

Pages are perfect for static content like About, Contact, or Documentation:

```bash
kk page "About Me"
kk page "Contact" --filename contact
```

Pages are created in `content/pages/` and use the page template automatically.

## Customizing Your Content

### Front Matter Explained

Every post and page starts with YAML front matter:

```yaml
---
title: "Your Post Title"
date: 2025-01-15T12:00:00Z
layout: post # or 'page' for pages
tags: ["tutorial", "guide"] # helps categorize content
toc: true # enables table of contents
draft: false # set to true to hide from site
---
```

### Adding Your Content

Below the front matter, write your content in Markdown:

````markdown
# Main Heading

Your content here with **bold text**, _italic text_, and
[links](https://example.com).

## Subheadings

- Lists work great
- Easy to read
- Organize your thoughts

### Code Examples

```javascript
console.log("Code highlighting works automatically!");
```
````

````

### Markdown Features

Krik supports rich Markdown including:

- **Tables** with automatic styling
- **Footnotes** with bidirectional navigation
- **Code highlighting** for 100+ languages
- **Math expressions** (LaTeX support)
- **Task lists** with checkboxes

## Publishing Your Site

### Generate Static Files

When you're ready to publish, generate static files:

```bash
kk
````

This creates the `_site/` directory with your complete website. Upload these
files to any web host!

### Development vs Production

During development, use the server:

```bash
kk server                    # Live reload for development
kk server --no-live-reload   # Disable live reload if needed
```

For production builds:

```bash
kk --input content --output _site --theme themes/default
```

## Advanced Features

### Internationalization

Create translations by adding language codes to filenames:

```bash
# Create English version
kk post "Welcome to My Blog"

# Create Italian translation (manually)
cp content/posts/welcome-to-my-blog.md content/posts/welcome-to-my-blog.it.md
# Edit the Italian version
```

### Table of Contents

Enable automatic TOC generation by adding `toc: true` to your front matter.
Perfect for long articles like this one!

### Theme Customization

The default theme includes:

- Automatic light/dark mode detection
- Responsive design for all devices
- Mobile-friendly hamburger menu
- Smooth scroll-to-top button
- Professional typography

## Tips for Success

### Writing Great Content

1. **Use descriptive titles** - They appear in navigation and feeds
2. **Add relevant tags** - Helps organize and categorize your posts
3. **Include dates** - Keeps your content chronologically organized
4. **Enable TOC for long posts** - Improves navigation
5. **Use drafts** - Set `draft: true` while working on content

### Workflow Optimization

1. **Start with the development server**: `kk server`
2. **Create content quickly**: `kk post "Title"` or `kk page "Title"`
3. **Edit in your favorite editor** - Changes appear instantly in browser
4. **Generate when ready**: `kk` to create static files

### Organization Best Practices

- Use `/posts/` for time-sensitive content (blog posts, news, updates)
- Use `/pages/` for static content (about, contact, documentation)
- Keep images organized in `/images/` subdirectories
- Use consistent naming conventions for translations

## Next Steps

Now that you have Krik set up:

1. **Customize your site.toml** with your information
2. **Replace sample content** with your own posts and pages
3. **Explore the theme system** for advanced customization
4. **Set up deployment** to your preferred hosting platform

## Getting Help

- **Documentation**: Check the comprehensive documentation page on this site
- **Examples**: Explore the sample posts to see features in action
- **Community**: Join discussions on GitHub

---

Congratulations! You now have everything you need to create amazing static
websites with Krik. The combination of powerful features and simple commands
makes it easy to focus on what matters most: your content.

Happy blogging! 🚀
