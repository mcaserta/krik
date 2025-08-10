// Theme management
function initTheme() {
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme) {
        document.documentElement.setAttribute('data-theme', savedTheme);
        updateThemeIcon(savedTheme);
    } else {
        // Detect OS preference
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const theme = prefersDark ? 'dark' : 'light';
        document.documentElement.setAttribute('data-theme', theme);
        updateThemeIcon(theme);
    }
}

function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
    const newTheme = currentTheme === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    updateThemeIcon(newTheme);
}

function updateThemeIcon(theme) {
    const icon = document.querySelector('.theme-icon');
    if (icon) {
        icon.textContent = theme === 'light' ? '🌙' : '☀️';
    }
}

// Listen for OS theme changes
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (!localStorage.getItem('theme')) {
        const theme = e.matches ? 'dark' : 'light';
        document.documentElement.setAttribute('data-theme', theme);
        updateThemeIcon(theme);
    }
});

// Initialize theme on page load
initTheme();

// Add return links to footnotes  
document.addEventListener('DOMContentLoaded', function() {
    // First, add IDs to footnote references for easier navigation
    var footnoteRefs = document.querySelectorAll('.footnote-reference a');
    for (var i = 0; i < footnoteRefs.length; i++) {
        var ref = footnoteRefs[i];
        var href = ref.getAttribute('href');
        if (href && href.startsWith('#')) {
            var footnoteNum = href.substring(1);
            ref.id = 'fnref' + footnoteNum;
        }
    }
    
    // Then add return links to footnote definitions
    var definitions = document.querySelectorAll('.footnote-definition');
    for (var i = 0; i < definitions.length; i++) {
        var def = definitions[i];
        if (def.id) {
            var footnoteId = def.id;
            var refId = 'fnref' + footnoteId;
            
            var link = document.createElement('a');
            link.href = '#' + refId;
            link.className = 'footnote-return';
            link.innerHTML = ' ↩';
            link.title = 'Return to text';
            
            // Smooth scroll back to the footnote reference
            link.onclick = (function(refId) {
                return function(e) {
                    e.preventDefault();
                    var target = document.getElementById(refId);
                    if (target) {
                        // Find the parent sup element to scroll to the entire footnote reference
                        var supElement = target.closest('.footnote-reference');
                        if (supElement) {
                            supElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
                        } else {
                            target.scrollIntoView({ behavior: 'smooth', block: 'center' });
                        }
                    }
                };
            })(refId);
            
            def.appendChild(link);
        }
    }
});

function switchLanguage(lang) {
    // Use the translations data passed from the template if available
    if (window.krikTranslations && window.krikTranslations.length > 0) {
        const translation = window.krikTranslations.find(t => t.lang === lang);
        if (translation && translation.path) {
            window.location.href = translation.path;
            return;
        }
    }
    
    // Fallback to the old behavior for compatibility
    const currentPath = window.location.pathname;
    const baseName = window.krikBaseName || 'index'; // Will be set by template
    const extension = '.html';
    let newPath;
    if (lang === 'en') {
        newPath = baseName + extension;
    } else {
        newPath = baseName + '.' + lang + extension;
    }
    window.location.href = newPath;
}

// Mobile menu functionality
function toggleMobileMenu() {
    const mobileMenu = document.getElementById('mobile-menu');
    if (mobileMenu) {
        mobileMenu.classList.toggle('show');
    }
}

// Close mobile menu when clicking outside or on a link
document.addEventListener('DOMContentLoaded', function() {
    const mobileMenu = document.getElementById('mobile-menu');
    const hamburgerBtn = document.querySelector('.hamburger-menu');
    
    if (mobileMenu && hamburgerBtn) {
        // Close menu when clicking on a link
        const mobileLinks = mobileMenu.querySelectorAll('a');
        mobileLinks.forEach(link => {
            link.addEventListener('click', function() {
                mobileMenu.classList.remove('show');
            });
        });
        
        // Close menu when clicking outside
        document.addEventListener('click', function(event) {
            if (!mobileMenu.contains(event.target) && !hamburgerBtn.contains(event.target)) {
                mobileMenu.classList.remove('show');
            }
        });
        
        // Close menu on escape key
        document.addEventListener('keydown', function(event) {
            if (event.key === 'Escape') {
                mobileMenu.classList.remove('show');
            }
        });
    }
});

// Scroll to top functionality
document.addEventListener('DOMContentLoaded', function() {
    const scrollToTopBtn = document.getElementById('scroll-to-top');
    
    if (!scrollToTopBtn) {
        return; // Button not found, exit
    }
    
    // Show/hide button based on scroll position
    function toggleScrollToTopButton() {
        if (window.pageYOffset > 300) {
            scrollToTopBtn.classList.add('visible');
        } else {
            scrollToTopBtn.classList.remove('visible');
        }
    }
    
    // Smooth scroll to top
    function scrollToTop() {
        window.scrollTo({
            top: 0,
            behavior: 'smooth'
        });
    }
    
    // Event listeners
    window.addEventListener('scroll', toggleScrollToTopButton);
    scrollToTopBtn.addEventListener('click', scrollToTop);
    
    // Initial check
    toggleScrollToTopButton();
});