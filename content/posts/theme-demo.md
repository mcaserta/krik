---
title: "Theme System Demo"
date: 2024-01-18T09:15:00Z
tags: ["themes", "design", "css"]
---

# Theme System Demo

This post showcases Krik's powerful theme system with automatic light/dark mode
detection and smooth transitions.

## Automatic Theme Detection

Krik automatically detects your operating system's theme preference:

- **macOS**: Follows System Preferences → General → Appearance
- **Windows**: Follows Settings → Personalization → Colors → Choose your color
- **Linux**: Follows your desktop environment's theme setting
- **Mobile**: Follows iOS/Android system theme

## Manual Theme Toggle

Look for the 🌙/☀️ button in the top navigation bar. Clicking it will:

1. Toggle between light and dark modes
2. Save your preference to localStorage
3. Override the automatic OS detection
4. Animate the transition smoothly

## Theme Features

### Color Scheme

The theme system uses CSS custom properties for easy customization:

**Light Mode Colors:**

- Background: Clean white and light grays
- Text: Dark grays for excellent readability
- Links: Blue tones for accessibility
- Surfaces: Subtle shadows and borders

**Dark Mode Colors:**

- Background: Deep grays and blacks
- Text: Light grays and whites
- Links: Lighter blue tones
- Surfaces: Darker shadows with subtle highlights

### Responsive Design

The theme adapts to different screen sizes:

- **Desktop**: Full sidebar navigation and wide content area
- **Tablet**: Collapsible navigation with optimized spacing
- **Mobile**: Touch-friendly interface with larger tap targets

### Smooth Transitions

All theme changes animate smoothly with 0.3-second transitions on:

- Background colors
- Text colors
- Border colors
- Shadow effects
- Button states

## Cross-Platform Support

The theme detection works across:

- **Browsers**: Chrome, Firefox, Safari, Edge
- **Operating Systems**: Windows, macOS, Linux, iOS, Android
- **Devices**: Desktop, laptop, tablet, smartphone

## Accessibility

The theme system is designed with accessibility in mind:

- **High Contrast**: Both themes meet WCAG guidelines
- **Focus Indicators**: Clear focus states for keyboard navigation
- **Screen Readers**: Proper ARIA labels and semantic HTML
- **Color Independence**: Information isn't conveyed by color alone

Try switching between light and dark modes to see the smooth transitions in
action!
