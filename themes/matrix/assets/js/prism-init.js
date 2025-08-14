// Prism initialization (mirrors other themes) with theme switch
if (window.addEventListener && document.querySelectorAll && Array.prototype.forEach) {
  function getCurrentTheme() {
    var storedTheme = localStorage.getItem('theme');
    if (storedTheme) return storedTheme;
    var dataTheme = document.documentElement.getAttribute('data-theme');
    if (dataTheme) return dataTheme;
    return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  function loadPrismTheme(theme) {
    var existingTheme = document.getElementById('prism-theme');
    if (existingTheme) existingTheme.remove();
    var link = document.createElement('link');
    link.id = 'prism-theme';
    link.rel = 'stylesheet';
    link.href = theme === 'dark'
      ? 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css'
      : 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css';
    link.onerror = function() { console.log('Prism theme failed to load'); };
    document.head.appendChild(link);
  }
  loadPrismTheme(getCurrentTheme());
  var script = document.createElement('script');
  script.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js';
  script.onload = function() {
    var autoloader = document.createElement('script');
    autoloader.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js';
    autoloader.onload = function() { if (window.Prism) Prism.highlightAll(); };
    autoloader.onerror = function() { console.log('Prism autoloader failed'); };
    document.head.appendChild(autoloader);
  };
  script.onerror = function() { console.log('Prism core failed'); };
  document.head.appendChild(script);
  var observer = new MutationObserver(function(mutations) {
    mutations.forEach(function(mutation) {
      if (mutation.type === 'attributes' && mutation.attributeName === 'data-theme') {
        var newTheme = document.documentElement.getAttribute('data-theme') || getCurrentTheme();
        loadPrismTheme(newTheme);
        if (window.Prism && Prism.highlightAll) setTimeout(function() { Prism.highlightAll(); }, 100);
      }
    });
  });
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}
