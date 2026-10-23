<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t, locale } from 'svelte-i18n';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { Search, Star, Library, Film, Sparkles } from 'lucide-svelte';
  import { settings } from '$lib/settings.svelte';
  import TitleModal from '$lib/components/TitleModal.svelte';
  import AnimeModal from '$lib/components/AnimeModal.svelte';
  import Spinner from '$lib/components/Spinner.svelte';

  type MovieItem = {
    tmdb_id: number; title: string; overview: string | null; poster_url: string | null;
    original_poster_url?: string | null;
    year: number | null; rating: number | null; media_type: string;
    original_language: string | null; genre_ids: number[];
  };

  type AnimeItem = {
    mal_id: number; title: string; title_original: string | null;
    cover_url: string | null; original_cover_url?: string | null;
    episodes: number | null; average_score: number | null;
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

  async function cacheCover(url: string | null): Promise<string | null> {
    if (!url) return null;
    if (!url.startsWith('http')) return url;
    try {
      return await invoke<string>('cache_cover', { url });
    } catch (e) {
      return url;
    }
  }

  function imgFallback(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    const orig = img.dataset.orig;
    if (orig && img.src !== orig) {
      img.src = orig;
      delete img.dataset.orig;
      return;
    }
    img.style.opacity = '0';
  }

  function imgLoaded(e: Event) {
    (e.currentTarget as HTMLImageElement).style.opacity = '1';
  }

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
      .then(res => { if (myGeneration === searchGeneration) localResults = res; })
      .catch(e => console.error(e));

    const tmdbPromise = invoke<MovieItem[]>('search_movies', {
      query: q, mediaType: 'multi', apiKey: settings.tmdbKey || null, locale: $locale,
    })
      .then(res => {
        if (myGeneration !== searchGeneration) return;
        const filtered = res.filter(item => !(item.original_language === 'ja' && item.genre_ids.includes(16)));
        tmdbResults = filtered;

        filtered.forEach(async (item) => {
          if (!item.poster_url) return;
          const originalUrl = item.poster_url.startsWith('http') 
            ? item.poster_url.replace('/w500/', '/w200/') 
            : TMDB_IMG + item.poster_url;
          
          const cached = await cacheCover(originalUrl);
          if (cached && myGeneration === searchGeneration) {
            tmdbResults = tmdbResults.map(it => it.tmdb_id === item.tmdb_id 
              ? { ...it, original_poster_url: originalUrl, poster_url: cached } 
              : it);
          }
        });
      })
      .catch(e => console.error(e));

    const animePromise = invoke<AnimeItem[]>('search_anime', { query: q, locale: $locale })
      .then(res => {
        if (myGeneration !== searchGeneration) return;
        animeResults = res;

        res.forEach(async (item) => {
          if (!item.cover_url) return;
          const originalUrl = item.cover_url;
          const cached = await cacheCover(originalUrl);
          if (cached && myGeneration === searchGeneration) {
            animeResults = animeResults.map(it => it.mal_id === item.mal_id 
              ? { ...it, original_cover_url: originalUrl, cover_url: cached } 
              : it);
          }
        });
      })
      .catch(e => console.error(e));

    await Promise.allSettled([localPromise, tmdbPromise, animePromise]);
    if (myGeneration === searchGeneration) loading = false;
  }

  function syncUrl() {
    const params = new URLSearchParams();
    if (query.trim()) params.set('q', query.trim());
    goto(params.toString() ? `/search?${params.toString()}` : '/search', { replaceState: true, noScroll: true });
  }

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
    if (lastLocale === '') { lastLocale = l; return; }
    if (lastLocale !== l) { lastLocale = l; if (query.trim()) runSearch(); }
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
        onkeydown={(e) => e.key === 'Enter' && syncUrl()}
      />
    </div>
    <button class="px-6 py-2.5 md:py-3 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink font-medium rounded-lg transition-all disabled:opacity-50 text-sm md:text-base shrink-0" onclick={syncUrl} disabled={loading}>
      {loading ? $t('search.loading') : $t('search.button')}
    </button>
  </div>

  {#if loading}
    <div class="flex items-center gap-3 px-4 py-3 rounded-xl bg-surface border border-gold/30">
      <Spinner size={28} />
      <div class="text-sm text-zinc-300 font-medium">{$t('search.loading')}</div>
    </div>
  {/if}

  {#if error}
    <div class="p-4 rounded-lg bg-red-950/50 border border-red-800 text-sm text-red-200"><pre class="whitespace-pre-wrap">{error}</pre></div>
  {/if}

  {#if query.trim() && localResults.length > 0}
    <section class="space-y-4">
      <h2 class="font-display text-lg md:text-xl font-semibold flex items-center gap-2">
        <Library size={18} class="text-gold" /> {$t('search.in_library')} <span class="text-sm font-normal text-zinc-500">({localResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
        {#each localResults as item (item.id)}
          {@const localSrc = item.poster_path
            ? (item.poster_path.startsWith('http') || item.poster_path.startsWith('data:')
              ? item.poster_path
              : TMDB_IMG + item.poster_path)
            : null}
          <a href={`/title/${item.tmdb_id}?type=${item.type_field}`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
            <div class="aspect-[2/3] bg-ink-light overflow-hidden relative">
              <div class="absolute inset-0 flex items-center justify-center text-zinc-600 pointer-events-none"><Film size={48} /></div>
              {#if localSrc}
                <img src={localSrc} alt={item.title} loading="eager" decoding="sync" referrerpolicy="no-referrer" onerror={imgFallback} onload={imgLoaded} class="absolute inset-0 w-full h-full object-cover z-10" />
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

  {#if query.trim() && (tmdbResults.length > 0 || loading)}
    <section class="space-y-4">
      <h2 class="font-display text-lg md:text-xl font-semibold flex items-center gap-2">
        <Search size={18} class="text-gold" /> TMDB {#if tmdbResults.length > 0}<span class="text-sm font-normal text-zinc-500">({tmdbResults.length})</span>{/if}
      </h2>
      {#if tmdbResults.length > 0}
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
          {#each tmdbResults as item (item.tmdb_id)}
            <button type="button" onclick={() => selectedTmdb = item} class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer">
              <div class="aspect-[2/3] bg-ink-light overflow-hidden relative">
                <div class="absolute inset-0 flex items-center justify-center text-zinc-600 pointer-events-none"><Film size={48} /></div>
                {#if item.poster_url}
                  <img src={item.poster_url} alt={item.title} loading="eager" decoding="sync" referrerpolicy="no-referrer" onerror={imgFallback} onload={imgLoaded} class="absolute inset-0 w-full h-full object-cover z-10 group-hover:scale-105 transition-transform duration-500" />
                {/if}
              </div>
              <div class="p-2 md:p-3">
                <p class="text-xs md:text-sm font-medium truncate">{item.title}</p>
                <div class="flex items-center justify-between mt-1">
                  <span class="text-[10px] md:text-xs text-zinc-500">{item.year ?? '—'}</span>
                  {#if item.rating}<span class="flex items-center gap-1 text-[10px] md:text-xs text-gold"><Star size={10} fill="currentColor" />{item.rating.toFixed(1)}</span>{/if}
                </div>
              </div>
            </button>
          {/each}
        </div>
      {:else}
        <div class="flex items-center justify-center py-8"><Spinner size={32} /></div>
      {/if}
    </section>
  {/if}

  {#if query.trim() && (animeResults.length > 0 || loading)}
    <section class="space-y-4">
      <h2 class="font-display text-lg md:text-xl font-semibold flex items-center gap-2">
        <Sparkles size={18} class="text-gold" /> {$t('nav.anime')} {#if animeResults.length > 0}<span class="text-sm font-normal text-zinc-500">({animeResults.length})</span>{/if}
      </h2>
      {#if animeResults.length > 0}
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
          {#each animeResults as item (item.mal_id)}
            <button type="button" onclick={() => selectedAnime = item} class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer">
              <div class="aspect-[2/3] bg-ink-light overflow-hidden relative">
                <div class="absolute inset-0 flex items-center justify-center text-zinc-600 pointer-events-none"><Film size={48} /></div>
                {#if item.cover_url}
                  <img src={item.cover_url} alt={item.title} loading="eager" decoding="sync" referrerpolicy="no-referrer" onerror={imgFallback} onload={imgLoaded} class="absolute inset-0 w-full h-full object-cover z-10 group-hover:scale-105 transition-transform duration-500" />
                {/if}
              </div>
              <div class="p-2 md:p-3">
                <p class="text-xs md:text-sm font-medium truncate">{item.title}</p>
                <div class="flex items-center justify-between mt-1">
                  <span class="text-[10px] md:text-xs text-zinc-500">{item.year ?? '—'}</span>
                  {#if item.average_score}<span class="flex items-center gap-1 text-[10px] md:text-xs text-gold"><Star size={10} fill="currentColor" />{item.average_score.toFixed(1)}</span>{/if}
                </div>
              </div>
            </button>
          {/each}
        </div>
      {:else}
        <div class="flex items-center justify-center py-8"><Spinner size={32} /></div>
      {/if}
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