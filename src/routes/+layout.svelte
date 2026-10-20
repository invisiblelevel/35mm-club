<script lang="ts">
  import '../app.css';
  import '$lib/i18n';
  import { i18nReady, locales, type Locale } from '$lib/i18n';
  import { locale, t } from 'svelte-i18n';
  import { settings } from '$lib/settings.svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';

  import '@fontsource/inter/400.css';
  import '@fontsource/inter/500.css';
  import '@fontsource/inter/600.css';
  import '@fontsource/geist/600.css';
  import '@fontsource/geist/700.css';

  import {
    Film, Tv, Clapperboard, BookOpen, FolderHeart,
    Trophy, User, Settings, Search, Info, Loader2, Menu, X
  } from 'lucide-svelte';

  import AboutModal from '$lib/components/AboutModal.svelte';
  import SupportModal from '$lib/components/SupportModal.svelte';
  import AchievementToast, { type ToastData } from '$lib/components/AchievementToast.svelte';

  let showAbout = $state(false);
  let showSupport = $state(false);
  let topSearch = $state('');
  let toastQueue: ToastData[] = $state([]);
  let translating = $state(false);
  let sidebarOpen = $state(false);

  const currentToast = $derived(toastQueue.length > 0 ? toastQueue[0] : null);

  onMount(() => {
    settings.init();

    const handleToast = (e: Event) => {
      const customEvent = e as CustomEvent<ToastData>;
      toastQueue = [...toastQueue, customEvent.detail];

      try {
        const audio = new Audio('/achivka.wav');
        audio.volume = 0.5;
        audio.play().catch((err) => {
          console.warn('Achievement sound failed:', err);
        });
      } catch (err) {
        console.warn('Achievement sound error:', err);
      }
    };
    window.addEventListener('achievement-toast', handleToast);
    return () => window.removeEventListener('achievement-toast', handleToast);
  });

  $effect(() => {
    if (page.url.pathname === '/search') {
      const q = page.url.searchParams.get('q');
      if (q !== null) topSearch = q;
    }
  });

  $effect(() => {
    page.url.pathname;
    sidebarOpen = false;
  });

  const navItems = [
    { href: '/profile', key: 'nav.profile', icon: User },
    { href: '/library/movies', key: 'nav.movies', icon: Film },
    { href: '/library/tv', key: 'nav.tv', icon: Tv },
    { href: '/library/anime', key: 'nav.anime', icon: Clapperboard },
    { href: '/diary', key: 'nav.diary', icon: BookOpen },
    { href: '/collections', key: 'nav.collections', icon: FolderHeart },
    { href: '/achievements', key: 'nav.achievements', icon: Trophy },
  ];

  async function changeLocale(l: Locale) {
    if (translating) return;
    locale.set(l);
    localStorage.setItem('35mm-locale', l);

    translating = true;
    try {
      await invoke('refresh_titles_language', {
        apiKey: settings.tmdbKey || null,
        locale: l,
      });
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      console.error('Failed to refresh titles:', e);
    } finally {
      translating = false;
    }
  }

  function submitTopSearch() {
    const q = topSearch.trim();
    if (!q) return;
    if (page.url.pathname === '/search') {
      goto(`/search?q=${encodeURIComponent(q)}`, { replaceState: true, noScroll: true });
    } else {
      goto(`/search?q=${encodeURIComponent(q)}`);
    }
  }
</script>

{#await i18nReady}
  <div class="min-h-screen bg-ink text-zinc-100 flex items-center justify-center">
    <span class="text-zinc-500">Loading...</span>
  </div>
{:then}
  <div class="min-h-screen bg-ink text-zinc-100 flex">
    {#if sidebarOpen}
      <div
        class="fixed inset-0 z-40 bg-black/60 backdrop-blur-sm md:hidden"
        onclick={() => sidebarOpen = false}
        role="presentation"
      ></div>
    {/if}

    <aside
      class="fixed md:static inset-y-0 left-0 z-50 w-64 bg-ink-light border-r border-border flex flex-col transform transition-transform duration-300 ease-out
             {sidebarOpen ? 'translate-x-0' : '-translate-x-full'} md:translate-x-0
             pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]"
    >
      <div class="p-6 border-b border-border flex items-center justify-between">
        <div>
          <h1 class="font-display text-2xl font-bold tracking-tight">
            35<span class="text-gold">mm</span> Club
          </h1>
          <p class="text-xs text-zinc-500 mt-1">{$t('app.tagline')}</p>
        </div>
        <button
          onclick={() => sidebarOpen = false}
          class="md:hidden text-zinc-400 hover:text-zinc-100"
          aria-label="Close menu"
        >
          <X size={22} />
        </button>
      </div>

      <nav class="flex-1 p-4 space-y-1 overflow-y-auto">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-100 hover:bg-surface transition-colors duration-200"
          >
            <svelte:component this={item.icon} size={18} />
            <span>{$t(item.key)}</span>
          </a>
        {/each}
      </nav>

      <div class="p-4 border-t border-border">
        <a
          href="/settings"
          class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-100 hover:bg-surface transition-colors duration-200"
        >
          <Settings size={18} />
          <span>{$t('nav.settings')}</span>
        </a>
      </div>
    </aside>

    <main class="flex-1 overflow-auto min-w-0 flex flex-col h-screen">
      <header class="sticky top-0 z-10 bg-ink/80 backdrop-blur-sm border-b border-border px-3 md:px-8 py-3 md:py-4 pt-[max(0.75rem,env(safe-area-inset-top))]">
        <div class="flex items-center gap-2 md:gap-3">
          <button
            onclick={() => sidebarOpen = true}
            class="md:hidden w-9 h-9 flex items-center justify-center rounded-lg bg-surface border border-border text-zinc-400 hover:text-gold transition-colors shrink-0"
            aria-label="Open menu"
          >
            <Menu size={20} />
          </button>

          <div class="relative flex-1 min-w-0">
            <Search size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500" />
            <input
              type="text"
              placeholder={$t('nav.search_placeholder')}
              bind:value={topSearch}
              onkeydown={(e) => e.key === 'Enter' && submitTopSearch()}
              class="w-full pl-10 pr-3 py-2 bg-surface border border-border rounded-lg text-sm text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none transition-colors"
            />
          </div>

          <div class="flex items-center gap-1.5 shrink-0">
            <div class="flex items-center gap-0.5 bg-surface border border-border rounded-lg p-0.5">
              {#each Object.entries(locales) as [code]}
                <button
                  onclick={() => changeLocale(code as Locale)}
                  disabled={translating}
                  class="px-2 py-1 rounded text-xs font-medium transition-colors disabled:opacity-50 {$locale === code ? 'bg-gold text-ink' : 'text-zinc-400 hover:text-zinc-100'}"
                >{code === 'zh-CN' ? '中文' : code.toUpperCase()}</button>
              {/each}
            </div>

            <button
              onclick={() => showAbout = true}
              class="w-9 h-9 flex items-center justify-center rounded-lg bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors"
            >
              <Info size={18} />
            </button>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 pb-[max(1rem,env(safe-area-inset-bottom))] flex-1 overflow-y-auto">
        <slot />
      </div>
    </main>
  </div>

  {#if showAbout}
    <AboutModal
      onClose={() => showAbout = false}
      onOpenSupport={() => { showAbout = false; showSupport = true; }}
    />
  {/if}

  {#if showSupport}
    <SupportModal onClose={() => showSupport = false} />
  {/if}

  {#if currentToast}
    <AchievementToast toast={currentToast} onClose={() => toastQueue = toastQueue.slice(1)} />
  {/if}
{/await}