use std::fs;
use std::path::Path;
use chrono::{Utc, DateTime};

/// Create a new blog post in the content/posts directory
pub fn create_post(content_dir: &Path, title: &str, custom_filename: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    let posts_dir = content_dir.join("posts");
    
    // Create posts directory if it doesn't exist
    if !posts_dir.exists() {
        fs::create_dir_all(&posts_dir)?;
        println!("📁 Created directory: {}", posts_dir.display());
    }
    
    // Generate filename
    let filename = if let Some(custom) = custom_filename {
        format!("{}.md", custom)
    } else {
        generate_filename_from_title(title)
    };
    
    let file_path = posts_dir.join(&filename);
    
    // Check if file already exists
    if file_path.exists() {
        return Err(format!(
            "Post file '{}' already exists. Use a different filename with --filename.",
            file_path.display()
        ).into());
    }
    
    // Generate post content with front matter
    let content = generate_post_content(title);
    
    // Write the file
    fs::write(&file_path, content)?;
    
    println!("📝 Created new blog post: {}", file_path.display());
    println!("✨ You can now edit the file and add your content!");
    
    Ok(())
}

/// Create a new page in the content/pages directory
pub fn create_page(content_dir: &Path, title: &str, custom_filename: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    let pages_dir = content_dir.join("pages");
    
    // Create pages directory if it doesn't exist
    if !pages_dir.exists() {
        fs::create_dir_all(&pages_dir)?;
        println!("📁 Created directory: {}", pages_dir.display());
    }
    
    // Generate filename
    let filename = if let Some(custom) = custom_filename {
        format!("{}.md", custom)
    } else {
        generate_filename_from_title(title)
    };
    
    let file_path = pages_dir.join(&filename);
    
    // Check if file already exists
    if file_path.exists() {
        return Err(format!(
            "Page file '{}' already exists. Use a different filename with --filename.",
            file_path.display()
        ).into());
    }
    
    // Generate page content with front matter
    let content = generate_page_content(title);
    
    // Write the file
    fs::write(&file_path, content)?;
    
    println!("📄 Created new page: {}", file_path.display());
    println!("✨ You can now edit the file and add your content!");
    
    Ok(())
}

/// Generate a filename from a title by converting to lowercase and replacing spaces with hyphens
fn generate_filename_from_title(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| {
            match c {
                'a'..='z' | '0'..='9' => c,
                ' ' | '-' | '_' => '-',
                _ => '-',
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-");
    
    format!("{}.md", slug)
}

/// Generate post content with YAML front matter
fn generate_post_content(title: &str) -> String {
    let now: DateTime<Utc> = Utc::now();
    let formatted_date = now.format("%Y-%m-%dT%H:%M:%SZ");
    
    format!(r#"---
title: "{}"
date: {}
layout: post
tags: []
draft: false
---

# {}

Write your blog post content here. You can use:

- **Markdown formatting** like bold and *italic* text
- Links: [Krik Documentation](https://github.com/mcaserta/krik)
- Code blocks with syntax highlighting
- Tables, lists, and more!

## Getting Started

This is a sample blog post created with Krik. Replace this content with your own ideas, thoughts, and stories.

### Tips for Writing

- Keep your audience in mind
- Use clear, engaging headlines
- Break up text with subheadings and lists
- Add relevant tags to help categorize your content

Happy writing! 🚀
"#, title, formatted_date, title)
}

/// Generate page content with YAML front matter
fn generate_page_content(title: &str) -> String {
    let now: DateTime<Utc> = Utc::now();
    let formatted_date = now.format("%Y-%m-%dT%H:%M:%SZ");
    
    format!(r#"---
title: "{}"
date: {}
layout: page
draft: false
---

# {}

This is a new page created with Krik. Add your content here.

## About This Page

Pages are different from blog posts in that they're typically more static content like:

- About pages
- Contact information
- Documentation
- Terms of service
- Privacy policies

## Formatting

You can use all the same Markdown features available in blog posts:

- **Bold** and *italic* text
- [Links](https://github.com/mcaserta/krik)
- Lists and tables
- Code blocks
- And much more!

Replace this placeholder content with your own information.
"#, title, formatted_date, title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_filename_from_title() {
        assert_eq!(generate_filename_from_title("Hello World"), "hello-world.md");
        assert_eq!(generate_filename_from_title("My Great Blog Post!"), "my-great-blog-post.md");
        assert_eq!(generate_filename_from_title("Special-Characters@#$"), "special-characters.md");
        assert_eq!(generate_filename_from_title("   Trimmed   Spaces   "), "trimmed-spaces.md");
    }

    #[test]
    fn test_generate_post_content() {
        let content = generate_post_content("Test Post");
        assert!(content.contains("title: \"Test Post\""));
        assert!(content.contains("layout: post"));
        assert!(content.contains("# Test Post"));
        assert!(content.contains("tags: []"));
        assert!(content.contains("draft: false"));
    }

    #[test]
    fn test_generate_page_content() {
        let content = generate_page_content("Test Page");
        assert!(content.contains("title: \"Test Page\""));
        assert!(content.contains("layout: page"));
        assert!(content.contains("# Test Page"));
        assert!(content.contains("draft: false"));
    }
}