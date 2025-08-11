/**
 * Prism.js syntax highlighting initialization with theme-aware styling
 * Respects language attributes from markdown code blocks (e.g., ```bash, ```java)
 * Loads appropriate theme based on current site theme
 * Degrades gracefully on unsupported browsers
 */

// Check if browser supports modern features needed for Prism
if (window.addEventListener && document.querySelectorAll && Array.prototype.forEach) {
    
    // Function to get current theme
    function getCurrentTheme() {
        var storedTheme = localStorage.getItem('theme');
        if (storedTheme) {
            return storedTheme;
        }
        // Check data-theme attribute
        var dataTheme = document.documentElement.getAttribute('data-theme');
        if (dataTheme) {
            return dataTheme;
        }
        // Fall back to system preference
        return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    
    // Function to load appropriate Prism theme
    function loadPrismTheme(theme) {
        // Remove existing Prism theme
        var existingTheme = document.getElementById('prism-theme');
        if (existingTheme) {
            existingTheme.remove();
        }
        
        // Load theme-appropriate CSS
        var link = document.createElement('link');
        link.id = 'prism-theme';
        link.rel = 'stylesheet';
        
        if (theme === 'dark') {
            link.href = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css';
        } else {
            link.href = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css';
        }
        
        link.onerror = function() {
            console.log('Prism theme failed to load - using browser defaults');
        };
        
        document.head.appendChild(link);
    }
    
    // Load initial theme
    loadPrismTheme(getCurrentTheme());
    
    // Load Prism core
    var script = document.createElement('script');
    script.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js';
    
    script.onload = function() {
        // Load autoloader after core is loaded
        var autoloader = document.createElement('script');
        autoloader.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js';
        
        autoloader.onload = function() {
            // Respect language attributes and highlight all code blocks
            if (window.Prism) {
                Prism.highlightAll();
            }
        };
        
        autoloader.onerror = function() {
            console.log('Prism autoloader failed to load - syntax highlighting disabled');
        };
        
        document.head.appendChild(autoloader);
    };
    
    script.onerror = function() {
        console.log('Prism core failed to load - syntax highlighting disabled');
    };
    
    document.head.appendChild(script);
    
    // Listen for theme changes and update Prism theme accordingly
    var observer = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.type === 'attributes' && mutation.attributeName === 'data-theme') {
                var newTheme = document.documentElement.getAttribute('data-theme') || getCurrentTheme();
                loadPrismTheme(newTheme);
                // Re-highlight code blocks with new theme
                if (window.Prism && Prism.highlightAll) {
                    setTimeout(function() {
                        Prism.highlightAll();
                    }, 100);
                }
            }
        });
    });
    
    observer.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ['data-theme']
    });
    
} else {
    // Browser doesn't support required features, skip syntax highlighting
    console.log('Browser does not support required features for syntax highlighting');
}