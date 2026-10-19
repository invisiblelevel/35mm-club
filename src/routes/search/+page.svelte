<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t, locale } from 'svelte-i18n';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { afterNavigate } from '$app/navigation';
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
  let lastUrlQ = '';

  const TMDB_IMG = 'https://image.tmdb.org/t/p/w500';

  async function runSearch() {
    if (!query.trim()) {
      tmdbResults = []; animeResults = []; localResults = [];
      return;
    }
    loading = true;
    error = '';
    try {
      const localPromise = invoke<LocalItem[]>('search_local_library', { query, contentType: null }).catch(() => []);
      const tmdbPromise = invoke<MovieItem[]>('search_movies', { query, mediaType: 'multi', apiKey: settings.tmdbKey || null, locale: $locale }).catch(() => []);
      const animePromise = invoke<AnimeItem[]>('search_anime', { query, locale: $locale }).catch(() => []);

      const [local, tmdb, anime] = await Promise.all([localPromise, tmdbPromise, animePromise]);

      localResults = local;
      animeResults = anime;

      tmdbResults = tmdb.filter(item => {
        const isAnime = item.original_language === 'ja' && item.genre_ids.includes(16);
        return !isAnime;
      });
    } catch (e) {
      error = String(e);
    } finally {
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
    syncUrl();
    runSearch();
  }

  // Срабатывает при монтировании И при любой навигации (включая replaceState из топбара)
  afterNavigate(({ to }) => {
    if (!to || to.url.pathname !== '/search') return;
    const q = to.url.searchParams.get('q') ?? '';
    if (q === lastUrlQ) return;
    lastUrlQ = q;
    if (q !== query) {
      query = q;
      if (q) runSearch();
      else { tmdbResults = []; animeResults = []; localResults = []; }
    }
  });

  let lastLocale = $state('');
  $effect(() => {
    const l = $locale;
    if (lastLocale === '') { lastLocale = l; return; }
    if (lastLocale !== l) { lastLocale = l; if (query.trim()) runSearch(); }
  });
</script>

<div class="space-y-6">
  <h1 class="font-display text-3xl font-bold">{$t('search.title')}</h1>

  <div class="flex gap-2">
    <div class="relative flex-1">
      <Search size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500" />
      <input
        class="w-full pl-10 pr-4 py-3 bg-surface border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none transition-colors"
        placeholder={$t('search.placeholder')}
        bind:value={query}
        onkeydown={(e) => e.key === 'Enter' && onSearchClick()}
      />
    </div>
    <button
      class="px-8 py-3 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg transition-colors disabled:opacity-50"
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
      <h2 class="font-display text-xl font-semibold flex items-center gap-2">
        <Library size={18} class="text-gold" />
        {$t('search.in_library')} <span class="text-sm font-normal text-zinc-500">({localResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        {#each localResults as item (item.id)}
          <a href={`/title/${item.tmdb_id}?type=${item.type_field}`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
            {#if item.poster_path}
              <div class="aspect-[2/3] overflow-hidden">
                <img src={TMDB_IMG + item.poster_path} alt={item.title} class="w-full h-full object-cover" />
              </div>
            {:else}
              <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600"><Film size={48} /></div>
            {/if}
            <div class="p-3">
              <p class="text-sm font-medium truncate">{item.title}</p>
              <div class="flex items-center justify-between mt-1">
                <span class="text-xs text-zinc-500">{item.year ?? '—'}</span>
                <span class="text-[10px] px-1.5 py-0.5 rounded bg-gold/10 text-gold uppercase">{$t('status.' + item.status)}</span>
              </div>
            </div>
          </a>
        {/each}
      </div>
    </section>
  {/if}

  {#if query.trim() && tmdbResults.length > 0}
    <section class="space-y-4">
      <h2 class="font-display text-xl font-semibold flex items-center gap-2">
        <Search size={18} class="text-gold" />
        TMDB <span class="text-sm font-normal text-zinc-500">({tmdbResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        {#each tmdbResults as item (item.tmdb_id)}
          <button type="button" onclick={() => selectedTmdb = item} class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer">
            {#if item.poster_url}
              <div class="aspect-[2/3] overflow-hidden">
                <img src={item.poster_url.startsWith('http') ? item.poster_url : TMDB_IMG + item.poster_url} alt={item.title} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
              </div>
            {:else}
              <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600"><Film size={48} /></div>
            {/if}
            <div class="p-3">
              <p class="text-sm font-medium truncate">{item.title}</p>
              <div class="flex items-center justify-between mt-1">
                <span class="text-xs text-zinc-500">{item.year ?? '—'}</span>
                {#if item.rating}
                  <span class="flex items-center gap-1 text-xs text-gold"><Star size={12} fill="currentColor" />{item.rating.toFixed(1)}</span>
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
      <h2 class="font-display text-xl font-semibold flex items-center gap-2">
        <Sparkles size={18} class="text-gold" />
        Аниме <span class="text-sm font-normal text-zinc-500">({animeResults.length})</span>
      </h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        {#each animeResults as item (item.mal_id)}
          <button type="button" onclick={() => selectedAnime = item} class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer">
            {#if item.cover_url}
              <div class="aspect-[2/3] overflow-hidden">
                <img src={item.cover_url} alt={item.title} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
              </div>
            {:else}
              <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600"><Film size={48} /></div>
            {/if}
            <div class="p-3">
              <p class="text-sm font-medium truncate">{item.title}</p>
              <div class="flex items-center justify-between mt-1">
                <span class="text-xs text-zinc-500">{item.year ?? '—'}</span>
                {#if item.average_score}
                  <span class="flex items-center gap-1 text-xs text-gold"><Star size={12} fill="currentColor" />{item.average_score.toFixed(1)}</span>
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
  <TitleModal item={selectedTmdb} onClose={() => { selectedTmdb = null; runSearch(); }} />
{/if}

{#if selectedAnime}
  <AnimeModal item={selectedAnime} onClose={() => { selectedAnime = null; runSearch(); }} />
{/if}