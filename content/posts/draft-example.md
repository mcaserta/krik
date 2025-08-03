---
title: "This is a Draft Post"
date: 2024-01-25T16:00:00Z
tags: ["draft", "example"]
draft: true
---

# This is a Draft Post

This post has `draft: true` in its front matter, so it will be skipped during
site generation.

This is useful for:

- Work-in-progress articles
- Content that's not ready for publication
- Testing new ideas without publishing them

When you're ready to publish, simply change `draft: true` to `draft: false` or
remove the draft field entirely.

## Draft Workflow

1. Create your post with `draft: true`
2. Work on the content locally
3. Generate the site to test (draft won't appear)
4. When ready, set `draft: false`
5. Regenerate the site

This allows you to work on multiple articles without accidentally publishing
unfinished content.
