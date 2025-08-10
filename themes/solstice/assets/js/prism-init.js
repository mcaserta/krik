// Minimal Prism setup with theme-awareness
(function() {
  if (!(window.addEventListener && document.querySelectorAll)) return;

  function currentTheme() {
    const stored = localStorage.getItem('solstice-theme');
    if (stored) return stored;
    const attr = document.documentElement.getAttribute('data-theme');
    if (attr) return attr;
    return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }

  function loadPrismTheme(theme) {
    const existing = document.getElementById('prism-theme');
    if (existing) existing.remove();
    const link = document.createElement('link');
    link.id = 'prism-theme';
    link.rel = 'stylesheet';
    link.href = theme === 'dark'
      ? 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css'
      : 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css';
    document.head.appendChild(link);
  }

  loadPrismTheme(currentTheme());

  const core = document.createElement('script');
  core.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js';
  core.onload = function() {
    const autoloader = document.createElement('script');
    autoloader.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js';
    autoloader.onload = function(){ if (window.Prism) Prism.highlightAll(); };
    document.head.appendChild(autoloader);
  };
  document.head.appendChild(core);

  const observer = new MutationObserver(function(muts) {
    muts.forEach(m => {
      if (m.type === 'attributes' && m.attributeName === 'data-theme') {
        loadPrismTheme(currentTheme());
        if (window.Prism && Prism.highlightAll) setTimeout(() => Prism.highlightAll(), 100);
      }
    })
  });
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
})();


