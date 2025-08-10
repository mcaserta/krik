// Theme handling
function initTheme() {
  try {
    const saved = localStorage.getItem('aurora-theme');
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
  try { localStorage.setItem('aurora-theme', next); } catch (_) {}
  updateThemeIcon(next);
}

function updateThemeIcon(theme) {
  const el = document.querySelector('.theme-icon');
  if (!el) return;
  el.textContent = theme === 'light' ? '◔' : '◑';
}

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
  if (!localStorage.getItem('aurora-theme')) {
    const theme = e.matches ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', theme);
    updateThemeIcon(theme);
  }
});

initTheme();

// Language switching
function switchLanguage(lang) {
  if (window.krikTranslations && window.krikTranslations.length > 0) {
    const translation = window.krikTranslations.find(t => t.lang === lang);
    if (translation && translation.path) {
      window.location.href = translation.path;
      return;
    }
  }
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
  const refs = document.querySelectorAll('.footnote-reference a');
  refs.forEach(ref => {
    const href = ref.getAttribute('href');
    if (href && href.startsWith('#')) {
      ref.id = 'fnref' + href.substring(1);
    }
  });
  const defs = document.querySelectorAll('.footnote-definition');
  defs.forEach(def => {
    if (!def.id) return;
    const refId = 'fnref' + def.id;
    const a = document.createElement('a');
    a.href = '#' + refId;
    a.className = 'footnote-return';
    a.textContent = ' ↩';
    a.title = 'Return to text';
    a.addEventListener('click', (e) => {
      e.preventDefault();
      const target = document.getElementById(refId);
      if (target) {
        const sup = target.closest('.footnote-reference');
        (sup || target).scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    });
    def.appendChild(a);
  });
});

// Expose
window.toggleTheme = toggleTheme;
window.switchLanguage = switchLanguage;
window.toggleMobileMenu = toggleMobileMenu;


