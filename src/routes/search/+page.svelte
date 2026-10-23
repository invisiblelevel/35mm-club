<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t, locale } from 'svelte-i18n';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { Search, Star, Library, Film, Sparkles } from 'lucide-svelte';
  import { settings } from '$lib/settings.svelte';
  import TitleModal from '$lib/components/TitleModal.svelte';
  import AnimeModal from '$lib/components/AnimeModal.svelte';

  type MovieItem = {
    tmdb_id: number; title: string; overview: string | null; poster_url: string | null;
    year: number | null; rating: number | null; media_type: string;
    original_language: string | null; genre_ids: number[];
  };

  type AnimeItem = {
    mal_id: number; title: string; title_original: string | null;
    cover_url: string | null; episodes: number | null; average_score: number | null;
    format: string | null; year: number | null;
  };

  type LocalItem = {
    id: number; tmdb_id: number | null; title: string; type_field: string;
    year: number | null; poster_path: string | null; status: string;
    rating: number | null; added_at: string;
  };

  let query = $state('');
  let tmdbResults: MovieItem[] = $state([]);
  let animeResults: AnimeItem[] = $state([]);
  let localResults: LocalItem[] = $state([]);
  let loading = $state(false);
  let error = $state('');
  let selectedTmdb: MovieItem | null = $state(null);
  let selectedAnime: AnimeItem | null = $state(null);

  const TMDB_IMG = 'https://image.tmdb.org/t/p/w200';

  let searchGeneration = 0;

  async function runSearch() {
    const q = query.trim();
    if (!q) {
      tmdbResults = [];
      animeResults = [];
      localResults = [];
      return;
    }

    const myGeneration = ++searchGeneration;
    loading = true;
    error = '';

    const localPromise = invoke<LocalItem[]>('search_local_library', { query: q, contentType: null })
      .then(res => {
        if (myGeneration === searchGeneration) localResults = res;
      })
      .catch(e => {
        console.error('local search failed:', e);
      });

    const tmdbPromise = invoke<MovieItem[]>('search_movies', {
      query: q,
      mediaType: 'multi',
      apiKey: settings.tmdbKey || null,
      locale: $locale,
    })
      .then(res => {
        if (myGeneration !== searchGeneration) return;
        tmdbResults = res.filter(item => {
          const isAnime = item.original_language === 'ja' && item.genre_ids.includes(16);
          return !isAnime;
        });
      })
      .catch(e => {
        console.error('tmdb search failed:', e);
      });

    const animePromise = invoke<AnimeItem[]>('search_anime', { query: q, locale: $locale })
      .then(res => {
        if (myGeneration === searchGeneration) animeResults = res;
      })
      .catch(e => {
        console.error('anime search failed:', e);
      });

    await Promise.allSettled([localPromise, tmdbPromise, animePromise]);

    if (myGeneration === searchGeneration) {
      loading = false;
    }
  }

  function syncUrl() {
    const params = new URLSearchParams();
    if (query.trim()) params.set('q', query.trim());
    const url = params.toString() ? `/search?${params.toString()}` : '/search';
    goto(url, { replaceState: true, noScroll: true });
  }

  function onSearchClick() {
    // Только обновляем URL — $effect на page.url.searchParams сам дёрнет runSearch().
    // Так и хедер, и кнопка, и Enter в инпуте идут одним путём, без двойных запросов.
    syncUrl();
  }

  function imgFallback(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    img.style.display = 'none';
    const parent = img.parentElement;
    if (parent) {
      parent.innerHTML = '<div class="w-full h-full flex items-center justify-center text-zinc-600"><svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="4" width="20" height="16" rx="2"/><path d="M2 16l5-5 4 4 3-3 8 8"/></svg></div>';
    }
  }

  // Реагируем на изменение q в URL (в т.ч. из хедера).
  // $effect на page.url.searchParams ловит и первый заход, и повторные goto.
  let lastQ = $state<string | null>(null);
  $effect(() => {
    const q = page.url.searchParams.get('q') ?? '';
    if (q === lastQ) return;
    lastQ = q;
    if (q !== query) query = q;
    if (q) runSearch();
  });

  let lastLocale = $state('');
  $effect(() => {
    const l = $locale;
    if (lastLocale === '') {
      lastLocale = l;
      return;
    }
    if (lastLocale !== l) {
      lastLocale = l;
      if (query.trim()) runSearch();
    }
  });
</script>

