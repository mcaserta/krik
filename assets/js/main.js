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

// Mobile TOC functionality
function toggleMobileTOC() {
    const mobileTOC = document.getElementById('mobile-toc');
    if (mobileTOC) {
        mobileTOC.classList.toggle('show');
    }
}

// Close mobile menu when clicking outside or on a link
document.addEventListener('DOMContentLoaded', function() {
    const mobileMenu = document.getElementById('mobile-menu');
    const hamburgerBtn = document.querySelector('.hamburger-menu');
    const mobileTOC = document.getElementById('mobile-toc');
    const tocToggleBtn = document.querySelector('.toc-toggle');
    
    // Mobile menu functionality
    if (mobileMenu && hamburgerBtn) {
        // Add click event listener (works for both mouse and touch)
        hamburgerBtn.addEventListener('click', function(e) {
            e.preventDefault();
            // Close TOC if open
            if (mobileTOC) mobileTOC.classList.remove('show');
            toggleMobileMenu();
        });
        
        // Add touch event support for better mobile interaction
        hamburgerBtn.addEventListener('touchend', function(e) {
            e.preventDefault(); // Prevent double-tap zoom and ghost clicks
            // Close TOC if open
            if (mobileTOC) mobileTOC.classList.remove('show');
            toggleMobileMenu();
        });
        
        hamburgerBtn.addEventListener('touchstart', function(e) {
            e.preventDefault(); // Prevent double-tap zoom
        });
        
        // Close menu when clicking on a link
        const mobileLinks = mobileMenu.querySelectorAll('a');
        mobileLinks.forEach(link => {
            link.addEventListener('click', function(e) {
                // Don't prevent default - let the link navigate
                mobileMenu.classList.remove('show');
            });
        });
    }
    
    // Mobile TOC functionality
    if (mobileTOC && tocToggleBtn) {
        // Add click event listener (works for both mouse and touch)
        tocToggleBtn.addEventListener('click', function(e) {
            e.preventDefault();
            // Close menu if open
            if (mobileMenu) mobileMenu.classList.remove('show');
            toggleMobileTOC();
        });
        
        // Add touch event support for better mobile interaction
        tocToggleBtn.addEventListener('touchend', function(e) {
            e.preventDefault(); // Prevent double-tap zoom and ghost clicks
            // Close menu if open
            if (mobileMenu) mobileMenu.classList.remove('show');
            toggleMobileTOC();
        });
        
        tocToggleBtn.addEventListener('touchstart', function(e) {
            e.preventDefault(); // Prevent double-tap zoom
        });
        
        // Close TOC when clicking on a link
        const tocLinks = mobileTOC.querySelectorAll('a');
        tocLinks.forEach(link => {
            link.addEventListener('click', function(e) {
                // Don't prevent default - let the link navigate
                mobileTOC.classList.remove('show');
            });
        });
    }
    
    // Close both menus when clicking/touching outside
    function closeMenusOnOutsideTouch(event) {
        const isMenuClick = mobileMenu && mobileMenu.contains(event.target);
        const isTOCClick = mobileTOC && mobileTOC.contains(event.target);
        const isHamburgerClick = hamburgerBtn && hamburgerBtn.contains(event.target);
        const isTOCToggleClick = tocToggleBtn && tocToggleBtn.contains(event.target);
        
        if (!isMenuClick && !isTOCClick && !isHamburgerClick && !isTOCToggleClick) {
            if (mobileMenu) mobileMenu.classList.remove('show');
            if (mobileTOC) mobileTOC.classList.remove('show');
        }
    }
    
    document.addEventListener('click', closeMenusOnOutsideTouch);
    document.addEventListener('touchend', closeMenusOnOutsideTouch);
    
    // Close menus on escape key
    document.addEventListener('keydown', function(event) {
        if (event.key === 'Escape') {
            if (mobileMenu) mobileMenu.classList.remove('show');
            if (mobileTOC) mobileTOC.classList.remove('show');
        }
    });
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