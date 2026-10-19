export type Theme = 'dark' | 'light' | 'auto';
export type Scale = '100' | '125' | '150' | '200';

function getInitialTheme(): Theme {
  if (typeof window === 'undefined') return 'dark';
  const saved = localStorage.getItem('35mm-theme') as Theme | null;
  if (saved && ['dark', 'light', 'auto'].includes(saved)) return saved;
  return 'dark'; // <-- Тёмная по умолчанию
}

function getInitialScale(): Scale {
  if (typeof window === 'undefined') return '100';
  const saved = localStorage.getItem('35mm-scale') as Scale | null;
  if (saved && ['100', '125', '150', '200'].includes(saved)) return saved;
  return '100';
}

function getInitialTmdbKey(): string {
  if (typeof window === 'undefined') return '';
  return localStorage.getItem('35mm-tmdb-key') ?? '';
}

let theme = $state<Theme>(getInitialTheme());
let scale = $state<Scale>(getInitialScale());
let tmdbKey = $state<string>(getInitialTmdbKey());

function applyTheme(t: Theme) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  if (t === 'auto') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  } else {
    root.setAttribute('data-theme', t);
  }
}

function applyScale(s: Scale) {
  if (typeof document === 'undefined') return;
  const factor = parseInt(s, 10) / 100;
  document.documentElement.style.setProperty('--app-scale', String(factor));
}

export const settings = {
  get theme() { return theme; },
  get scale() { return scale; },
  get tmdbKey() { return tmdbKey; },

  setTheme(t: Theme) {
    theme = t;
    localStorage.setItem('35mm-theme', t);
    applyTheme(t);
  },

  setScale(s: Scale) {
    scale = s;
    localStorage.setItem('35mm-scale', s);
    applyScale(s);
  },

  setTmdbKey(k: string) {
    tmdbKey = k.trim();
    localStorage.setItem('35mm-tmdb-key', tmdbKey);
  },

  init() {
    applyTheme(theme);
    applyScale(scale);
    if (typeof window !== 'undefined') {
      window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        if (theme === 'auto') applyTheme('auto');
      });
    }
  }
};