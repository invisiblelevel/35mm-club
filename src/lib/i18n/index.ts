import { register, init, waitLocale } from 'svelte-i18n';

export type Locale = 'ru' | 'en' | 'zh-CN';

export const locales: Record<Locale, string> = {
  ru: 'Русский',
  en: 'English',
  'zh-CN': '中文'
};

function getInitialLocale(): Locale {
  if (typeof window === 'undefined') return 'ru';
  const saved = localStorage.getItem('35mm-locale') as Locale | null;
  if (saved && saved in locales) return saved;
  const browserLang = navigator.language.toLowerCase();
  if (browserLang.startsWith('ru')) return 'ru';
  if (browserLang.startsWith('zh')) return 'zh-CN';
  return 'en'; // всё остальное — английский
}

register('ru', () => Promise.all([
  import('./locales/ru/common.json'),
  import('./locales/ru/achievements.json'),
  import('./locales/ru/roulette.json')
]).then(([common, ach, roulette]) => ({
  ...common.default,
  achievements: {
    ...common.default.achievements,
    ...ach.default
  },
  roulette: roulette.default
})));

register('en', () => Promise.all([
  import('./locales/en/common.json'),
  import('./locales/en/achievements.json'),
  import('./locales/en/roulette.json')
]).then(([common, ach, roulette]) => ({
  ...common.default,
  achievements: {
    ...common.default.achievements,
    ...ach.default
  },
  roulette: roulette.default
})));

register('zh-CN', () => Promise.all([
  import('./locales/zh-CN/common.json'),
  import('./locales/zh-CN/achievements.json'),
  import('./locales/zh-CN/roulette.json')
]).then(([common, ach, roulette]) => ({
  ...common.default,
  achievements: {
    ...common.default.achievements,
    ...ach.default
  },
  roulette: roulette.default
})));

init({
  fallbackLocale: 'en',
  initialLocale: getInitialLocale()
});

export const i18nReady = waitLocale();