// Theme handling
function initTheme() {
  try {
    const saved = localStorage.getItem('solstice-theme');
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const theme = saved || (prefersDark ? 'dark' : 'light');
    document.documentElement.setAttribute('data-theme', theme);
    updateThemeIcon(theme);
  } catch (_) {}
}

function toggleTheme() {
  const current = document.documentElement.getAttribute('data-theme') || 'light';
  const next = current === 'light' ? 'dark' : 'light';
  document.documentElement.setAttribute('data-theme', next);
  try { localStorage.setItem('solstice-theme', next); } catch (_) {}
  updateThemeIcon(next);
}

function updateThemeIcon(theme) {
  const el = document.querySelector('.theme-icon');
  if (!el) return;
  el.textContent = theme === 'light' ? '◐' : '◑';
}

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
  if (!localStorage.getItem('solstice-theme')) {
    const theme = e.matches ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', theme);
    updateThemeIcon(theme);
  }
});

initTheme();

// Language switching (compatible with Krik default)
function switchLanguage(lang) {
  const baseName = window.krikBaseName || 'index';
  const extension = '.html';
  const newPath = lang === 'en' ? baseName + extension : baseName + '.' + lang + extension;
  window.location.href = newPath;
}

// Mobile menu
function toggleMobileMenu() {
  const menu = document.getElementById('mobile-menu');
  if (menu) menu.classList.toggle('show');
}

// Close mobile menu on link click or outside click
document.addEventListener('DOMContentLoaded', function() {
  const menu = document.getElementById('mobile-menu');
  const button = document.querySelector('.hamburger-menu');
  if (menu) {
    menu.querySelectorAll('a').forEach(a => a.addEventListener('click', () => menu.classList.remove('show')));
    document.addEventListener('click', (e) => {
      if (!menu.contains(e.target) && button && !button.contains(e.target)) {
        menu.classList.remove('show');
      }
    });
  }
});

// Scroll to top
document.addEventListener('DOMContentLoaded', function() {
  const btn = document.getElementById('scroll-to-top');
  if (!btn) return;
  function onScroll() {
    if (window.pageYOffset > 300) btn.classList.add('visible');
    else btn.classList.remove('visible');
  }
  window.addEventListener('scroll', onScroll);
  btn.addEventListener('click', () => window.scrollTo({ top: 0, behavior: 'smooth' }));
  onScroll();
});

// Footnote return links
document.addEventListener('DOMContentLoaded', function() {
  // First, add IDs to footnote references for easier navigation
  const footnoteRefs = document.querySelectorAll('.footnote-reference a');
  footnoteRefs.forEach(ref => {
    const href = ref.getAttribute('href');
    if (href && href.startsWith('#')) {
      const footnoteNum = href.substring(1);
      ref.id = 'fnref' + footnoteNum;
    }
  });
  
  // Then add return links to footnote definitions
  const defs = document.querySelectorAll('.footnote-definition');
  defs.forEach(def => {
    if (!def.id) return;
    const footnoteId = def.id;
    const refId = 'fnref' + footnoteId;
    
    const a = document.createElement('a');
    a.href = '#' + refId;
    a.className = 'footnote-return';
    a.textContent = ' ↩';
    a.title = 'Return to text';
    
    // Smooth scroll back to the footnote reference
    a.addEventListener('click', (e) => {
      e.preventDefault();
      const target = document.getElementById(refId);
      if (target) {
        // Find the parent sup element to scroll to the entire footnote reference
        const supElement = target.closest('.footnote-reference');
        if (supElement) {
          supElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
        } else {
          target.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
      }
    });
    
    def.appendChild(a);
  });
});

// Expose
window.toggleTheme = toggleTheme;
window.switchLanguage = switchLanguage;
window.toggleMobileMenu = toggleMobileMenu;


