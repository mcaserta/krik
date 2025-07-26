---
title: "Markdown Feature Showcase"
date: 2024-01-20T14:30:00Z
tags: ["markdown", "features", "demo"]
toc: true
---

# Markdown Feature Showcase

This post demonstrates all the markdown features supported by Krik, including some advanced formatting options.

## Basic Text Formatting

Here are the basic text formatting options:

- **Bold text** using `**bold**`
- *Italic text* using `*italic*`
- ~~Strikethrough text~~ using `~~strikethrough~~`
- `Inline code` using backticks
- You can also combine ***bold and italic*** text

## Headings

Krik supports all heading levels from H1 to H6. Since this post has TOC enabled, you can see how they're organized in the table of contents.

### This is H3
#### This is H4
##### This is H5
###### This is H6

## Code Blocks

### Syntax Highlighting Ready

```rust
// Rust code example
struct Config {
    title: String,
    base_url: Option<String>,
}

impl Config {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            base_url: None,
        }
    }
}
```

```python
# Python code example
def generate_site(input_dir, output_dir):
    """Generate static site from markdown files."""
    for file in input_dir.glob("**/*.md"):
        process_markdown_file(file, output_dir)
    
    print("Site generated successfully!")
```

```html
<!-- HTML code example -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Krik Demo</title>
</head>
<body>
    <h1>Hello, World!</h1>
</body>
</html>
```

## Lists and Task Lists

### Unordered Lists

- First level item
- Another first level item
  - Second level item
  - Another second level item
    - Third level item
    - Another third level item
- Back to first level

### Ordered Lists

1. First numbered item
2. Second numbered item
   1. Nested numbered item
   2. Another nested item
3. Third numbered item

### Task Lists

- [x] Completed task
- [x] Another completed task
- [ ] Incomplete task
- [ ] Another incomplete task
  - [x] Nested completed task
  - [ ] Nested incomplete task

## Tables

Here's a comprehensive table showing Krik's features:

| Feature Category | Feature | Status | Description |
|------------------|---------|--------|-------------|
| **Core** | Markdown Processing | ✅ | Full GFM support |
| **Core** | Front Matter | ✅ | YAML metadata |
| **Core** | Draft Support | ✅ | Skip processing |
| **Theme** | Light/Dark Mode | ✅ | Auto-detection + toggle |
| **Theme** | Responsive Design | ✅ | Mobile-first approach |
| **i18n** | Multi-language | ✅ | Filename-based detection |
| **i18n** | Language Selector | ✅ | Dropdown navigation |
| **Navigation** | TOC Generation | ✅ | Auto-generated from headings |
| **Navigation** | Footnote Links | ✅ | Bidirectional navigation |
| **Navigation** | Scroll to Top | ✅ | Smart visibility |
| **Advanced** | Atom Feeds | ✅ | RFC 4287 compliant |
| **Advanced** | Asset Copying | ✅ | Preserves structure |

## Blockquotes

> This is a simple blockquote.

> This is a multi-line blockquote.
> It can span multiple lines and even include other markdown elements.
> 
> Like **bold text** and *italic text*.

> ### Blockquotes with headings
> 
> You can include headings in blockquotes:
> 
> 1. Numbered lists
> 2. Work too
> 
> ```
> Even code blocks!
> ```

## Horizontal Rules

You can create horizontal rules using three or more hyphens:

---

Or three or more asterisks:

***

## Links

- [External link to Rust](https://www.rust-lang.org/)
- [Link to another post](welcome.html)
- [Link with title](https://www.rust-lang.org/ "The Rust Programming Language")

### Reference Links

This is a paragraph with [reference link][rust-lang] and another [link][github].

[rust-lang]: https://www.rust-lang.org/ "Rust Programming Language"
[github]: https://github.com/ "GitHub"

## Images

Here's how images work (note: no actual image file in this demo):

![Alt text for image](../images/example.png "Optional title")

## Footnotes in Detail

Footnotes are particularly powerful in Krik[^footnote1]. They create bidirectional links with smooth scrolling[^complex-footnote].

You can have multiple paragraphs in footnotes[^multiline], and they support full markdown formatting.

## Escaping Characters

Sometimes you need to escape markdown characters: \*not italic\*, \`not code\`, \[not a link\].

---

This covers most of the markdown features supported by Krik. The combination of these features with the theme system, internationalization, and advanced navigation makes Krik a powerful choice for static site generation.

[^footnote1]: This is a simple footnote that demonstrates the basic functionality.

[^complex-footnote]: This footnote contains **bold text**, *italic text*, and even `inline code`. It shows how footnotes can contain rich formatting.

[^multiline]: This is a multiline footnote.

    It can contain multiple paragraphs, code blocks, and other markdown elements:
    
    ```
    code in footnotes works too!
    ```
    
    Pretty neat, right?