<div class="space-y-4 md:space-y-6">
  <h1 class="font-display text-2xl md:text-3xl font-bold">{$t('search.title')}</h1>

  <div class="flex flex-col sm:flex-row gap-2">
    <div class="relative flex-1">
      <Search size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500" />
      <input
        class="w-full pl-10 pr-4 py-2.5 md:py-3 bg-surface border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none transition-colors text-sm md:text-base"
        placeholder={$t('search.placeholder')}
        bind:value={query}
        onkeydown={(e) => e.key === 'Enter' && onSearchClick()}
      />
    </div>
    <button
      class="px-6 py-2.5 md:py-3 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink font-medium rounded-lg transition-all disabled:opacity-50 text-sm md:text-base shrink-0"
      onclick={onSearchClick}
      disabled={loading}
    >{loading ? $t('search.loading') : $t('search.button')}</button>
  </div>

  {#if error}
    <div class="p-4 rounded-lg bg-red-950/50 border border-red-800 text-sm text-red-200">
      <pre class="whitespace-pre-wrap">{error}</pre>
    </div>
  {/if}

  {#if query.trim() && localResults.length > 0}
    <section class="space-y-4">
      <h2 class="font-display text-lg md:text-xl font-semibold flex items-center gap-2">
        <Library size={18} class="text-gold" />
        {$t('search.in_library')} <span class="text-sm font-normal text-zinc-500">({localResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
        {#each localResults as item, i (item.id)}
          <a href={`/title/${item.tmdb_id}?type=${item.type_field}`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
            <div class="aspect-[2/3] bg-ink-light overflow-hidden">
              {#if item.poster_path}
                <img src={TMDB_IMG + item.poster_path} alt={item.title} loading={i < 6 ? 'eager' : 'lazy'} decoding="async" onerror={imgFallback} class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-zinc-600"><Film size={48} /></div>
              {/if}
            </div>
            <div class="p-2 md:p-3">
              <p class="text-xs md:text-sm font-medium truncate">{item.title}</p>
              <div class="flex items-center justify-between mt-1">
                <span class="text-[10px] md:text-xs text-zinc-500">{item.year ?? '—'}</span>
                <span class="text-[9px] md:text-[10px] px-1.5 py-0.5 rounded bg-gold/10 text-gold uppercase">{$t('status.' + item.status)}</span>
              </div>
            </div>
          </a>
        {/each}
      </div>
    </section>
  {/if}

  {#if query.trim() && tmdbResults.length > 0}
    <section class="space-y-4">
      <h2 class="font-display text-lg md:text-xl font-semibold flex items-center gap-2">
        <Search size={18} class="text-gold" />
        TMDB <span class="text-sm font-normal text-zinc-500">({tmdbResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
        {#each tmdbResults as item, i (item.tmdb_id)}
          <button type="button" onclick={() => selectedTmdb = item} class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer">
            <div class="aspect-[2/3] bg-ink-light overflow-hidden">
              {#if item.poster_url}
                <img src={item.poster_url.startsWith('http') ? item.poster_url.replace('/w500/', '/w200/') : TMDB_IMG + item.poster_url} alt={item.title} loading={i < 6 ? 'eager' : 'lazy'} decoding="async" onerror={imgFallback} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-zinc-600"><Film size={48} /></div>
              {/if}
            </div>
            <div class="p-2 md:p-3">
              <p class="text-xs md:text-sm font-medium truncate">{item.title}</p>
              <div class="flex items-center justify-between mt-1">
                <span class="text-[10px] md:text-xs text-zinc-500">{item.year ?? '—'}</span>
                {#if item.rating}
                  <span class="flex items-center gap-1 text-[10px] md:text-xs text-gold"><Star size={10} fill="currentColor" />{item.rating.toFixed(1)}</span>
                {/if}
              </div>
            </div>
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if query.trim() && animeResults.length > 0}
    <section class="space-y-4">
      <h2 class="font-display text-lg md:text-xl font-semibold flex items-center gap-2">
        <Sparkles size={18} class="text-gold" />
        {$t('nav.anime')} <span class="text-sm font-normal text-zinc-500">({animeResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
        {#each animeResults as item, i (item.mal_id)}
          <button type="button" onclick={() => selectedAnime = item} class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer">
            <div class="aspect-[2/3] bg-ink-light overflow-hidden">
              {#if item.cover_url}
                <img src={item.cover_url} alt={item.title} loading={i < 6 ? 'eager' : 'lazy'} decoding="async" onerror={imgFallback} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-zinc-600"><Film size={48} /></div>
              {/if}
            </div>
            <div class="p-2 md:p-3">
              <p class="text-xs md:text-sm font-medium truncate">{item.title}</p>
              <div class="flex items-center justify-between mt-1">
                <span class="text-[10px] md:text-xs text-zinc-500">{item.year ?? '—'}</span>
                {#if item.average_score}
                  <span class="flex items-center gap-1 text-[10px] md:text-xs text-gold"><Star size={10} fill="currentColor" />{item.average_score.toFixed(1)}</span>
                {/if}
              </div>
            </div>
          </button>
        {/each}
      </div>
    </section>
  {/if}

  {#if query.trim() && !loading && tmdbResults.length === 0 && animeResults.length === 0 && localResults.length === 0 && !error}
    <p class="text-center text-zinc-500 mt-12">{$t('search.empty')}</p>
  {/if}
</div>

{#if selectedTmdb}
  <TitleModal item={selectedTmdb} onClose={() => { selectedTmdb = null; }} />
{/if}

{#if selectedAnime}
  <AnimeModal item={selectedAnime} onClose={() => { selectedAnime = null; }} />
{/if}